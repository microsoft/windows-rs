/********************************************************************************
*                                                                               *
* mmiscapi.h -- ApiSet Contract for api-ms-win-mm-misc-l1-1                     *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _MMISCAPI_H_
#define _MMISCAPI_H_

#include <apiset.h>
#include <apisetcconv.h>

#include <mmsyscom.h> // mm common definitions

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef MMNODRV

/****************************************************************************

                        Installable driver support

****************************************************************************/

#ifdef _WIN32
typedef struct DRVCONFIGINFOEX {
    DWORD   dwDCISize;
    LPCWSTR  lpszDCISectionName;
    LPCWSTR  lpszDCIAliasName;
    DWORD    dnDevNode;
} DRVCONFIGINFOEX, *PDRVCONFIGINFOEX, NEAR *NPDRVCONFIGINFOEX, FAR *LPDRVCONFIGINFOEX;

#else
typedef struct DRVCONFIGINFOEX {
    DWORD   dwDCISize;
    LPCSTR  lpszDCISectionName;
    LPCSTR  lpszDCIAliasName;
    DWORD    dnDevNode;
} DRVCONFIGINFOEX, *PDRVCONFIGINFOEX, NEAR *NPDRVCONFIGINFOEX, FAR *LPDRVCONFIGINFOEX;
#endif

#if (WINVER < 0x030a) || defined(_WIN32)

#ifndef DRV_LOAD

/* Driver messages */
#define DRV_LOAD                0x0001
#define DRV_ENABLE              0x0002
#define DRV_OPEN                0x0003
#define DRV_CLOSE               0x0004
#define DRV_DISABLE             0x0005
#define DRV_FREE                0x0006
#define DRV_CONFIGURE           0x0007
#define DRV_QUERYCONFIGURE      0x0008
#define DRV_INSTALL             0x0009
#define DRV_REMOVE              0x000A
#define DRV_EXITSESSION         0x000B
#define DRV_POWER               0x000F
#define DRV_RESERVED            0x0800
#define DRV_USER                0x4000

/* LPARAM of DRV_CONFIGURE message */
#ifdef _WIN32
typedef struct tagDRVCONFIGINFO {
    DWORD   dwDCISize;
    LPCWSTR  lpszDCISectionName;
    LPCWSTR  lpszDCIAliasName;
} DRVCONFIGINFO, *PDRVCONFIGINFO, NEAR *NPDRVCONFIGINFO, FAR *LPDRVCONFIGINFO;
#else
typedef struct tagDRVCONFIGINFO {
    DWORD   dwDCISize;
    LPCSTR  lpszDCISectionName;
    LPCSTR  lpszDCIAliasName;
} DRVCONFIGINFO, *PDRVCONFIGINFO, NEAR *NPDRVCONFIGINFO, FAR *LPDRVCONFIGINFO;
#endif

/* Supported return values for DRV_CONFIGURE message */
#define DRVCNF_CANCEL           0x0000
#define DRVCNF_OK               0x0001
#define DRVCNF_RESTART          0x0002

/* installable driver function prototypes */
#ifdef _WIN32

typedef LRESULT (CALLBACK* DRIVERPROC)(DWORD_PTR, HDRVR, UINT, LPARAM, LPARAM);

WINMMAPI
LRESULT
WINAPI
CloseDriver(
    _In_ HDRVR hDriver,
    _In_ LPARAM lParam1,
    _In_ LPARAM lParam2
    );

WINMMAPI
HDRVR
WINAPI
OpenDriver(
    _In_ LPCWSTR szDriverName,
    _In_ LPCWSTR szSectionName,
    _In_ LPARAM lParam2
    );

WINMMAPI
LRESULT
WINAPI
SendDriverMessage(
    _In_ HDRVR hDriver,
    _In_ UINT message,
    _In_ LPARAM lParam1,
    _In_ LPARAM lParam2
    );

WINMMAPI
HMODULE
WINAPI
DrvGetModuleHandle(
    _In_ HDRVR hDriver
    );

WINMMAPI
HMODULE
WINAPI
GetDriverModuleHandle(
    _In_ HDRVR hDriver
    );

WINMMAPI
LRESULT
WINAPI
DefDriverProc(
    _In_ DWORD_PTR dwDriverIdentifier,
    _In_ HDRVR hdrvr,
    _In_ UINT uMsg,
    _In_ LPARAM lParam1,
    _In_ LPARAM lParam2
    );

