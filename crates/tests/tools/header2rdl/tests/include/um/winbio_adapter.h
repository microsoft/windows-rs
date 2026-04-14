/*++

Copyright (c) 2008 Microsoft Corporation


Module Name:

    winbio_adapter.h

Abstract:

    Interface definitions used by WinBio Service plug-in
    components.


Environment:

    User mode.


--*/
#ifndef _WINBIO_ADAPTER_H_2C0E14E1_5330_4f60_9B4F_836CFFD7452A_
#define _WINBIO_ADAPTER_H_2C0E14E1_5330_4f60_9B4F_836CFFD7452A_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (NTDDI_VERSION >= NTDDI_WIN7)


///////////////////////////////////////////////////////////////////////////////
//
// Dependencies...
//
///////////////////////////////////////////////////////////////////////////////
#include "winbio_types.h"
#include "winbio_err.h"


#ifdef __cplusplus
extern "C"{
#endif

///////////////////////////////////////////////////////////////////////////////
//
// Types used throughout
//
///////////////////////////////////////////////////////////////////////////////
//
// Forward declarations of interface pointers
//
typedef struct _WINBIO_SENSOR_INTERFACE *PWINBIO_SENSOR_INTERFACE;
typedef struct _WINBIO_ENGINE_INTERFACE *PWINBIO_ENGINE_INTERFACE;
typedef struct _WINBIO_STORAGE_INTERFACE *PWINBIO_STORAGE_INTERFACE;

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

typedef struct _WINBIO_FRAMEWORK_INTERFACE *PWINBIO_FRAMEWORK_INTERFACE;

#endif

//
// Forward declarations of Adapter-private context structures
//
typedef struct _WINIBIO_SENSOR_CONTEXT *PWINIBIO_SENSOR_CONTEXT;
typedef struct _WINIBIO_ENGINE_CONTEXT *PWINIBIO_ENGINE_CONTEXT;
typedef struct _WINIBIO_STORAGE_CONTEXT *PWINIBIO_STORAGE_CONTEXT;

typedef ULONG WINBIO_HASH_TYPE, *PWINBIO_HASH_TYPE;

typedef ULONG WINBIO_SENSOR_CAPABILITIES, *PWINBIO_SENSOR_CAPABILITIES;
typedef ULONG WINBIO_ENGINE_CAPABILITIES, *PWINBIO_ENGINE_CAPABILITIES;
typedef ULONG WINBIO_STORAGE_CAPABILITIES, *PWINBIO_STORAGE_CAPABILITIES;

///////////////////////////////////////////////////////////////////////////////
//
// Contents of a single storage record. The Storage Adapter fills in one
// of these structure in response to a 'GetCurrentRecord' call.
//
// NOTE:
//  The memory pointed to by this structure belongs to the Storage Adapter
//  and is only valid until the pipeline executes its next database operation,
//  or until the pipeline is cleared.
//
///////////////////////////////////////////////////////////////////////////////
typedef struct _WINBIO_STORAGE_RECORD {
    PWINBIO_IDENTITY Identity;
    WINBIO_BIOMETRIC_SUBTYPE SubFactor;
    PULONG IndexVector;
    SIZE_T IndexElementCount;
    PUCHAR TemplateBlob;
    SIZE_T TemplateBlobSize;
    PUCHAR PayloadBlob;
    SIZE_T PayloadBlobSize;
} WINBIO_STORAGE_RECORD, *PWINBIO_STORAGE_RECORD;


///////////////////////////////////////////////////////////////////////////////
//
// Context structure passed up and down the Biometric Unit's plug-in
// component stack. Guaranteed to be unique per Biometric Unit, even
// if a single plug-in Adapter is supporting more than one Biometric
// Unit.
//
///////////////////////////////////////////////////////////////////////////////
typedef struct _WINBIO_PIPELINE {
    //
    // Handles to the hardware available to each
    // plug-in Adapter.
    //
    HANDLE SensorHandle;
    HANDLE EngineHandle;
    HANDLE StorageHandle;

    //
    // Pointers to interface dispatch tables for each
    // Adapter in the plug-in stack. These should be
    // considered read-only by the plug-in Adapters.
    //
    PWINBIO_SENSOR_INTERFACE    SensorInterface;
    PWINBIO_ENGINE_INTERFACE    EngineInterface;
    PWINBIO_STORAGE_INTERFACE   StorageInterface;

    //
    // Pointers to per-plug-in private data. Each private
    // data block is opaque to everyone except the Adapter
    // that owns it.
    //
    PWINIBIO_SENSOR_CONTEXT     SensorContext;
    PWINIBIO_ENGINE_CONTEXT     EngineContext;
    PWINIBIO_STORAGE_CONTEXT    StorageContext;

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

    PWINBIO_FRAMEWORK_INTERFACE FrameworkInterface;

#endif

} WINBIO_PIPELINE, *PWINBIO_PIPELINE;

///////////////////////////////////////////////////////////////////////////////
//
// Adapter types...
//
///////////////////////////////////////////////////////////////////////////////
typedef ULONG WINBIO_ADAPTER_TYPE, *PWINBIO_ADAPTER_TYPE;

#define WINBIO_ADAPTER_TYPE_SENSOR      ((WINBIO_ADAPTER_TYPE)1)
#define WINBIO_ADAPTER_TYPE_ENGINE      ((WINBIO_ADAPTER_TYPE)2)
#define WINBIO_ADAPTER_TYPE_STORAGE     ((WINBIO_ADAPTER_TYPE)3)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define WINBIO_ADAPTER_TYPE_FRAMEWORK   ((WINBIO_ADAPTER_TYPE)4)

#endif

///////////////////////////////////////////////////////////////////////////////
//
// Interface version control...
//
///////////////////////////////////////////////////////////////////////////////

typedef struct _WINBIO_ADAPTER_INTERFACE_VERSION
{
    USHORT MajorVersion;
    USHORT MinorVersion;
} WINBIO_ADAPTER_INTERFACE_VERSION, *PWINBIO_ADAPTER_INTERFACE_VERSION;

#define WINBIO_MAKE_INTERFACE_VERSION(major, minor) {(USHORT)major, (USHORT)minor}

#define WINBIO_IS_INTERFACE_VERSION_COMPATIBLE(loader, adapter)    \
    ((loader).MajorVersion >= (adapter).MajorVersion)

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// Sensor Adapter methods...
//
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// Methods available in V1.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_ATTACH_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_DETACH_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

// Clears out all Sensor-specific pipeline context
typedef HRESULT
(WINAPI *PIBIO_SENSOR_CLEAR_CONTEXT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_QUERY_STATUS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_SENSOR_STATUS Status
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_RESET_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_SET_MODE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_SENSOR_MODE Mode
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_SET_INDICATOR_STATUS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_INDICATOR_STATUS IndicatorStatus
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_GET_INDICATOR_STATUS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_INDICATOR_STATUS IndicatorStatus
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_START_CAPTURE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _Out_ LPOVERLAPPED *Overlapped
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_FINISH_CAPTURE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    );

//
// Export raw capture buffer
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_BIR *SampleBuffer,
    _Out_ PSIZE_T SampleSize
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_CANCEL_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

//
// Push current sample into the Engine and
// convert it into a feature set for use in
// additional processing.
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _In_ WINBIO_BIR_DATA_FLAGS Flags,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_CONTROL_UNIT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// Additional methods available in V2.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
// Additional methods available in V3.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_PIPELINE_INIT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_PIPELINE_CLEANUP_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_ACTIVATE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_DEACTIVATE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(SensorInfoSize) PWINBIO_EXTENDED_SENSOR_INFO SensorInfo,
    _In_ SIZE_T SensorInfoSize
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_to_(FormatArraySize, *FormatCount) PWINBIO_UUID FormatArray,
    _In_ SIZE_T FormatArraySize,
    _Out_ PSIZE_T FormatCount
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID Format
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(CalibrationBufferSize) PUCHAR CalibrationBuffer,
    _In_ SIZE_T CalibrationBufferSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
//
// Additional methods available in V4.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_opt_(RawBufferSize) PUCHAR RawBufferAddress,
    _In_ SIZE_T RawBufferSize,
    _Outptr_result_bytebuffer_maybenull_(*ResultBufferSize) PUCHAR *ResultBufferAddress,
    _Out_opt_ SIZE_T *ResultBufferSize
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ GUID SecureBufferIdentifier,
    _In_reads_bytes_opt_(MetadataBufferSize) PUCHAR MetadataBufferAddress,
    _In_ SIZE_T MetadataBufferSize,
    _Outptr_result_bytebuffer_maybenull_(*ResultBufferSize) PUCHAR *ResultBufferAddress,
    _Out_opt_ SIZE_T *ResultBufferSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
//
// Additional methods available in V5.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_to_(TypeInfoBufferSize, *TypeInfoDataSize) PUCHAR TypeInfoBufferAddress,
    _In_ SIZE_T TypeInfoBufferSize,
    _Out_ SIZE_T *TypeInfoDataSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
//
// Additional methods available in V6.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_SENSOR_CONNECT_SECURE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ const WINBIO_SECURE_CONNECTION_PARAMS* ConnectionParams,
    _Outptr_ WINBIO_SECURE_CONNECTION_DATA** ConnectionData
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_START_CAPTURE_EX_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _In_reads_bytes_opt_(NonceSize) const UCHAR* Nonce,
    _In_ SIZE_T NonceSize,
    _In_ WINBIO_BIR_DATA_FLAGS Flags,
    _Out_ OVERLAPPED** Overlapped
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_START_NOTIFY_WAKE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ OVERLAPPED** Overlapped
    );

typedef HRESULT
(WINAPI *PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ WINBIO_WAKE_REASON* Reason
    );

#endif

