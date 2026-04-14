/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    spb.h

Abstract:

    This module contains the Simple Peripheral Bus IOCTL interface for
    use by client applications & drivers.

Environment:

    User-mode and kernel-mode.

--*/

#ifndef _SPB_H_
#define _SPB_H_

#include <winapifamily.h>

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef _MSC_VER
#pragma once
#endif

#pragma warning(push)
#pragma warning(disable:4480)   // type specifier for enum

//
// When opening an connection to a device attached to the SPB controller,
// this is the root of the file object name.
//

#define SPB_TARGET_NAME_PREFIX L"\\SPB\\"

typedef enum SpbIoctl
{
    IOCTL_SPB_LOCK_CONTROLLER =    CTL_CODE(FILE_DEVICE_CONTROLLER,
                                            0x600,
                                            METHOD_BUFFERED,
                                            FILE_ANY_ACCESS),

    IOCTL_SPB_UNLOCK_CONTROLLER =  CTL_CODE(FILE_DEVICE_CONTROLLER, 
                                            0x601,                  
                                            METHOD_BUFFERED,        
                                            FILE_ANY_ACCESS),

    IOCTL_SPB_EXECUTE_SEQUENCE =   CTL_CODE(FILE_DEVICE_CONTROLLER,   
                                            0x602,                    
                                            METHOD_BUFFERED,          
                                            FILE_ANY_ACCESS),

    IOCTL_SPB_LOCK_CONNECTION =    CTL_CODE(FILE_DEVICE_CONTROLLER,   
                                            0x603,                    
                                            METHOD_BUFFERED,          
                                            FILE_ANY_ACCESS),

    IOCTL_SPB_UNLOCK_CONNECTION =  CTL_CODE(FILE_DEVICE_CONTROLLER,   
                                            0x604,                    
                                            METHOD_BUFFERED,          
                                            FILE_ANY_ACCESS),

    IOCTL_SPB_FULL_DUPLEX =        CTL_CODE(FILE_DEVICE_CONTROLLER,   
                                            0x605,                    
                                            METHOD_BUFFERED,          
                                            FILE_ANY_ACCESS),

    IOCTL_SPB_MULTI_SPI_TRANSFER = CTL_CODE(FILE_DEVICE_CONTROLLER,   
                                            0x606,                    
                                            METHOD_BUFFERED,          
                                            FILE_ANY_ACCESS)
}
SpbIoctl, *PSpbIoctl;

typedef enum SPB_TRANSFER_DIRECTION
{
    SpbTransferDirectionNone,
    SpbTransferDirectionFromDevice,
    SpbTransferDirectionToDevice,
    SpbTransferDirectionMax
}
SPB_TRANSFER_DIRECTION, *PSPB_TRANSFER_DIRECTION;

typedef enum SPB_TRANSFER_BUFFER_FORMAT
{
    SpbTransferBufferFormatInvalid,
    SpbTransferBufferFormatSimple,
    SpbTransferBufferFormatList,
    SpbTransferBufferFormatSimpleNonPaged,
    SpbTransferBufferFormatMdl,
    SpbTransferBufferFormatMax
}
SPB_TRANSFER_BUFFER_FORMAT, *PSPB_TRANSFER_BUFFER_FORMAT;

//
// A single entry in a transfer buffer list.
//

typedef struct SPB_TRANSFER_BUFFER_LIST_ENTRY
{
    //
    // The data buffer.
    //

    PVOID Buffer;

    //
    // The length of the buffer in bytes.
    //

    ULONG BufferCb;
}
SPB_TRANSFER_BUFFER_LIST_ENTRY, *PSPB_TRANSFER_BUFFER_LIST_ENTRY;

#ifndef _NTDDK_
#ifndef PMDL
#define PMDL PVOID
#endif
#endif

//
// A buffer - either a simple one-entry buffer or a buffer list
//

