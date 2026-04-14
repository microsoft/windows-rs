/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1991  Microsoft Corporation

Module Name:

    ntiologc.h

Abstract:

    Constant definitions for the I/O error code log values.

Author:

Revision History:

--*/

#ifndef _NTIOLOGC_
#define _NTIOLOGC_

//
//  Status values are 32 bit values layed out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-------------------------+-------------------------------+
//  |Sev|C|       Facility          |               Code            |
//  +---+-+-------------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//

//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-+-----------------------+-------------------------------+
//  |Sev|C|R|     Facility          |               Code            |
//  +---+-+-+-----------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      R - is a reserved bit
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//
#define FACILITY_IO_ERROR_CODE           0x4
#define FACILITY_MCA_ERROR_CODE          0x5
#define FACILITY_VOLMGR                  0x38


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_INFORMATIONAL    0x1
#define STATUS_SEVERITY_WARNING          0x2
#define STATUS_SEVERITY_ERROR            0x3


//
// MessageId: IO_ERR_INSUFFICIENT_RESOURCES
//
// MessageText:
//
// The driver could not allocate something necessary for the request for %1.
//
#define IO_ERR_INSUFFICIENT_RESOURCES    ((NTSTATUS)0xC0040002L)

//
// MessageId: IO_ERR_DRIVER_ERROR
//
// MessageText:
//
// Driver detected an internal error in its data structures for %1.
//
#define IO_ERR_DRIVER_ERROR              ((NTSTATUS)0xC0040004L)

//
// MessageId: IO_ERR_SEEK_ERROR
//
// MessageText:
//
// The device, %1, had a seek error.
//
#define IO_ERR_SEEK_ERROR                ((NTSTATUS)0xC0040006L)

//
// MessageId: IO_ERR_BAD_BLOCK
//
// MessageText:
//
// The device, %1, has a bad block.
//
#define IO_ERR_BAD_BLOCK                 ((NTSTATUS)0xC0040007L)

//
// MessageId: IO_ERR_TIMEOUT
//
// MessageText:
//
// The device, %1, did not respond within the timeout period.
//
#define IO_ERR_TIMEOUT                   ((NTSTATUS)0xC0040009L)

//
// MessageId: IO_ERR_CONTROLLER_ERROR
//
// MessageText:
//
// The driver detected a controller error on %1.
//
#define IO_ERR_CONTROLLER_ERROR          ((NTSTATUS)0xC004000BL)

//
// MessageId: IO_ERR_NOT_READY
//
// MessageText:
//
// The device, %1, is not ready for access yet.
//
#define IO_ERR_NOT_READY                 ((NTSTATUS)0xC004000FL)

//
// MessageId: IO_ERR_INVALID_REQUEST
//
// MessageText:
//
// The request is incorrectly formatted for %1.
//
#define IO_ERR_INVALID_REQUEST           ((NTSTATUS)0xC0040010L)

//
// MessageId: IO_ERR_RESET
//
// MessageText:
//
// The device, %1, has been reset.
//
#define IO_ERR_RESET                     ((NTSTATUS)0xC0040013L)

//
// MessageId: IO_ERR_BAD_FIRMWARE
//
// MessageText:
//
// The driver has detected a device with old or out-of-date firmware.  The
// device will not be used.
//
#define IO_ERR_BAD_FIRMWARE              ((NTSTATUS)0xC0040019L)

//
// MessageId: IO_WRN_BAD_FIRMWARE
//
// MessageText:
//
// The driver has detected that device %1 has old or out-of-date firmware.
// Reduced performance may result.
//
#define IO_WRN_BAD_FIRMWARE              ((NTSTATUS)0x8004001AL)

//
// MessageId: IO_WRITE_CACHE_ENABLED
//
// MessageText:
//
// The driver detected that the device %1 has its write cache enabled. Data corruption
// may occur.
//
#define IO_WRITE_CACHE_ENABLED           ((NTSTATUS)0x80040020L)

//
// MessageId: IO_RECOVERED_VIA_ECC
//
// MessageText:
//
// Data was recovered using error correction code on device %1.
//
#define IO_RECOVERED_VIA_ECC             ((NTSTATUS)0x80040021L)

//
// MessageId: IO_WRITE_CACHE_DISABLED
//
// MessageText:
//
// The driver disabled the write cache on device %1.
//
#define IO_WRITE_CACHE_DISABLED          ((NTSTATUS)0x80040022L)

//
// MessageId: IO_WARNING_PAGING_FAILURE
//
// MessageText:
//
// An error was detected on device %1 during a paging operation.
//
#define IO_WARNING_PAGING_FAILURE        ((NTSTATUS)0x80040033L)

//
// MessageId: IO_WRN_FAILURE_PREDICTED
//
// MessageText:
//
// The driver has detected that device %1 has predicted that it will fail.
// Immediately back up your data and replace your hard disk drive. A failure
// may be imminent.
//
#define IO_WRN_FAILURE_PREDICTED         ((NTSTATUS)0x80040034L)

//
// MessageId: IO_WARNING_ALLOCATION_FAILED
//
// MessageText:
//
// The driver failed to allocate memory.
//
#define IO_WARNING_ALLOCATION_FAILED     ((NTSTATUS)0x80040038L)

//
// MessageId: IO_WARNING_DUPLICATE_SIGNATURE
//
// MessageText:
//
// The disk signature of disk %2 is equal to the disk signature of disk %3.
//
#define IO_WARNING_DUPLICATE_SIGNATURE   ((NTSTATUS)0x8004003AL)

