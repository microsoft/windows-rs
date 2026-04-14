//*@@@+++@@@@******************************************************************
//
// Microsoft Windows Media Foundation
// Copyright (C) Microsoft Corporation. All rights reserved.
//
// ksopmapi.h - definitions for kernel mode code OPM communication
//
//*@@@---@@@@******************************************************************

#ifndef _KSOPMAPI_
#define _KSOPMAPI_

#if defined (_MSC_VER) && (_MSC_VER >= 1020) && !defined(__midl)
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#if (WINVER >= _WIN32_WINNT_WIN7)

//=============================================================================
// Description:
//
//  Ks Property set to use with AVStram drivers
// KSPROPSETID_OPMVideoOutput {06F414BB-F43A-4fe2-A566-774B4C81F0DB}
#ifdef DEFINE_GUIDSTRUCT
#define STATIC_KSPROPSETID_OPMVideoOutput \
0x6f414bb, 0xf43a, 0x4fe2, 0xa5, 0x66, 0x77, 0x4b, 0x4c, 0x81, 0xf0, 0xdb                         
DEFINE_GUIDSTRUCT("06F414BB-F43A-4fe2-A566-774B4C81F0DB", KSPROPSETID_OPMVideoOutput);          
#define KSPROPSETID_OPMVideoOutput DEFINE_GUIDNAMED(KSPROPSETID_OPMVideoOutput)                   
#endif
                                                                                                  
typedef enum
{                                                                                    
    //  Output is OPM_RANDOM_NUMBER followed by certifiate                                        
    KSMETHOD_OPMVIDEOOUTPUT_STARTINITIALIZATION = 0,                                              
                                                                                                  
    //  Input OPM_ENCRYPTED_INITIALIZATION_PARAMETERS                                             
    //  Output OPM_STANDARD_INFORMATION                                                           
    KSMETHOD_OPMVIDEOOUTPUT_FINISHINITIALIZATION = 1,                                             
                                                                                                  
    //  Input is OPM_GET_INFO_PARAMETERS, output is OPM_REQUESTED_INFORMATION                     
    //  Use KsMethod - both input and output in the buffer (not after the KSMETHOD structure)     
    KSMETHOD_OPMVIDEOOUTPUT_GETINFORMATION = 2                                                    
} KSMETHOD_OPMVIDEOOUTPUT;              

//  Currently on this GetInformation call is supported
DEFINE_GUID( OPM_GET_CODEC_INFO, 0x4f374491, 0x8f5f, 0x4445, 0x9d, 0xba, 0x95, 0x58, 0x8f, 0x6b, 0x58, 0xb4);

typedef struct _OPM_RANDOM_NUMBER
{
    BYTE abRandomNumber[ 16 ];
}OPM_RANDOM_NUMBER, *POPM_RANDOM_NUMBER;

enum
{
    OPM_OMAC_SIZE                                  = 16,
    OPM_128_BIT_RANDOM_NUMBER_SIZE                 = 16,
    OPM_ENCRYPTED_INITIALIZATION_PARAMETERS_SIZE   = 256,
    OPM_CONFIGURE_SETTING_DATA_SIZE                = 4056,
    OPM_GET_INFORMATION_PARAMETERS_SIZE            = 4056,
    OPM_REQUESTED_INFORMATION_SIZE                 = 4076,
    OPM_HDCP_KEY_SELECTION_VECTOR_SIZE             = 5,
    OPM_PROTECTION_TYPE_SIZE                       = 4,
    OPM_BUS_TYPE_MASK                              = 0xFFFF,
    OPM_BUS_IMPLEMENTATION_MODIFIER_MASK           = 0x7FFF,
};

typedef struct _OPM_OMAC
{
    BYTE abOMAC[OPM_OMAC_SIZE];  

} OPM_OMAC;

typedef struct _OPM_GET_INFO_PARAMETERS
{
    OPM_OMAC omac;  
    OPM_RANDOM_NUMBER rnRandomNumber;
    GUID guidInformation;
    ULONG ulSequenceNumber;
    ULONG cbParametersSize;
    BYTE abParameters[OPM_GET_INFORMATION_PARAMETERS_SIZE];

} OPM_GET_INFO_PARAMETERS;

typedef struct _OPM_REQUESTED_INFORMATION
{
    OPM_OMAC omac;  
    ULONG cbRequestedInformationSize;
    BYTE abRequestedInformation[OPM_REQUESTED_INFORMATION_SIZE];

} OPM_REQUESTED_INFORMATION;

typedef struct _OPM_ENCRYPTED_INITIALIZATION_PARAMETERS
{
    BYTE abEncryptedInitializationParameters[OPM_ENCRYPTED_INITIALIZATION_PARAMETERS_SIZE];

} OPM_ENCRYPTED_INITIALIZATION_PARAMETERS;

typedef struct _OPM_STANDARD_INFORMATION
{
    OPM_RANDOM_NUMBER rnRandomNumber;
    ULONG ulStatusFlags;
    ULONG ulInformation;
    ULONG ulReserved;
    ULONG ulReserved2;

} OPM_STANDARD_INFORMATION;

typedef struct _OPM_GET_CODEC_INFO_PARAMETERS
{
    DWORD  cbVerifier;
    BYTE   Verifier[OPM_GET_INFORMATION_PARAMETERS_SIZE - 4];  // Class ID of MFT or symbolic link for AVStream 
                          // drivers
} OPM_GET_CODEC_INFO_PARAMETERS;

typedef struct _OPM_GET_CODEC_INFO_INFORMATION
{
    OPM_RANDOM_NUMBER rnRandomNumber;
    DWORD Merit;   // Merit assigned to the codec
} OPM_GET_CODEC_INFO_INFORMATION;

#endif // (WINVER >= _WIN32_WINNT_WIN7)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif//_KSOPMAPI_