///////////////////////////////////////////////////////////////////////////////
//
// Sensor Adapter interface table.
//
///////////////////////////////////////////////////////////////////////////////
//
// Available interface versions...
//
#define WINBIO_SENSOR_INTERFACE_VERSION_1    WINBIO_MAKE_INTERFACE_VERSION(1,0)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define WINBIO_SENSOR_INTERFACE_VERSION_2    WINBIO_MAKE_INTERFACE_VERSION(2,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define WINBIO_SENSOR_INTERFACE_VERSION_3    WINBIO_MAKE_INTERFACE_VERSION(3,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define WINBIO_SENSOR_INTERFACE_VERSION_4    WINBIO_MAKE_INTERFACE_VERSION(4,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define WINBIO_SENSOR_INTERFACE_VERSION_5    WINBIO_MAKE_INTERFACE_VERSION(5,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#define WINBIO_SENSOR_INTERFACE_VERSION_6    WINBIO_MAKE_INTERFACE_VERSION(6,0)
#endif

typedef struct _WINBIO_SENSOR_INTERFACE {
    WINBIO_ADAPTER_INTERFACE_VERSION            Version;
    WINBIO_ADAPTER_TYPE                         Type;
    SIZE_T                                      Size;
    GUID                                        AdapterId;

    PIBIO_SENSOR_ATTACH_FN                      Attach;
    PIBIO_SENSOR_DETACH_FN                      Detach;

    PIBIO_SENSOR_CLEAR_CONTEXT_FN               ClearContext;

    PIBIO_SENSOR_QUERY_STATUS_FN                QueryStatus;
    PIBIO_SENSOR_RESET_FN                       Reset;
    PIBIO_SENSOR_SET_MODE_FN                    SetMode;

    PIBIO_SENSOR_SET_INDICATOR_STATUS_FN        SetIndicatorStatus;
    PIBIO_SENSOR_GET_INDICATOR_STATUS_FN        GetIndicatorStatus;

    PIBIO_SENSOR_START_CAPTURE_FN               StartCapture;
    PIBIO_SENSOR_FINISH_CAPTURE_FN              FinishCapture;
    PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN          ExportSensorData;
    PIBIO_SENSOR_CANCEL_FN                      Cancel;

    PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN         PushDataToEngine;

    PIBIO_SENSOR_CONTROL_UNIT_FN                ControlUnit;
    PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN     ControlUnitPrivileged;

#if (NTDDI_VERSION >= NTDDI_WIN8)
    //
    // V2.0 methods begin here...
    //
    PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN         NotifyPowerChange;
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
    //
    // V3.0 methods begin here...
    //
    PIBIO_SENSOR_PIPELINE_INIT_FN               PipelineInit;
    PIBIO_SENSOR_PIPELINE_CLEANUP_FN            PipelineCleanup;
    PIBIO_SENSOR_ACTIVATE_FN                    Activate;
    PIBIO_SENSOR_DEACTIVATE_FN                  Deactivate;
    PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN         QueryExtendedInfo;
    PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN   QueryCalibrationFormats;
    PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN      SetCalibrationFormat;
    PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN     AcceptCalibrationData;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
    //
    // V4.0 methods begin here...
    //
    PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN     AsyncImportRawBuffer;
    PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN  AsyncImportSecureBuffer;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
    //
    // V5.0 methods begin here...
    //
    PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN   QueryPrivateSensorType;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
    //
    // V6.0 methods begin here...
    //
    PIBIO_SENSOR_CONNECT_SECURE_FN              ConnectSecure;
    PIBIO_SENSOR_START_CAPTURE_EX_FN            StartCaptureEx;
    PIBIO_SENSOR_START_NOTIFY_WAKE_FN           StartNotifyWake;
    PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN          FinishNotifyWake;
#endif
} WINBIO_SENSOR_INTERFACE, *PWINBIO_SENSOR_INTERFACE;

//
// Interface-retrieval function exported by the Sensor Adapter
// plug-in DLL. It *MUST* be called 'WbioQuerySensorInterface'.
//
HRESULT
WINAPI
WbioQuerySensorInterface(
    _Out_ PWINBIO_SENSOR_INTERFACE *SensorInterface
    );

//
// Pointer to an interface-retrieval function used by the
// framework after a 'GetProcAddress' call.
//
typedef HRESULT
(WINAPI *PWINBIO_QUERY_SENSOR_INTERFACE_FN)(
    _Out_ PWINBIO_SENSOR_INTERFACE *SensorInterface
    );

#define WINBIO_QUERY_SENSOR_INTERFACE_FN_NAME   ("WbioQuerySensorInterface")


///////////////////////////////////////////////////////////////////////////////
//
// Inline functions for calling Sensor Adapter methods
//
///////////////////////////////////////////////////////////////////////////////
inline HRESULT
WbioSensorAttach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->Attach))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->Attach(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorDetach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->Detach))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->Detach(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorClearContext(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->ClearContext))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->ClearContext(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorQueryStatus(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_SENSOR_STATUS Status
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->QueryStatus))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->QueryStatus(Pipeline, Status);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorReset(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->Reset))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->Reset(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorSetMode(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_SENSOR_MODE Mode
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->SetMode))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->SetMode(Pipeline, Mode);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorStartCapture(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _Out_ LPOVERLAPPED *Overlapped
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->StartCapture))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->StartCapture(
                                            Pipeline,
                                            Purpose,
                                            Overlapped
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorFinishCapture(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->FinishCapture))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->FinishCapture(
                                            Pipeline,
                                            RejectDetail
                                            );
    }
}
//-----------------------------------------------------------------------------

//
// Export raw capture buffer
//
inline HRESULT
WbioSensorExportSensorData(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_BIR *SampleBuffer,
    _Out_ PSIZE_T SampleSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->ExportSensorData))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->ExportSensorData(
                                            Pipeline,
                                            SampleBuffer,
                                            SampleSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorCancel(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->Cancel))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->Cancel(Pipeline);
    }
}
//-----------------------------------------------------------------------------

//
// Push current sample into the Engine and
// convert it into a feature set for use in
// additional processing.
//
inline HRESULT
WbioSensorPushDataToEngine(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _In_ WINBIO_BIR_DATA_FLAGS Flags,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->PushDataToEngine))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->PushDataToEngine(
                                            Pipeline,
                                            Purpose,
                                            Flags,
                                            RejectDetail
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorControlUnit(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->ControlUnit))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->ControlUnit(
                                            Pipeline,
                                            ControlCode,
                                            SendBuffer,
                                            SendBufferSize,
                                            ReceiveBuffer,
                                            ReceiveBufferSize,
                                            ReceiveDataSize,
                                            OperationStatus
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorControlUnitPrivileged(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->ControlUnitPrivileged))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->ControlUnitPrivileged(
                                            Pipeline,
                                            ControlCode,
                                            SendBuffer,
                                            SendBufferSize,
                                            ReceiveBuffer,
                                            ReceiveBufferSize,
                                            ReceiveDataSize,
                                            OperationStatus
                                            );
    }
}
//-----------------------------------------------------------------------------

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// V2.0 methods begin here...
//

inline HRESULT
WbioSensorNotifyPowerChange(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->NotifyPowerChange))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->NotifyPowerChange(
                                            Pipeline,
                                            PowerEventType
                                            );
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
// V3.0 methods begin here...
//

inline HRESULT
WbioSensorPipelineInit(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->PipelineInit))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->PipelineInit(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorPipelineCleanup(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->PipelineCleanup))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->PipelineCleanup(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorActivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->Activate))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->Activate(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorDeactivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->Deactivate))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->Deactivate(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorQueryExtendedInfo(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(SensorInfoSize) PWINBIO_EXTENDED_SENSOR_INFO SensorInfo,
    _In_ SIZE_T SensorInfoSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface) ||
        !ARGUMENT_PRESENT(SensorInfo))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->QueryExtendedInfo))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->QueryExtendedInfo(
                                            Pipeline,
                                            SensorInfo,
                                            SensorInfoSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorQueryCalibrationFormats(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_to_(FormatArraySize, *FormatCount) PWINBIO_UUID FormatArray,
    _In_ SIZE_T FormatArraySize,
    _Out_ PSIZE_T FormatCount
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface) ||
        !ARGUMENT_PRESENT(FormatArray) ||
        !ARGUMENT_PRESENT(FormatCount))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->QueryCalibrationFormats))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->QueryCalibrationFormats(
                                            Pipeline,
                                            FormatArray,
                                            FormatArraySize,
                                            FormatCount
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorSetCalibrationFormat(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID Format
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface) ||
        !ARGUMENT_PRESENT(Format))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->SetCalibrationFormat))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->SetCalibrationFormat(
                                            Pipeline,
                                            Format
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorAcceptCalibrationData(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(CalibrationBufferSize) PUCHAR CalibrationBuffer,
    _In_ SIZE_T CalibrationBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface) ||
        !ARGUMENT_PRESENT(CalibrationBuffer))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->AcceptCalibrationData))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->AcceptCalibrationData(
                                            Pipeline,
                                            CalibrationBuffer,
                                            CalibrationBufferSize
                                            );
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
//
// V4.0 methods begin here...
//

inline HRESULT
WbioSensorAsyncImportRawBuffer(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_opt_(RawBufferSize) PUCHAR RawBufferAddress,
    _In_ SIZE_T RawBufferSize,
    _Outptr_result_bytebuffer_maybenull_(*ResultBufferSize) PUCHAR *ResultBufferAddress,
    _Out_opt_ SIZE_T *ResultBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->AsyncImportRawBuffer))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->AsyncImportRawBuffer(
                                            Pipeline,
                                            RawBufferAddress,
                                            RawBufferSize,
                                            ResultBufferAddress,
                                            ResultBufferSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorAsyncImportSecureBuffer(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ GUID SecureBufferIdentifier,
    _In_reads_bytes_opt_(MetadataBufferSize) PUCHAR MetadataBufferAddress,
    _In_ SIZE_T MetadataBufferSize,
    _Outptr_result_bytebuffer_maybenull_(*ResultBufferSize) PUCHAR *ResultBufferAddress,
    _Out_opt_ SIZE_T *ResultBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->AsyncImportSecureBuffer))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->AsyncImportSecureBuffer(
                                            Pipeline,
                                            SecureBufferIdentifier,
                                            MetadataBufferAddress,
                                            MetadataBufferSize,
                                            ResultBufferAddress,
                                            ResultBufferSize
                                            );
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
//
// V5.0 methods begin here...
//