//
// MessageId: IO_WARNING_DUPLICATE_PATH
//
// MessageText:
//
// Disk %2 will not be used because it is a redundant path for disk %3.
//
#define IO_WARNING_DUPLICATE_PATH        ((NTSTATUS)0x8004003BL)

//
// MessageId: IO_WARNING_WRITE_FUA_PROBLEM
//
// MessageText:
//
// Device %1 is not correctly processing some write requests. In Device Manager, ensure that the Write Cache option is disabled for this device or data corruption may occur.
//
#define IO_WARNING_WRITE_FUA_PROBLEM     ((NTSTATUS)0x80040084L)

//
// MessageId: IO_WARNING_VOLUME_LOST_DISK_EXTENT
//
// MessageText:
//
// Extent %4 on disk %3 that is part of the fault-tolerant volume %2 is no longer accessible.
//
#define IO_WARNING_VOLUME_LOST_DISK_EXTENT ((NTSTATUS)0x8004008EL)

//
// MessageId: IO_WARNING_DEVICE_HAS_INTERNAL_DUMP
//
// MessageText:
//
// The storage device produced an internal dump.
//
#define IO_WARNING_DEVICE_HAS_INTERNAL_DUMP ((NTSTATUS)0x8004008FL)

//
// MessageId: IO_WARNING_SOFT_THRESHOLD_REACHED
//
// MessageText:
//
// Disk %2 has crossed a capacity utilization threshold.
//
#define IO_WARNING_SOFT_THRESHOLD_REACHED ((NTSTATUS)0x80040090L)

//
// MessageId: IO_WARNING_SOFT_THRESHOLD_REACHED_EX
//
// MessageText:
//
// Disk %2 has crossed a capacity utilization threshold and used %3 bytes. When the threshold was crossed, there were %4 bytes of remaining capacity.
//
#define IO_WARNING_SOFT_THRESHOLD_REACHED_EX ((NTSTATUS)0x80040091L)

//
// MessageId: IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_LUN
//
// MessageText:
//
// Disk %2 has crossed a capacity utilization threshold and used %3 bytes. When the threshold was crossed, the disk had %4 bytes of remaining capacity.
//
#define IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_LUN ((NTSTATUS)0x80040092L)

//
// MessageId: IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_POOL
//
// MessageText:
//
// Disk %2 has crossed a capacity utilization threshold and used %3 bytes. When the threshold was crossed, the pool had %4 bytes of remaining capacity.
//
#define IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_POOL ((NTSTATUS)0x80040093L)

//
// MessageId: IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_LUN
//
// MessageText:
//
// Disk %2 has crossed a capacity utilization threshold. When the threshold was crossed, %3 bytes from the pool were used and the disk had %4 bytes of remaining capacity.
//
#define IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_LUN ((NTSTATUS)0x80040094L)

//
// MessageId: IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_POOL
//
// MessageText:
//
// Disk %2 has crossed a capacity utilization threshold. When the threshold was crossed, %3 bytes from the pool were used and the pool had %4 bytes of remaining capacity.
//
#define IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_POOL ((NTSTATUS)0x80040095L)

//
// MessageId: IO_ERROR_DISK_RESOURCES_EXHAUSTED
//
// MessageText:
//
// Disk %2 has reached a logical block provisioning permanent resource exhaustion condition.
//
#define IO_ERROR_DISK_RESOURCES_EXHAUSTED ((NTSTATUS)0xC0040096L)

//
// MessageId: IO_WARNING_DISK_CAPACITY_CHANGED
//
// MessageText:
//
// The capacity of Disk %2 has changed.
//
#define IO_WARNING_DISK_CAPACITY_CHANGED ((NTSTATUS)0x80040097L)

//
// MessageId: IO_WARNING_DISK_PROVISIONING_TYPE_CHANGED
//
// MessageText:
//
// The logical block provisioning type for Disk %2 has changed.
//
#define IO_WARNING_DISK_PROVISIONING_TYPE_CHANGED ((NTSTATUS)0x80040098L)

//
// MessageId: IO_WARNING_IO_OPERATION_RETRIED
//
// MessageText:
//
// The IO operation at logical block address %2 for Disk %3 (PDO name: %4) was retried.
//
#define IO_WARNING_IO_OPERATION_RETRIED  ((NTSTATUS)0x80040099L)

//
// MessageId: IO_ERROR_IO_HARDWARE_ERROR
//
// MessageText:
//
// The IO operation at logical block address %2 for Disk %3 (PDO name: %4) failed due to a hardware error.
//
#define IO_ERROR_IO_HARDWARE_ERROR       ((NTSTATUS)0xC004009AL)

//
// MessageId: IO_WARNING_COMPLETION_TIME
//
// MessageText:
//
// The IO operation at logical block address %2 for Disk %3 took %4 milliseconds. This was longer than the maximum expected time of %5 milliseconds.
//
#define IO_WARNING_COMPLETION_TIME       ((NTSTATUS)0x8004009BL)

//
// MessageId: IO_WARNING_DISK_SURPRISE_REMOVED
//
// MessageText:
//
// Disk %2 has been surprise removed.
//
#define IO_WARNING_DISK_SURPRISE_REMOVED ((NTSTATUS)0x8004009DL)

//
// MessageId: IO_WARNING_REPEATED_DISK_GUID
//
// MessageText:
//
// Disk %2 has the same disk identifiers as one or more disks connected to the system. Go to Microsoft's support website (http://support.microsoft.com) and search for KB2983588 to resolve the issue.
//
#define IO_WARNING_REPEATED_DISK_GUID    ((NTSTATUS)0x8004009EL)

//
// MessageId: IO_WARNING_DISK_FIRMWARE_UPDATED
//
// MessageText:
//
// Firmware update for Disk %2 is completed.
//
#define IO_WARNING_DISK_FIRMWARE_UPDATED ((NTSTATUS)0x4004009FL)

