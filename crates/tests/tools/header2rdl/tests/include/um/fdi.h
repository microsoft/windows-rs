/***    fdi_int.h - Diamond File Decompression Interface definitions
 *                      
 *      Copyright (C) Microsoft Corporation 1993-1997
 *      All Rights Reserved.
 *
 *  Author:
 *      Chuck Strouss, Benjamin W. Slivka
 *
 *  History:
 *      30-Nov-1993 chuckst Created
 *      21-Dec-1993 bens    Updated with comments from 12/21/93 design review
 *      09-Mar-1994 bens    Add new error code
 *      17-Mar-1994 bens    Specify structure packing explicitly
 *      21-Mar-1994 bens    Spruce up comments
 *      25-Mar-1994 bens    Add fdintCABINET_INFO notification
 *      31-Mar-1994 bens    Clarify handling of open files when errors occur
 *      01-Apr-1994 bens    Add FDIIsCabinet() function.
 *      07-Apr-1994 bens    Add Decryption interfaces; remove fdintPROGRESS
 *      11-Apr-1994 bens    Add more guidance on how to respond to FDI errors.
 *      13-Apr-1994 bens    Add date & time & attribs to fdintCOPY_FILE
 *      18-Apr-1994 bens    Changed CDECL to DIAMONDAPI
 *      05-May-1994 bens    Clarified error handling (billhu/alanr/migueldc)
 *      11-May-1994 bens    Added setId/iCabinet to fdintNEXT_CABINET
 *      07-Jul-1994 bens    Support Quantum virtual file -- PLEASE note the
 *                              comments about PFNOPEN/PFNCLOSE changes, and
 *                              about reserving memory, if necessary, before
 *                              calling FDICreate()!
 *      19-Aug-1994 bens    Add cpuType parameter to FDICreate().
 *      03-Apr-1995 jeffwe  Added chaining indicators to FDICABINETINFO
 *      22-Nov-1996 msliger Backed out fdintNEXT_FOLDER, added iFolder to
 *                              FDINOTIFICATION for fdintCOPY_FILE calls.
 *      20-Feb-1997 msliger Added fdintENUMERATE message.
 *      11-Nov-2005 kinshu  SAL annotation
 *
 *
 *  ATTENTION:
 *      This is the only documentation on the Diamond File Decompression
 *      Interface (FDI).  Please read it carefully, as there are some subtle
 *      points in FDI that are carefully explained below.
 *
 *  Concepts:
 *      A *cabinet* file contains one or more *folders*.  A folder contains
 *      one or more (pieces of) *files*.  A folder is by definition a
 *      decompression unit, i.e., to extract a file from a folder, all of
 *      the data from the start of the folder up through and including the
 *      desired file must be read and decompressed.
 *
 *      A folder can span one (or more) cabinet boundaries, and by implication
 *      a file can also span one (or more) cabinet boundaries.  Indeed, more
 *      than one file can span a cabinet boundary, since Diamond concatenates
 *      files together into a single data stream before compressing (actually,
 *      at most one file will span any one cabinet boundary, but Diamond does
 *      not know which file this is, since the mapping from uncompressed bytes
 *      to compressed bytes is pretty obscure.  Also, since Diamond compresses
 *      in blocks of 32K (at present), any files with data in a 32K block that
 *      spans a cabinet boundary require Diamond to read both cabinet files
 *      to get the two halves of the compressed block).
 *
 *  Overview:
 *      The File Decompression Interface is used to simplify the reading of
 *      Diamond cabinet files.  A setup program will proceed in a manner very
 *      similar to the pseudo code below.  An FDI context is created, the
 *      setup program calls FDICopy() for each cabinet to be processed.  For
 *      each file in the cabinet, FDICopy() calls a notification callback
 *      routine, asking the setup program if the file should be copied.
 *      This call-back approach is great because it allows the cabinet file
 *      to be read and decompressed in an optimal manner, and also makes FDI
 *      independent of the run-time environment -- FDI makes *no* C run-time
 *      calls whatsoever.  All memory allocation and file I/O functions are
 *      passed into FDI by the client.
 *
 *      main(...)
 *      {
 *          // Read INF file to construct list of desired files.   
 *          //  Ideally, these would be sorted in the same order as the
 *          //  files appear in the cabinets, so that you can just walk
 *          //  down the list in response to fdintCOPY_FILE notifications.
 *
 *          // Construct list of required cabinets. 
 *
 *          hfdi = FDICreate(...);          // Create FDI context
 *          For (cabinet in List of Cabinets) {
 *              FDICopy(hfdi,cabinet,fdiNotify,...);  // Process each cabinet
 *          }
 *          FDIDestroy(hfdi);
 *          ...
 *      }
 *
 *      // Notification callback function 
 *      fdiNotify(fdint,...)
 *      {
 *          If (User Aborted)               // Permit cancellation
 *              if (fdint == fdintCLOSE_FILE_INFO)
 *                  close open file
 *              return -1;
 *          switch (fdint) {
 *              case fdintCOPY_FILE:        // File to copy, maybe
 *                  // Check file against list of desired files 
 *                  if want to copy file
 *                      open destination file and return handle
 *                  else
 *                      return NULL;        // Skip file
 *              case fdintCLOSE_FILE_INFO:
 *                  close file
 *                  set date, time, and attributes
 *
 *              case fdintNEXT_CABINET:
 *                  if not an error callback
 *                      Tell FDI to use suggested directory name
 *                  else
 *                      Tell user what the problem was, and prompt
 *                          for a new disk and/or path.
 *                      if user aborts
 *                          Tell FDI to abort
 *                      else
 *                          return to FDI to try another cabinet
 *                  //NOTE: Be sure to see the (sample) code in EXTRACT.C
 *                  //      for an example of how to do this!
 *              default:
 *                  return 0;               // more messages may be defined
 *              ...
 *      }
 *
 *  Error Handling Suggestions:
 *      Since you the client have passed in *all* of the functions that
 *      FDI uses to interact with the "outside" world, you are in prime
 *      position to understand and deal with errors.
 *
 *      The general philosophy of FDI is to pass all errors back up to
 *      the client.  FDI returns fairly generic error codes in the case
 *      where one of the callback functions (PFNOPEN, PFNREAD, etc.) fail,
 *      since it assumes that the callback function will save enough
 *      information in a static/global so that when FDICopy() returns
 *      fail, the client can examine this information and report enough
 *      detail about the problem that the user can take corrective action.
 *
 *      For very specific errors (CORRUPT_CABINET, for example), FDI returns
 *      very specific error codes.
 *
 *      THE BEST POLICY IS FOR YOUR CALLBACK ROUTINES TO AVOID RETURNING
 *      ERRORS TO FDI!
 *
 *      Examples:
 *          (1) If the disk is getting full, instead of returning an error
 *              from your PFNWRITE function, you should -- inside your
 *              PFNWRITE function -- put up a dialog telling the user to free
 *              some disk space.
 *          (2) When you get the fdintNEXT_CABINET notification, you should
 *              verify that the cabinet you return is the correct one (call
 *              FDIIsCabinet(), and make sure the setID matches the one for
 *              the current cabinet specified in the fdintCABINET_INFO, and
 *              that the disk number is one greater.
 *
 *              NOTE: FDI will continue to call fdintNEXT_CABINET until it
 *                    gets the cabinet it wants, or until you return -1
 *                    to abort the FDICopy() call.
 *
 *      The documentation below on the FDI error codes provides explicit
 *      guidance on how to avoid each error.
 *
 *      If you find you must return a failure to FDI from one of your
 *      callback functions, then FDICopy() frees all resources it allocated
 *      and closes all files.  If you can figure out how to overcome the
 *      problem, you can call FDICopy() again on the last cabinet, and
 *      skip any files that you already copied.  But, note that FDI does
 *      *not* maintain any state between FDICopy() calls, other than possibly
 *      memory allocated for the decompressor.
 *
 *      See FDIERROR for details on FDI error codes and recommended actions.
 *
 *
 *  Progress Indicator Suggestions:
 *      As above, all of the file I/O functions are supplied by you.  So,
 *      updating a progress indicator is very simple.  You keep track of
 *      the target files handles you have opened, along with the uncompressed
 *      size of the target file.  When you see writes to the handle of a
 *      target file, you use the write count to update your status!
 *      Since this method is available, there is no separate callback from
 *      FDI just for progess indication.
 */