inline HRESULT
WbioSensorQueryPrivateSensorType(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_to_(TypeInfoBufferSize, *TypeInfoDataSize) PUCHAR TypeInfoBufferAddress,
    _In_ SIZE_T TypeInfoBufferSize,
    _Out_ SIZE_T *TypeInfoDataSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->QueryPrivateSensorType))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->QueryPrivateSensorType(
                                            Pipeline,
                                            TypeInfoBufferAddress,
                                            TypeInfoBufferSize,
                                            TypeInfoDataSize
                                            );
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
//
// V6.0 methods begin here...
//

inline HRESULT
WbioSensorConnectSecure(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ const WINBIO_SECURE_CONNECTION_PARAMS* ConnectionParams,
    _Outptr_ WINBIO_SECURE_CONNECTION_DATA** ConnectionData
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->ConnectSecure))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->ConnectSecure(
                                            Pipeline,
                                            ConnectionParams,
                                            ConnectionData
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorStartCaptureEx(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _In_reads_bytes_opt_(NonceSize) const UCHAR* Nonce,
    _In_ SIZE_T NonceSize,
    _In_ WINBIO_BIR_DATA_FLAGS Flags,
    _Out_ OVERLAPPED** Overlapped
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->StartCaptureEx))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->StartCaptureEx(
                                            Pipeline,
                                            Purpose,
                                            Nonce,
                                            NonceSize,
                                            Flags,
                                            Overlapped
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorStartNotifyWake(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ OVERLAPPED** Overlapped
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->StartNotifyWake))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->StartNotifyWake(Pipeline, Overlapped);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioSensorFinishNotifyWake(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ WINBIO_WAKE_REASON* Reason
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->SensorInterface->FinishNotifyWake))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->SensorInterface->FinishNotifyWake(Pipeline, Reason);
    }
}
//-----------------------------------------------------------------------------

#endif

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// Engine Adapter methods...
//
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// Methods available in V1.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_ENGINE_ATTACH_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_DETACH_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