////////////////////////////////////////////////////////
// Facility IO
//
//
// SUCCESS codes
//
//
// MessageId: IO_ERR_RETRY_SUCCEEDED
//
// MessageText:
//
// A retry was successful on %1.
//
#define IO_ERR_RETRY_SUCCEEDED           ((NTSTATUS)0x00040001L)

//
// MessageId: IO_DUMP_CREATION_SUCCESS
//
// MessageText:
//
// Dump file generation succeded.
//
#define IO_DUMP_CREATION_SUCCESS         ((NTSTATUS)0x000400A2L)

//
// INFORMATIONAL codes
//
//
// MessageId: IO_FILE_QUOTA_THRESHOLD
//
// MessageText:
//
// A user hit their quota threshold on volume %2.
//
#define IO_FILE_QUOTA_THRESHOLD          ((NTSTATUS)0x40040024L)

//
// MessageId: IO_FILE_QUOTA_LIMIT
//
// MessageText:
//
// A user hit their quota limit on volume %2.
//
#define IO_FILE_QUOTA_LIMIT              ((NTSTATUS)0x40040025L)

//
// MessageId: IO_FILE_QUOTA_STARTED
//
// MessageText:
//
// The system has started rebuilding the user disk quota information on
// device %1 with label "%2".
//
#define IO_FILE_QUOTA_STARTED            ((NTSTATUS)0x40040026L)

//
// MessageId: IO_FILE_QUOTA_SUCCEEDED
//
// MessageText:
//
// The system has successfully rebuilt the user disk quota information on
// device %1 with label "%2".
//
#define IO_FILE_QUOTA_SUCCEEDED          ((NTSTATUS)0x40040027L)

//
// MessageId: IO_INFO_THROTTLE_COMPLETE
//
// MessageText:
//
// The driver for device %1 delayed non-paging Io requests for %2 ms to recover from a low memory condition.
//
#define IO_INFO_THROTTLE_COMPLETE        ((NTSTATUS)0x40040077L)

//
// MessageId: IO_CDROM_EXCLUSIVE_LOCK
//
// MessageText:
//
// Device %1 is locked for exclusive access.
//
#define IO_CDROM_EXCLUSIVE_LOCK          ((NTSTATUS)0x40040085L)


// codes 0x9f defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_WARNING_ADAPTER_FIRMWARE_UPDATED
//
// MessageText:
//
// Firmware update for Adapter %1 is completed.
//
#define IO_WARNING_ADAPTER_FIRMWARE_UPDATED ((NTSTATUS)0x400400A0L)

//
// WARNING codes
//

// codes 0x1a, 0x20-0x22 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_FILE_QUOTA_FAILED
//
// MessageText:
//
// The system has encounted an error rebuilding the user disk quota
// information on device %1 with label "%2".
//
#define IO_FILE_QUOTA_FAILED             ((NTSTATUS)0x80040028L)

//
// MessageId: IO_LOST_DELAYED_WRITE
//
// MessageText:
//
// {Delayed Write Failed}
// Windows was unable to save all the data for the file %2. The data has been lost.
// This error may be caused by a failure of your computer hardware or network connection. Please try to save this file elsewhere.
//
#define IO_LOST_DELAYED_WRITE            ((NTSTATUS)0x80040032L)


// codes 0x33, 0x34 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_WARNING_INTERRUPT_STILL_PENDING
//
// MessageText:
//
// A pending interrupt was detected on device %1 during a timeout operation.  A
// large number of these warnings may indicate that the system is not correctly
// receiving or processing interrupts from the device.
//
#define IO_WARNING_INTERRUPT_STILL_PENDING ((NTSTATUS)0x80040035L)

//
// MessageId: IO_DRIVER_CANCEL_TIMEOUT
//
// MessageText:
//
// An Io Request to the device %1 did not complete or canceled within the
// specific timeout. This can occur if the device driver does not set a
// cancel routine for a given IO request packet.
//
#define IO_DRIVER_CANCEL_TIMEOUT         ((NTSTATUS)0x80040036L)


// codes 0x38 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_WARNING_LOG_FLUSH_FAILED
//
// MessageText:
//
// The system failed to flush data to the transaction log. Corruption may occur.
//
#define IO_WARNING_LOG_FLUSH_FAILED      ((NTSTATUS)0x80040039L)


// codes 0x3a, 0x3b defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_WARNING_BUS_RESET
//
// MessageText:
//
// The driver for device %1 performed a bus reset upon request.
//
#define IO_WARNING_BUS_RESET             ((NTSTATUS)0x80040076L)

//
// MessageId: IO_WARNING_RESET
//
// MessageText:
//
// Reset to device, %1, was issued.
//
#define IO_WARNING_RESET                 ((NTSTATUS)0x80040081L)


// codes 0x84 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_LOST_DELAYED_WRITE_NETWORK_DISCONNECTED
//
// MessageText:
//
// {Delayed Write Failed}
// Windows was unable to save all the data for the file %2; the data has been lost.
// This error may be caused by network connectivity issues. Please try to save this file elsewhere.
//
#define IO_LOST_DELAYED_WRITE_NETWORK_DISCONNECTED ((NTSTATUS)0x8004008BL)

//
// MessageId: IO_LOST_DELAYED_WRITE_NETWORK_SERVER_ERROR
//
// MessageText:
//
// {Delayed Write Failed}
// Windows was unable to save all the data for the file %2; the data has been lost.
// This error was returned by the server on which the file exists. Please try to save this file elsewhere.
//
#define IO_LOST_DELAYED_WRITE_NETWORK_SERVER_ERROR ((NTSTATUS)0x8004008CL)