typedef struct SPB_TRANSFER_BUFFER 
{
    //
    // the format of the transfer buffer.
    //

    SPB_TRANSFER_BUFFER_FORMAT Format;

#pragma warning(push)
#pragma warning(disable:4201)
    union 
    {
        //
        // Simple buffer or simple non-paged buffer.
        //

        SPB_TRANSFER_BUFFER_LIST_ENTRY Simple;

        // 
        // List of buffers.
        //

        struct
        {
            PSPB_TRANSFER_BUFFER_LIST_ENTRY List;
            ULONG                           ListCe;
        }
        BufferList;

        //
        // MDL format
        //

        PMDL Mdl;
    };
#pragma warning(pop)
}
SPB_TRANSFER_BUFFER, *PSPB_TRANSFER_BUFFER;

//
// A single transfer and its buffer
//

typedef struct SPB_TRANSFER_LIST_ENTRY
{
    //
    // Direction.
    //

    SPB_TRANSFER_DIRECTION Direction;

    //
    // The delay (in us) before starting this transfer.
    //

    ULONG DelayInUs;

    //
    // The buffer to transfer to/from
    //

    SPB_TRANSFER_BUFFER Buffer;
}
SPB_TRANSFER_LIST_ENTRY, *PSPB_TRANSFER_LIST_ENTRY;

FORCEINLINE
SPB_TRANSFER_LIST_ENTRY
SPB_TRANSFER_LIST_ENTRY_INIT_SIMPLE(
    _In_ SPB_TRANSFER_DIRECTION Direction,
    _In_ ULONG                  DelayInUs,
    _Pre_writable_byte_size_(BufferCb) PVOID                  Buffer,
    _In_ ULONG                  BufferCb
    )
{
    SPB_TRANSFER_LIST_ENTRY entry;

    entry.Direction = Direction;
    entry.DelayInUs = DelayInUs;
    entry.Buffer.Format = SpbTransferBufferFormatSimple;
    entry.Buffer.Simple.Buffer = Buffer;
    entry.Buffer.Simple.BufferCb = BufferCb;
    return entry;
}

FORCEINLINE
SPB_TRANSFER_LIST_ENTRY
SPB_TRANSFER_LIST_ENTRY_INIT_NON_PAGED(
    _In_ SPB_TRANSFER_DIRECTION Direction,
    _In_ ULONG                  DelayInUs,
    _Pre_writable_byte_size_(BufferCb) PVOID                  Buffer,
    _In_ ULONG                  BufferCb
    )
{
    SPB_TRANSFER_LIST_ENTRY entry;

    entry.Direction = Direction;
    entry.DelayInUs = DelayInUs;
    entry.Buffer.Format = SpbTransferBufferFormatSimpleNonPaged;
    entry.Buffer.Simple.Buffer = Buffer;
    entry.Buffer.Simple.BufferCb = BufferCb;
    return entry;
}

FORCEINLINE
SPB_TRANSFER_LIST_ENTRY
SPB_TRANSFER_LIST_ENTRY_INIT_MDL(
    _In_ SPB_TRANSFER_DIRECTION Direction,
    _In_ ULONG                  DelayInUs,
    _In_ PMDL                   Mdl
    )
{
    SPB_TRANSFER_LIST_ENTRY entry;

    entry.Direction = Direction;
    entry.DelayInUs = DelayInUs;
    entry.Buffer.Format = SpbTransferBufferFormatMdl;
    entry.Buffer.Mdl = Mdl;
    return entry;
}

FORCEINLINE
SPB_TRANSFER_LIST_ENTRY
SPB_TRANSFER_LIST_ENTRY_INIT_BUFFER_LIST(
    _In_ SPB_TRANSFER_DIRECTION          Direction,
    _In_ ULONG                           DelayInUs,
    _In_ SPB_TRANSFER_BUFFER_LIST_ENTRY  BufferList[], 
    _In_ ULONG                           BufferListCe
    )
{
    SPB_TRANSFER_LIST_ENTRY entry;

    entry.Direction = Direction;
    entry.DelayInUs = DelayInUs;
    entry.Buffer.Format = SpbTransferBufferFormatList;
    entry.Buffer.BufferList.List = BufferList;
    entry.Buffer.BufferList.ListCe = BufferListCe;
    return entry;
}
    

