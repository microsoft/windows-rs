//------------------------------------------------------------------------------
// File: AVIRIFF.h
//
// Desc: Structures and defines for the RIFF AVI file format extended to
//       handle very large/long files.
//
// Copyright (c) 1996 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#pragma warning(disable: 4097 4511 4512 4514 4705)


#if !defined AVIRIFF_H
#define AVIRIFF_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if !defined NUMELMS
  #define NUMELMS(aa) (sizeof(aa)/sizeof((aa)[0]))
#endif

// all structures in this file are packed on word boundaries
//
#include <pshpack2.h>

/*
 * heres the general layout of an AVI riff file (new format)
 *
 * RIFF (3F??????) AVI       <- not more than 1 GB in size
 *     LIST (size) hdrl
 *         avih (0038)
 *         LIST (size) strl
 *             strh (0038)
 *             strf (????)
 *             indx (3ff8)   <- size may vary, should be sector sized
 *         LIST (size) strl
 *             strh (0038)
 *             strf (????)
 *             indx (3ff8)   <- size may vary, should be sector sized
 *         LIST (size) odml
 *             dmlh (????)
 *         JUNK (size)       <- fill to align to sector - 12
 *     LIST (7f??????) movi  <- aligned on sector - 12
 *         00dc (size)       <- sector aligned
 *         01wb (size)       <- sector aligned
 *         ix00 (size)       <- sector aligned
 *     idx1 (00??????)       <- sector aligned
 * RIFF (7F??????) AVIX
 *     JUNK (size)           <- fill to align to sector -12
 *     LIST (size) movi
 *         00dc (size)       <- sector aligned
 * RIFF (7F??????) AVIX      <- not more than 2GB in size
 *     JUNK (size)           <- fill to align to sector - 12
 *     LIST (size) movi
 *         00dc (size)       <- sector aligned
 *
 *-===================================================================*/

//
// structures for manipulating RIFF headers
//
#define FCC(ch4) ((((DWORD)(ch4) & 0xFF) << 24) |     \
                  (((DWORD)(ch4) & 0xFF00) << 8) |    \
                  (((DWORD)(ch4) & 0xFF0000) >> 8) |  \
                  (((DWORD)(ch4) & 0xFF000000) >> 24))

typedef struct _riffchunk {
   FOURCC fcc;
   DWORD  cb;
   } RIFFCHUNK, * LPRIFFCHUNK;
typedef struct _rifflist {
   FOURCC fcc;
   DWORD  cb;
   FOURCC fccListType;
   } RIFFLIST, * LPRIFFLIST;

#define RIFFROUND(cb) ((cb) + ((cb)&1))
#define RIFFNEXT(pChunk) (LPRIFFCHUNK)((LPBYTE)(pChunk) \
                          + sizeof(RIFFCHUNK) \
                          + RIFFROUND(((LPRIFFCHUNK)pChunk)->cb))


//
// ==================== avi header structures ===========================
//

// main header for the avi file (compatibility header)
//
#define ckidMAINAVIHEADER FCC('avih')
typedef struct _avimainheader {
    FOURCC fcc;                    // 'avih'
    DWORD  cb;                     // size of this structure -8
    DWORD  dwMicroSecPerFrame;     // frame display rate (or 0L)
    DWORD  dwMaxBytesPerSec;       // max. transfer rate
    DWORD  dwPaddingGranularity;   // pad to multiples of this size; normally 2K.
    DWORD  dwFlags;                // the ever-present flags
    #define AVIF_HASINDEX        0x00000010 // Index at end of file?
    #define AVIF_MUSTUSEINDEX    0x00000020
    #define AVIF_ISINTERLEAVED   0x00000100
    #define AVIF_TRUSTCKTYPE     0x00000800 // Use CKType to find key frames
    #define AVIF_WASCAPTUREFILE  0x00010000
    #define AVIF_COPYRIGHTED     0x00020000
    DWORD  dwTotalFrames;          // # frames in first movi list
    DWORD  dwInitialFrames;
    DWORD  dwStreams;
    DWORD  dwSuggestedBufferSize;
    DWORD  dwWidth;
    DWORD  dwHeight;
    DWORD  dwReserved[4];
    } AVIMAINHEADER;