//
// MessageId: IO_LOST_DELAYED_WRITE_NETWORK_LOCAL_DISK_ERROR
//
// MessageText:
//
// {Delayed Write Failed}
// Windows was unable to save all the data for the file %2; the data has been lost.
// This error may be caused if the device has been removed or the media is write-protected.
//
#define IO_LOST_DELAYED_WRITE_NETWORK_LOCAL_DISK_ERROR ((NTSTATUS)0x8004008DL)


// codes 0x8e-0x95, 0x97-0x99, 0x9b defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_WARNING_DUMP_DISABLED_DEVICE_GONE
//
// MessageText:
//
// Crash dump is disabled. Crash dump device is not present in the system.
//
#define IO_WARNING_DUMP_DISABLED_DEVICE_GONE ((NTSTATUS)0x8004009CL)


// codes 0x9e defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// ERROR codes
//

// codes 0x2 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_CONFIGURATION_ERROR
//
// MessageText:
//
// Driver or device is incorrectly configured for %1.
//
#define IO_ERR_CONFIGURATION_ERROR       ((NTSTATUS)0xC0040003L)


// codes 0x4 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_PARITY
//
// MessageText:
//
// A parity error was detected on %1.
//
#define IO_ERR_PARITY                    ((NTSTATUS)0xC0040005L)


// codes 0x6,0x7 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_OVERRUN_ERROR
//
// MessageText:
//
// An overrun occurred on %1.
//
#define IO_ERR_OVERRUN_ERROR             ((NTSTATUS)0xC0040008L)


// codes 0x9 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_SEQUENCE
//
// MessageText:
//
// The driver detected an unexpected sequence by the device, %1.
//
#define IO_ERR_SEQUENCE                  ((NTSTATUS)0xC004000AL)


// codes 0xb defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_INTERNAL_ERROR
//
// MessageText:
//
// The driver detected an internal driver error on %1.
//
#define IO_ERR_INTERNAL_ERROR            ((NTSTATUS)0xC004000CL)

//
// MessageId: IO_ERR_INCORRECT_IRQL
//
// MessageText:
//
// The driver was configured with an incorrect interrupt for %1.
//
#define IO_ERR_INCORRECT_IRQL            ((NTSTATUS)0xC004000DL)

//
// MessageId: IO_ERR_INVALID_IOBASE
//
// MessageText:
//
// The driver was configured with an invalid I/O base address for %1.
//
#define IO_ERR_INVALID_IOBASE            ((NTSTATUS)0xC004000EL)


// codes 0xf, 0x10 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_VERSION
//
// MessageText:
//
// The wrong version of the driver has been loaded.
//
#define IO_ERR_VERSION                   ((NTSTATUS)0xC0040011L)

//
// MessageId: IO_ERR_LAYERED_FAILURE
//
// MessageText:
//
// The driver beneath this one has failed in some way for %1.
//
#define IO_ERR_LAYERED_FAILURE           ((NTSTATUS)0xC0040012L)


// codes 0x13 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_PROTOCOL
//
// MessageText:
//
// A transport driver received a frame which violated the protocol.
//
#define IO_ERR_PROTOCOL                  ((NTSTATUS)0xC0040014L)

//
// MessageId: IO_ERR_MEMORY_CONFLICT_DETECTED
//
// MessageText:
//
// A conflict has been detected between two drivers which claimed two overlapping
// memory regions.
// Driver %2, with device <%3>, claimed a memory range with starting address
// in data address 0x28 and 0x2c, and length in data address 0x30.
//
#define IO_ERR_MEMORY_CONFLICT_DETECTED  ((NTSTATUS)0xC0040015L)

//
// MessageId: IO_ERR_PORT_CONFLICT_DETECTED
//
// MessageText:
//
// A conflict has been detected between two drivers which claimed two overlapping
// Io port regions.
// Driver %2, with device <%3>, claimed an IO port range with starting address
// in data address 0x28 and 0x2c, and length in data address 0x30.
//
#define IO_ERR_PORT_CONFLICT_DETECTED    ((NTSTATUS)0xC0040016L)

//
// MessageId: IO_ERR_DMA_CONFLICT_DETECTED
//
// MessageText:
//
// A conflict has been detected between two drivers which claimed equivalent DMA
// channels.
// Driver %2, with device <%3>, claimed the DMA Channel in data address 0x28, with
// optinal port in data address 0x2c.
//
#define IO_ERR_DMA_CONFLICT_DETECTED     ((NTSTATUS)0xC0040017L)

//
// MessageId: IO_ERR_IRQ_CONFLICT_DETECTED
//
// MessageText:
//
// A conflict has been detected between two drivers which claimed equivalent IRQs.
// Driver %2, with device <%3>, claimed an interrupt with Level in data address
// 0x28, vector in data address 0x2c and Affinity in data address 0x30.
//
#define IO_ERR_IRQ_CONFLICT_DETECTED     ((NTSTATUS)0xC0040018L)


// codes 0x19 defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERR_DMA_RESOURCE_CONFLICT
//
// MessageText:
//
// The device could not allocate one or more required resources due to conflicts
// with other devices.  The device DMA setting of '%2' could not be
// satisified due to a conflict with Driver '%3'.
//
#define IO_ERR_DMA_RESOURCE_CONFLICT     ((NTSTATUS)0xC004001BL)

//
// MessageId: IO_ERR_INTERRUPT_RESOURCE_CONFLICT
//
// MessageText:
//
// The device could not allocate one or more required resources due to conflicts
// with other devices.  The device interrupt setting of '%2' could not be
// satisified due to a conflict with Driver '%3'.
//
#define IO_ERR_INTERRUPT_RESOURCE_CONFLICT ((NTSTATUS)0xC004001CL)