//
// a list of transfers to be executed in a sequence.
//

typedef struct SPB_TRANSFER_LIST
{
    //
    // List size - must be set to sizeof(SPB_TRANSFER_LIST)
    //

    _Field_range_(==, sizeof(SPB_TRANSFER_LIST))
    ULONG Size;

    //
    // Reserved flags for internal use - must be zero.
    //

    ULONG Reserved;

    //
    // The number of input or output transfers for this packet.
    // The packet should be followed by this number of transfer descriptors.
    //
    // Transfers must consist of at least one packet.
    //

    ULONG TransferCount;

    //
    // The array of transfer descriptors in the list.
    //

    _Field_size_(TransferCount) SPB_TRANSFER_LIST_ENTRY Transfers[1];
}
SPB_TRANSFER_LIST, *PSPB_TRANSFER_LIST;

//
// A macro to simplify defining a sequence with multiple 
// transfers.
//

#define SPB_TRANSFER_LIST_AND_ENTRIES(n) struct { \
    SPB_TRANSFER_LIST List;                           \
    SPB_TRANSFER_LIST_ENTRY ExtraTransfers[(n)-1];      \
}

VOID
FORCEINLINE
SPB_TRANSFER_LIST_INIT(
    _Out_ SPB_TRANSFER_LIST *TransferList,
    _In_  ULONG              TransferCount
    )
{
    memset(TransferList, 
           0,
           (sizeof(SPB_TRANSFER_LIST) + 
            (sizeof(SPB_TRANSFER_LIST_ENTRY) * (TransferCount - 1))));
    TransferList->Size = sizeof(SPB_TRANSFER_LIST);
    TransferList->TransferCount = TransferCount;
}

typedef enum SPB_MULTI_SPI_TRANSFER_MODE
{
    SpbMultiSpiTransferModeInvalid,
    SpbMultiSpiTransferModeDualSpi,
    SpbMultiSpiTransferModeQuadSpi,
    SpbMultiSpiTransferModeMax
}
SPB_MULTI_SPI_TRANSFER_MODE, *PSPB_MULTI_SPI_TRANSFER_MODE;

//
// A multi-SPI transfer operation header
// This is the structure to be accessed by the SpbCx client (controller driver).
//

typedef struct SPB_MULTI_SPI_TRANSFER_HEADER
{
    //
    // Structure size - must be set to sizeof(SPB_MULTI_SPI_TRANSFER)
    //

    _Field_range_(==, sizeof(struct SPB_MULTI_SPI_TRANSFER))
    ULONG Size;

    //
    // Line mode
    //

    SPB_MULTI_SPI_TRANSFER_MODE Mode;

    //
    // The count of bytes at the beginning of the write phase to be
    // transferred in single-SPI mode, before line mode switch to
    // the mode specified in the Mode member of this struct.
    //

    ULONG WritePhaseSingleSpiByteCount;

    //
    // The number of wait cycles represented in bytes (representing multi-SPI
    // transfer cycles - e.g. 1 byte => 2 wait cycles at quad-SPI), between the
    // write phase and read phase of the transfer.
    // These wait cycle bytes should be present, and of an undefined value
    // at the end of the WritePhaseBuffer. This should be set to 0 if the
    // transfer does not have a read phase.
    //

    ULONG WaitCycleByteCount;
}
SPB_MULTI_SPI_TRANSFER_HEADER, *PSPB_MULTI_SPI_TRANSFER_HEADER;

//
// Full representation of a multi-SPI transfer operation, including the
// transfer list. This structure should not be used directly by the SpbCx client
// (controller driver).
//

typedef struct SPB_MULTI_SPI_TRANSFER {

    //
    // Transfer header, containing parameters for the multi-SPI transfer
    //

    SPB_MULTI_SPI_TRANSFER_HEADER  Header;

    //
    // Number of transfer phases, either 1 for write, or 2 for read.
    //

    ULONG TransferPhaseCount;

    //
    // The transfer entries for write, and optionally read phase of the transfer.
    //

    _Field_size_(TransferPhaseCount) SPB_TRANSFER_LIST_ENTRY TransferPhases[1];
}
SPB_MULTI_SPI_TRANSFER, *PSPB_MULTI_SPI_TRANSFER;