#define ckidODML          FCC('odml')
#define ckidAVIEXTHEADER  FCC('dmlh')
typedef struct _aviextheader {
   FOURCC  fcc;                    // 'dmlh'
   DWORD   cb;                     // size of this structure -8
   DWORD   dwGrandFrames;          // total number of frames in the file
   DWORD   dwFuture[61];           // to be defined later
   } AVIEXTHEADER;

//
// structure of an AVI stream header riff chunk
//
#define ckidSTREAMLIST   FCC('strl')

#ifndef ckidSTREAMHEADER
#define ckidSTREAMHEADER FCC('strh')
#endif
typedef struct _avistreamheader {
   FOURCC fcc;          // 'strh'
   DWORD  cb;           // size of this structure - 8

   FOURCC fccType;      // stream type codes

   #ifndef streamtypeVIDEO
   #define streamtypeVIDEO FCC('vids')
   #define streamtypeAUDIO FCC('auds')
   #define streamtypeMIDI  FCC('mids')
   #define streamtypeTEXT  FCC('txts')
   #endif

   FOURCC fccHandler;
   DWORD  dwFlags;
   #define AVISF_DISABLED          0x00000001
   #define AVISF_VIDEO_PALCHANGES  0x00010000

   WORD   wPriority;
   WORD   wLanguage;
   DWORD  dwInitialFrames;
   DWORD  dwScale;
   DWORD  dwRate;       // dwRate/dwScale is stream tick rate in ticks/sec
   DWORD  dwStart;
   DWORD  dwLength;
   DWORD  dwSuggestedBufferSize;
   DWORD  dwQuality;
   DWORD  dwSampleSize;
   struct {
      short int left;
      short int top;
      short int right;
      short int bottom;
      }   rcFrame;
   } AVISTREAMHEADER;


//
// structure of an AVI stream format chunk
//
#ifndef ckidSTREAMFORMAT
#define ckidSTREAMFORMAT FCC('strf')
#endif
//
// avi stream formats are different for each stream type
//
// BITMAPINFOHEADER for video streams
// WAVEFORMATEX or PCMWAVEFORMAT for audio streams
// nothing for text streams
// nothing for midi streams


#pragma warning(disable:4200)
//
// structure of old style AVI index
//
#define ckidAVIOLDINDEX FCC('idx1')
typedef struct _avioldindex {
   FOURCC  fcc;        // 'idx1'
   DWORD   cb;         // size of this structure -8
   struct _avioldindex_entry {
      DWORD   dwChunkId;
      DWORD   dwFlags;

      #ifndef AVIIF_LIST
      #define AVIIF_LIST       0x00000001
      #define AVIIF_KEYFRAME   0x00000010
      #endif
     
      #define AVIIF_NO_TIME    0x00000100
      #define AVIIF_COMPRESSOR 0x0FFF0000  // unused?
      DWORD   dwOffset;    // offset of riff chunk header for the data
      DWORD   dwSize;      // size of the data (excluding riff header size)
      } aIndex[];          // size of this array
   } AVIOLDINDEX;


//
// ============ structures for timecode in an AVI file =================
//

#ifndef TIMECODE_DEFINED
#define TIMECODE_DEFINED

// defined
// timecode time structure
//
typedef union _timecode {
   struct {
      WORD   wFrameRate;
      WORD   wFrameFract;
      LONG   cFrames;
      };
   DWORDLONG  qw;
   } TIMECODE;

typedef struct tagTIMECODE_SAMPLE {
    LONGLONG qwTick;
    TIMECODE timecode;
    DWORD dwUser;
    DWORD dwFlags;
} TIMECODE_SAMPLE;

#endif // TIMECODE_DEFINED

#define TIMECODE_RATE_30DROP 0   // this MUST be zero

// struct for all the SMPTE timecode info
//
typedef struct _timecodedata {
   TIMECODE time;
   DWORD    dwSMPTEflags;
   DWORD    dwUser;
   } TIMECODEDATA;