//
// MessageId: IO_ERR_MEMORY_RESOURCE_CONFLICT
//
// MessageText:
//
// The device could not allocate one or more required resources due to conflicts
// with other devices.  The device memory setting of '%2' could not be
// satisified due to a conflict with Driver '%3'.
//
#define IO_ERR_MEMORY_RESOURCE_CONFLICT  ((NTSTATUS)0xC004001DL)

//
// MessageId: IO_ERR_PORT_RESOURCE_CONFLICT
//
// MessageText:
//
// The device could not allocate one or more required resources due to conflicts
// with other devices.  The device port setting of '%2' could not be
// satisified due to a conflict with Driver '%3'.
//
#define IO_ERR_PORT_RESOURCE_CONFLICT    ((NTSTATUS)0xC004001EL)

//
// MessageId: IO_BAD_BLOCK_WITH_NAME
//
// MessageText:
//
// The file %2 on device %1 contains a bad disk block.
//
#define IO_BAD_BLOCK_WITH_NAME           ((NTSTATUS)0xC004001FL)

//
// MessageId: IO_FILE_SYSTEM_CORRUPT
//
// MessageText:
//
// The file system structure on the disk is corrupt and unusable.
// Please run the chkdsk utility on the device %1 with label "%2".
//
#define IO_FILE_SYSTEM_CORRUPT           ((NTSTATUS)0xC0040029L)

//
// MessageId: IO_FILE_QUOTA_CORRUPT
//
// MessageText:
//
// The user disk quota information is unusable.
// To ensure accuracy, the file system quota information on the device %1 with label "%2" will
// be rebuilt.
//
#define IO_FILE_QUOTA_CORRUPT            ((NTSTATUS)0xC004002AL)

//
// MessageId: IO_SYSTEM_SLEEP_FAILED
//
// MessageText:
//
// The system sleep operation failed
//
#define IO_SYSTEM_SLEEP_FAILED           ((NTSTATUS)0xC004002BL)

//
// MessageId: IO_DUMP_POINTER_FAILURE
//
// MessageText:
//
// The system could not get file retrieval pointers for the dump file.
//
#define IO_DUMP_POINTER_FAILURE          ((NTSTATUS)0xC004002CL)

//
// MessageId: IO_DUMP_DRIVER_LOAD_FAILURE
//
// MessageText:
//
// The system could not sucessfully load the crash dump driver.
//
#define IO_DUMP_DRIVER_LOAD_FAILURE      ((NTSTATUS)0xC004002DL)

//
// MessageId: IO_DUMP_INITIALIZATION_FAILURE
//
// MessageText:
//
// Crash dump initialization failed!
//
#define IO_DUMP_INITIALIZATION_FAILURE   ((NTSTATUS)0xC004002EL)

//
// MessageId: IO_DUMP_DUMPFILE_CONFLICT
//
// MessageText:
//
// A valid crash dump was found in the paging file while trying to configure
// a direct dump. Direct dump is disabled! This occurs when the direct dump
// option is set in the registry but a stop error occured before configuration
// completed
//
#define IO_DUMP_DUMPFILE_CONFLICT        ((NTSTATUS)0xC004002FL)

//
// MessageId: IO_DUMP_DIRECT_CONFIG_FAILED
//
// MessageText:
//
// Direct dump configuration failed. Validate the filename and make sure the target device
// is not a Fault Tolerant set member, remote, or floppy device. The failure may
// be because there is not enough room on the dump device to create the dump file.
//
#define IO_DUMP_DIRECT_CONFIG_FAILED     ((NTSTATUS)0xC0040030L)

//
// MessageId: IO_DUMP_PAGE_CONFIG_FAILED
//
// MessageText:
//
// Configuring the Page file for crash dump failed. Make sure there is a page
// file on the boot partition and that is large enough to contain all physical
// memory.
//
#define IO_DUMP_PAGE_CONFIG_FAILED       ((NTSTATUS)0xC0040031L)

//
// MessageId: IO_FILE_SYSTEM_CORRUPT_WITH_NAME
//
// MessageText:
//
// The file system structure on the disk is corrupt and unusable.
// Please run the chkdsk utility on the volume %2.
//
#define IO_FILE_SYSTEM_CORRUPT_WITH_NAME ((NTSTATUS)0xC0040037L)

//
// MessageId: IO_ERR_THREAD_STUCK_IN_DEVICE_DRIVER
//
// MessageText:
//
// The driver %3 for the %2 device %1 got stuck in an infinite loop. This
// usually indicates a problem with the device itself or with the device
// driver programming the hardware incorrectly. Please check with your
// hardware device vendor for any driver updates.
//
#define IO_ERR_THREAD_STUCK_IN_DEVICE_DRIVER ((NTSTATUS)0xC004006CL)

//
// MessageId: IO_ERR_PORT_TIMEOUT
//
// MessageText:
//
// The driver for device %1 detected a port timeout due to prolonged inactivity. All associated busses were reset in an effort to clear the condition.
//
#define IO_ERR_PORT_TIMEOUT              ((NTSTATUS)0xC0040075L)


// codes 0x96, 0x9a defined in minkernel\nlsmsg_base\ntiologc_base.mc

//
// MessageId: IO_ERROR_DUMP_CREATION_ERROR
//
// MessageText:
//
// Dump file creation failed due to error during dump creation. BugCheckProgress was: %2
//
#define IO_ERROR_DUMP_CREATION_ERROR     ((NTSTATUS)0xC00400A1L)