#else
LRESULT   WINAPI DrvClose(HDRVR hdrvr, LPARAM lParam1, LPARAM lParam2);
HDRVR     WINAPI DrvOpen(LPCSTR szDriverName, LPCSTR szSectionName, LPARAM lParam2);
LRESULT   WINAPI DrvSendMessage(HDRVR hdrvr, UINT uMsg, LPARAM lParam1, LPARAM lParam2);
HINSTANCE WINAPI DrvGetModuleHandle(HDRVR hdrvr);
LRESULT   WINAPI DrvDefDriverProc(DWORD dwDriverIdentifier, HDRVR hdrvr, UINT uMsg, LPARAM lParam1, LPARAM lParam2);
#define DefDriverProc DrvDefDriverProc
#endif /* ifdef _WIN32 */
#endif /* DRV_LOAD */
#endif /* ifdef (WINVER < 0x030a) || defined(_WIN32) */

#if (WINVER >= 0x030a)
/* return values from DriverProc() function */
#define DRV_CANCEL             DRVCNF_CANCEL
#define DRV_OK                 DRVCNF_OK
#define DRV_RESTART            DRVCNF_RESTART

#endif /* ifdef WINVER >= 0x030a */

#define DRV_MCI_FIRST          DRV_RESERVED
#define DRV_MCI_LAST           (DRV_RESERVED + 0xFFF)

/***************************************************************************

                      Driver Helper function moved from mmddk.h

***************************************************************************/
BOOL
APIENTRY
DriverCallback(
    DWORD_PTR dwCallback,
    DWORD dwFlags,
    HDRVR hDevice,
    DWORD dwMsg,
    DWORD_PTR dwUser,
    DWORD_PTR dwParam1,
    DWORD_PTR dwParam2
    );

/****************************************************************************

  Sound schemes

****************************************************************************/LONG
WINAPI
sndOpenSound(
    _In_ LPCWSTR EventName,
    _In_ LPCWSTR AppName,
    _In_ INT32 Flags,
    _Outptr_ PHANDLE FileHandle
    );

//
// removed from winmmi.h

//
/****************************************************************************

  API to install/remove/query a MMSYS driver

****************************************************************************/

/* generic prototype for audio device driver entry-point functions
// midMessage(), modMessage(), widMessage(), wodMessage(), auxMessage()
*/
typedef DWORD (APIENTRY *DRIVERMSGPROC)(DWORD, DWORD, DWORD_PTR, DWORD_PTR, DWORD_PTR);

UINT
APIENTRY
mmDrvInstall(
    HDRVR hDriver,
    LPCWSTR wszDrvEntry,
    DRIVERMSGPROC drvMessage,
    UINT wFlags
    );

#endif  /* ifndef MMNODRV */

#ifndef MMNOMMIO
/****************************************************************************

                        Multimedia File I/O support

****************************************************************************/

/* MMIO error return values */
#define MMIOERR_BASE                256
#define MMIOERR_FILENOTFOUND        (MMIOERR_BASE + 1)  /* file not found */
#define MMIOERR_OUTOFMEMORY         (MMIOERR_BASE + 2)  /* out of memory */
#define MMIOERR_CANNOTOPEN          (MMIOERR_BASE + 3)  /* cannot open */
#define MMIOERR_CANNOTCLOSE         (MMIOERR_BASE + 4)  /* cannot close */
#define MMIOERR_CANNOTREAD          (MMIOERR_BASE + 5)  /* cannot read */
#define MMIOERR_CANNOTWRITE         (MMIOERR_BASE + 6)  /* cannot write */
#define MMIOERR_CANNOTSEEK          (MMIOERR_BASE + 7)  /* cannot seek */
#define MMIOERR_CANNOTEXPAND        (MMIOERR_BASE + 8)  /* cannot expand file */
#define MMIOERR_CHUNKNOTFOUND       (MMIOERR_BASE + 9)  /* chunk not found */
#define MMIOERR_UNBUFFERED          (MMIOERR_BASE + 10) /*  */
#define MMIOERR_PATHNOTFOUND        (MMIOERR_BASE + 11) /* path incorrect */
#define MMIOERR_ACCESSDENIED        (MMIOERR_BASE + 12) /* file was protected */
#define MMIOERR_SHARINGVIOLATION    (MMIOERR_BASE + 13) /* file in use */
#define MMIOERR_NETWORKERROR        (MMIOERR_BASE + 14) /* network not responding */
#define MMIOERR_TOOMANYOPENFILES    (MMIOERR_BASE + 15) /* no more file handles  */
#define MMIOERR_INVALIDFILE         (MMIOERR_BASE + 16) /* default error file error */

