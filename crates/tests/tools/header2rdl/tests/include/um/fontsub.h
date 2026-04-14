/*************************************************************************
*                                                                        *
* fontsub.h -- font subsetting services (fontsub.dll)                    *
*                                                                        *
* (c) Microsoft Corporation. All Rights Reserved.                        *
*                                                                        *
*************************************************************************/

#ifndef FONTSUB_DOT_H_DEFINED
#define FONTSUB_DOT_H_DEFINED
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

  
#ifdef __cplusplus
extern "C" {
#endif

#ifndef CONST
#define CONST const
#endif     

typedef void *(__cdecl *CFP_ALLOCPROC)(size_t);
typedef void *(__cdecl *CFP_REALLOCPROC)(void *, size_t);
typedef void (__cdecl *CFP_FREEPROC)(void *);

unsigned long __cdecl CreateFontPackage(
	CONST unsigned char * puchSrcBuffer, /* input TTF or TTC buffer */
	CONST unsigned long ulSrcBufferSize, /* size of input TTF or TTC buffer data */
	unsigned char ** ppuchFontPackageBuffer, /* output package buffer */
	unsigned long * pulFontPackageBufferSize, /* output package buffer size */
	unsigned long * pulBytesWritten,  /* output package buffer data length */
	CONST unsigned short usFlag, /* subset, compress, or both, TTF or TTC, Chars or Glyphs */
	CONST unsigned short usTTCIndex,	/* TTC Index, only used if TTC bit set */
	CONST unsigned short usSubsetFormat, /* Old Subset, Subset or Delta */
	CONST unsigned short usSubsetLanguage,	/* Language in Name table to keep */
	CONST unsigned short usSubsetPlatform,	/* if ListType is Character, Platform of cmap to use for glyph list */
	CONST unsigned short usSubsetEncoding,	/* if ListType is Character, Encoding of cmap to use for glyph list */
	CONST unsigned short *pusSubsetKeepList, /* List of Characters or Glyphs to keep */
	CONST unsigned short usSubsetListCount,	 /* number of elements in list */
	CFP_ALLOCPROC lpfnAllocate,	  /* call back function to allocate temp buffers and output buffers */
	CFP_REALLOCPROC lpfnReAllocate,	  /* call back function to reallocate temp and output buffers */
	CFP_FREEPROC lpfnFree,	 /* call back function to free buffer allocated with lpfnAllocate and lpfnReAllocate */
	void * lpvReserved);

/* for usSubsetFormat */
#define TTFCFP_SUBSET 0	  /* Straight Subset Font - Backward compatibility */
#define TTFCFP_SUBSET1 1	  /* Subset font with full TTO and Kern tables. For later merge */
#define TTFCFP_DELTA 2	  /* Delta font, for merge with a subset1 font */

/* for usSubsetPlatform ID values */
#define TTFCFP_UNICODE_PLATFORMID 0
#define TTFCFP_APPLE_PLATFORMID   1
#define TTFCFP_ISO_PLATFORMID     2
#define TTFCFP_MS_PLATFORMID      3

/* for usSubsetEncoding values */
#define TTFCFP_STD_MAC_CHAR_SET  0	/* goes with TTFSUB_APPLE_PLATFORMID */
#define TTFCFP_SYMBOL_CHAR_SET  0	/* goes with TTFSUB_MS_PLATFORMID */
#define TTFCFP_UNICODE_CHAR_SET  1	/* goes with TTFSUB_MS_PLATFORMID */
#define TTFCFP_DONT_CARE  0xFFFF

/* for usSubsetLanguage values */
#define TTFCFP_LANG_KEEP_ALL 0

/* for usFlags values */
#define TTFCFP_FLAGS_SUBSET 0x0001	/* if bit off, don't subset */
#define TTFCFP_FLAGS_COMPRESS 0x0002  /* if bit off, don't compress */
#define TTFCFP_FLAGS_TTC 0x0004  /* if bit off, its a TTF */
#define TTFCFP_FLAGS_GLYPHLIST 0x0008 /* if bit off, list is characters */

unsigned long __cdecl MergeFontPackage(CONST unsigned char * puchMergeFontBuffer, /* buffer containing font to merge with */
			CONST unsigned long ulMergeFontBufferSize,	/* size of buffer containing font to merge with */
			CONST unsigned char * puchFontPackageBuffer, /* buffer containing a font package to merge with the MergeFontBuffer */
			CONST unsigned long ulFontPackageBufferSize, /* length of FontPakageBuffer */
			unsigned char **ppuchDestBuffer, /* output: pointer to output buffer containing a TTF to install */
			unsigned long *pulDestBufferSize, /* output: length of output buffer containing TTF to install */
			unsigned long *pulBytesWritten, /* output: number of bytes in buffer used for TTF */
			CONST unsigned short usMode, /* kind of action to perform, see #defines below */ 
			CFP_ALLOCPROC lpfnAllocate,	  /* call back function to allocate output and intermediate buffers */
			CFP_REALLOCPROC lpfnReAllocate,	  /* call back function to allocate or reallocate output and intermediate buffers */
			CFP_FREEPROC lpfnFree,	 /* call back function to free buffer allocated with lpfnReAllocate */
			void *lpvReserved);

/* for usModes */
#define TTFMFP_SUBSET 0   /* copy a Straight Subset Font package to Dest buffer */
#define TTFMFP_SUBSET1 1  /* Expand a format 1 font into a format 3 font */
#define TTFMFP_DELTA 2	   /* Merge a format 2 with a format 3 font */

/* Error codes */
#ifndef NO_ERROR
#define NO_ERROR 0
#endif

#ifndef ERR_GENERIC
#define ERR_GENERIC 1000  
#define ERR_READOUTOFBOUNDS 1001	/* trying to read from memory not allowed - data error? */
#define ERR_WRITEOUTOFBOUNDS 1002	/* trying to write to memory not allowed - data error? */
#define ERR_READCONTROL 1003	/* read control structure does not match data */
#define ERR_WRITECONTROL 1004	/* write control structure does not match data */
#define ERR_MEM 1005   /* error allocating memory */
#define ERR_FORMAT 1006 /* input data format error */

#define ERR_WOULD_GROW 1007 /* action would cause data to grow. use original data */
#define ERR_VERSION 1008	/* major dttf.version of the input data is greater than the version this program can read */
#define ERR_NO_GLYPHS 1009
#define ERR_INVALID_MERGE_FORMATS 1010 /* trying to merge fonts with the wrong dttf formats */
#define ERR_INVALID_MERGE_CHECKSUMS 1011  /* trying to merge 2 fonts from different mother font */
#define ERR_INVALID_MERGE_NUMGLYPHS 1012  /* trying to merge 2 fonts from different mother font */
#define	ERR_INVALID_DELTA_FORMAT	1013  /* trying to subset a format 1 or 2 font */
#define ERR_NOT_TTC 1014
#define ERR_INVALID_TTC_INDEX 1015

#define ERR_MISSING_CMAP 1030
#define ERR_MISSING_GLYF 1031
#define ERR_MISSING_HEAD 1032
#define ERR_MISSING_HHEA 1033
#define ERR_MISSING_HMTX 1034
#define ERR_MISSING_LOCA 1035
#define ERR_MISSING_MAXP 1036
#define ERR_MISSING_NAME 1037
#define ERR_MISSING_POST 1038
#define ERR_MISSING_OS2  1039
#define ERR_MISSING_VHEA 1040
#define ERR_MISSING_VMTX 1041
#define ERR_MISSING_HHEA_OR_VHEA 1042
#define ERR_MISSING_HMTX_OR_VMTX 1043
#define ERR_MISSING_EBDT 1044

#define ERR_INVALID_CMAP 1060
#define ERR_INVALID_GLYF 1061
#define ERR_INVALID_HEAD 1062
#define ERR_INVALID_HHEA 1063
#define ERR_INVALID_HMTX 1064
#define ERR_INVALID_LOCA 1065
#define ERR_INVALID_MAXP 1066
#define ERR_INVALID_NAME 1067
#define ERR_INVALID_POST 1068
#define ERR_INVALID_OS2 1069
#define ERR_INVALID_VHEA 1070
#define ERR_INVALID_VMTX 1071
#define ERR_INVALID_HHEA_OR_VHEA 1072
#define ERR_INVALID_HMTX_OR_VMTX 1073
																															 
#define ERR_INVALID_TTO 1080
#define ERR_INVALID_GSUB 1081
#define ERR_INVALID_GPOS 1082
#define ERR_INVALID_GDEF 1083
#define ERR_INVALID_JSTF 1084
#define ERR_INVALID_BASE 1085
#define ERR_INVALID_EBLC 1086
#define ERR_INVALID_LTSH 1087
#define	ERR_INVALID_VDMX 1088
#define	ERR_INVALID_HDMX 1089

#define ERR_PARAMETER0 1100  /* calling function argument 0 is invalid */
#define ERR_PARAMETER1 1101  /* calling function argument 1 is invalid */
#define ERR_PARAMETER2 1102  /* calling function argument 2 is invalid */
#define ERR_PARAMETER3 1103  /* calling function argument 3 is invalid */
#define ERR_PARAMETER4 1104  /* calling function argument 4 is invalid */
#define ERR_PARAMETER5 1105  /* calling function argument 5 is invalid */
#define ERR_PARAMETER6 1106  /* calling function argument 6 is invalid */
#define ERR_PARAMETER7 1107  /* calling function argument 7 is invalid */
#define ERR_PARAMETER8 1108  /* calling function argument 8 is invalid */
#define ERR_PARAMETER9 1109  /* calling function argument 9 is invalid */
#define ERR_PARAMETER10 1110  /* calling function argument 10 is invalid */
#define ERR_PARAMETER11 1111  /* calling function argument 11 is invalid */
#define ERR_PARAMETER12 1112  /* calling function argument 12 is invalid */
#define ERR_PARAMETER13 1113  /* calling function argument 13 is invalid */
#define ERR_PARAMETER14 1114  /* calling function argument 14 is invalid */
#define ERR_PARAMETER15 1115  /* calling function argument 15 is invalid */
#define ERR_PARAMETER16 1116  /* calling function argument 16 is invalid */
#endif /* ERR_GENERIC */


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* FONTSUB_DOT_H_DEFINED */