// Clears out all Engine-specific pipeline context
typedef HRESULT
(WINAPI *PIBIO_ENGINE_CLEAR_CONTEXT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

// 'QUERY_PREFERRED_FORMAT' - SENSOR ASKS ENGINE WHAT DATA
// FORMAT THE ENGINE PREFERS TO RECEIVE FROM THE SENSOR
typedef HRESULT
(WINAPI *PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REGISTERED_FORMAT StandardFormat,
    _Out_ PWINBIO_UUID VendorFormat
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T IndexElementCount
    );

//
// RETURN AN ARRAY OF UTF-8 OIDS FOR HASH
// ALGS SUPPORTED BY THE ENGINE
//
typedef HRESULT
(WINAPI *PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T AlgorithmCount,
    _Out_ PSIZE_T AlgorithmBufferSize,
    _Outptr_result_bytebuffer_all_(*AlgorithmBufferSize) _At_(*AlgorithmBuffer, _Post_ _NullNull_terminated_) PUCHAR *AlgorithmBuffer
    );

//
// SELECT ONE OID FOR THE ENGINE TO USE
//
typedef HRESULT
(WINAPI *PIBIO_ENGINE_SET_HASH_ALGORITHM_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ SIZE_T AlgorithmBufferSize,
    _In_reads_z_(AlgorithmBufferSize) PUCHAR AlgorithmBuffer
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T SampleHint
    );


typedef HRESULT
(WINAPI *PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(SampleSize) PWINBIO_BIR SampleBuffer,
    _In_ SIZE_T SampleSize,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    );

// CAN RETURN:
//      'FEATURE SET' - AFTER A PUSH FROM THE SENSOR - WINBIO_DATA_FLAG_INTERMEDIATE
//      'ENROLLMENT TEMPLATE' - AFTER AN ENROLLMENT CYCLE - WINBIO_DATA_FLAG_PROCESSED
//      'MATCHING TEMPLATE' - AFTER AN IDENTIFY OR VERIFY OPERATION - WINBIO_DATA_FLAG_PROCESSED
//
typedef HRESULT
(WINAPI *PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_DATA_FLAGS Flags,
    _Outptr_result_bytebuffer_all_(*SampleSize) PWINBIO_BIR *SampleBuffer,
    _Out_ PSIZE_T SampleSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_VERIFY_FEATURE_SET_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Out_ PBOOLEAN Match,
    _Outptr_result_bytebuffer_all_(*PayloadBlobSize) PUCHAR *PayloadBlob,
    _Out_ PSIZE_T PayloadBlobSize,
    _Outptr_result_bytebuffer_all_(*HashSize) PUCHAR *HashValue,
    _Out_ PSIZE_T HashSize,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_IDENTITY Identity,
    _Out_ PWINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Outptr_result_bytebuffer_all_(*PayloadBlobSize) PUCHAR *PayloadBlob,
    _Out_ PSIZE_T PayloadBlobSize,
    _Outptr_result_bytebuffer_all_(*HashSize) PUCHAR *HashValue,
    _Out_ PSIZE_T HashSize,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_CREATE_ENROLLMENT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_UPDATE_ENROLLMENT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Outptr_result_bytebuffer_all_(*HashSize) PUCHAR *HashValue,
    _Out_ PSIZE_T HashSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_IDENTITY Identity,
    _Out_ PWINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Out_ PBOOLEAN Duplicate
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_COMMIT_ENROLLMENT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _In_reads_bytes_(PayloadBlobSize) PUCHAR PayloadBlob,
    _In_ SIZE_T PayloadBlobSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_DISCARD_ENROLLMENT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_CONTROL_UNIT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// Additional methods available in V2.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_RESERVED_1_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_IDENTITY Identity
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
// Additional methods available in V3.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_ENGINE_PIPELINE_INIT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_PIPELINE_CLEANUP_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_ACTIVATE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_DEACTIVATE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(EngineInfoSize) PWINBIO_EXTENDED_ENGINE_INFO EngineInfo,
    _In_ SIZE_T EngineInfoSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_IDENTIFY_ALL_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T PresenceCount,
    _Out_ PWINBIO_PRESENCE *PresenceArray
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONGLONG SelectorValue
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_EXTENDED_ENROLLMENT_PARAMETERS Parameters
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(EnrollmentStatusSize) PWINBIO_EXTENDED_ENROLLMENT_STATUS EnrollmentStatus,
    _In_ SIZE_T EnrollmentStatusSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_REFRESH_CACHE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(FormatCount) PWINBIO_UUID FormatArray,
    _In_ SIZE_T FormatCount,
    _Out_ PWINBIO_UUID SelectedFormat,
    _Out_ PSIZE_T MaxBufferSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PBOOLEAN DiscardAndRepeatCapture,
    _Out_writes_bytes_to_(MaxBufferSize, *CalibrationBufferSize) PUCHAR CalibrationBuffer,
    _Out_ PSIZE_T CalibrationBufferSize,
    _In_ SIZE_T MaxBufferSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(PolicyItemCount) PWINBIO_ACCOUNT_POLICY PolicyItemArray,
    _In_ SIZE_T PolicyItemCount
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
//
// Additional methods available in V4.0 and later
//

typedef HRESULT
(WINAPI *PIBIO_ENGINE_CREATE_KEY_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(KeySize) const UCHAR* Key,
    _In_ SIZE_T KeySize,
    _Out_writes_bytes_to_(KeyIdentifierSize, *ResultSize) PUCHAR KeyIdentifier,
    _In_ SIZE_T KeyIdentifierSize,
    _Out_ PSIZE_T ResultSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(NonceSize) const UCHAR* Nonce,
    _In_ SIZE_T NonceSize,
    _In_reads_(KeyIdentifierSize) const UCHAR* KeyIdentifier,
    _In_ SIZE_T KeyIdentifierSize,
    _Out_ PWINBIO_IDENTITY Identity,
    _Out_ PWINBIO_BIOMETRIC_SUBTYPE SubFactor,
	_Out_ PWINBIO_REJECT_DETAIL RejectDetail,
    _Outptr_result_bytebuffer_(*AuthorizationSize) PUCHAR *Authorization,
    _Out_ PSIZE_T AuthorizationSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
//
// Additional methods available in V5.0 and later
//

typedef HRESULT
(WINAPI *PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(TypeInfoBufferSize) const UCHAR* TypeInfoBufferAddress,
    _In_ SIZE_T TypeInfoBufferSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
//
// Additional methods available in V6.0 and later
//

typedef HRESULT
(WINAPI *PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Outptr_result_bytebuffer_(*NonceSize) PUCHAR* Nonce,
    _Out_ SIZE_T* NonceSize
    );

typedef HRESULT
(WINAPI *PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(NonceSize) const UCHAR* Nonce,
    _In_ SIZE_T NonceSize,
    _Out_ WINBIO_IDENTITY* Identity,
    _Out_ WINBIO_BIOMETRIC_SUBTYPE* SubFactor,
    _Out_ WINBIO_REJECT_DETAIL* RejectDetail,
    _Outptr_result_bytebuffer_(*AuthenticationSize) PUCHAR* Authentication,
    _Out_ SIZE_T* AuthenticationSize
    );

#endif

///////////////////////////////////////////////////////////////////////////////
//
// Engine Adapter interface table.
//
///////////////////////////////////////////////////////////////////////////////
//
// Available interface versions...
//
#define WINBIO_ENGINE_INTERFACE_VERSION_1   WINBIO_MAKE_INTERFACE_VERSION(1,0)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define WINBIO_ENGINE_INTERFACE_VERSION_2   WINBIO_MAKE_INTERFACE_VERSION(2,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define WINBIO_ENGINE_INTERFACE_VERSION_3   WINBIO_MAKE_INTERFACE_VERSION(3,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define WINBIO_ENGINE_INTERFACE_VERSION_4   WINBIO_MAKE_INTERFACE_VERSION(4,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define WINBIO_ENGINE_INTERFACE_VERSION_5   WINBIO_MAKE_INTERFACE_VERSION(5,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#define WINBIO_ENGINE_INTERFACE_VERSION_6   WINBIO_MAKE_INTERFACE_VERSION(6,0)
#endif

typedef struct _WINBIO_ENGINE_INTERFACE {
    WINBIO_ADAPTER_INTERFACE_VERSION            Version;
    WINBIO_ADAPTER_TYPE                         Type;
    SIZE_T                                      Size;
    GUID                                        AdapterId;

    PIBIO_ENGINE_ATTACH_FN                      Attach;
    PIBIO_ENGINE_DETACH_FN                      Detach;

    PIBIO_ENGINE_CLEAR_CONTEXT_FN               ClearContext;

    PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN      QueryPreferredFormat;
    PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN     QueryIndexVectorSize;
    PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN       QueryHashAlgorithms;
    PIBIO_ENGINE_SET_HASH_ALGORITHM_FN          SetHashAlgorithm;

    PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN           QuerySampleHint;

    PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN          AcceptSampleData;       // PROCESSES CURRENT BUFFER FROM PIPELINE AND GENERATES A FEATURE SET IN THE PIPELINE
    PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN          ExportEngineData;       // EXPORTS FEATURE SET OR TEMPLATE

    PIBIO_ENGINE_VERIFY_FEATURE_SET_FN          VerifyFeatureSet;
    PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN        IdentifyFeatureSet;

    PIBIO_ENGINE_CREATE_ENROLLMENT_FN           CreateEnrollment;       // ATTACHES AN EMPTY ENROLLMENT TEMPLATE TO THE PIPELINE
    PIBIO_ENGINE_UPDATE_ENROLLMENT_FN           UpdateEnrollment;       // CONVERTS CURRENT PIPELINE FEATURE SET INTO SOMETHING THAT CAN BE ADDED TO A TEMPLATE
    PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN       GetEnrollmentStatus;    // QUERIES TEMPLATE ATTACHED TO THE PIPELINE TO SEE IF IT IS READY TO COMMIT
    PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN         GetEnrollmentHash;
    PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN         CheckForDuplicate;      // DETERMINES WHETHER TEMPLATE IS ALREADY ENROLLED
    PIBIO_ENGINE_COMMIT_ENROLLMENT_FN           CommitEnrollment;
    PIBIO_ENGINE_DISCARD_ENROLLMENT_FN          DiscardEnrollment;

    PIBIO_ENGINE_CONTROL_UNIT_FN                ControlUnit;
    PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN     ControlUnitPrivileged;

#if (NTDDI_VERSION >= NTDDI_WIN8)
    //
    // V2.0 methods begin here...
    //
    PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN         NotifyPowerChange;
    PIBIO_ENGINE_RESERVED_1_FN                  Reserved_1;
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
    //
    // V3.0 methods begin here...
    //
    PIBIO_ENGINE_PIPELINE_INIT_FN                       PipelineInit;
    PIBIO_ENGINE_PIPELINE_CLEANUP_FN                    PipelineCleanup;
    PIBIO_ENGINE_ACTIVATE_FN                            Activate;
    PIBIO_ENGINE_DEACTIVATE_FN                          Deactivate;
    PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN                 QueryExtendedInfo;
    PIBIO_ENGINE_IDENTIFY_ALL_FN                        IdentifyAll;
    PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN             SetEnrollmentSelector;
    PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN           SetEnrollmentParameters;
    PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN    QueryExtendedEnrollmentStatus;
    PIBIO_ENGINE_REFRESH_CACHE_FN                       RefreshCache;
    PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN           SelectCalibrationFormat;
    PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN              QueryCalibrationData;
    PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN                  SetAccountPolicy;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
    //
    // V4.0 methods begin here...
    //
    PIBIO_ENGINE_CREATE_KEY_FN                     CreateKey;
    PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN    IdentifyFeatureSetSecure;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
    //
    // V5.0 methods begin here...
    //
    PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN AcceptPrivateSensorTypeInfo;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
    //
    // V6.0 methods begin here...
    //
    PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN    CreateEnrollmentAuthenticated;
    PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN IdentifyFeatureSetAuthenticated;
#endif

} WINBIO_ENGINE_INTERFACE, *PWINBIO_ENGINE_INTERFACE;

//
// Interface-retrieval function exported by the Engine Adapter
// plug-in DLL. It *MUST* be called 'WbioQueryEngineInterface'.
//
HRESULT
WINAPI
WbioQueryEngineInterface(
    _Out_ PWINBIO_ENGINE_INTERFACE *EngineInterface
    );

//
// Pointer to an interface-retrieval function used by the
// framework after a 'GetProcAddress' call.
//
typedef HRESULT
(WINAPI *PWINBIO_QUERY_ENGINE_INTERFACE_FN)(
    _Out_ PWINBIO_ENGINE_INTERFACE *EngineInterface
    );

#define WINBIO_QUERY_ENGINE_INTERFACE_FN_NAME   ("WbioQueryEngineInterface")


///////////////////////////////////////////////////////////////////////////////
//
// Inline functions for calling Engine Adapter methods
//
///////////////////////////////////////////////////////////////////////////////
inline HRESULT
WbioEngineAttach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->Attach))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->Attach(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineDetach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->Detach))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->Detach(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineClearContext(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->ClearContext))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->ClearContext(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineQueryPreferredFormat(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REGISTERED_FORMAT StandardFormat,
    _Out_ PWINBIO_UUID VendorFormat
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->QueryPreferredFormat))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->QueryPreferredFormat(
                                            Pipeline,
                                            StandardFormat,
                                            VendorFormat
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineQueryIndexVectorSize(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T IndexElementCount
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->QueryIndexVectorSize))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->QueryIndexVectorSize(
                                            Pipeline,
                                            IndexElementCount
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineQueryHashAlgorithms(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T AlgorithmCount,
    _Out_ PSIZE_T AlgorithmBufferSize,
    _Outptr_result_bytebuffer_all_(*AlgorithmBufferSize) _At_(*AlgorithmBuffer, _Post_ _NullNull_terminated_) PUCHAR *AlgorithmBuffer
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->QueryHashAlgorithms))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->QueryHashAlgorithms(
                                            Pipeline,
                                            AlgorithmCount,
                                            AlgorithmBufferSize,
                                            AlgorithmBuffer
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineSetHashAlgorithm(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ SIZE_T AlgorithmBufferSize,
    _In_reads_z_(AlgorithmBufferSize) PUCHAR AlgorithmBuffer
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->SetHashAlgorithm))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->SetHashAlgorithm(
                                            Pipeline,
                                            AlgorithmBufferSize,
                                            AlgorithmBuffer
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineQuerySampleHint(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T SampleHint
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->QuerySampleHint))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->QuerySampleHint(
                                            Pipeline,
                                            SampleHint
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineAcceptSampleData(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(SampleSize) PWINBIO_BIR SampleBuffer,
    _In_ SIZE_T SampleSize,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->AcceptSampleData))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->AcceptSampleData(
                                            Pipeline,
                                            SampleBuffer,
                                            SampleSize,
                                            Purpose,
                                            RejectDetail
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineExportEngineData(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_DATA_FLAGS Flags,
    _Outptr_result_bytebuffer_all_(*SampleSize) PWINBIO_BIR *SampleBuffer,
    _Out_ PSIZE_T SampleSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->ExportEngineData))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->ExportEngineData(
                                            Pipeline,
                                            Flags,
                                            SampleBuffer,
                                            SampleSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineVerifyFeatureSet(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Out_ PBOOLEAN Match,
    _Outptr_result_bytebuffer_all_(*PayloadBlobSize) PUCHAR *PayloadBlob,
    _Out_ PSIZE_T PayloadBlobSize,
    _Outptr_result_bytebuffer_all_(*HashSize) PUCHAR *HashValue,
    _Out_ PSIZE_T HashSize,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->VerifyFeatureSet))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->VerifyFeatureSet(
                                            Pipeline,
                                            Identity,
                                            SubFactor,
                                            Match,
                                            PayloadBlob,
                                            PayloadBlobSize,
                                            HashValue,
                                            HashSize,
                                            RejectDetail
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineIdentifyFeatureSet(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_IDENTITY Identity,
    _Out_ PWINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Outptr_result_bytebuffer_all_(*PayloadBlobSize) PUCHAR *PayloadBlob,
    _Out_ PSIZE_T PayloadBlobSize,
    _Outptr_result_bytebuffer_all_(*HashSize) PUCHAR *HashValue,
    _Out_ PSIZE_T HashSize,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->IdentifyFeatureSet))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->IdentifyFeatureSet(
                                            Pipeline,
                                            Identity,
                                            SubFactor,
                                            PayloadBlob,
                                            PayloadBlobSize,
                                            HashValue,
                                            HashSize,
                                            RejectDetail
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineCreateEnrollment(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->CreateEnrollment))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->CreateEnrollment(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineUpdateEnrollment(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->UpdateEnrollment))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->UpdateEnrollment(
                                            Pipeline,
                                            RejectDetail
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineGetEnrollmentStatus(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->GetEnrollmentStatus))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->GetEnrollmentStatus(
                                            Pipeline,
                                            RejectDetail
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineGetEnrollmentHash(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Outptr_result_bytebuffer_all_(*HashSize) PUCHAR *HashValue,
    _Out_ PSIZE_T HashSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->GetEnrollmentHash))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->GetEnrollmentHash(
                                            Pipeline,
                                            HashValue,
                                            HashSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineCheckForDuplicate(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_IDENTITY Identity,
    _Out_ PWINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Out_ PBOOLEAN Duplicate
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->CheckForDuplicate))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->CheckForDuplicate(
                                            Pipeline,
                                            Identity,
                                            SubFactor,
                                            Duplicate
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineCommitEnrollment(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _In_reads_bytes_(PayloadBlobSize) PUCHAR PayloadBlob,
    _In_ SIZE_T PayloadBlobSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->CommitEnrollment))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->CommitEnrollment(
                                            Pipeline,
                                            Identity,
                                            SubFactor,
                                            PayloadBlob,
                                            PayloadBlobSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineDiscardEnrollment(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->DiscardEnrollment))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->DiscardEnrollment(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineControlUnit(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->ControlUnit))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->ControlUnit(
                                            Pipeline,
                                            ControlCode,
                                            SendBuffer,
                                            SendBufferSize,
                                            ReceiveBuffer,
                                            ReceiveBufferSize,
                                            ReceiveDataSize,
                                            OperationStatus
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineControlUnitPrivileged(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->ControlUnitPrivileged))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->ControlUnitPrivileged(
                                            Pipeline,
                                            ControlCode,
                                            SendBuffer,
                                            SendBufferSize,
                                            ReceiveBuffer,
                                            ReceiveBufferSize,
                                            ReceiveDataSize,
                                            OperationStatus
                                            );
    }
}
//-----------------------------------------------------------------------------

