/****************************************************************************
*
*  (C) COPYRIGHT 2004, MICROSOFT CORP.
*
*  VERSION:     1.0
*
*  DESCRIPTION:
*    Structures and constants needed to issue vendor-specific Media Transfer
*    Protocol (MTP) commands through DeviceIOControl mechanism.
*
*****************************************************************************/
#pragma once
#include <winapifamily.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


//
// Pass this value in the dwIoControlCode argument of IWMDMDevice3::DeviceIoControl
// to execute a direct MTP command
//
#define IOCTL_MTP_CUSTOM_COMMAND    0x3150544d

//
// MTP command request
//
const DWORD MTP_COMMAND_MAX_PARAMS  = 5;
const DWORD MTP_RESPONSE_MAX_PARAMS = 5;

//
// MTP response codes
//
const WORD MTP_RESPONSE_OK = 0x2001;

#pragma pack(push, Old, 1)

typedef struct _MTP_COMMAND_DATA_IN
{
    WORD    OpCode;                         // Opcode
    DWORD   NumParams;                      // Number of parameters passed in
    DWORD   Params[MTP_COMMAND_MAX_PARAMS]; // Parameters to the command
    DWORD   NextPhase;                      // Indicates whether the command has a read data,
                                            // write data, or no data phase.
    DWORD   CommandWriteDataSize;           // Number of bytes contained in CommandWriteData.
    BYTE    CommandWriteData[1];            // Optional first byte of data to
                                            // write to the device if NextPhase is MTP_NEXTPHASE_WRITE_DATA
} MTP_COMMAND_DATA_IN, *PMTP_COMMAND_DATA_IN;

//
// MTP response block
//
typedef struct _MTP_COMMAND_DATA_OUT
{
    WORD    ResponseCode;                       // Response code
    DWORD   NumParams;                          // Number of parameters for this response
    DWORD   Params[MTP_RESPONSE_MAX_PARAMS];    // Parameters of the response
    DWORD   CommandReadDataSize;                // Number of bytes contained in CommandReadData.
    BYTE    CommandReadData[1];                 // Optional first byte of data to
                                                // read from the device if 
                                                // MTP_COMMAND_DATA_IN::NextPhase was MTP_NEXTPHASE_READ_DATA
} MTP_COMMAND_DATA_OUT, *PMTP_COMMAND_DATA_OUT;

#pragma pack(pop, Old)

//
// Handy structure size constants
//
#define SIZEOF_REQUIRED_COMMAND_DATA_IN (sizeof(MTP_COMMAND_DATA_IN) - 1)
#define SIZEOF_REQUIRED_COMMAND_DATA_OUT (sizeof(MTP_COMMAND_DATA_OUT) - 1)

//
// NextPhase constants
//
#define MTP_NEXTPHASE_READ_DATA     1
#define MTP_NEXTPHASE_WRITE_DATA    2
#define MTP_NEXTPHASE_NO_DATA       3



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