#include <fdi_fci_types.h>

#include <basetsd.h>

#ifndef INCLUDED_FDI
#define INCLUDED_FDI    1

#include <specstrings.h>
typedef _Null_terminated_ char* LPSTR;

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif  /* __cplusplus */

//** Specify structure packing explicitly for clients of FDI

#ifndef _WIN64
#include <pshpack4.h>
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/***    FDIERROR - Error codes returned in erf.erfOper field
 *
 *  In general, FDI will only fail if one of the passed in memory or
 *  file I/O functions fails.  Other errors are pretty unlikely, and are
 *  caused by corrupted cabinet files, passing in a file which is not a
 *  cabinet file, or cabinet files out of order.
 *
 *  Description:    Summary of error.
 *  Cause:          List of possible causes of this error.
 *  Response:       How client might respond to this error, or avoid it in
 *                  the first place.
 */
typedef enum {
    FDIERROR_NONE,
        // Description: No error
        // Cause:       Function was successfull.
        // Response:    Keep going!

    FDIERROR_CABINET_NOT_FOUND,
        // Description: Cabinet not found
        // Cause:       Bad file name or path passed to FDICopy(), or returned
        //              to fdintNEXT_CABINET.
        // Response:    To prevent this error, validate the existence of the
        //              the cabinet *before* passing the path to FDI.

    FDIERROR_NOT_A_CABINET,
        // Description: Cabinet file does not have the correct format
        // Cause:       File passed to to FDICopy(), or returned to
        //              fdintNEXT_CABINET, is too small to be a cabinet file,
        //              or does not have the cabinet signature in its first
        //              four bytes.
        // Response:    To prevent this error, call FDIIsCabinet() to check a
        //              cabinet before calling FDICopy() or returning the
        //              cabinet path to fdintNEXT_CABINET.

    FDIERROR_UNKNOWN_CABINET_VERSION,
        // Description: Cabinet file has an unknown version number.
        // Cause:       File passed to to FDICopy(), or returned to
        //              fdintNEXT_CABINET, has what looks like a cabinet file
        //              header, but the version of the cabinet file format
        //              is not one understood by this version of FDI.  The
        //              erf.erfType field is filled in with the version number
        //              found in the cabinet file.
        // Response:    To prevent this error, call FDIIsCabinet() to check a
        //              cabinet before calling FDICopy() or returning the
        //              cabinet path to fdintNEXT_CABINET.

    FDIERROR_CORRUPT_CABINET,
        // Description: Cabinet file is corrupt
        // Cause:       FDI returns this error any time it finds a problem
        //              with the logical format of a cabinet file, and any
        //              time one of the passed-in file I/O calls fails when
        //              operating on a cabinet (PFNOPEN, PFNSEEK, PFNREAD,
        //              or PFNCLOSE).  The client can distinguish these two
        //              cases based upon whether the last file I/O call
        //              failed or not.
        // Response:    Assuming this is not a real corruption problem in
        //              a cabinet file, the file I/O functions could attempt
        //              to do retries on failure (for example, if there is a
        //              temporary network connection problem).  If this does
        //              not work, and the file I/O call has to fail, then the
        //              FDI client will have to clean up and call the
        //              FDICopy() function again.

    FDIERROR_ALLOC_FAIL,
        // Description: Could not allocate enough memory
        // Cause:       FDI tried to allocate memory with the PFNALLOC
        //              function, but it failed.
        // Response:    If possible, PFNALLOC should take whatever steps
        //              are possible to allocate the memory requested.  If
        //              memory is not immediately available, it might post a
        //              dialog asking the user to free memory, for example.
        //              Note that the bulk of FDI's memory allocations are
        //              made at FDICreate() time and when the first cabinet
        //              file is opened during FDICopy().

    FDIERROR_BAD_COMPR_TYPE,
        // Description: Unknown compression type in a cabinet folder
        // Cause:       [Should never happen.]  A folder in a cabinet has an
        //              unknown compression type.  This is probably caused by
        //              a mismatch between the version of Diamond used to
        //              create the cabinet and the FDI. LIB used to read the
        //              cabinet.
        // Response:    Abort.

    FDIERROR_MDI_FAIL,
        // Description: Failure decompressing data from a cabinet file
        // Cause:       The decompressor found an error in the data coming
        //              from the file cabinet.  The cabinet file was corrupted.
        //              [11-Apr-1994 bens When checksuming is turned on, this
        //              error should never occur.]
        // Response:    Probably should abort; only other choice is to cleanup
        //              and call FDICopy() again, and hope there was some
        //              intermittent data error that will not reoccur.

    FDIERROR_TARGET_FILE,
        // Description: Failure writing to target file
        // Cause:       FDI returns this error any time it gets an error back
        //              from one of the passed-in file I/O calls fails when
        //              writing to a file being extracted from a cabinet.
        // Response:    To avoid or minimize this error, the file I/O functions
        //              could attempt to avoid failing.  A common cause might
        //              be disk full -- in this case, the PFNWRITE function
        //              could have a check for free space, and put up a dialog
        //              asking the user to free some disk space.

    FDIERROR_RESERVE_MISMATCH,
        // Description: Cabinets in a set do not have the same RESERVE sizes
        // Cause:       [Should never happen]. FDI requires that the sizes of
        //              the per-cabinet, per-folder, and per-data block
        //              RESERVE sections be consistent across all the cabinet
        //              in a set.  Diamond will only generate cabinet sets
        //              with these properties.
        // Response:    Abort.

    FDIERROR_WRONG_CABINET,
        // Description: Cabinet returned on fdintNEXT_CABINET is incorrect
        // Cause:       NOTE: THIS ERROR IS NEVER RETURNED BY FDICopy()!
        //              Rather, FDICopy() keeps calling the fdintNEXT_CABINET
        //              callback until either the correct cabinet is specified,
        //              or you return ABORT.
        //              When FDICopy() is extracting a file that crosses a
        //              cabinet boundary, it calls fdintNEXT_CABINET to ask
        //              for the path to the next cabinet.  Not being very
        //              trusting, FDI then checks to make sure that the
        //              correct continuation cabinet was supplied!  It does
        //              this by checking the "setID" and "iCabinet" fields
        //              in the cabinet.  When DIAMOND.EXE creates a set of
        //              cabinets, it constructs the "setID" using the sum
        //              of the bytes of all the destination file names in
        //              the cabinet set.  FDI makes sure that the 16-bit
        //              setID of the continuation cabinet matches the
        //              cabinet file just processed.  FDI then checks that
        //              the cabinet number (iCabinet) is one more than the
        //              cabinet number for the cabinet just processed.
        // Response:    You need code in your fdintNEXT_CABINET (see below)
        //              handler to do retries if you get recalled with this
        //              error.  See the sample code (EXTRACT.C) to see how
        //              this should be handled.

    FDIERROR_USER_ABORT,
        // Description: FDI aborted.
        // Cause:       An FDI callback returned -1 (usually).
        // Response:    Up to client.

    FDIERROR_EOF,
        // Description: Unexpected end of file.
        // Cause:       This error may be returned instead of
        //              FDIERROR_CORRUPT_CABINET if PFNREAD returned 0.
        // Response:    See FDIERROR_CORRUPT_CABINET above.

} FDIERROR;


