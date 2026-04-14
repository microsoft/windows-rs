//------------------------------------------------------------------------------
// File: DV.h
//
// Desc: DV typedefs and defines.
//
// Copyright (c) 1997 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef _DV_H_
#define _DV_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define DV_DVSD_NTSC_FRAMESIZE	120000L
#define DV_DVSD_PAL_FRAMESIZE	144000L

#define DV_SMCHN	0x0000e000
#define DV_AUDIOMODE    0x00000f00
#define DV_AUDIOSMP	0x38000000

#define DV_AUDIOQU	0x07000000
#define DV_NTSCPAL	0x00200000
#define DV_STYPE	0x001f0000


//There are NTSC or PAL DV camcorders  
#define DV_NTSC		    0
#define DV_PAL		    1
//DV camcorder can output sd/hd/sl  
#define DV_SD		    0x00
#define DV_HD		    0x01
#define DV_SL		    0x02
//user can choice 12 bits or 16 bits audio from DV camcorder
#define DV_CAP_AUD16Bits    0x00
#define DV_CAP_AUD12Bits    0x01

#define SIZE_DVINFO	    0x20    

typedef struct Tag_DVAudInfo
{
	BYTE    bAudStyle[2];           
	//LSB 6 bits for starting DIF sequence number
	//MSB 2 bits: 0 for mon. 1: stereo in one 5/6 DIF sequences, 2: stereo audio in both 5/6 DIF sequences
	//example: 0x00: mon, audio in first 5/6 DIF sequence
	//                 0x05: mon, audio in 2nd 5 DIF sequence
	//                 0x15: stereo, audio only in 2nd 5 DIF sequence
	//                 0x10: stereo, audio only in 1st 5/6 DIF sequence
	//                 0x20: stereo, left ch in 1st 5/6 DIF sequence, right ch in 2nd 5/6 DIF sequence
	//                 0x26: stereo, rightch in 1st 6 DIF sequence, left ch in 2nd 6 DIF sequence
	BYTE    bAudQu[2];                      //qbits, only support 12, 16,           
		
	BYTE    bNumAudPin;                     //how many pin(language)
	WORD    wAvgSamplesPerPinPerFrm[2];     //samples size for one audio pin in one frame(which has 10 or 12 DIF sequence) 
	WORD    wBlkMode;                       //45 for NTSC, 54 for PAL
	WORD    wDIFMode;                       //5  for NTSC, 6 for PAL
	WORD    wBlkDiv;                        //15  for NTSC, 18 for PAL
} DVAudInfo;
	  

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _DV_H_