//
// MessageId: IO_DUMP_CALLBACK_EXCEPTION
//
// MessageText:
//
// A callback exception was logged during dump. See DumpStack.log for details.
//
#define IO_DUMP_CALLBACK_EXCEPTION       ((NTSTATUS)0xC00400A3L)

//
// MessageId: IO_DUMP_INIT_DEDICATED_DUMP_FAILURE
//
// MessageText:
//
// Configuring the dedicated dump for crash dump failed. Make sure the dedicated
// dump file path is set correctly, or the disk has enough space to create the dump file.
//
#define IO_DUMP_INIT_DEDICATED_DUMP_FAILURE ((NTSTATUS)0xC00400A4L)

////////////////////////////////////////////////////////
// Facility MCA
//
//
// INFORMATIONAL codes
//
//
// MessageId: MCA_INFO_CPU_THERMAL_THROTTLING_REMOVED
//
// MessageText:
//
// Machine Check Event reported is a CPU thermal throttling event reported from CPU %1. The CPU has dropped below the temperature limit and throttling has been removed. %2 additional error(s) are contained within the record.
//
#define MCA_INFO_CPU_THERMAL_THROTTLING_REMOVED ((NTSTATUS)0x40050070L)

//
// MessageId: MCA_INFO_NO_MORE_CORRECTED_ERROR_LOGS
//
// MessageText:
//
// The maximum number of Machine Check Event corrected error events that can be saved to the Event Log has been reached. Logging of these events has been disabled.
//
#define MCA_INFO_NO_MORE_CORRECTED_ERROR_LOGS ((NTSTATUS)0x40050073L)

//
// MessageId: MCA_INFO_MEMORY_PAGE_MARKED_BAD
//
// MessageText:
//
// The memory page at physical address %1 has encountered multiple corrected hardware error events. As a result it will no longer be used by Windows.
//
#define MCA_INFO_MEMORY_PAGE_MARKED_BAD  ((NTSTATUS)0x40050074L)

//
// WARNING codes
//
//
// MessageId: MCA_WARNING_CACHE
//
// MessageText:
//
// Machine Check Event reported is a corrected level %3 Cache error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_CACHE                ((NTSTATUS)0x8005003CL)

//
// MessageId: MCA_WARNING_TLB
//
// MessageText:
//
// Machine Check Event reported is a corrected level %3 translation Buffer error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_TLB                  ((NTSTATUS)0x8005003EL)

//
// MessageId: MCA_WARNING_CPU_BUS
//
// MessageText:
//
// Machine Check Event reported is a corrected External/Internal bus error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_CPU_BUS              ((NTSTATUS)0x80050040L)

//
// MessageId: MCA_WARNING_REGISTER_FILE
//
// MessageText:
//
// Machine Check Event reported is a corrected internal CPU register access error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_REGISTER_FILE        ((NTSTATUS)0x80050042L)

//
// MessageId: MCA_WARNING_MAS
//
// MessageText:
//
// Machine Check Event reported is a corrected Micro Architecture Structure error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_MAS                  ((NTSTATUS)0x80050044L)

//
// MessageId: MCA_WARNING_MEM_UNKNOWN
//
// MessageText:
//
// Machine Check Event reported is a corrected ECC memory error at an unknown physical address reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_MEM_UNKNOWN          ((NTSTATUS)0x80050046L)

//
// MessageId: MCA_WARNING_MEM_1_2
//
// MessageText:
//
// Machine Check Event reported is a corrected ECC memory error at physical address %3 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_MEM_1_2              ((NTSTATUS)0x80050048L)

//
// MessageId: MCA_WARNING_MEM_1_2_5
//
// MessageText:
//
// Machine Check Event reported is a corrected ECC memory error at physical address %3 on memory module %4 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_MEM_1_2_5            ((NTSTATUS)0x8005004AL)

//
// MessageId: MCA_WARNING_MEM_1_2_5_4
//
// MessageText:
//
// Machine Check Event reported is a corrected ECC memory error at physical address %3 on memory module %4 on memory card %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_MEM_1_2_5_4          ((NTSTATUS)0x8005004CL)

//
// MessageId: MCA_WARNING_SYSTEM_EVENT
//
// MessageText:
//
// Machine Check Event reported is a corrected System Event error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_SYSTEM_EVENT         ((NTSTATUS)0x8005004EL)

//
// MessageId: MCA_WARNING_PCI_BUS_PARITY
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus Parity error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_PARITY       ((NTSTATUS)0x80050050L)

//
// MessageId: MCA_WARNING_PCI_BUS_PARITY_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus Parity error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_PARITY_NO_INFO ((NTSTATUS)0x80050052L)

//
// MessageId: MCA_WARNING_PCI_BUS_SERR
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus SERR error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_SERR         ((NTSTATUS)0x80050054L)

//
// MessageId: MCA_WARNING_PCI_BUS_SERR_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus SERR error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_SERR_NO_INFO ((NTSTATUS)0x80050056L)

//
// MessageId: MCA_WARNING_PCI_BUS_MASTER_ABORT
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus Master abort error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_MASTER_ABORT ((NTSTATUS)0x80050058L)

//
// MessageId: MCA_WARNING_PCI_BUS_MASTER_ABORT_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus Master abort error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_MASTER_ABORT_NO_INFO ((NTSTATUS)0x8005005AL)

//
// MessageId: MCA_WARNING_PCI_BUS_TIMEOUT
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus Timeout error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_TIMEOUT      ((NTSTATUS)0x8005005CL)

//
// MessageId: MCA_WARNING_PCI_BUS_TIMEOUT_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI bus Timeout error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_TIMEOUT_NO_INFO ((NTSTATUS)0x8005005EL)