/*
 * FAT file attribute flag used by FCI/FDI to indicate that
 * the filename in the CAB is a UTF string
 */
#ifndef _A_NAME_IS_UTF
#define _A_NAME_IS_UTF  0x80
#endif

/*
 * FAT file attribute flag used by FCI/FDI to indicate that
 * the file should be executed after extraction
 */
#ifndef _A_EXEC
#define _A_EXEC         0x40
#endif


/***    HFDI - Handle to an FDI context
 *
 *  FDICreate() creates this, and it must be passed to all other FDI
 *  functions.
 */
typedef void FAR *HFDI; /* hfdi */


/***    FDICABINETINFO - Information about a cabinet
 *
 */
typedef struct {
    long        cbCabinet;              // Total length of cabinet file
    USHORT      cFolders;               // Count of folders in cabinet
    USHORT      cFiles;                 // Count of files in cabinet
    USHORT      setID;                  // Cabinet set ID
    USHORT      iCabinet;               // Cabinet number in set (0 based)
    BOOL        fReserve;               // TRUE => RESERVE present in cabinet
    BOOL        hasprev;                // TRUE => Cabinet is chained prev
    BOOL        hasnext;                // TRUE => Cabinet is chained next
} FDICABINETINFO; /* fdici */
typedef FDICABINETINFO FAR *PFDICABINETINFO; /* pfdici */


/***    FDIDECRYPTTYPE - PFNFDIDECRYPT command types
 *
 */
typedef enum {
    fdidtNEW_CABINET,                   // New cabinet
    fdidtNEW_FOLDER,                    // New folder
    fdidtDECRYPT,                       // Decrypt a data block
} FDIDECRYPTTYPE; /* fdidt */


/***    FDIDECRYPT - Data for PFNFDIDECRYPT function
 *
 */