// dwSMPTEflags masks/values
//
#define TIMECODE_SMPTE_BINARY_GROUP 0x07
#define TIMECODE_SMPTE_COLOR_FRAME  0x08

//
// ============ structures for new style AVI indexes =================
//

// index type codes
//
#define AVI_INDEX_OF_INDEXES       0x00
#define AVI_INDEX_OF_CHUNKS        0x01
#define AVI_INDEX_OF_TIMED_CHUNKS  0x02
#define AVI_INDEX_OF_SUB_2FIELD    0x03
#define AVI_INDEX_IS_DATA          0x80

// index subtype codes
//
#define AVI_INDEX_SUB_DEFAULT     0x00

// INDEX_OF_CHUNKS subtype codes
//
#define AVI_INDEX_SUB_2FIELD      0x01

// meta structure of all avi indexes
//
typedef struct _avimetaindex {
   FOURCC fcc;
   UINT   cb;
   WORD   wLongsPerEntry;
   BYTE   bIndexSubType;
   BYTE   bIndexType;
   DWORD  nEntriesInUse;
   DWORD  dwChunkId;
   DWORD  dwReserved[3];
   DWORD  adwIndex[];
   } AVIMETAINDEX;

#define STDINDEXSIZE 0x4000
#define NUMINDEX(wLongsPerEntry) ((STDINDEXSIZE-32)/4/(wLongsPerEntry))
#define NUMINDEXFILL(wLongsPerEntry) ((STDINDEXSIZE/4) - NUMINDEX(wLongsPerEntry))

// structure of a super index (INDEX_OF_INDEXES)
//
#define ckidAVISUPERINDEX FCC('indx')
typedef struct _avisuperindex {
   FOURCC   fcc;               // 'indx'
   UINT     cb;                // size of this structure
   WORD     wLongsPerEntry;    // ==4
   BYTE     bIndexSubType;     // ==0 (frame index) or AVI_INDEX_SUB_2FIELD 
   BYTE     bIndexType;        // ==AVI_INDEX_OF_INDEXES
   DWORD    nEntriesInUse;     // offset of next unused entry in aIndex
   DWORD    dwChunkId;         // chunk ID of chunks being indexed, (i.e. RGB8)
   DWORD    dwReserved[3];     // must be 0
   struct _avisuperindex_entry {
      DWORDLONG qwOffset;    // 64 bit offset to sub index chunk
      DWORD    dwSize;       // 32 bit size of sub index chunk
      DWORD    dwDuration;   // time span of subindex chunk (in stream ticks)
      } aIndex[NUMINDEX(4)];
   } AVISUPERINDEX;
#define Valid_SUPERINDEX(pi) (*(DWORD *)(&((pi)->wLongsPerEntry)) == (4 | (AVI_INDEX_OF_INDEXES << 24)))

// struct of a standard index (AVI_INDEX_OF_CHUNKS)
//
typedef struct _avistdindex_entry {
   DWORD dwOffset;       // 32 bit offset to data (points to data, not riff header)
   DWORD dwSize;         // 31 bit size of data (does not include size of riff header), bit 31 is deltaframe bit
   } AVISTDINDEX_ENTRY;
#define AVISTDINDEX_DELTAFRAME ( 0x80000000) // Delta frames have the high bit set
#define AVISTDINDEX_SIZEMASK   (~0x80000000)

typedef struct _avistdindex {
   FOURCC   fcc;               // 'indx' or '##ix'
   UINT     cb;                // size of this structure
   WORD     wLongsPerEntry;    // ==2
   BYTE     bIndexSubType;     // ==0
   BYTE     bIndexType;        // ==AVI_INDEX_OF_CHUNKS
   DWORD    nEntriesInUse;     // offset of next unused entry in aIndex
   DWORD    dwChunkId;         // chunk ID of chunks being indexed, (i.e. RGB8)
   DWORDLONG qwBaseOffset;     // base offset that all index intries are relative to
   DWORD    dwReserved_3;      // must be 0
   AVISTDINDEX_ENTRY aIndex[NUMINDEX(2)];
   } AVISTDINDEX;