//
// Structures & macros to simplify defining a multi-SPI transfer.
//

typedef struct SPB_MULTI_SPI_WRITE_TRANSFER {
    SPB_MULTI_SPI_TRANSFER SpiTransfer;
} SPB_MULTI_SPI_WRITE_TRANSFER, *PSPB_MULTI_SPI_WRITE_TRANSFER;

typedef struct SPB_MULTI_SPI_READ_TRANSFER {
    SPB_MULTI_SPI_TRANSFER SpiTransfer;
    SPB_TRANSFER_LIST_ENTRY ExtraTransfer;
} SPB_MULTI_SPI_READ_TRANSFER, *PSPB_MULTI_SPI_READ_TRANSFER;

C_ASSERT(sizeof(SPB_MULTI_SPI_READ_TRANSFER) == sizeof(SPB_MULTI_SPI_TRANSFER) + sizeof(SPB_TRANSFER_LIST_ENTRY));

VOID
FORCEINLINE
SPB_MULTI_SPI_TRANSFER_INIT(
    _Out_ SPB_MULTI_SPI_TRANSFER *SpiTransfer,
    _In_  SPB_MULTI_SPI_TRANSFER_MODE Mode,
    _In_  ULONG TransferPhaseCount,
    _In_  ULONG WritePhaseSingleSpiByteCount,
    _In_  ULONG WaitCycleByteCount
)
{
    memset(SpiTransfer, 0, sizeof(SPB_MULTI_SPI_TRANSFER) + (sizeof(SPB_TRANSFER_LIST_ENTRY) * (TransferPhaseCount - 1)));
    SpiTransfer->Header.Size = sizeof(SPB_MULTI_SPI_TRANSFER);
    SpiTransfer->Header.Mode = Mode;
    SpiTransfer->Header.WritePhaseSingleSpiByteCount = WritePhaseSingleSpiByteCount;
    SpiTransfer->Header.WaitCycleByteCount = WaitCycleByteCount;
    SpiTransfer->TransferPhaseCount = TransferPhaseCount;
}

VOID
FORCEINLINE
SPB_MULTI_SPI_WRITE_TRANSFER_INIT(
    _Out_ SPB_MULTI_SPI_WRITE_TRANSFER *SpiTransfer,
    _In_  SPB_MULTI_SPI_TRANSFER_MODE Mode,
    _In_  ULONG WritePhaseSingleSpiByteCount,
    _In_  ULONG WaitCycleByteCount
)
{
    SPB_MULTI_SPI_TRANSFER_INIT(&SpiTransfer->SpiTransfer, Mode, 1, WritePhaseSingleSpiByteCount, WaitCycleByteCount);
}

VOID
FORCEINLINE
SPB_MULTI_SPI_READ_TRANSFER_INIT(
    _Out_ SPB_MULTI_SPI_READ_TRANSFER *SpiTransfer,
    _In_  SPB_MULTI_SPI_TRANSFER_MODE Mode,
    _In_  ULONG WritePhaseSingleSpiByteCount,
    _In_  ULONG WaitCycleByteCount
)
{
    SPB_MULTI_SPI_TRANSFER_INIT(&SpiTransfer->SpiTransfer, Mode, 2, WritePhaseSingleSpiByteCount, WaitCycleByteCount);
}

// begin_wpp config
// CUSTOM_TYPE(SPB_TRANSFER_DIRECTION, ItemEnum(_SPB_TRANSFER_DIRECTION));
// CUSTOM_TYPE(SPB_TRANSFER_BUFFER_FORMAT, ItemEnum(_SPB_TRANSFER_BUFFER_FORMAT));
// end_wpp

#endif // NTDDI_VERSION >= NTDDI_WIN8

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#endif // _SPB_H_