//
// MessageId: MCA_WARNING_PCI_BUS_UNKNOWN
//
// MessageText:
//
// Machine Check Event reported is an unknown corrected PCI bus error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_BUS_UNKNOWN      ((NTSTATUS)0x80050060L)

//
// MessageId: MCA_WARNING_PCI_DEVICE
//
// MessageText:
//
// Machine Check Event reported is a corrected PCI component error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PCI_DEVICE           ((NTSTATUS)0x80050062L)

//
// MessageId: MCA_WARNING_SMBIOS
//
// MessageText:
//
// Machine Check Event reported is a corrected SMBIOS Device Type %3 error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_SMBIOS               ((NTSTATUS)0x80050064L)

//
// MessageId: MCA_WARNING_PLATFORM_SPECIFIC
//
// MessageText:
//
// Machine Check Event reported is a corrected Platform Specific error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_PLATFORM_SPECIFIC    ((NTSTATUS)0x80050066L)

//
// MessageId: MCA_WARNING_UNKNOWN
//
// MessageText:
//
// Machine Check Event reported is a corrected error reported to CPU %1.
//
#define MCA_WARNING_UNKNOWN              ((NTSTATUS)0x80050068L)

//
// MessageId: MCA_WARNING_UNKNOWN_NO_CPU
//
// MessageText:
//
// Machine Check Event reported is a corrected error.
//
#define MCA_WARNING_UNKNOWN_NO_CPU       ((NTSTATUS)0x8005006AL)

//
// MessageId: MCA_WARNING_CMC_THRESHOLD_EXCEEDED
//
// MessageText:
//
// Corrected Machine Check Interrupt threshold exceeded. Interrupt has been disabled. Polling mode has been enabled.
//
#define MCA_WARNING_CMC_THRESHOLD_EXCEEDED ((NTSTATUS)0x8005006DL)

//
// MessageId: MCA_WARNING_CPE_THRESHOLD_EXCEEDED
//
// MessageText:
//
// Corrected Platform Error Interrupt threshold exceeded. Interrupt has been disabled. Polling mode has been enabled.
//
#define MCA_WARNING_CPE_THRESHOLD_EXCEEDED ((NTSTATUS)0x8005006EL)

//
// MessageId: MCA_WARNING_CPU_THERMAL_THROTTLED
//
// MessageText:
//
// Machine Check Event reported is a CPU thermal throttling event reported from CPU %1. The CPU has exceeded the temperature limit and has been throttled down. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_CPU_THERMAL_THROTTLED ((NTSTATUS)0x8005006FL)

//
// MessageId: MCA_WARNING_CPU
//
// MessageText:
//
// Machine Check Event reported is a corrected CPU error reported from CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_WARNING_CPU                  ((NTSTATUS)0x80050071L)

//
// ERROR codes
//
//
// MessageId: MCA_ERROR_CACHE
//
// MessageText:
//
// Machine Check Event reported is a fatal level %3 Cache error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_CACHE                  ((NTSTATUS)0xC005003DL)

//
// MessageId: MCA_ERROR_TLB
//
// MessageText:
//
// Machine Check Event reported is a fatal level %3 translation Buffer error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_TLB                    ((NTSTATUS)0xC005003FL)

//
// MessageId: MCA_ERROR_CPU_BUS
//
// MessageText:
//
// Machine Check Event reported is a fatal External/Internal bus error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_CPU_BUS                ((NTSTATUS)0xC0050041L)

//
// MessageId: MCA_ERROR_REGISTER_FILE
//
// MessageText:
//
// Machine Check Event reported is a fatal internal CPU register access error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_REGISTER_FILE          ((NTSTATUS)0xC0050043L)

//
// MessageId: MCA_ERROR_MAS
//
// MessageText:
//
// Machine Check Event reported is a fatal Micro Architecture Structure error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_MAS                    ((NTSTATUS)0xC0050045L)

//
// MessageId: MCA_ERROR_MEM_UNKNOWN
//
// MessageText:
//
// Machine Check Event reported is a fatal ECC memory error at an unknown physical address reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_MEM_UNKNOWN            ((NTSTATUS)0xC0050047L)

//
// MessageId: MCA_ERROR_MEM_1_2
//
// MessageText:
//
// Machine Check Event reported is a fatal ECC memory error at physical address %3 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_MEM_1_2                ((NTSTATUS)0xC0050049L)

//
// MessageId: MCA_ERROR_MEM_1_2_5
//
// MessageText:
//
// Machine Check Event reported is a fatal ECC memory error at physical address %3 on memory module %4 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_MEM_1_2_5              ((NTSTATUS)0xC005004BL)

//
// MessageId: MCA_ERROR_MEM_1_2_5_4
//
// MessageText:
//
// Machine Check Event reported is a fatal ECC memory error at physical address %3 on memory module %4 on memory card %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_MEM_1_2_5_4            ((NTSTATUS)0xC005004DL)

//
// MessageId: MCA_ERROR_SYSTEM_EVENT
//
// MessageText:
//
// Machine Check Event reported is a fatal System Event error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_SYSTEM_EVENT           ((NTSTATUS)0xC005004FL)

//
// MessageId: MCA_ERROR_PCI_BUS_PARITY
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus Parity error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_PARITY         ((NTSTATUS)0xC0050051L)

//
// MessageId: MCA_ERROR_PCI_BUS_PARITY_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus Parity error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_PARITY_NO_INFO ((NTSTATUS)0xC0050053L)

//
// MessageId: MCA_ERROR_PCI_BUS_SERR
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus SERR error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_SERR           ((NTSTATUS)0xC0050055L)