// struct of a time variant standard index (AVI_INDEX_OF_TIMED_CHUNKS)
//
typedef struct _avitimedindex_entry {
   DWORD dwOffset;       // 32 bit offset to data (points to data, not riff header)
   DWORD dwSize;         // 31 bit size of data (does not include size of riff header) (high bit is deltaframe bit)
   DWORD dwDuration;     // how much time the chunk should be played (in stream ticks)
   } AVITIMEDINDEX_ENTRY;

typedef struct _avitimedindex {
   FOURCC   fcc;               // 'indx' or '##ix'
   UINT     cb;                // size of this structure
   WORD     wLongsPerEntry;    // ==3
   BYTE     bIndexSubType;     // ==0
   BYTE     bIndexType;        // ==AVI_INDEX_OF_TIMED_CHUNKS
   DWORD    nEntriesInUse;     // offset of next unused entry in aIndex
   DWORD    dwChunkId;         // chunk ID of chunks being indexed, (i.e. RGB8)
   DWORDLONG qwBaseOffset;     // base offset that all index intries are relative to
   DWORD    dwReserved_3;      // must be 0
   AVITIMEDINDEX_ENTRY aIndex[NUMINDEX(3)];
   DWORD adwTrailingFill[NUMINDEXFILL(3)]; // to align struct to correct size
   } AVITIMEDINDEX;

// structure of a timecode stream
//
typedef struct _avitimecodeindex {
   FOURCC   fcc;               // 'indx' or '##ix'
   UINT     cb;                // size of this structure
   WORD     wLongsPerEntry;    // ==4
   BYTE     bIndexSubType;     // ==0
   BYTE     bIndexType;        // ==AVI_INDEX_IS_DATA
   DWORD    nEntriesInUse;     // offset of next unused entry in aIndex
   DWORD    dwChunkId;         // 'time'
   DWORD    dwReserved[3];     // must be 0
   TIMECODEDATA aIndex[NUMINDEX(sizeof(TIMECODEDATA)/sizeof(LONG))];
   } AVITIMECODEINDEX;

// structure of a timecode discontinuity list (when wLongsPerEntry == 7)
//
typedef struct _avitcdlindex_entry {
    DWORD    dwTick;           // stream tick time that maps to this timecode value
    TIMECODE time;
    DWORD    dwSMPTEflags;
    DWORD    dwUser;
    TCHAR    szReelId[12];
    } AVITCDLINDEX_ENTRY;

typedef struct _avitcdlindex {
   FOURCC   fcc;               // 'indx' or '##ix'
   UINT     cb;                // size of this structure
   WORD     wLongsPerEntry;    // ==7 (must be 4 or more all 'tcdl' indexes
   BYTE     bIndexSubType;     // ==0
   BYTE     bIndexType;        // ==AVI_INDEX_IS_DATA
   DWORD    nEntriesInUse;     // offset of next unused entry in aIndex
   DWORD    dwChunkId;         // 'tcdl'
   DWORD    dwReserved[3];     // must be 0
   AVITCDLINDEX_ENTRY aIndex[NUMINDEX(7)];
   DWORD adwTrailingFill[NUMINDEXFILL(7)]; // to align struct to correct size
   } AVITCDLINDEX;

typedef struct _avifieldindex_chunk {
   FOURCC   fcc;               // 'ix##'
   DWORD    cb;                // size of this structure
   WORD     wLongsPerEntry;    // must be 3 (size of each entry in
                               // aIndex array)
   BYTE     bIndexSubType;     // AVI_INDEX_2FIELD
   BYTE     bIndexType;        // AVI_INDEX_OF_CHUNKS
   DWORD    nEntriesInUse;     //
   DWORD    dwChunkId;         // '##dc' or '##db'
   DWORDLONG qwBaseOffset;     // offsets in aIndex array are relative to this
   DWORD    dwReserved3;       // must be 0
   struct _avifieldindex_entry {
      DWORD    dwOffset;
      DWORD    dwSize;         // size of all fields
                               // (bit 31 set for NON-keyframes)
      DWORD    dwOffsetField2; // offset to second field
   } aIndex[  ];
} AVIFIELDINDEX, * PAVIFIELDINDEX;


#include <poppack.h>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