typedef struct {
    FDIDECRYPTTYPE    fdidt;            // Command type (selects union below)
    void FAR         *pvUser;           // Decryption context
    union {
        struct {                        // fdidtNEW_CABINET
            void FAR *pHeaderReserve;   // RESERVE section from CFHEADER
            USHORT    cbHeaderReserve;  // Size of pHeaderReserve
            USHORT    setID;            // Cabinet set ID
            int       iCabinet;         // Cabinet number in set (0 based)
        } cabinet;

        struct {                        // fdidtNEW_FOLDER
            void FAR *pFolderReserve;   // RESERVE section from CFFOLDER
            USHORT    cbFolderReserve;  // Size of pFolderReserve
            USHORT    iFolder;          // Folder number in cabinet (0 based)
        } folder;

        struct {                        // fdidtDECRYPT
            void FAR *pDataReserve;     // RESERVE section from CFDATA
            USHORT    cbDataReserve;    // Size of pDataReserve
            void FAR *pbData;           // Data buffer
            USHORT    cbData;           // Size of data buffer
            BOOL      fSplit;           // TRUE if this is a split data block
            USHORT    cbPartial;        // 0 if this is not a split block, or
                                        //  the first piece of a split block;
                                        // Greater than 0 if this is the
                                        //  second piece of a split block.
        } decrypt;
    };
} FDIDECRYPT; /* fdid */
typedef FDIDECRYPT FAR *PFDIDECRYPT; /* pfdid */


/***    FNALLOC - Memory Allocation
 *      FNFREE  - Memory Free
 *
 *  These are modeled after the C run-time routines malloc() and free()
 *  (16-bit clients please note -- the size is a ULONG, so you may need
 *  to write a wrapper routine for halloc!).  FDI expects error
 *  handling to be identical to these C run-time routines.
 *
 *  As long as you faithfully copy the semantics of malloc() and free(),
 *  you can supply any functions you like!
 *
 *  WARNING: You should never assume anything about the sequence of
 *           PFNALLOC and PFNFREE calls -- incremental releases of
 *           Diamond/FDI may have radically different numbers of
 *           PFNALLOC calls and allocation sizes!
 */
//** Memory functions for FDI
typedef void HUGE * (FAR DIAMONDAPI *PFNALLOC)(ULONG cb); /* pfna */
#define FNALLOC(fn) void HUGE * FAR DIAMONDAPI fn(ULONG cb)

typedef void (FAR DIAMONDAPI *PFNFREE)(_In_opt_ void HUGE *pv); /* pfnf */
#define FNFREE(fn) void FAR DIAMONDAPI fn(_In_opt_ void HUGE *pv)


//** File I/O functions for FDI
typedef INT_PTR (FAR DIAMONDAPI *PFNOPEN) (_In_ LPSTR pszFile, int oflag, int pmode);
typedef UINT (FAR DIAMONDAPI *PFNREAD) (_In_ INT_PTR hf, _Out_writes_bytes_(cb) void FAR *pv, UINT cb);
typedef UINT (FAR DIAMONDAPI *PFNWRITE)(_In_ INT_PTR hf, _In_reads_bytes_(cb) void FAR *pv, UINT cb);
typedef int  (FAR DIAMONDAPI *PFNCLOSE)(_In_ INT_PTR hf);
typedef long (FAR DIAMONDAPI *PFNSEEK) (_In_ INT_PTR hf, long dist, int seektype);

#define FNOPEN(fn) INT_PTR FAR DIAMONDAPI fn(_In_ LPSTR pszFile, int oflag, int pmode)
#define FNREAD(fn) UINT FAR DIAMONDAPI fn(_In_ INT_PTR hf, _Out_writes_bytes_(cb) void FAR *pv, UINT cb)
#define FNWRITE(fn) UINT FAR DIAMONDAPI fn(_In_ INT_PTR hf, _In_reads_bytes_(cb) void FAR *pv, UINT cb)
#define FNCLOSE(fn) int FAR DIAMONDAPI fn(_In_ INT_PTR hf)
#define FNSEEK(fn) long FAR DIAMONDAPI fn(_In_ INT_PTR hf, long dist, int seektype)



