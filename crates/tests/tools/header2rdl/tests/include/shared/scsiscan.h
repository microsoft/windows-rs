/****************************************************************************
*
*  (C) COPYRIGHT 1996-2000, MICROSOFT CORP.
*
*  FILE:        scsiscan.h
*
*  VERSION:     1.0
*
*  DATE:        2/11/1997
*
*  DESCRIPTION:
*    IOCTL definitions for the SCSI scanner device driver.
*
*****************************************************************************/

//
// Turns off []
//
#pragma warning(disable : 4200)

#ifndef _SCSISCAN_H_
#define _SCSISCAN_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// SCSISCAN_CMD.SrbFlags

#define SRB_FLAGS_DISABLE_SYNCH_TRANSFER    0x00000008
#define SRB_FLAGS_DISABLE_AUTOSENSE         0x00000020
#define SRB_FLAGS_DATA_IN                   0x00000040
#define SRB_FLAGS_DATA_OUT                  0x00000080
#define SRB_FLAGS_NO_DATA_TRANSFER          0x00000000

// SCSISCAN_CMD.SrbStatus definitions

#define SRB_STATUS_PENDING                  0x00
#define SRB_STATUS_SUCCESS                  0x01
#define SRB_STATUS_ABORTED                  0x02
#define SRB_STATUS_ABORT_FAILED             0x03
#define SRB_STATUS_ERROR                    0x04
#define SRB_STATUS_BUSY                     0x05
#define SRB_STATUS_INVALID_REQUEST          0x06
#define SRB_STATUS_INVALID_PATH_ID          0x07
#define SRB_STATUS_NO_DEVICE                0x08
#define SRB_STATUS_TIMEOUT                  0x09
#define SRB_STATUS_SELECTION_TIMEOUT        0x0A
#define SRB_STATUS_COMMAND_TIMEOUT          0x0B
#define SRB_STATUS_MESSAGE_REJECTED         0x0D
#define SRB_STATUS_BUS_RESET                0x0E
#define SRB_STATUS_PARITY_ERROR             0x0F
#define SRB_STATUS_REQUEST_SENSE_FAILED     0x10
#define SRB_STATUS_NO_HBA                   0x11
#define SRB_STATUS_DATA_OVERRUN             0x12
#define SRB_STATUS_UNEXPECTED_BUS_FREE      0x13
#define SRB_STATUS_PHASE_SEQUENCE_FAILURE   0x14
#define SRB_STATUS_BAD_SRB_BLOCK_LENGTH     0x15
#define SRB_STATUS_REQUEST_FLUSHED          0x16
#define SRB_STATUS_INVALID_LUN              0x20
#define SRB_STATUS_INVALID_TARGET_ID        0x21
#define SRB_STATUS_BAD_FUNCTION             0x22
#define SRB_STATUS_ERROR_RECOVERY           0x23

#define SRB_STATUS_QUEUE_FROZEN             0x40
#define SRB_STATUS_AUTOSENSE_VALID          0x80

#define SRB_STATUS(Status) (Status & ~(SRB_STATUS_AUTOSENSE_VALID | SRB_STATUS_QUEUE_FROZEN))

typedef struct _SCSISCAN_CMD {
	ULONG   Reserved1;	
	ULONG   Size;
	ULONG   SrbFlags;				
	UCHAR   CdbLength;
	UCHAR   SenseLength;
	UCHAR	Reserved2;
	UCHAR	Reserved3;
	ULONG   TransferLength;
	UCHAR	Cdb[16];	
	PUCHAR  pSrbStatus;
	PUCHAR	pSenseBuffer;
} SCSISCAN_CMD, *PSCSISCAN_CMD;
	
// Temporarily set to 128. Should be determined by other definition.
#define MAX_STRING 128

typedef struct _SCSISCAN_INFO{
	ULONG   Size;
	ULONG   Flags;
    UCHAR   PortNumber;
    UCHAR   PathId;
    UCHAR   TargetId;
    UCHAR   Lun;
    UCHAR   AdapterName[MAX_STRING];
	ULONG   Reserved;
} SCSISCAN_INFO, *PSCSISCAN_INFO;

#define SCSISCAN_RESERVED         0x000
#define SCSISCAN_CMD_CODE         0x004
#define SCSISCAN_LOCKDEVICE       0x005
#define SCSISCAN_UNLOCKDEVICE     0x006
#define SCSISCAN_SET_TIMEOUT      0x007
#define SCSISCAN_GET_INFO         0x008

//---------------------------------------------------------------------------
// IOCTL definitions.
// Use these definitions when calling DeviceIoControl
//---------------------------------------------------------------------------
#define IOCTL_SCSISCAN_CMD		     CTL_CODE(FILE_DEVICE_SCANNER, SCSISCAN_CMD_CODE,	    METHOD_OUT_DIRECT, FILE_ANY_ACCESS)
#define IOCTL_SCSISCAN_LOCKDEVICE    CTL_CODE(FILE_DEVICE_SCANNER, SCSISCAN_LOCKDEVICE,		METHOD_OUT_DIRECT, FILE_ANY_ACCESS)
#define IOCTL_SCSISCAN_UNLOCKDEVICE  CTL_CODE(FILE_DEVICE_SCANNER, SCSISCAN_UNLOCKDEVICE,	METHOD_OUT_DIRECT, FILE_ANY_ACCESS)
#define IOCTL_SCSISCAN_SET_TIMEOUT   CTL_CODE(FILE_DEVICE_SCANNER, SCSISCAN_SET_TIMEOUT,	METHOD_BUFFERED,   FILE_ANY_ACCESS)
#define IOCTL_SCSISCAN_GET_INFO      CTL_CODE(FILE_DEVICE_SCANNER, SCSISCAN_GET_INFO   ,	METHOD_OUT_DIRECT, FILE_ANY_ACCESS)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