/* MMIO constants */
#define CFSEPCHAR       '+'             /* compound file name separator char. */

/* MMIO data types */
typedef DWORD           FOURCC;         /* a four character code */
typedef char _huge *    HPSTR;          /* a huge version of LPSTR */
DECLARE_HANDLE(HMMIO);                  /* a handle to an open file */
typedef LRESULT (CALLBACK MMIOPROC)(LPSTR lpmmioinfo, UINT uMsg,
            LPARAM lParam1, LPARAM lParam2);
typedef MMIOPROC FAR *LPMMIOPROC;

/* general MMIO information data structure */
typedef struct _MMIOINFO
{
        /* general fields */
        DWORD           dwFlags;        /* general status flags */
        FOURCC          fccIOProc;      /* pointer to I/O procedure */
        LPMMIOPROC      pIOProc;        /* pointer to I/O procedure */
        UINT            wErrorRet;      /* place for error to be returned */
        HTASK           htask;          /* alternate local task */

        /* fields maintained by MMIO functions during buffered I/O */
        LONG            cchBuffer;      /* size of I/O buffer (or 0L) */
        HPSTR           pchBuffer;      /* start of I/O buffer (or NULL) */
        HPSTR           pchNext;        /* pointer to next byte to read/write */
        HPSTR           pchEndRead;     /* pointer to last valid byte to read */
        HPSTR           pchEndWrite;    /* pointer to last byte to write */
        LONG            lBufOffset;     /* disk offset of start of buffer */

        /* fields maintained by I/O procedure */
        LONG            lDiskOffset;    /* disk offset of next read or write */
        DWORD           adwInfo[3];     /* data specific to type of MMIOPROC */

        /* other fields maintained by MMIO */
        DWORD           dwReserved1;    /* reserved for MMIO use */
        DWORD           dwReserved2;    /* reserved for MMIO use */
        HMMIO           hmmio;          /* handle to open file */
} MMIOINFO, *PMMIOINFO, NEAR *NPMMIOINFO, FAR *LPMMIOINFO;
typedef const MMIOINFO FAR *LPCMMIOINFO;

/* RIFF chunk information data structure */
typedef struct _MMCKINFO
{
        FOURCC          ckid;           /* chunk ID */
        DWORD           cksize;         /* chunk size */
        FOURCC          fccType;        /* form type or list type */
        DWORD           dwDataOffset;   /* offset of data portion of chunk */
        DWORD           dwFlags;        /* flags used by MMIO functions */
} MMCKINFO, *PMMCKINFO, NEAR *NPMMCKINFO, FAR *LPMMCKINFO;
typedef const MMCKINFO *LPCMMCKINFO;

/* bit field masks */
#define MMIO_RWMODE     0x00000003      /* open file for reading/writing/both */
#define MMIO_SHAREMODE  0x00000070      /* file sharing mode number */

/* constants for dwFlags field of MMIOINFO */
#define MMIO_CREATE     0x00001000      /* create new file (or truncate file) */
#define MMIO_PARSE      0x00000100      /* parse new file returning path */
#define MMIO_DELETE     0x00000200      /* create new file (or truncate file) */
#define MMIO_EXIST      0x00004000      /* checks for existence of file */
#define MMIO_ALLOCBUF   0x00010000      /* mmioOpen() should allocate a buffer */
#define MMIO_GETTEMP    0x00020000      /* mmioOpen() should retrieve temp name */

#define MMIO_DIRTY      0x10000000      /* I/O buffer is dirty */

/* read/write mode numbers (bit field MMIO_RWMODE) */
#define MMIO_READ       0x00000000      /* open file for reading only */
#define MMIO_WRITE      0x00000001      /* open file for writing only */
#define MMIO_READWRITE  0x00000002      /* open file for reading and writing */

/* share mode numbers (bit field MMIO_SHAREMODE) */
#define MMIO_COMPAT     0x00000000      /* compatibility mode */
#define MMIO_EXCLUSIVE  0x00000010      /* exclusive-access mode */
#define MMIO_DENYWRITE  0x00000020      /* deny writing to other processes */
#define MMIO_DENYREAD   0x00000030      /* deny reading to other processes */
#define MMIO_DENYNONE   0x00000040      /* deny nothing to other processes */