/***    PFNFDIDECRYPT - FDI Decryption callback
 *
 *  If this function is passed on the FDICopy() call, then FDI calls it
 *  at various times to update the decryption state and to decrypt FCDATA
 *  blocks.
 *
 *  Common Entry Conditions:
 *      pfdid->fdidt  - Command type
 *      pfdid->pvUser - pvUser value from FDICopy() call
 *
 *  fdidtNEW_CABINET:   //** Notification of a new cabinet
 *      Entry:
 *        pfdid->cabinet.
 *          pHeaderReserve  - RESERVE section from CFHEADER
 *          cbHeaderReserve - Size of pHeaderReserve
 *          setID           - Cabinet set ID
 *          iCabinet        - Cabinet number in set (0 based)
 *      Exit-Success:
 *          returns anything but -1;
 *      Exit-Failure:
 *          returns -1; FDICopy() is aborted.
 *      Notes:
 *      (1) This call allows the decryption code to pick out any information
 *          from the cabinet header reserved area (placed there by DIACRYPT)
 *          needed to perform decryption.  If there is no such information,
 *          this call would presumably be ignored.
 *      (2) This call is made very soon after fdintCABINET_INFO.
 *
 *  fdidtNEW_FOLDER:    //** Notification of a new folder
 *      Entry:
 *        pfdid->folder.
 *          pFolderReserve  - RESERVE section from CFFOLDER
 *          cbFolderReserve - Size of pFolderReserve
 *          iFolder         - Folder number in cabinet (0 based)
 *      Exit-Success:
 *          returns anything but -1;
 *      Exit-Failure:
 *          returns -1; FDICopy() is aborted.
 *      Notes:
 *          This call allows the decryption code to pick out any information
 *          from the folder reserved area (placed there by DIACRYPT) needed
 *          to perform decryption.  If there is no such information, this
 *          call would presumably be ignored.
 *
 *  fdidtDECRYPT:       //** Decrypt a data buffer
 *      Entry:
 *        pfdid->folder.
 *          pDataReserve  - RESERVE section for this CFDATA block
 *          cbDataReserve - Size of pDataReserve
 *          pbData        - Data buffer
 *          cbData        - Size of data buffer
 *          fSplit        - TRUE if this is a split data block
 *          cbPartial     - 0 if this is not a split block, or the first
 *                              piece of a split block; Greater than 0 if
 *                              this is the second piece of a split block.
 *      Exit-Success:
 *          returns TRUE;
 *      Exit-Failure:
 *          returns FALSE; error during decrypt
 *          returns -1; FDICopy() is aborted.
 *      Notes:
 *          Diamond will split CFDATA blocks across cabinet boundaries if
 *          necessary.  To provide maximum flexibility, FDI will call the
 *          fdidtDECRYPT function twice on such split blocks, once when
 *          the first portion is read, and again when the second portion
 *          is read.  And, of course, most data blocks will not be split.
 *          So, there are three cases:
 *
 *           1) fSplit == FALSE
 *              You have the entire data block, so decrypt it.
 *
 *           2) fSplit == TRUE, cbPartial == 0
 *              This is the first portion of a split data block, so cbData
 *              is the size of this portion.  You can either choose to decrypt
 *              this piece, or ignore this call and decrypt the full CFDATA
 *              block on the next (second) fdidtDECRYPT call.
 *
 *           3) fSplit == TRUE, cbPartial > 0
 *              This is the second portion of a split data block (indeed,
 *              cbPartial will have the same value as cbData did on the
 *              immediately preceeding fdidtDECRYPT call!).  If you decrypted
 *              the first portion on the first call, then you can decrypt the
 *              second portion now.  If you ignored the first call, then you
 *              can decrypt the entire buffer.
 *              NOTE: pbData points to the second portion of the split data
 *                    block in this case, *not* the entire data block.  If
 *                    you want to wait until the second piece to decrypt the
 *                    *entire* block, pbData-cbPartial is the address of the
 *                    start of the whole block, and cbData+cbPartial is its
 *                    size.
 */
typedef int (FAR DIAMONDAPI *PFNFDIDECRYPT)(PFDIDECRYPT pfdid); /* pfnfdid */
#define FNFDIDECRYPT(fn) int FAR DIAMONDAPI fn(PFDIDECRYPT pfdid)


/***    FDINOTIFICATION - Notification structure for PFNFDINOTIFY
 *
 *  See the FDINOTIFICATIONTYPE definition for information on usage and
 *  meaning of these fields.
 */
typedef struct {
// long fields
    long      cb;
    char FAR *psz1;
    char FAR *psz2;
    char FAR *psz3;                     // Points to a 256 character buffer
    void FAR *pv;                       // Value for client

// int fields
    INT_PTR   hf;

// short fields
    USHORT    date;
    USHORT    time;
    USHORT    attribs;

    USHORT    setID;                    // Cabinet set ID
    USHORT    iCabinet;                 // Cabinet number (0-based)
    USHORT    iFolder;                  // Folder number (0-based)

    FDIERROR  fdie;
} FDINOTIFICATION, FAR *PFDINOTIFICATION;  /* fdin, pfdin */