#if (NTDDI_VERSION >= NTDDI_WIN8)

inline HRESULT
WbioEngineNotifyPowerChange(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->NotifyPowerChange))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->NotifyPowerChange(
                                            Pipeline,
                                            PowerEventType
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineReserved_1(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_IDENTITY Identity
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface) ||
        !ARGUMENT_PRESENT(Identity))
    {
        return E_POINTER;
    }
    else
    {
        return E_NOTIMPL;
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
// V3.0 methods begin here...
//

inline HRESULT
WbioEnginePipelineInit(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->PipelineInit))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->PipelineInit(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEnginePipelineCleanup(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->PipelineCleanup))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->PipelineCleanup(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineActivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->Activate))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->Activate(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineDeactivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->Deactivate))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->Deactivate(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineQueryExtendedInfo(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(EngineInfoSize) PWINBIO_EXTENDED_ENGINE_INFO EngineInfo,
    _In_ SIZE_T EngineInfoSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface) ||
        !ARGUMENT_PRESENT(EngineInfo))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->QueryExtendedInfo))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->QueryExtendedInfo(
                                            Pipeline,
                                            EngineInfo,
                                            EngineInfoSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineIdentifyAll(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T PresenceCount,
    _Out_ PWINBIO_PRESENCE *PresenceArray
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface) ||
        !ARGUMENT_PRESENT(PresenceCount) ||
        !ARGUMENT_PRESENT(PresenceArray))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->IdentifyAll))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->IdentifyAll(
                                            Pipeline,
                                            PresenceCount,
                                            PresenceArray
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineSetEnrollmentSelector(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONGLONG SelectorValue
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->SetEnrollmentSelector))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->SetEnrollmentSelector(
                                            Pipeline,
                                            SelectorValue
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineQueryExtendedEnrollmentStatus(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(EnrollmentStatusSize) PWINBIO_EXTENDED_ENROLLMENT_STATUS EnrollmentStatus,
    _In_ SIZE_T EnrollmentStatusSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface) ||
        !ARGUMENT_PRESENT(EnrollmentStatus))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->QueryExtendedEnrollmentStatus))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->QueryExtendedEnrollmentStatus(
                                            Pipeline,
                                            EnrollmentStatus,
                                            EnrollmentStatusSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineSetEnrollmentParameters(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_EXTENDED_ENROLLMENT_PARAMETERS Parameters
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->SetEnrollmentParameters))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->SetEnrollmentParameters(
                                            Pipeline,
                                            Parameters
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineRefreshCache(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->RefreshCache))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->RefreshCache(
                                            Pipeline
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineSelectCalibrationFormat(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(FormatCount) PWINBIO_UUID FormatArray,
    _In_ SIZE_T FormatCount,
    _Out_ PWINBIO_UUID SelectedFormat,
    _Out_ PSIZE_T MaxBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface) ||
        !ARGUMENT_PRESENT(FormatArray) ||
        !ARGUMENT_PRESENT(SelectedFormat) ||
        !ARGUMENT_PRESENT(MaxBufferSize))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->SelectCalibrationFormat))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->SelectCalibrationFormat(
                                            Pipeline,
                                            FormatArray,
                                            FormatCount,
                                            SelectedFormat,
                                            MaxBufferSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineQueryCalibrationData(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PBOOLEAN DiscardAndRepeatCapture,
    _Out_writes_bytes_to_(MaxBufferSize, *CalibrationBufferSize) PUCHAR CalibrationBuffer,
    _Out_ PSIZE_T CalibrationBufferSize,
    _In_ SIZE_T MaxBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface) ||
        !ARGUMENT_PRESENT(DiscardAndRepeatCapture) ||
        !ARGUMENT_PRESENT(CalibrationBuffer) ||
        !ARGUMENT_PRESENT(CalibrationBufferSize))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->QueryCalibrationData))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->QueryCalibrationData(
                                            Pipeline,
                                            DiscardAndRepeatCapture,
                                            CalibrationBuffer,
                                            CalibrationBufferSize,
                                            MaxBufferSize
                                            );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineSetAccountPolicy(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(PolicyItemCount) PWINBIO_ACCOUNT_POLICY PolicyItemArray,
    _In_ SIZE_T PolicyItemCount
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface) ||
        !ARGUMENT_PRESENT(PolicyItemArray))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->SetAccountPolicy))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->SetAccountPolicy(
                                            Pipeline,
                                            PolicyItemArray,
                                            PolicyItemCount
                                            );
    }
}
//-----------------------------------------------------------------------------

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
//
// V4.0 methods begin here...
//

inline HRESULT
WbioEngineCreateKey(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(KeySize) const UCHAR* Key,
    _In_ SIZE_T KeySize,
    _Out_writes_bytes_to_(KeyIdentifierSize, *ResultSize) PUCHAR KeyIdentifier,
    _In_ SIZE_T KeyIdentifierSize,
    _Out_ PSIZE_T ResultSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->CreateKey))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->CreateKey(
                                              Pipeline,
                                              Key,
                                              KeySize,
                                              KeyIdentifier,
                                              KeyIdentifierSize,
                                              ResultSize);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineIdentifyFeatureSetSecure(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(NonceSize) const UCHAR* Nonce,
    _In_ SIZE_T NonceSize,
    _In_reads_(KeyIdentifierSize) const UCHAR* KeyIdentifier,
    _In_ SIZE_T KeyIdentifierSize,
    _Out_ PWINBIO_IDENTITY Identity,
    _Out_ PWINBIO_BIOMETRIC_SUBTYPE SubFactor,
	_Out_ PWINBIO_REJECT_DETAIL RejectDetail,
    _Outptr_result_bytebuffer_(*AuthorizationSize) PUCHAR *Authorization,
    _Out_ PSIZE_T AuthorizationSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->IdentifyFeatureSetSecure))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->IdentifyFeatureSetSecure(
                                            Pipeline,
                                            Nonce,
                                            NonceSize,
                                            KeyIdentifier,
                                            KeyIdentifierSize,
                                            Identity,
                                            SubFactor,
										    RejectDetail,
                                            Authorization,
                                            AuthorizationSize);
    }
}
//-----------------------------------------------------------------------------

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
//
// V5.0 methods begin here...
//

inline HRESULT
WbioEngineAcceptPrivateSensorTypeInfo(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_(TypeInfoBufferSize) const UCHAR* TypeInfoBufferAddress,
    _In_ SIZE_T TypeInfoBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->AcceptPrivateSensorTypeInfo))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->AcceptPrivateSensorTypeInfo(
                                              Pipeline,
                                              TypeInfoBufferAddress,
                                              TypeInfoBufferSize);
    }
}
//-----------------------------------------------------------------------------

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
//
// V6.0 methods begin here...
//

inline HRESULT
WbioEngineCreateEnrollmentAuthenticated(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Outptr_result_bytebuffer_(*NonceSize) PUCHAR* Nonce,
    _Out_ SIZE_T* NonceSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->CreateEnrollmentAuthenticated))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->CreateEnrollmentAuthenticated(
                                              Pipeline,
                                              Nonce,
                                              NonceSize);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioEngineIdentifyFeatureSetAuthenticated(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(NonceSize) const UCHAR* Nonce,
    _In_ SIZE_T NonceSize,
    _Out_ WINBIO_IDENTITY* Identity,
    _Out_ WINBIO_BIOMETRIC_SUBTYPE* SubFactor,
    _Out_ WINBIO_REJECT_DETAIL* RejectDetail,
    _Outptr_result_bytebuffer_(*AuthenticationSize) PUCHAR* Authentication,
    _Out_ SIZE_T* AuthenticationSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->EngineInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->EngineInterface->IdentifyFeatureSetAuthenticated))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->EngineInterface->IdentifyFeatureSetAuthenticated(
                                              Pipeline,
                                              Nonce,
                                              NonceSize,
                                              Identity,
                                              SubFactor,
                                              RejectDetail,
                                              Authentication,
                                              AuthenticationSize);
    }
}
//-----------------------------------------------------------------------------

#endif

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// Storage Adapter methods...
//
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// Methods available in V1.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_STORAGE_ATTACH_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_DETACH_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

// Clears out all Storage-specific pipeline context
typedef HRESULT
(WINAPI *PIBIO_STORAGE_CLEAR_CONTEXT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_CREATE_DATABASE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ WINBIO_BIOMETRIC_TYPE Factor,
    _In_ PWINBIO_UUID Format,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString,
    _In_ SIZE_T IndexElementCount,
    _In_ SIZE_T InitialSize
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_ERASE_DATABASE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_OPEN_DATABASE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_CLOSE_DATABASE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_GET_DATA_FORMAT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_UUID Format,
    _Out_ PWINBIO_VERSION Version
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_GET_DATABASE_SIZE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T AvailableRecordCount,
    _Out_ PSIZE_T TotalRecordCount
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_ADD_RECORD_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_STORAGE_RECORD RecordContents
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_DELETE_RECORD_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_QUERY_BY_SUBJECT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_QUERY_BY_CONTENT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _In_reads_(IndexElementCount) ULONG IndexVector[],
    _In_ SIZE_T IndexElementCount
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_GET_RECORD_COUNT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T RecordCount
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_FIRST_RECORD_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_NEXT_RECORD_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_GET_CURRENT_RECORD_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_STORAGE_RECORD RecordContents
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_CONTROL_UNIT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize) PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize)  PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// Additional methods available in V2.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
// Additional methods available in V3.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_STORAGE_PIPELINE_INIT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_PIPELINE_CLEANUP_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_ACTIVATE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_DEACTIVATE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(StorageInfoSize) PWINBIO_EXTENDED_STORAGE_INFO StorageInfo,
    _In_ SIZE_T StorageInfoSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
