//
//    Copyright (C) Microsoft.  All rights reserved.
//
/*

 	dls2.h

 	Description:

 	Interface defines and structures for the DLS2 extensions of DLS.


     Written by Microsoft 1998.  Released for public use.

*/

#ifndef _INC_DLS2
#define _INC_DLS2

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
     FOURCC's used in the DLS2 file, in addition to DLS1 chunks
*/

#define FOURCC_RGN2  mmioFOURCC('r','g','n','2')
#define FOURCC_LAR2  mmioFOURCC('l','a','r','2')
#define FOURCC_ART2  mmioFOURCC('a','r','t','2')
#define FOURCC_CDL   mmioFOURCC('c','d','l',' ')
#define FOURCC_DLID  mmioFOURCC('d','l','i','d')

/*
     Articulation connection graph definitions. These are in addition to
     the definitions in the DLS1 header.
*/

/* Generic Sources (in addition to DLS1 sources. */
#define CONN_SRC_POLYPRESSURE		0x0007	/* Polyphonic Pressure */
#define CONN_SRC_CHANNELPRESSURE		0x0008	/* Channel Pressure */
#define CONN_SRC_VIBRATO			0x0009	/* Vibrato LFO */
#define CONN_SRC_MONOPRESSURE       	0x000a  /* MIDI Mono pressure */


/* Midi Controllers */
#define CONN_SRC_CC91			0x00db	/* Reverb Send */
#define CONN_SRC_CC93			0x00dd	/* Chorus Send */


/* Generic Destinations */
#define CONN_DST_GAIN			0x0001	/* Same as CONN_DST_ ATTENUATION, but more appropriate terminology. */
#define CONN_DST_KEYNUMBER 0x0005  /* Key Number Generator */

/* Audio Channel Output Destinations */
#define CONN_DST_LEFT			0x0010	/* Left Channel Send */
#define CONN_DST_RIGHT			0x0011	/* Right Channel Send */
#define CONN_DST_CENTER			0x0012	/* Center Channel Send */
#define CONN_DST_LEFTREAR			0x0013	/* Left Rear Channel Send */
#define CONN_DST_RIGHTREAR			0x0014	/* Right Rear Channel Send */
#define CONN_DST_LFE_CHANNEL		0x0015	/* LFE Channel Send */
#define CONN_DST_CHORUS			0x0080	/* Chorus Send */
#define CONN_DST_REVERB			0x0081	/* Reverb Send */

/* Vibrato LFO Destinations */
#define CONN_DST_VIB_FREQUENCY		0x0114	/* Vibrato Frequency */
#define CONN_DST_VIB_STARTDELAY		0x0115	/* Vibrato Start Delay */

/* EG1 Destinations */
#define CONN_DST_EG1_DELAYTIME		0x020B	/* EG1 Delay Time */
#define CONN_DST_EG1_HOLDTIME		0x020C	/* EG1 Hold Time */
#if (NTDDI_VERSION >= NTDDI_WINXP) /* Windows XP or greater */
#define CONN_DST_EG1_SHUTDOWNTIME	0x020D	/* EG1 Shutdown Time */
#endif

/* EG2 Destinations */
#define CONN_DST_EG2_DELAYTIME		0x030F	/* EG2 Delay Time */
#define CONN_DST_EG2_HOLDTIME		0x0310	/* EG2 Hold Time */


/* Filter Destinations */
#define CONN_DST_FILTER_CUTOFF		0x0500	/* Filter Cutoff Frequency */
#define CONN_DST_FILTER_Q			0x0501	/* Filter Resonance */


/* Transforms */
#define CONN_TRN_CONVEX			0x0002	/* Convex Transform */
#define CONN_TRN_SWITCH			0x0003	/* Switch Transform */