/* various MMIO flags */
#define MMIO_FHOPEN             0x0010  /* mmioClose: keep file handle open */
#define MMIO_EMPTYBUF           0x0010  /* mmioFlush: empty the I/O buffer */
#define MMIO_TOUPPER            0x0010  /* mmioStringToFOURCC: to u-case */
#define MMIO_INSTALLPROC    0x00010000  /* mmioInstallIOProc: install MMIOProc */
#define MMIO_GLOBALPROC     0x10000000  /* mmioInstallIOProc: install globally */
#define MMIO_REMOVEPROC     0x00020000  /* mmioInstallIOProc: remove MMIOProc */
#define MMIO_UNICODEPROC    0x01000000  /* mmioInstallIOProc: Unicode MMIOProc */
#define MMIO_FINDPROC       0x00040000  /* mmioInstallIOProc: find an MMIOProc */
#define MMIO_FINDCHUNK          0x0010  /* mmioDescend: find a chunk by ID */
#define MMIO_FINDRIFF           0x0020  /* mmioDescend: find a LIST chunk */
#define MMIO_FINDLIST           0x0040  /* mmioDescend: find a RIFF chunk */
#define MMIO_CREATERIFF         0x0020  /* mmioCreateChunk: make a LIST chunk */
#define MMIO_CREATELIST         0x0040  /* mmioCreateChunk: make a RIFF chunk */

/* message numbers for MMIOPROC I/O procedure functions */
#define MMIOM_READ      MMIO_READ       /* read */
#define MMIOM_WRITE    MMIO_WRITE       /* write */
#define MMIOM_SEEK              2       /* seek to a new position in file */
#define MMIOM_OPEN              3       /* open file */
#define MMIOM_CLOSE             4       /* close file */
#define MMIOM_WRITEFLUSH        5       /* write and flush */

#if (WINVER >= 0x030a)
#define MMIOM_RENAME            6       /* rename specified file */
#endif /* ifdef WINVER >= 0x030a */

#define MMIOM_USER         0x8000       /* beginning of user-defined messages */

/* standard four character codes */
#define FOURCC_RIFF     mmioFOURCC('R', 'I', 'F', 'F')
#define FOURCC_LIST     mmioFOURCC('L', 'I', 'S', 'T')

/* four character codes used to identify standard built-in I/O procedures */
#define FOURCC_DOS      mmioFOURCC('D', 'O', 'S', ' ')
#define FOURCC_MEM      mmioFOURCC('M', 'E', 'M', ' ')

/* flags for mmioSeek() */
#ifndef SEEK_SET
#define SEEK_SET        0               /* seek to an absolute position */
#define SEEK_CUR        1               /* seek relative to current position */
#define SEEK_END        2               /* seek relative to end of file */
#endif  /* ifndef SEEK_SET */

/* other constants */
#define MMIO_DEFAULTBUFFER      8192    /* default buffer size */

/* MMIO macros */
#define mmioFOURCC(ch0, ch1, ch2, ch3)  MAKEFOURCC(ch0, ch1, ch2, ch3)

/* MMIO function prototypes */
#ifdef _WIN32

WINMMAPI
FOURCC
WINAPI
mmioStringToFOURCCA(
    LPCSTR sz,
    _In_ UINT uFlags
    );

WINMMAPI
FOURCC
WINAPI
mmioStringToFOURCCW(
    LPCWSTR sz,
    _In_ UINT uFlags
    );

#ifdef UNICODE
#define mmioStringToFOURCC  mmioStringToFOURCCW
#else
#define mmioStringToFOURCC  mmioStringToFOURCCA
#endif // !UNICODE
WINMMAPI
LPMMIOPROC
WINAPI
mmioInstallIOProcA(
    _In_ FOURCC fccIOProc,
    _In_opt_ LPMMIOPROC pIOProc,
    _In_ DWORD dwFlags
    );

WINMMAPI
LPMMIOPROC
WINAPI
mmioInstallIOProcW(
    _In_ FOURCC fccIOProc,
    _In_opt_ LPMMIOPROC pIOProc,
    _In_ DWORD dwFlags
    );

#ifdef UNICODE
#define mmioInstallIOProc  mmioInstallIOProcW
#else
#define mmioInstallIOProc  mmioInstallIOProcA
#endif // !UNICODE
WINMMAPI
HMMIO
WINAPI
mmioOpenA(
    _Inout_updates_bytes_opt_(128) LPSTR pszFileName,
    _Inout_opt_ LPMMIOINFO pmmioinfo,
    _In_ DWORD fdwOpen
    );

WINMMAPI
HMMIO
WINAPI
mmioOpenW(
    _Inout_updates_bytes_opt_(128) LPWSTR pszFileName,
    _Inout_opt_ LPMMIOINFO pmmioinfo,
    _In_ DWORD fdwOpen
    );