//
// Additional methods available in V4.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ BOOLEAN RecordsAdded
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
//
// Additional methods available in V5.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_STORAGE_RESERVED_1_FN)(
    _Reserved_ PWINBIO_PIPELINE Pipeline,
    _Reserved_ PWINBIO_IDENTITY Identity,
    _Reserved_ PULONGLONG Reserved1,
    _Reserved_ PULONGLONG Reserved2
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_RESERVED_2_FN)(
    _Reserved_ PWINBIO_PIPELINE Pipeline,
    _Reserved_ PWINBIO_IDENTITY Identity
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Out_ PWINBIO_STORAGE_RECORD RecordContents
    );

typedef HRESULT
(WINAPI *PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_STORAGE_RECORD RecordContents
    );

#endif

///////////////////////////////////////////////////////////////////////////////
//
// Storage Adapter interface table.
//
///////////////////////////////////////////////////////////////////////////////
//
// Available interface versions...
//
#define WINBIO_STORAGE_INTERFACE_VERSION_1  WINBIO_MAKE_INTERFACE_VERSION(1,0)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define WINBIO_STORAGE_INTERFACE_VERSION_2  WINBIO_MAKE_INTERFACE_VERSION(2,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define WINBIO_STORAGE_INTERFACE_VERSION_3  WINBIO_MAKE_INTERFACE_VERSION(3,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define WINBIO_STORAGE_INTERFACE_VERSION_4  WINBIO_MAKE_INTERFACE_VERSION(4,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define WINBIO_STORAGE_INTERFACE_VERSION_5  WINBIO_MAKE_INTERFACE_VERSION(5,0)
#endif

typedef struct _WINBIO_STORAGE_INTERFACE {
    WINBIO_ADAPTER_INTERFACE_VERSION            Version;
    WINBIO_ADAPTER_TYPE                         Type;
    SIZE_T                                      Size;
    GUID                                        AdapterId;

    PIBIO_STORAGE_ATTACH_FN                     Attach;
    PIBIO_STORAGE_DETACH_FN                     Detach;

    PIBIO_STORAGE_CLEAR_CONTEXT_FN              ClearContext;

    PIBIO_STORAGE_CREATE_DATABASE_FN            CreateDatabase;
    PIBIO_STORAGE_ERASE_DATABASE_FN             EraseDatabase;

    PIBIO_STORAGE_OPEN_DATABASE_FN              OpenDatabase;
    PIBIO_STORAGE_CLOSE_DATABASE_FN             CloseDatabase;

    PIBIO_STORAGE_GET_DATA_FORMAT_FN            GetDataFormat;
    PIBIO_STORAGE_GET_DATABASE_SIZE_FN          GetDatabaseSize;

    PIBIO_STORAGE_ADD_RECORD_FN                 AddRecord;
    PIBIO_STORAGE_DELETE_RECORD_FN              DeleteRecord;

    PIBIO_STORAGE_QUERY_BY_SUBJECT_FN           QueryBySubject;
    PIBIO_STORAGE_QUERY_BY_CONTENT_FN           QueryByContent;

    PIBIO_STORAGE_GET_RECORD_COUNT_FN           GetRecordCount;
    PIBIO_STORAGE_FIRST_RECORD_FN               FirstRecord;
    PIBIO_STORAGE_NEXT_RECORD_FN                NextRecord;
    PIBIO_STORAGE_GET_CURRENT_RECORD_FN         GetCurrentRecord;

    PIBIO_STORAGE_CONTROL_UNIT_FN               ControlUnit;
    PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN    ControlUnitPrivileged;

#if (NTDDI_VERSION >= NTDDI_WIN8)
    //
    // V2.0 methods begin here...
    //
    PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN        NotifyPowerChange;
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
    //
    // V3.0 methods begin here...
    //
    PIBIO_STORAGE_PIPELINE_INIT_FN              PipelineInit;
    PIBIO_STORAGE_PIPELINE_CLEANUP_FN           PipelineCleanup;
    PIBIO_STORAGE_ACTIVATE_FN                   Activate;
    PIBIO_STORAGE_DEACTIVATE_FN                 Deactivate;
    PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN        QueryExtendedInfo;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
    //
    // Additional methods available in V4.0 and later
    //
    PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN     NotifyDatabaseChange;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
    //
    // Additional methods available in V5.0 and later
    //
    PIBIO_STORAGE_RESERVED_1_FN                 Reserved1;
    PIBIO_STORAGE_RESERVED_2_FN                 Reserved2;
    PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN        UpdateRecordBegin;
    PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN       UpdateRecordCommit;
#endif

} WINBIO_STORAGE_INTERFACE, *PWINBIO_STORAGE_INTERFACE;


//
// Interface-retrieval function exported by the Storage Adapter
// plug-in DLL. It *MUST* be called 'WbioQueryStorageInterface'.
//
HRESULT
WINAPI
WbioQueryStorageInterface(
    _Out_ PWINBIO_STORAGE_INTERFACE *StorageInterface
    );

//
// Pointer to an interface-retrieval function used by the
// framework after a 'GetProcAddress' call.
//
typedef HRESULT
(WINAPI *PWINBIO_QUERY_STORAGE_INTERFACE_FN)(
    _Out_ PWINBIO_STORAGE_INTERFACE *StorageInterface
    );

#define WINBIO_QUERY_STORAGE_INTERFACE_FN_NAME  ("WbioQueryStorageInterface")



///////////////////////////////////////////////////////////////////////////////
//
// Inline functions for calling Storage Adapter methods
//
///////////////////////////////////////////////////////////////////////////////
inline HRESULT
WbioStorageAttach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->Attach))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->Attach(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageDetach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->Detach))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->Detach(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageClearContext(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->ClearContext))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->ClearContext(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageCreateDatabase(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ WINBIO_BIOMETRIC_TYPE Factor,
    _In_ PWINBIO_UUID Format,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString,
    _In_ SIZE_T IndexElementCount,
    _In_ SIZE_T InitialSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->CreateDatabase))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->CreateDatabase(
                                                Pipeline,
                                                DatabaseId,
                                                Factor,
                                                Format,
                                                FilePath,
                                                ConnectString,
                                                IndexElementCount,
                                                InitialSize
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageEraseDatabase(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->EraseDatabase))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->EraseDatabase(
                                                Pipeline,
                                                DatabaseId,
                                                FilePath,
                                                ConnectString
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageOpenDatabase(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->OpenDatabase))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->OpenDatabase(
                                                Pipeline,
                                                DatabaseId,
                                                FilePath,
                                                ConnectString
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageCloseDatabase(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->CloseDatabase))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->CloseDatabase(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageGetDataFormat(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_UUID Format,
    _Out_ PWINBIO_VERSION Version
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->GetDataFormat))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->GetDataFormat(
                                                Pipeline,
                                                Format,
                                                Version
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageGetDatabaseSize(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T AvailableRecordCount,
    _Out_ PSIZE_T TotalRecordCount
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->GetDatabaseSize))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->GetDatabaseSize(
                                                Pipeline,
                                                AvailableRecordCount,
                                                TotalRecordCount
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageAddRecord(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_STORAGE_RECORD RecordContents
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->AddRecord))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->AddRecord(
                                                Pipeline,
                                                RecordContents
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageDeleteRecord(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->DeleteRecord))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->DeleteRecord(
                                                Pipeline,
                                                Identity,
                                                SubFactor
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageQueryBySubject(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->QueryBySubject))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->QueryBySubject(
                                                Pipeline,
                                                Identity,
                                                SubFactor
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageQueryByContent(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _In_reads_(IndexElementCount) ULONG IndexVector[],
    _In_ SIZE_T IndexElementCount
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->QueryByContent))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->QueryByContent(
                                                Pipeline,
                                                SubFactor,
                                                IndexVector,
                                                IndexElementCount
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageGetRecordCount(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T RecordCount
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->GetRecordCount))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->GetRecordCount(
                                                Pipeline,
                                                RecordCount
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageFirstRecord(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->FirstRecord))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->FirstRecord(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageNextRecord(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->NextRecord))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->NextRecord(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageGetCurrentRecord(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PWINBIO_STORAGE_RECORD RecordContents
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->GetCurrentRecord))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->GetCurrentRecord(
                                                Pipeline,
                                                RecordContents
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageControlUnit(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize)  PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->ControlUnit))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->ControlUnit(
                                                Pipeline,
                                                ControlCode,
                                                SendBuffer,
                                                SendBufferSize,
                                                ReceiveBuffer,
                                                ReceiveBufferSize,
                                                ReceiveDataSize,
                                                OperationStatus
                                                );
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageControlUnitPrivileged(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG ControlCode,
    _In_reads_bytes_(SendBufferSize) PUCHAR SendBuffer,
    _In_ SIZE_T SendBufferSize,
    _Out_writes_bytes_to_(ReceiveBufferSize,*ReceiveDataSize)  PUCHAR ReceiveBuffer,
    _In_ SIZE_T ReceiveBufferSize,
    _Out_ PSIZE_T ReceiveDataSize,
    _Out_ PULONG OperationStatus
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->ControlUnitPrivileged))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->ControlUnitPrivileged(
                                                Pipeline,
                                                ControlCode,
                                                SendBuffer,
                                                SendBufferSize,
                                                ReceiveBuffer,
                                                ReceiveBufferSize,
                                                ReceiveDataSize,
                                                OperationStatus
                                                );
    }
}
//-----------------------------------------------------------------------------

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// V2.0 methods begin here...
//

inline HRESULT
WbioStorageNotifyPowerChange(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->NotifyPowerChange))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->NotifyPowerChange(
                                                Pipeline,
                                                PowerEventType
                                                );
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
// V3.0 methods begin here...
//

inline HRESULT
WbioStoragePipelineInit(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->PipelineInit))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->PipelineInit(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStoragePipelineCleanup(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->PipelineCleanup))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->PipelineCleanup(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageActivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->Activate))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->Activate(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageDeactivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->Deactivate))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->Deactivate(Pipeline);
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioStorageQueryExtendedInfo(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(StorageInfoSize) PWINBIO_EXTENDED_STORAGE_INFO StorageInfo,
    _In_ SIZE_T StorageInfoSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface) ||
        !ARGUMENT_PRESENT(StorageInfo))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->QueryExtendedInfo))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->QueryExtendedInfo(
                                                Pipeline,
                                                StorageInfo,
                                                StorageInfoSize
                                                );
    }
}
//-----------------------------------------------------------------------------

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
//
// V4.0 methods begin here...
//