/*	Conditional chunk operators */
 #define DLS_CDL_AND			0x0001	/* X = X & Y */
 #define DLS_CDL_OR			0x0002	/* X = X | Y */
 #define DLS_CDL_XOR			0x0003	/* X = X ^ Y */
 #define DLS_CDL_ADD			0x0004	/* X = X + Y */
 #define DLS_CDL_SUBTRACT		0x0005	/* X = X - Y */
 #define DLS_CDL_MULTIPLY		0x0006	/* X = X * Y */
 #define DLS_CDL_DIVIDE		0x0007	/* X = X / Y */
 #define DLS_CDL_LOGICAL_AND	0x0008	/* X = X && Y */
 #define DLS_CDL_LOGICAL_OR		0x0009	/* X = X || Y */
 #define DLS_CDL_LT			0x000A	/* X = (X < Y) */
 #define DLS_CDL_LE			0x000B	/* X = (X <= Y) */
 #define DLS_CDL_GT			0x000C	/* X = (X > Y) */
 #define DLS_CDL_GE			0x000D	/* X = (X >= Y) */
 #define DLS_CDL_EQ			0x000E	/* X = (X == Y) */
 #define DLS_CDL_NOT			0x000F	/* X = !X */
 #define DLS_CDL_CONST		0x0010	/* 32-bit constant */
 #define DLS_CDL_QUERY		0x0011	/* 32-bit value returned from query */
 #define DLS_CDL_QUERYSUPPORTED	0x0012	/* Test to see if query is supported by synth */

/*
  Loop and release
*/

#if (NTDDI_VERSION < NTDDI_WINXP) /* Windows 2000 and SPs */
#define WLOOP_TYPE_RELEASE 2
#else
#define WLOOP_TYPE_RELEASE 1
#endif

/*
  WaveLink chunk <wlnk-ck>
*/

#if (NTDDI_VERSION >= NTDDI_WINXP) /* Windows XP or greater */
#define F_WAVELINK_MULTICHANNEL 0x0002
#endif

/*
  WaveLink chunk <wlnk-ck>
*/

#define F_WAVELINK_MULTICHANNEL 0x0002


/*
  DLSID queries for <cdl-ck>
*/

DEFINE_GUID(DLSID_GMInHardware, 0x178f2f24, 0xc364, 0x11d1, 0xa7, 0x60, 0x00, 0x00, 0xf8, 0x75, 0xac, 0x12);
DEFINE_GUID(DLSID_GSInHardware, 0x178f2f25, 0xc364, 0x11d1, 0xa7, 0x60, 0x00, 0x00, 0xf8, 0x75, 0xac, 0x12);
DEFINE_GUID(DLSID_XGInHardware, 0x178f2f26, 0xc364, 0x11d1, 0xa7, 0x60, 0x00, 0x00, 0xf8, 0x75, 0xac, 0x12);
DEFINE_GUID(DLSID_SupportsDLS1, 0x178f2f27, 0xc364, 0x11d1, 0xa7, 0x60, 0x00, 0x00, 0xf8, 0x75, 0xac, 0x12);
DEFINE_GUID(DLSID_SupportsDLS2, 0xf14599e5, 0x4689, 0x11d2, 0xaf, 0xa6, 0x0, 0xaa, 0x0, 0x24, 0xd8, 0xb6);
DEFINE_GUID(DLSID_SampleMemorySize, 0x178f2f28, 0xc364, 0x11d1, 0xa7, 0x60, 0x00, 0x00, 0xf8, 0x75, 0xac, 0x12);
DEFINE_GUID(DLSID_ManufacturersID, 0xb03e1181, 0x8095, 0x11d2, 0xa1, 0xef, 0x0, 0x60, 0x8, 0x33, 0xdb, 0xd8);
DEFINE_GUID(DLSID_ProductID, 0xb03e1182, 0x8095, 0x11d2, 0xa1, 0xef, 0x0, 0x60, 0x8, 0x33, 0xdb, 0xd8);
DEFINE_GUID(DLSID_SamplePlaybackRate, 0x2a91f713, 0xa4bf, 0x11d2, 0xbb, 0xdf, 0x0, 0x60, 0x8, 0x33, 0xdb, 0xd8);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _INC_DLS2 */