/***    FDINOTIFICATIONTYPE - FDICopy notification types
 *
 *  The notification function for FDICopy can be called with the following
 *  values for the fdint parameter.  In all cases, the pfdin->pv field is
 *  filled in with the value of the pvUser argument passed in to FDICopy().
 *
 *  A typical sequence of calls will be something like this:
 *      fdintCABINET_INFO     // Info about the cabinet
 *      fdintENUMERATE        // Starting enumeration
 *      fdintPARTIAL_FILE     // Only if this is not the first cabinet, and
 *                            // one or more files were continued from the
 *                            // previous cabinet.
 *      ...
 *      fdintPARTIAL_FILE
 *      fdintCOPY_FILE        // The first file that starts in this cabinet
 *      ...
 *      fdintCOPY_FILE        // Now let's assume you want this file...
 *      // PFNWRITE called multiple times to write to this file.
 *      fdintCLOSE_FILE_INFO  // File done, set date/time/attributes
 *
 *      fdintCOPY_FILE        // Now let's assume you want this file...
 *      // PFNWRITE called multiple times to write to this file.
 *      fdintNEXT_CABINET     // File was continued to next cabinet!
 *      fdintCABINET_INFO     // Info about the new cabinet
 *      // PFNWRITE called multiple times to write to this file.
 *      fdintCLOSE_FILE_INFO  // File done, set date/time/attributes
 *      ...
 *      fdintENUMERATE        // Ending enumeration
 *
 *  fdintCABINET_INFO:
 *        Called exactly once for each cabinet opened by FDICopy(), including
 *        continuation cabinets opened due to file(s) spanning cabinet
 *        boundaries. Primarily intended to permit EXTRACT.EXE to
 *        automatically select the next cabinet in a cabinet sequence even if
 *        not copying files that span cabinet boundaries.
 *      Entry:
 *          pfdin->psz1     = name of next cabinet
 *          pfdin->psz2     = name of next disk
 *          pfdin->psz3     = cabinet path name
 *          pfdin->setID    = cabinet set ID (a random 16-bit number)
 *          pfdin->iCabinet = Cabinet number within cabinet set (0-based)
 *      Exit-Success:
 *          Return anything but -1
 *      Exit-Failure:
 *          Returns -1 => Abort FDICopy() call
 *      Notes:
 *          This call is made *every* time a new cabinet is examined by
 *          FDICopy().  So if "foo2.cab" is examined because a file is
 *          continued from "foo1.cab", and then you call FDICopy() again
 *          on "foo2.cab", you will get *two* fdintCABINET_INFO calls all
 *          told.
 *
 *  fdintCOPY_FILE:
 *        Called for each file that *starts* in the current cabinet, giving
 *        the client the opportunity to request that the file be copied or
 *        skipped.
 *      Entry:
 *          pfdin->psz1    = file name in cabinet
 *          pfdin->cb      = uncompressed size of file
 *          pfdin->date    = file date
 *          pfdin->time    = file time
 *          pfdin->attribs = file attributes
 *          pfdin->iFolder = file's folder index
 *      Exit-Success:
 *          Return non-zero file handle for destination file; FDI writes
 *          data to this file use the PFNWRITE function supplied to FDICreate,
 *          and then calls fdintCLOSE_FILE_INFO to close the file and set
 *          the date, time, and attributes.  NOTE: This file handle returned
 *          must also be closeable by the PFNCLOSE function supplied to
 *          FDICreate, since if an error occurs while writing to this handle,
 *          FDI will use the PFNCLOSE function to close the file so that the
 *          client may delete it.
 *      Exit-Failure:
 *          Returns 0  => Skip file, do not copy
 *          Returns -1 => Abort FDICopy() call
 *
 *  fdintCLOSE_FILE_INFO:
 *        Called after all of the data has been written to a target file.
 *        This function must close the file and set the file date, time,
 *        and attributes.
 *      Entry:
 *          pfdin->psz1    = file name in cabinet
 *          pfdin->hf      = file handle
 *          pfdin->date    = file date
 *          pfdin->time    = file time
 *          pfdin->attribs = file attributes
 *          pfdin->iFolder = file's folder index
 *          pfdin->cb      = Run After Extract (0 - don't run, 1 Run)
 *      Exit-Success:
 *          Returns TRUE
 *      Exit-Failure:
 *          Returns FALSE, or -1 to abort;
 *
 *              IMPORTANT NOTE IMPORTANT:
 *                  pfdin->cb is overloaded to no longer be the size of
 *                  the file but to be a binary indicated run or not
 *
 *              IMPORTANT NOTE:
 *                  FDI assumes that the target file was closed, even if this
 *                  callback returns failure.  FDI will NOT attempt to use
 *                  the PFNCLOSE function supplied on FDICreate() to close
 *                  the file!
 *
 *  fdintPARTIAL_FILE:
 *        Called for files at the front of the cabinet that are CONTINUED
 *        from a previous cabinet.  This callback occurs only when FDICopy is
 *        started on second or subsequent cabinet in a series that has files
 *        continued from a previous cabinet.
 *      Entry:
 *          pfdin->psz1 = file name of file CONTINUED from a PREVIOUS cabinet
 *          pfdin->psz2 = name of cabinet where file starts
 *          pfdin->psz3 = name of disk where file starts
 *      Exit-Success:
 *          Return anything other than -1; enumeration continues
 *      Exit-Failure:
 *          Returns -1 => Abort FDICopy() call
 *
 *  fdintENUMERATE:
 *        Called once after a call to FDICopy() starts scanning a CAB's
 *        CFFILE entries, and again when there are no more CFFILE entries.
 *        If CAB spanning occurs, an additional call will occur after the
 *        first spanned file is completed.  If the pfdin->iFolder value is
 *        changed from zero, additional calls will occur next time it reaches
 *        zero.  If iFolder is changed to zero, FDICopy will terminate, as if
 *        there were no more CFFILE entries.  Primarily intended to allow an
 *        application with it's own file list to help FDI advance quickly to
 *        a CFFILE entry of interest.  Can also be used to allow an
 *        application to determine the cb values for each file in the CAB.
 *      Entry:
 *        pfdin->cb        = current CFFILE position
 *        pfdin->iFolder   = number of files remaining
 *        pfdin->setID     = current CAB's setID value
 *      Exit-Don't Care:
 *        Don't change anything.
 *        Return anything but -1.
 *      Exit-Forcing a skip:
 *        pfdin->cb        = desired CFFILE position
 *        pfdin->iFolder   = desired # of files remaining
 *        Return anything but -1.
 *      Exit-Stop:
 *        pfdin->iFolder    = set to 0
 *        Return anything but -1.
 *      Exit-Failure:
 *        Return -1 => Abort FDICopy call ("user aborted".)
 *      Notes:
 *        This call can be ignored by applications which want normal file
 *        searching.  The application can adjust the supplied values to
 *        force FDICopy() to continue it's search at another location, or
 *        to force FDICopy() to terminate the search, by setting iFolder to 0.
 *        (FDICopy() will report no error when terminated this way.)
 *        FDI has no means to verify the supplied cb or iFolder values.
 *        Arbitrary values are likely to cause undesirable results.  An
 *        application should cross-check pfdin->setID to be certain the
 *        external database is in sync with the CAB.  Reverse-skips are OK
 *        (but may be inefficient) unless fdintNEXT_CABINET has been called.
 *
 *  fdintNEXT_CABINET:
 *        This function is *only* called when fdintCOPY_FILE was told to copy
 *        a file in the current cabinet that is continued to a subsequent
 *        cabinet file.  It is important that the cabinet path name (psz3)
 *        be validated before returning!  This function should ensure that
 *        the cabinet exists and is readable before returning.  So, this
 *        is the function that should, for example, issue a disk change
 *        prompt and make sure the cabinet file exists.
 *
 *        When this function returns to FDI, FDI will check that the setID
 *        and iCabinet match the expected values for the next cabinet.
 *        If not, FDI will continue to call this function until the correct
 *        cabinet file is specified, or until this function returns -1 to
 *        abort the FDICopy() function.  pfdin->fdie is set to
 *        FDIERROR_WRONG_CABINET to indicate this case.
 *
 *        If you *haven't* ensured that the cabinet file is present and
 *        readable, or the cabinet file has been damaged, pfdin->fdie will
 *        receive other appropriate error codes:
 *
 *              FDIERROR_CABINET_NOT_FOUND
 *              FDIERROR_NOT_A_CABINET
 *              FDIERROR_UNKNOWN_CABINET_VERSION
 *              FDIERROR_CORRUPT_CABINET
 *              FDIERROR_BAD_COMPR_TYPE
 *              FDIERROR_RESERVE_MISMATCH
 *              FDIERROR_WRONG_CABINET
 *
 *      Entry:
 *          pfdin->psz1 = name of next cabinet where current file is continued
 *          pfdin->psz2 = name of next disk where current file is continued
 *          pfdin->psz3 = cabinet path name; FDI concatenates psz3 with psz1
 *                          to produce the fully-qualified path for the cabinet
 *                          file.  The 256-byte buffer pointed at by psz3 may
 *                          be modified, but psz1 may not!
 *          pfdin->fdie = FDIERROR_WRONG_CABINET if the previous call to
 *                        fdintNEXT_CABINET specified a cabinet file that
 *                        did not match the setID/iCabinet that was expected.
 *      Exit-Success:
 *          Return anything but -1
 *      Exit-Failure:
 *          Returns -1 => Abort FDICopy() call
 *      Notes:
 *          This call is almost always made when a target file is open and
 *          being written to, and the next cabinet is needed to get more
 *          data for the file.
 */