inline HRESULT
WbioStorageNotifyDatabaseChange(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ BOOLEAN RecordsAdded
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->NotifyDatabaseChange))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->NotifyDatabaseChange(
                                                Pipeline,
                                                RecordsAdded
                                                );
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
//
// V5.0 methods begin here...
//

inline HRESULT
WbioStorageUpdateRecordBegin(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Out_ PWINBIO_STORAGE_RECORD RecordContents
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->UpdateRecordBegin))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->UpdateRecordBegin(
                                                Pipeline,
                                                Identity,
                                                SubFactor,
                                                RecordContents
                                                );
    }
}

inline HRESULT
WbioStorageUpdateRecordCommit(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_STORAGE_RECORD RecordContents
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->StorageInterface))
    {
        return E_POINTER;
    }
    else if (!ARGUMENT_PRESENT(Pipeline->StorageInterface->UpdateRecordCommit))
    {
        return E_NOTIMPL;
    }
    else
    {
        return Pipeline->StorageInterface->UpdateRecordCommit(
                                                Pipeline,
                                                RecordContents
                                                );
    }
}
//-----------------------------------------------------------------------------
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// WinBio Framework methods...
//
///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//
// Methods available in V3.1 and later
//
typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(ExtendedStatusSize) PWINBIO_EXTENDED_UNIT_STATUS ExtendedStatus,
    _In_ SIZE_T ExtendedStatusSize
    );

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

//
// Methods available in V4.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ SIZE_T RequiredCapacity,
    _Out_ PSIZE_T MaxBufferSize
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(BufferSize) PUCHAR BufferAddress,
    _In_ SIZE_T BufferSize
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T RequiredCapacity,
    _Out_ PSIZE_T MaxBufferSize
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_to_(BufferSize,*ReturnedDataSize)  PUCHAR BufferAddress,
    _In_ SIZE_T BufferSize,
    _Out_ PSIZE_T ReturnedDataSize
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline
    );

#endif  // NTDDI_VERSION >= NTDDI_WIN10_RS1

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

//
// Methods available in V5.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN)(
    _Reserved_ PWINBIO_PIPELINE Pipeline,
    _In_ SIZE_T Reserved1,
    _Reserved_ PSIZE_T Reserved2
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN)(
    _Reserved_ PWINBIO_PIPELINE Pipeline,
    _Reserved_ PUCHAR Reserved1,
    _In_ SIZE_T Reserved2
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN)(
    _Reserved_ PWINBIO_PIPELINE Pipeline
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ SIZE_T AllocationSize,
    _Out_ PVOID *Address
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_FREE_MEMORY_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PVOID Address
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_GET_PROPERTY_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_PROPERTY_TYPE PropertyType,
    _In_ WINBIO_PROPERTY_ID PropertyId,
    _In_opt_ WINBIO_IDENTITY *Identity,
    _In_opt_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Outptr_result_bytebuffer_maybenull_(*PropertyBufferSize) PVOID *PropertyBuffer,
    _Out_opt_ SIZE_T *PropertyBufferSize
    );

#endif  // NTDDI_VERSION >= NTDDI_WIN10_RS2

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

//
// Methods available in V6.0 and later
//
typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ GUID SecureBufferIdentifier,
    _Outptr_result_bytebuffer_(*SecureBufferSize) PVOID *SecureBufferAddress,
    _Out_ SIZE_T *SecureBufferSize
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ GUID SecureBufferIdentifier
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS3

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

//
// Methods available in V7.0 and later
//

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _Out_ SIZE_T *SecureIdentityCount,
    _Outptr_result_buffer_(*SecureIdentityCount) PWINBIO_IDENTITY *SecureIdentities
    );