#ifdef UNICODE
#define mmioOpen  mmioOpenW
#else
#define mmioOpen  mmioOpenA
#endif // !UNICODE
WINMMAPI
MMRESULT
WINAPI
mmioRenameA(
    _In_ LPCSTR pszFileName,
    _In_ LPCSTR pszNewFileName,
    _In_opt_ LPCMMIOINFO pmmioinfo,
    _In_ DWORD fdwRename
    );

WINMMAPI
MMRESULT
WINAPI
mmioRenameW(
    _In_ LPCWSTR pszFileName,
    _In_ LPCWSTR pszNewFileName,
    _In_opt_ LPCMMIOINFO pmmioinfo,
    _In_ DWORD fdwRename
    );

#ifdef UNICODE
#define mmioRename  mmioRenameW
#else
#define mmioRename  mmioRenameA
#endif // !UNICODE
#else
FOURCC WINAPI mmioStringToFOURCC( LPCSTR sz, UINT uFlags);
LPMMIOPROC WINAPI mmioInstallIOProc( FOURCC fccIOProc, LPMMIOPROC pIOProc, DWORD dwFlags);
HMMIO WINAPI mmioOpen(_Inout_opt_ LPSTR pszFileName, LPMMIOINFO pmmioinfo, DWORD fdwOpen);
#if (WINVER >= 0x030a)
MMRESULT WINAPI mmioRename( _In_ LPCSTR pszFileName, _In_ LPCSTR pszNewFileName, _In_opt_ const MMIOINFO FAR* pmmioinfo, _In_ DWORD fdwRename);
#endif /* ifdef WINVER >= 0x030a */
#endif

WINMMAPI
MMRESULT
WINAPI
mmioClose(
    _In_ HMMIO hmmio,
    _In_ UINT fuClose
    );

WINMMAPI
LONG
WINAPI
mmioRead(
    _In_ HMMIO hmmio,
    _Out_writes_bytes_(cch) HPSTR pch,
    _In_ LONG cch
    );

WINMMAPI
LONG
WINAPI
mmioWrite(
    _In_ HMMIO hmmio,
    _In_reads_bytes_(cch) const char  _huge * pch,
    _In_ LONG cch
    );

WINMMAPI
LONG
WINAPI
mmioSeek(
    _In_ HMMIO hmmio,
    _In_ LONG lOffset,
    _In_ int iOrigin
    );

WINMMAPI
MMRESULT
WINAPI
mmioGetInfo(
    _In_ HMMIO hmmio,
    _Out_ LPMMIOINFO pmmioinfo,
    _In_ UINT fuInfo
    );

WINMMAPI
MMRESULT
WINAPI
mmioSetInfo(
    _In_ HMMIO hmmio,
    _In_ LPCMMIOINFO pmmioinfo,
    _In_ UINT fuInfo
    );

WINMMAPI
MMRESULT
WINAPI
mmioSetBuffer(
    _In_ HMMIO hmmio,
    _Out_writes_opt_(cchBuffer) LPSTR pchBuffer,
    _In_ LONG cchBuffer,
    _In_ UINT fuBuffer
    );

WINMMAPI
MMRESULT
WINAPI
mmioFlush(
    _In_ HMMIO hmmio,
    _In_ UINT fuFlush
    );

WINMMAPI
MMRESULT
WINAPI
mmioAdvance(
    _In_ HMMIO hmmio,
    _In_opt_ LPMMIOINFO pmmioinfo,
    _In_ UINT fuAdvance
    );

WINMMAPI
LRESULT
WINAPI
mmioSendMessage(
    _In_ HMMIO hmmio,
    _In_ UINT uMsg,
    _In_opt_ LPARAM lParam1,
    _In_opt_ LPARAM lParam2
    );

WINMMAPI
MMRESULT
WINAPI
mmioDescend(
    _In_ HMMIO hmmio,
    _Inout_ LPMMCKINFO pmmcki,
    _In_opt_ const MMCKINFO  FAR * pmmckiParent,
    _In_ UINT fuDescend
    );

WINMMAPI
MMRESULT
WINAPI
mmioAscend(
    _In_ HMMIO hmmio,
    _In_ LPMMCKINFO pmmcki,
    _In_ UINT fuAscend
    );

WINMMAPI
MMRESULT
WINAPI
mmioCreateChunk(
    _In_ HMMIO hmmio,
    _In_ LPMMCKINFO pmmcki,
    _In_ UINT fuCreate
    );

#endif  /* ifndef MMNOMMIO */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _MMISCAPI_H_