typedef enum {
    fdintCABINET_INFO,              // General information about cabinet
    fdintPARTIAL_FILE,              // First file in cabinet is continuation
    fdintCOPY_FILE,                 // File to be copied
    fdintCLOSE_FILE_INFO,           // close the file, set relevant info
    fdintNEXT_CABINET,              // File continued to next cabinet
    fdintENUMERATE,                 // Enumeration status
} FDINOTIFICATIONTYPE; /* fdint */

typedef INT_PTR (FAR DIAMONDAPI *PFNFDINOTIFY)(FDINOTIFICATIONTYPE fdint,
                                               PFDINOTIFICATION    pfdin); /* pfnfdin */

#define FNFDINOTIFY(fn) INT_PTR FAR DIAMONDAPI fn(FDINOTIFICATIONTYPE fdint, \
                                                  PFDINOTIFICATION    pfdin)


/***    PFNOPEN  - File I/O callbacks for FDI
 *      PFNREAD
 *      PFNWRITE
 *      PFNCLOSE
 *      PFNSEEK
 *
 *  These are modeled after the C run-time routines _open, _read,
 *  _write, _close, and _lseek.  The values for the PFNOPEN oflag
 *  and pmode calls are those defined for _open.  FDI expects error
 *  handling to be identical to these C run-time routines.
 *
 *  As long as you faithfully copy these aspects, you can supply
 *  any functions you like!
 *
 *
 *  SPECIAL NOTE FOR QUANTUM DECOMPRESSION:
 *      When using Quantum compression, at compress time (with Diamond)
 *      you specify how much memory Quantum requires at *decompress* time
 *      to store the decompression history buffer.  This can be as large
 *      as *2Mb*, and in an MS-DOS environment, for example, this much
 *      memory may not be available (certainly not under 640K!).  To permit
 *      large CompressionMemory settings on any machine, the Quantum
 *      decompressor will attempt to create a "spill file" if there is not
 *      sufficient memory available.
 *
 *      For PFNOPEN, a special pszFile parameter is passed to indicate that
 *      a temporary "spill file" is requested.  The name passed is "*", and
 *      you should cast the pszFile parameter to an FDISPILLFILE pointer,
 *      and get the requested file size.  You then need to create a file
 *      of the specified size with read/write access, save the file name and
 *      handle for later use by PFNCLOSE, and then return the handle.  If
 *      you cannot create the file of the specified size, you should return
 *      an error (-1).  This file should be placed on a fast local hard disk,
 *      to maximize the speed of decompression.
 *
 *      For PFNCLOSE, you should check the handle to see if it the spill file
 *      created previously by PFNOPEN (FDI will create at most one spill file
 *      per FDICreate() call).  If it is the spill file handle, you should
 *      close the handle and then delete the file, using the file name you
 *      saved when you created the spill file in PFNOPEN.
 *
 *  WARNING: You should never assume you know what file is being
 *           opened at any one point in time!  FDI will usually
 *           stick to opening cabinet files, but it is possible
 *           that in a future implementation it may open temporary
 *           files or open cabinet files in a different order.
 *
 *  Notes for Memory Mapped File fans:
 *      You can write wrapper routines to allow FDI to work on memory
 *      mapped files.  You'll have to create your own "handle" type so that
 *      you can store the base memory address of the file and the current
 *      seek position, and then you'll allocate and fill in one of these
 *      structures and return a pointer to it in response to the PFNOPEN
 *      call and the fdintCOPY_FILE call.  Your PFNREAD and PFNWRITE
 *      functions will do memcopy(), and update the seek position in your
 *      "handle" structure.  PFNSEEK will just change the seek position
 *      in your "handle" structure.
 */

#ifndef _WIN64
#pragma pack (1)
#endif

/** FDISPILLFILE - Pass as pszFile on PFNOPEN to create spill file
 *
 *  ach    - A two byte string to signal to PFNOPEN that a spill file is
 *           requested.  Value is '*','\0'.
 *  cbFile - Required spill file size, in bytes.
 */
typedef struct {
    char    ach[2];                 // Set to { '*', '\0' }
    long    cbFile;                 // Required spill file size
} FDISPILLFILE; /* fdisf */
typedef FDISPILLFILE *PFDISPILLFILE; /* pfdisf */

#ifndef _WIN64
#pragma pack ()
#endif