//
// MessageId: MCA_ERROR_PCI_BUS_SERR_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus SERR error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_SERR_NO_INFO   ((NTSTATUS)0xC0050057L)

//
// MessageId: MCA_ERROR_PCI_BUS_MASTER_ABORT
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus Master abort error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_MASTER_ABORT   ((NTSTATUS)0xC0050059L)

//
// MessageId: MCA_ERROR_PCI_BUS_MASTER_ABORT_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus Master abort error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_MASTER_ABORT_NO_INFO ((NTSTATUS)0xC005005BL)

//
// MessageId: MCA_ERROR_PCI_BUS_TIMEOUT
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus Timeout error during a transaction type %3 at address %4 on PCI bus %5 reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_TIMEOUT        ((NTSTATUS)0xC005005DL)

//
// MessageId: MCA_ERROR_PCI_BUS_TIMEOUT_NO_INFO
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI bus Timeout error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_TIMEOUT_NO_INFO ((NTSTATUS)0xC005005FL)

//
// MessageId: MCA_ERROR_PCI_BUS_UNKNOWN
//
// MessageText:
//
// Machine Check Event reported is an unknown fatal PCI bus error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_BUS_UNKNOWN        ((NTSTATUS)0xC0050061L)

//
// MessageId: MCA_ERROR_PCI_DEVICE
//
// MessageText:
//
// Machine Check Event reported is a fatal PCI component error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PCI_DEVICE             ((NTSTATUS)0xC0050063L)

//
// MessageId: MCA_ERROR_SMBIOS
//
// MessageText:
//
// Machine Check Event reported is a fatal SMBIOS Device Type %3 error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_SMBIOS                 ((NTSTATUS)0xC0050065L)

//
// MessageId: MCA_ERROR_PLATFORM_SPECIFIC
//
// MessageText:
//
// Machine Check Event reported is a fatal Platform Specific error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_PLATFORM_SPECIFIC      ((NTSTATUS)0xC0050067L)

//
// MessageId: MCA_ERROR_UNKNOWN
//
// MessageText:
//
// Machine Check Event reported is a fatal error reported to CPU %1.
//
#define MCA_ERROR_UNKNOWN                ((NTSTATUS)0xC0050069L)

//
// MessageId: MCA_ERROR_UNKNOWN_NO_CPU
//
// MessageText:
//
// Machine Check Event reported is a fatal error.
//
#define MCA_ERROR_UNKNOWN_NO_CPU         ((NTSTATUS)0xC005006BL)

//
// MessageId: MCA_ERROR_CPU
//
// MessageText:
//
// Machine Check Event reported is a fatal CPU error reported to CPU %1. %2 additional error(s) are contained within the record.
//
#define MCA_ERROR_CPU                    ((NTSTATUS)0xC0050072L)

//
// MessageId: MCA_MEMORYHIERARCHY_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal memory hierarchy error.%r Trasaction Type: %1%r Memory Hierarchy Level: %2%r Request Type: %3%r Address: %4
//
#define MCA_MEMORYHIERARCHY_ERROR        ((NTSTATUS)0xC0050078L)

//
// MessageId: MCA_TLB_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal TLB error.%r Transaction Type: %1%r Memory Hierarchy Level: %2%r Address: %3
//
#define MCA_TLB_ERROR                    ((NTSTATUS)0xC0050079L)

//
// MessageId: MCA_BUS_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal Bus or Interconnect error.%r Memory Hierarchy Level: %1%r Participation: %2%r Request Type: %3%r Memory/IO: %4%r Address: %5
//
#define MCA_BUS_ERROR                    ((NTSTATUS)0xC005007AL)

//
// MessageId: MCA_BUS_TIMEOUT_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal Bus or Interconnect timeout error.%r Memory Hierarchy Level: %1%r Participation: %2%r Request Type: %3%r Memory/IO: %4%r Address: %5
//
#define MCA_BUS_TIMEOUT_ERROR            ((NTSTATUS)0xC005007BL)

//
// MessageId: MCA_INTERNALTIMER_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal internal watchdog timer error.
//
#define MCA_INTERNALTIMER_ERROR          ((NTSTATUS)0xC005007CL)

//
// MessageId: MCA_MICROCODE_ROM_PARITY_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal microsoft ROM parity error.
//
#define MCA_MICROCODE_ROM_PARITY_ERROR   ((NTSTATUS)0xC005007EL)

//
// MessageId: MCA_EXTERNAL_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal condition. A processor received an external signal that an unrecoverable error has occurred.
//
#define MCA_EXTERNAL_ERROR               ((NTSTATUS)0xC005007FL)

//
// MessageId: MCA_FRC_ERROR
//
// MessageText:
//
// Machine Check Event reported is a fatal functional redundancy check error.
//
#define MCA_FRC_ERROR                    ((NTSTATUS)0xC0050080L)

////////////////////////////////////////////////////////
// Facility VOLMGR
//
//
// WARNING codes
//
//
// MessageId: VOLMGR_KSR_ERROR
//
// MessageText:
//
// There was an error validating KSR data - operation %2 status %3.
//
#define VOLMGR_KSR_ERROR                 ((NTSTATUS)0x80380001L)

//
// MessageId: VOLMGR_KSR_READ_ERROR
//
// MessageText:
//
// Failed to read KSR data - disk %2 sector %3 operation %4 status %5.
//
#define VOLMGR_KSR_READ_ERROR            ((NTSTATUS)0x80380002L)

//
// MessageId: VOLMGR_KSR_BYPASS
//
// MessageText:
//
// A %2 operation took place during KSR - disk %3 offset %4.
//
#define VOLMGR_KSR_BYPASS                ((NTSTATUS)0x80380003L)

#endif /* _NTIOLOGC_ */