typedef HRESULT
(WINAPI *PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN)(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(AuthenticationSize) const UCHAR* Authentication,
    _In_ SIZE_T AuthenticationSize,
    _In_reads_bytes_(IvSize) const UCHAR* Iv,
    _In_ SIZE_T IvSize,
    _Inout_updates_bytes_(EncryptedDataSize) UCHAR* EncryptedData,
    _In_ SIZE_T EncryptedDataSize
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS4


///////////////////////////////////////////////////////////////////////////////
//
// Framework interface table.
//
///////////////////////////////////////////////////////////////////////////////
//
// Available interface versions...
//
#define WINBIO_FRAMEWORK_INTERFACE_VERSION_3_1  WINBIO_MAKE_INTERFACE_VERSION(3,1)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0  WINBIO_MAKE_INTERFACE_VERSION(4,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0  WINBIO_MAKE_INTERFACE_VERSION(5,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define WINBIO_FRAMEWORK_INTERFACE_VERSION_6_0  WINBIO_MAKE_INTERFACE_VERSION(6,0)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#define WINBIO_FRAMEWORK_INTERFACE_VERSION_7_0  WINBIO_MAKE_INTERFACE_VERSION(7,0)
#endif

typedef struct _WINBIO_FRAMEWORK_INTERFACE {
    WINBIO_ADAPTER_INTERFACE_VERSION            Version;
    WINBIO_ADAPTER_TYPE                         Type;
    SIZE_T                                      Size;
    GUID                                        AdapterId;

    //
    // V3.1 methods begin here...
    //
    PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN          SetUnitStatus;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
    //
    // V4.0 methods begin here...
    //
    // (Forwarding functions...)
    //
    PIBIO_STORAGE_ATTACH_FN                     VsmStorageAttach;
    PIBIO_STORAGE_DETACH_FN                     VsmStorageDetach;
    PIBIO_STORAGE_CLEAR_CONTEXT_FN              VsmStorageClearContext;
    PIBIO_STORAGE_CREATE_DATABASE_FN            VsmStorageCreateDatabase;
    PIBIO_STORAGE_OPEN_DATABASE_FN              VsmStorageOpenDatabase;
    PIBIO_STORAGE_CLOSE_DATABASE_FN             VsmStorageCloseDatabase;
    PIBIO_STORAGE_DELETE_RECORD_FN              VsmStorageDeleteRecord;
    PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN        VsmStorageNotifyPowerChange;
    PIBIO_STORAGE_PIPELINE_INIT_FN              VsmStoragePipelineInit;
    PIBIO_STORAGE_PIPELINE_CLEANUP_FN           VsmStoragePipelineCleanup;
    PIBIO_STORAGE_ACTIVATE_FN                   VsmStorageActivate;
    PIBIO_STORAGE_DEACTIVATE_FN                 VsmStorageDeactivate;
    PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN        VsmStorageQueryExtendedInfo;

    //
    // (Upcalls...)
    //
    PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN          VsmStorageCacheClear;
    PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN   VsmStorageCacheImportBegin;
    PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN    VsmStorageCacheImportNext;
    PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN     VsmStorageCacheImportEnd;
    PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN   VsmStorageCacheExportBegin;
    PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN    VsmStorageCacheExportNext;
    PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN     VsmStorageCacheExportEnd;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
    //
    // V5.0 methods begin here...
    //
    // (Forwarding functions...)
    //
    PIBIO_SENSOR_ATTACH_FN                                      VsmSensorAttach;
    PIBIO_SENSOR_DETACH_FN                                      VsmSensorDetach;
    PIBIO_SENSOR_CLEAR_CONTEXT_FN                               VsmSensorClearContext;
    PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN                         VsmSensorPushDataToEngine;
    PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN                         VsmSensorNotifyPowerChange;
    PIBIO_SENSOR_PIPELINE_INIT_FN                               VsmSensorPipelineInit;
    PIBIO_SENSOR_PIPELINE_CLEANUP_FN                            VsmSensorPipelineCleanup;
    PIBIO_SENSOR_ACTIVATE_FN                                    VsmSensorActivate;
    PIBIO_SENSOR_DEACTIVATE_FN                                  VsmSensorDeactivate;
    PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN                     VsmSensorAsyncImportRawBuffer;
    PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN                  VsmSensorAsyncImportSecureBuffer;

    PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN                   Reserved1;
    PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN                   Reserved2;
    PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN                   Reserved3;
    PIBIO_STORAGE_RESERVED_1_FN                                 Reserved4;
    PIBIO_STORAGE_RESERVED_2_FN                                 Reserved5;

    //
    // (Upcalls...)
    //
    PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN                          AllocateMemory;
    PIBIO_FRAMEWORK_FREE_MEMORY_FN                              FreeMemory;
    PIBIO_FRAMEWORK_GET_PROPERTY_FN                             GetProperty;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
    //
    // V6.0 methods begin here...
    //
    // (Upcalls...)
    //
    PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN          LockAndValidateSecureBuffer;
    PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN                    ReleaseSecureBuffer;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
    //
    // V7.0 methods begin here...
    //
    // (Upcalls...)
    //
    PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN         QueryAuthorizedEnrollments;
    PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN                       DecryptSample;
#endif

} WINBIO_FRAMEWORK_INTERFACE, *PWINBIO_FRAMEWORK_INTERFACE;


#define WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(pipeline, requiredVersion)                       \
    ((pipeline->FrameworkInterface->Version.MajorVersion > requiredVersion.MajorVersion) ||     \
     ((pipeline->FrameworkInterface->Version.MajorVersion == requiredVersion.MajorVersion) &&   \
      (pipeline->FrameworkInterface->Version.MinorVersion >= requiredVersion.MinorVersion)))


///////////////////////////////////////////////////////////////////////////////
//
// Inline functions for calling WinBio Framework methods
//
///////////////////////////////////////////////////////////////////////////////
inline HRESULT
WbioFrameworkSetUnitStatus(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(ExtendedStatusSize) PWINBIO_EXTENDED_UNIT_STATUS ExtendedStatus,
    _In_ SIZE_T ExtendedStatusSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_3_1;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->SetUnitStatus))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->SetUnitStatus(
                                                    Pipeline,
                                                    ExtendedStatus,
                                                    ExtendedStatusSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

#endif  // NTDDI_VERSION >= NTDDI_WINTHRESHOLD

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

inline HRESULT
WbioFrameworkVsmStorageAttach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageAttach))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageAttach(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageDetach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageDetach))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageDetach(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageClearContext(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageClearContext))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageClearContext(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCreateDatabase(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ WINBIO_BIOMETRIC_TYPE Factor,
    _In_ PWINBIO_UUID Format,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString,
    _In_ SIZE_T IndexElementCount,
    _In_ SIZE_T InitialSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCreateDatabase))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCreateDatabase(
                                                    Pipeline,
                                                    DatabaseId,
                                                    Factor,
                                                    Format,
                                                    FilePath,
                                                    ConnectString,
                                                    IndexElementCount,
                                                    InitialSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageOpenDatabase(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_UUID DatabaseId,
    _In_ LPCWSTR FilePath,
    _In_ LPCWSTR ConnectString
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageOpenDatabase))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageOpenDatabase(
                                                    Pipeline,
                                                    DatabaseId,
                                                    FilePath,
                                                    ConnectString
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCloseDatabase(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCloseDatabase))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCloseDatabase(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageDeleteRecord(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _In_ WINBIO_BIOMETRIC_SUBTYPE SubFactor
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageDeleteRecord))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageDeleteRecord(
                                                    Pipeline,
                                                    Identity,
                                                    SubFactor
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageNotifyPowerChange(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageNotifyPowerChange))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageNotifyPowerChange(
                                                    Pipeline,
                                                    PowerEventType
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStoragePipelineInit(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStoragePipelineInit))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStoragePipelineInit(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStoragePipelineCleanup(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStoragePipelineCleanup))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStoragePipelineCleanup(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageActivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageActivate))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageActivate(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageDeactivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageDeactivate))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageDeactivate(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageQueryExtendedInfo(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_(StorageInfoSize) PWINBIO_EXTENDED_STORAGE_INFO StorageInfo,
    _In_ SIZE_T StorageInfoSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageQueryExtendedInfo))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageQueryExtendedInfo(
                                                    Pipeline,
                                                    StorageInfo,
                                                    StorageInfoSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCacheClear(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCacheClear))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCacheClear(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCacheImportBegin(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ SIZE_T RequiredCapacity,
    _Out_ PSIZE_T MaxBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCacheImportBegin))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCacheImportBegin(
                                                    Pipeline,
                                                    RequiredCapacity,
                                                    MaxBufferSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCacheImportNext(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(BufferSize) PUCHAR BufferAddress,
    _In_ SIZE_T BufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCacheImportNext))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCacheImportNext(
                                                    Pipeline,
                                                    BufferAddress,
                                                    BufferSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCacheImportEnd(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCacheImportEnd))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCacheImportEnd(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCacheExportBegin(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_ PSIZE_T RequiredCapacity,
    _Out_ PSIZE_T MaxBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCacheExportBegin))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCacheExportBegin(
                                                    Pipeline,
                                                    RequiredCapacity,
                                                    MaxBufferSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCacheExportNext(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _Out_writes_bytes_to_(BufferSize,*ReturnedDataSize)  PUCHAR BufferAddress,
    _In_ SIZE_T BufferSize,
    _Out_ PSIZE_T ReturnedDataSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCacheExportNext))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCacheExportNext(
                                                    Pipeline,
                                                    BufferAddress,
                                                    BufferSize,
                                                    ReturnedDataSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmStorageCacheExportEnd(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_4_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmStorageCacheExportEnd))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmStorageCacheExportEnd(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

#endif  // NTDDI_VERSION >= NTDDI_WIN10_RS1

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

inline HRESULT
WbioFrameworkVsmSensorAttach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorAttach))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorAttach(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensoreDetach(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorDetach))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorDetach(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorClearContext(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorClearContext))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorClearContext(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorPushDataToEngine(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_BIR_PURPOSE Purpose,
    _In_ WINBIO_BIR_DATA_FLAGS Flags,
    _Out_ PWINBIO_REJECT_DETAIL RejectDetail
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorPushDataToEngine))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorPushDataToEngine(
                                                    Pipeline,
                                                    Purpose,
                                                    Flags,
                                                    RejectDetail
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorNotifyPowerChange(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ ULONG PowerEventType
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorNotifyPowerChange))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorNotifyPowerChange(
                                                    Pipeline,
                                                    PowerEventType
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorPipelineInit(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorPipelineInit))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorPipelineInit(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorPipelineCleanup(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorPipelineCleanup))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorPipelineCleanup(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorActivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorActivate))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorActivate(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorDeactivate(
    _Inout_ PWINBIO_PIPELINE Pipeline
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorDeactivate))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorDeactivate(Pipeline);
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorAsyncImportRawBuffer(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_opt_(RawBufferSize) PUCHAR RawBufferAddress,
    _In_ SIZE_T RawBufferSize,
    _Outptr_result_bytebuffer_maybenull_(*ResultBufferSize) PUCHAR *ResultBufferAddress,
    _Out_opt_ SIZE_T *ResultBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorAsyncImportRawBuffer))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorAsyncImportRawBuffer(
                                                    Pipeline,
                                                    RawBufferAddress,
                                                    RawBufferSize,
                                                    ResultBufferAddress,
                                                    ResultBufferSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmSensorAsyncImportSecureBuffer(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ GUID SecureBufferIdentifier,
    _In_reads_bytes_opt_(MetadataBufferSize) PUCHAR MetadataBufferAddress,
    _In_ SIZE_T MetadataBufferSize,
    _Outptr_result_bytebuffer_maybenull_(*ResultBufferSize) PUCHAR *ResultBufferAddress,
    _Out_opt_ SIZE_T *ResultBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->VsmSensorAsyncImportSecureBuffer))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->VsmSensorAsyncImportSecureBuffer(
                                                    Pipeline,
                                                    SecureBufferIdentifier,
                                                    MetadataBufferAddress,
                                                    MetadataBufferSize,
                                                    ResultBufferAddress,
                                                    ResultBufferSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkAllocateMemory(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ SIZE_T AllocationSize,
    _Out_ PVOID *Address
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->AllocateMemory))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->AllocateMemory(
                                                    Pipeline,
                                                    AllocationSize,
                                                    Address
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkFreeMemory(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PVOID Address
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->FreeMemory))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->FreeMemory(
                                                    Pipeline,
                                                    Address
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkGetProperty(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ WINBIO_PROPERTY_TYPE PropertyType,
    _In_ WINBIO_PROPERTY_ID PropertyId,
    _In_opt_ WINBIO_IDENTITY *Identity,
    _In_opt_ WINBIO_BIOMETRIC_SUBTYPE SubFactor,
    _Outptr_result_bytebuffer_maybenull_(*PropertyBufferSize)PVOID *PropertyBuffer,
    _Out_opt_ SIZE_T *PropertyBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_5_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->GetProperty))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->GetProperty(
                                                    Pipeline,
                                                    PropertyType,
                                                    PropertyId,
                                                    Identity,
                                                    SubFactor,
                                                    PropertyBuffer,
                                                    PropertyBufferSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS2

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

inline HRESULT
WbioFrameworkLockAndValidateSecureBuffer(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ GUID SecureBufferIdentifier,
    _Outptr_result_bytebuffer_(*SecureBufferSize) PVOID *SecureBufferAddress,
    _Out_ SIZE_T *SecureBufferSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_6_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->SetUnitStatus))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->LockAndValidateSecureBuffer(
                                                    Pipeline,
                                                    SecureBufferIdentifier,
                                                    SecureBufferAddress,
                                                    SecureBufferSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkReleaseSecureBuffer(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ GUID SecureBufferIdentifier
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_6_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->SetUnitStatus))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->ReleaseSecureBuffer(
                                                    Pipeline,
                                                    SecureBufferIdentifier
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS3

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

inline HRESULT
WbioFramweworkVsmQueryAuthorizedEnrollments(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_ PWINBIO_IDENTITY Identity,
    _Out_ SIZE_T *SecureIdentityCount,
    _Outptr_result_buffer_(*SecureIdentityCount) PWINBIO_IDENTITY *SecureIdentities
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_7_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->QueryAuthorizedEnrollments))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->QueryAuthorizedEnrollments(
                                                    Pipeline,
                                                    Identity,
                                                    SecureIdentityCount,
                                                    SecureIdentities
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

inline HRESULT
WbioFrameworkVsmDecryptSample(
    _Inout_ PWINBIO_PIPELINE Pipeline,
    _In_reads_bytes_(AuthenticationSize) const UCHAR* Authentication,
    _In_ SIZE_T AuthenticationSize,
    _In_reads_bytes_(IvSize) const UCHAR* Iv,
    _In_ SIZE_T IvSize,
    _Inout_updates_bytes_(EncryptedDataSize) UCHAR* EncryptedData,
    _In_ SIZE_T EncryptedDataSize
    )
{
    if (!ARGUMENT_PRESENT(Pipeline) ||
        !ARGUMENT_PRESENT(Pipeline->SensorInterface))
    {
        return E_POINTER;
    }
    else
    {
        WINBIO_ADAPTER_INTERFACE_VERSION requiredFrameworkVersion = WINBIO_FRAMEWORK_INTERFACE_VERSION_7_0;

        if (Pipeline->SensorInterface->Version.MinorVersion == 0 ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface) ||
            !WINBIO_IS_FRAMEWORK_VERSION_COMPATIBLE(Pipeline, requiredFrameworkVersion) ||
            !ARGUMENT_PRESENT(Pipeline->FrameworkInterface->DecryptSample))
        {
            return E_NOTIMPL;
        }
        else
        {
            return Pipeline->FrameworkInterface->DecryptSample(
                                                    Pipeline,
                                                    Authentication,
                                                    AuthenticationSize,
                                                    Iv,
                                                    IvSize,
                                                    EncryptedData,
                                                    EncryptedDataSize
                                                    );
        }
    }
}
//-----------------------------------------------------------------------------

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS4

#ifdef __cplusplus
} // extern "C"
#endif

#endif // (NTDDI_VERSION >= NTDDI_WIN7)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _WINBIO_ADAPTER_H_2C0E14E1_5330_4f60_9B4F_836CFFD7452A_