/*** cpuType values for FDICreate()
 *
 *  WARNING: For 16-bit Windows applications, the CPU detection may not
 *           correctly detect 286 CPUs.  Instead, use the following code:
 *
 *              DWORD   flags;
 *              int     cpuType;
 *
 *              flags = GetWinFlags();
 *              if (flags & WF_CPU286)
 *                  cpuType = cpu80286;
 *              else
 *                  cpuType = cpu80386;
 *
 *              hfdi = FDICreate(....,cpuType,...);
 */
#define     cpuUNKNOWN         (-1)    /* FDI does detection */
#define     cpu80286           (0)     /* '286 opcodes only */
#define     cpu80386           (1)     /* '386 opcodes used */


/***    FDICreate - Create an FDI context
 *
 *  Entry:
 *      pfnalloc
 *      pfnfree
 *      pfnopen
 *      pfnread
 *      pfnwrite
 *      pfnclose
 *      pfnlseek
 *      cpuType  - Select CPU type (auto-detect, 286, or 386+)
 *                 WARNING: Don't use auto-detect from a 16-bit Windows
 *                          application!  Use GetWinFlags()!
 *                 NOTE: For the 32-bit FDI.LIB, this parameter is ignored!
 *      perf
 *
 *  Exit-Success:
 *      Returns non-NULL FDI context handle.
 *
 *  Exit-Failure:
 *      Returns NULL; perf filled in with error code
 *
 *  Special notes for Quantum Decompression:
 *      If you have used a high setting for CompressionMemory in creating
 *      the cabinet file(s), then FDI will attempt to allocate a lot of
 *      memory (as much as 2Mb, if you specified 21 for CompressionMemory).
 *      Therefore, if you plan to allocate additional memory *after* the
 *      FDICreate() call, you should reserve some memory *prior* to calling
 *      FDICreate(), and then free it up afterwards (or do all your allocation
 *      before calling FDICreate().
 */
HFDI FAR DIAMONDAPI FDICreate(__callback PFNALLOC pfnalloc,
                              __callback PFNFREE  pfnfree,
                              __callback PFNOPEN  pfnopen,
                              __callback PFNREAD  pfnread,
                              __callback PFNWRITE pfnwrite,
                              __callback PFNCLOSE pfnclose,
                              __callback PFNSEEK  pfnseek,
                              int                 cpuType,
                              _Inout_    PERF     perf);


/***    FDIIsCabinet - Determines if file is a cabinet, returns info if it is
 *
 *  Entry:
 *      hfdi   - Handle to FDI context (created by FDICreate())
 *      hf     - File handle suitable for PFNREAD/PFNSEEK, positioned
 *               at offset 0 in the file to test.
 *      pfdici - Buffer to receive info about cabinet if it is one.
 *
 *  Exit-Success:
 *      Returns TRUE; file is a cabinet, pfdici filled in.
 *
 *  Exit-Failure:
 *      Returns FALSE, file is not a cabinet;  If an error occurred,
 *          perf (passed on FDICreate call!) filled in with error.
 */
BOOL FAR DIAMONDAPI FDIIsCabinet(_In_  HFDI             hfdi,
                                 _In_  INT_PTR          hf,
                                 _Out_opt_ PFDICABINETINFO  pfdici);


/***    FDICopy - extracts files from a cabinet
 *
 *  Entry:
 *      hfdi        - handle to FDI context (created by FDICreate())
 *      pszCabinet  - main name of cabinet file
 *      pszCabPath  - Path to cabinet file(s)
 *      flags       - Flags to modify behavior
 *      pfnfdin     - Notification function
 *      pfnfdid     - Decryption function (pass NULL if not used)
 *      pvUser      - User specified value to pass to notification function
 *
 *  Exit-Success:
 *      Returns TRUE;
 *
 *  Exit-Failure:
 *      Returns FALSE, perf (passed on FDICreate call!) filled in with
 *          error.
 *
 *  Notes:
 *  (1) If FDICopy() fails while a target file is being written out, then
 *      FDI will use the PFNCLOSE function to close the file handle for that
 *      target file that was returned from the fdintCOPY_FILE notification.
 *      The client application is then free to delete the target file, since
 *      it will not be in a valid state (since there was an error while
 *      writing it out).
 */
BOOL FAR DIAMONDAPI FDICopy(_In_ HFDI                   hfdi,
                            _In_ LPSTR                  pszCabinet,
                            _In_ LPSTR                  pszCabPath,
                            int                         flags,
                            __callback PFNFDINOTIFY     pfnfdin,
                            __callback PFNFDIDECRYPT    pfnfdid,
                            _In_opt_ void FAR           *pvUser);


/***    FDIDestroy - Destroy an FDI context
 *
 *  Entry:
 *      hfdi - handle to FDI context (created by FDICreate())
 *
 *  Exit-Success:
 *      Returns TRUE;
 *
 *  Exit-Failure:
 *      Returns FALSE;
 */
BOOL FAR DIAMONDAPI FDIDestroy(_In_ HFDI hfdi);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/***    FDITruncateCabinet - truncate a cabinet, starting at folder #
 *
 *  Entry:
 *      hfdi        - handle to FDI context (created by FDICreate())
 *      pszCabinet  - full pathname of cabinet file
 *      iFolderToDelete - first folder # to delete
 *
 *  Exit-Success:
 *      Returns TRUE;
 *
 *  Exit-Failure:
 *      Returns FALSE;
 *
 *  Most likely failure is unable to open cabinet for writing.
 *
 *  NOTE:   This function relies on additional feature in the
 *          supplied PFNWRITE function, that a write of 0 bytes
 *          will truncate the file at the current position.
 */

BOOL FAR DIAMONDAPI FDITruncateCabinet(_In_ HFDI    hfdi,
                                       _In_ LPSTR   pszCabinetName,
                                       USHORT       iFolderToDelete);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

//** Revert to default structure packing
#ifndef _WIN64
#include <poppack.h>
#endif

#ifdef __cplusplus
}
#endif  /* __cplusplus */


#endif //!INCLUDED_FDI
