/*++

    Copyright (c) 2002  Microsoft Corporation

    Module Name:

	    FILEHC.H

    Abstract:

	    This file defines the public interfaces for issuing async
        Reads/Writes to a file using the fcache wrapper library.

--*/

#ifndef	_FILEHC_H_
#define	_FILEHC_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef	__cplusplus	
extern	"C"	{
#endif

struct FIO_CONTEXT;
struct FH_OVERLAPPED;

typedef	VOID
(*PFN_IO_COMPLETION)(
		IN	struct	FIO_CONTEXT*	pContext,
		IN	struct	FH_OVERLAPPED*	lpo, 
		IN	DWORD		cb, 
		IN	DWORD		dwCompletionStatus
		);



struct	FH_OVERLAPPED	{
/*++

	This structure defines the extended OVERLAPPED structure
	used by the File IO layer implemented in this module.

	The first 5 elements of this structure are identical to 
	NT's OVERLAPPED structure and have the exact same semantics.
	
	The final additional parameter is a pointer to a 
	function that will be called to complete the IO.

--*/
	UINT_PTR	Internal ;
	UINT_PTR	InternalHigh ;
	DWORD		Offset ;
	DWORD		OffsetHigh ;
	HANDLE		hEvent ;
	PFN_IO_COMPLETION	pfnCompletion ;	
	UINT_PTR	Reserved1 ;
	UINT_PTR	Reserved2 ;
	UINT_PTR	Reserved3 ;
	UINT_PTR	Reserved4 ;
} ;

typedef	struct	FH_OVERLAPPED*	PFH_OVERLAPPED ;

struct	FIO_CONTEXT	{
/*++

	This structure defines the context object
	that is used to represent file handles.

--*/
    //
    //  Temporary hack - mailmsg object assumes it can put a NULL in us !
    //
    DWORD       m_dwTempHack ;

	//
	//	The context signature !
	//
	DWORD		m_dwSignature ;

	//
	//	The users file handle !
	//
	HANDLE		m_hFile ;

	//
	//  The offset to back fill Lines header - nntp aware only
	//
	DWORD       m_dwLinesOffset;

	//
	//  Header length - nntp aware only
	//
	DWORD       m_dwHeaderLength;
} ;

typedef	struct	FIO_CONTEXT*	PFIO_CONTEXT ;


#ifdef	_FILEHC_IMPLEMENTATION_
#define	FILEHC_EXPORT	__declspec( dllexport )	
#else
#define	FILEHC_EXPORT	__declspec( dllimport )	
#endif


//
//	Initialize the DLL for Async IO - 
//	This is a counting initialize - for each call to FIOInitialize()
//	there should be a matching call to FIOTerminate
//
FILEHC_EXPORT
BOOL	__stdcall
FIOInitialize(
    IN DWORD dwFlags
    );

//
//	Terminate the DLL's support for Async IO !
//
FILEHC_EXPORT
BOOL	__stdcall
FIOTerminate(
    VOID
    );

//
//	Do an async read against the File !
//
FILEHC_EXPORT
BOOL	__stdcall
FIOReadFile(
    IN  PFIO_CONTEXT	pContext,
    _In_reads_bytes_(BytesToRead) IN  LPVOID			lpBuffer,
    IN  DWORD			BytesToRead,
    _Inout_ IN struct FH_OVERLAPPED *	lpo
    );

//
//	Do an async read against the file - pass extra args
//	so that if the FIO_CONTEXT is doing dot stuffing for the user
//	it can do so efficiently !
//
FILEHC_EXPORT
BOOL	__stdcall
FIOReadFileEx(
    IN  PFIO_CONTEXT	pContext,
    _In_reads_bytes_(BytesToRead) IN  LPVOID			lpBuffer,
    IN  DWORD			BytesToRead,
	IN	DWORD			BytesAvailable, // must be >= BytesToWrite - number of bytes I can mess with.
    _Inout_ IN struct FH_OVERLAPPED *	lpo,
	IN	BOOL			fFinalWrite,	// Is this the final write ? 
	IN	BOOL			fIncludeTerminator	// if TRUE contains CRLF.CRLF terminator which shouldn't be stuffed
    );


//
//	Do an async write against the file !
//
FILEHC_EXPORT
BOOL	__stdcall
FIOWriteFile(
    IN  PFIO_CONTEXT	pContext,
    _Inout_updates_bytes_(BytesToWrite) IN  LPCVOID			lpBuffer,
    IN  DWORD			BytesToWrite,
    _Inout_ IN struct FH_OVERLAPPED * lpo
    );

//
//	Do an async write against the file - pass extra args
//	so that if the FIO_CONTEXT is doing dot stuffing for the user
//	it can do so efficiently !
//
FILEHC_EXPORT
BOOL	__stdcall
FIOWriteFileEx(
	IN	PFIO_CONTEXT	pContext,
	_Inout_updates_bytes_(BytesAvailable) IN	LPVOID			lpBuffer,
	IN	DWORD			BytesToWrite,
	IN	DWORD			BytesAvailable, // must be >= BytesToWrite - number of bytes I can mess with.
	_Inout_ IN	struct FH_OVERLAPPED*	lpo,
	IN	BOOL			fFinalWrite,	// Is this the final write ? 
	IN	BOOL			fIncludeTerminator	// if TRUE contains CRLF.CRLF terminator which shouldn't be stuffed
	) ;

//
//	Callback functions which create things in the cache !
//
//	NOTE: this is equivalent to FCACHE_RICHCREATE_CALLBACK where
//
//	pfDidWeScanIt - returns FALSE
//	pfIsStuffed - return FALSE
//	pfStoredWithDots - return FALSE
//
typedef	
HANDLE	
(__stdcall	*FCACHE_CREATE_CALLBACK) (
		IN	LPSTR	lpstrName, 
		IN	LPVOID	lpvData, 
		OUT	DWORD*	cbFileSize,
		OUT	DWORD*	cbFileSizeHigh
		) ;


//
//	Callback functions which create things in the cache !
//
//	This function will be called by CacheRichCreateFile().
//
//	lpstrName - the name of the file 
//	lpvData - User provided data, provided to CacheRichCreateFile
//	cbFileSize - The function should return the size of the file through this
//	cbFileSizeHigh - place to return the High DWORD of the file size
//	pfDidWeScanIt - if THIS is true then at some point the created file has been
//		scanned for DOTs appearing at the beginning of lines
//	pfIsStuffed - This is only meaningfull if pfDidWeScanIt==TRUE, in which case
//		if this is TRUE this indicates that there are DOTs at the beginning of lines
//	pfStoredWithDots - If this is TRUE then it indicates that any DOTs that appear
//		at the beginning of lines are stored with an extra dot as required in NNTP, 
//		SMTP and POP3 protocols.  if this is FALSE then the message is stored without
//		DOT stuffing.
//
typedef	
HANDLE	
(__stdcall	*FCACHE_RICHCREATE_CALLBACK) (
		IN	LPSTR	lpstrName, 
		IN	LPVOID	lpvData, 
		OUT	DWORD*	cbFileSize, 
		OUT	DWORD*	cbFileSizeHigh,
        OUT BOOL*   pfDidWeScanIt,
        OUT BOOL*   pfIsStuffed,
		OUT	BOOL*	pfStoredWithDots, 
		OUT	BOOL*	pfStoredWithTerminatingDot
		) ;

//
//	Initialize the File Handle Cache - 
//
//	NOTE : this will automatically initialize the DLL for async
//	IO as well !
//
FILEHC_EXPORT
BOOL	__stdcall
InitializeCache() ;

//
//	Terminate the cache !
//	
//	NOTE : this will terminate the DLL for async IO as well !
//
FILEHC_EXPORT
BOOL	__stdcall
TerminateCache() ;

//
//	Associate a file with an async context !
//
FILEHC_EXPORT
PFIO_CONTEXT	__stdcall	
AssociateFile(	_In_ HANDLE	hFile	) ;

//
//	This allows the user to specify whether file stores content with extra DOTS
//	added for RFC 822 protocols (i.e. NNTP and SMTP DATA commands).
//
//	NOTE: AssociateFile() is the same as AssociateFileEx( hFile, FALSE ) ;
//
//	hFile - The file that contains message content, or in which we will write message content
//	fStoreWithDots - if TRUE then each period or DOT in the file which starts a line
//		but is NOT part of the terminating CRLF.CRLF will be stored with an extra dot
//		adjacent to it.  This is the on the wire format for NNTP for instance.
//
FILEHC_EXPORT
PFIO_CONTEXT	__stdcall
AssociateFileEx(	_In_ HANDLE	hFile,
					BOOL	fStoreWithDots, 
					BOOL	fStoredWithTerminatingDot 
					) ;

//
//	Add a reference to a context - 
//	
//	Each call to AddRefContext() must be matched by a corresponding
//	call to ReleaseContext().   Both AssociateFile and CacheCreateFile()
//	also add a single reference which must be matched by a call to ReleaseContext().
//
FILEHC_EXPORT
void	__stdcall	
AddRefContext(	PFIO_CONTEXT ) ;

//
//	Release a Context !
//
//	FIO_CONTEXT's are reference counted - the user must call
//	this for each successfull call to CacheCreateFile(), and 
//	each call to InsertFile() where fKeepReference is TRUE
//
FILEHC_EXPORT
void	__stdcall
ReleaseContext(	PFIO_CONTEXT ) ;

//
//	Close a handle associated with a non-cached FIO_CONTEXT
//
//	This is used to Close the file handle within a context.
//	This only succeeds if the FIO_CONTEXT is not cached !
//
FILEHC_EXPORT
BOOL	__stdcall
CloseNonCachedFile(	PFIO_CONTEXT	) ;

//
//	Create a file in the cache, or find an existing one !
//
//	If the file is not in the cache, the cache will call 
//	pfnCallBack with lpv to do the actual work of calling
//	CreateFile().
//
FILEHC_EXPORT
struct FIO_CONTEXT*	__stdcall
CacheCreateFile(	_In_ IN	LPSTR	lpstrName, 
					IN	FCACHE_CREATE_CALLBACK	pfnCallBack, 
					_In_ IN	LPVOID	lpv, 
					IN	BOOL	fAsyncContext
					) ;
					
//
//	Create a file in the cache or find an existing one, 
//	if we create the file we can add properties onto it in 
//	the cache !
//
FILEHC_EXPORT
struct FIO_CONTEXT*	__stdcall
CacheRichCreateFile(	_In_ IN	LPSTR	lpstrName, 
						IN	FCACHE_RICHCREATE_CALLBACK	pfnCallBack, 
						IN	LPVOID	lpv, 
						IN	BOOL	fAsyncContext
						) ;

//
//	This function allows a user to remove all files with the specified 
//	Name from the cache.  if fAllPrefixes is TRUE, we will remove all files
//	where the Name matches the beginning of the path !
//	If fAllPrefixes is FALSE then we will remove only the one file which 
//	exactly matches lpstrName !
//
FILEHC_EXPORT
void	__stdcall
CacheRemoveFiles(	_In_ IN	LPSTR	lpstrName,
					IN	BOOL	fAllPrefixes
					) ;
//
//	Insert the file into the cache !
//
//	This function will add the file handle in the FIO_CONTEXT
//	to the cache.  All searches by lpstrName will find this
//	item untill it expires from the cache.
//
//	If fKeepReference is TRUE then the user must make a call to 
//	ReleaseContext() for the inserted FIO_CONTEXT !
//
FILEHC_EXPORT
BOOL	__stdcall	
InsertFile(		_In_ IN	LPSTR	lpstrName, 
				IN	struct FIO_CONTEXT*	pContext,
				IN	BOOL	fKeepReference 
				) ;

//
//	Report the file size that we've cached with the handle
//
FILEHC_EXPORT
DWORD	__stdcall
GetFileSizeFromContext(	IN	struct FIO_CONTEXT*	pContext, 
						_Out_ OUT	DWORD*			pcbFileSizeHigh
						) ;

//----------------------------------------------------------------------
// NAME CACHE NAME CACHE NAME CACHE - 
//
//	Name Cache API's
//
//

//
//	This is the function pointer provided by clients to compare 
//	keys.  This must be provided on all calls.
//
//	The function has memcmp() semantics, i.e. it must order the keys
//	consistently, and return <0 if key1 is smaller then key2, ==0 if the
//	keys match and >0 if key1 is greater then key2.
//
typedef	
int
(__stdcall	*CACHE_KEY_COMPARE)(	IN	DWORD	cbKey1, 
									IN	LPBYTE	lpbKey1,
									IN	DWORD	cbKey2, 
									IN	LPBYTE	lpbKey2
									) ;

//
//	This is the function provided by clients to compute a hash 
//	value on Keys - NOTE: The Cache will provide a hash function 
//	IF the user does not, however the internally provided hash
//	function is best only for things that appear to be regular strings.
//
typedef
DWORD
(__stdcall	*CACHE_KEY_HASH)(	IN	LPBYTE	lpbKey, 
								IN	DWORD	cbKey
								) ;

//
//	This is the generic callback function that is provided to the 
//	cache to help examine items within the cache.
//	The BOOL return value is meaningfull to the Cache API's only
//	on the following calls : 
//
//
typedef	
BOOL
(__stdcall	*CACHE_READ_CALLBACK)(	IN	DWORD	cb, 
									IN	LPBYTE	lpb, 
									IN	LPVOID	lpvContext
									) ;

//	
//	This is a callback that is called whenever we destroy an entry in 
//	the name cache - this is called once for both key and data components, 
//	and gives the client a chance to track any relationships 
//
//	NOTE : if the client does not associate 
//	data with the name, the function will only be called for the Key data.
//
typedef
void
(__stdcall	*CACHE_DESTROY_CALLBACK)(	IN	DWORD	cb, 
										IN	LPBYTE	lpb
										) ;

//
//	This is a callback this is called whenever we evaluate a security descriptor.
//	If it is not provided we will call the standard NT AccessCheck() call !
//
//	The function has the same signature as AccessCheck, however there are arguments
//	we don't use - PrivilegeSet will always be NULL and PrivilegeSetLength will always be 0 !
//
typedef
BOOL
(WINAPI	*CACHE_ACCESS_CHECK)(	IN	PSECURITY_DESCRIPTOR	pSecurityDescriptor,
								IN	HANDLE					hClientToken,
								IN	DWORD					dwDesiredAccess, 
								IN	PGENERIC_MAPPING		GenericMapping, 
								IN	PRIVILEGE_SET*			PrivilegeSet, 
								IN	LPDWORD					PrivilegeSetLength,
								IN	LPDWORD					GrantedAccess, 
								IN	LPBOOL					AccessStatus
								) ;


//
//	This is the externally exposed structure representing a Name Cache - 
//	it doesn't contain any fields usefull for a client, but must be passed
//	back into all of the name cache API's
//
struct	NAME_CACHE_CONTEXT	{
	//
	//	Signature DWORD ! - user must not touch this !
	//
	DWORD		m_dwSignature ;
} ;

typedef	struct	NAME_CACHE_CONTEXT*	PNAME_CACHE_CONTEXT ;

//
//	API's for creating/manging NAME CACHE's
//	NOTE : Name Cache's are reference counted, and if this
//	function is called twice with the same name we will 
//	Add a reference to an existing Name Cache.
//
FILEHC_EXPORT
PNAME_CACHE_CONTEXT	__stdcall
FindOrCreateNameCache(
		//
		//	Must not be NULL ! - this is CASE SENSITVE !
		//
		_In_ LPSTR	lpstrName, 
		//
		//	Must not be NULL !
		//
		_In_ CACHE_KEY_COMPARE		pfnKeyCompare, 
		//
		//	This may be NULL, in which case the cache will provide one !
		//
		CACHE_KEY_HASH			pfnKeyHash, 
		//
		//	The following two function pointers may be NULL !
		//
		CACHE_DESTROY_CALLBACK	pfnKeyDestroy, 
		CACHE_DESTROY_CALLBACK	pfnDataDestroy
		) ;

//
//	API's for releasing the NAME CACHE !
//
//	The caller must guarantee the thread safety of this call - This function must not 
//	be called if any other thread is simultanesouly executing within 
//	CacheFindContectFromName(), AssociateContextWithName(), AssociateDataWithName(), or InvalidateName() 
//
FILEHC_EXPORT
long	__stdcall
ReleaseNameCache(
		//
		//	Must not be NULL !
		//
		_Inout_ PNAME_CACHE_CONTEXT		pNameCache
		) ;


//
//	API's for setting options on the name cache - this can be used to change
//	how Security is evaluated !
//
FILEHC_EXPORT
BOOL	__stdcall
SetNameCacheSecurityFunction(
		//
		//	Must not be NULL !
		//
		_Out_opt_ PNAME_CACHE_CONTEXT		pNameCache, 
		//
		//	This is the function pointer that will be used to evaluate security - 
		//	this may be NULL - if it is we will use the Win32 Access Check !
		//
		CACHE_ACCESS_CHECK		pfnAccessCheck
		) ;

//
//	Find the FIO_CONTEXT that is associated with some user name.
//
//	The function returns TRUE if the Name was found in the cache.
//	FALSE if the name was not found in the cache.
//	
//	If the function returns FALSE then the pfnCallback function will not be 
//	called.
//
//	If the function returns TRUE, ppFIOContext may return a NULL pointer, 
//	if the user passed a NULL FIO_CONTEXT to AssociateContextWithName() !
//
//
FILEHC_EXPORT
BOOL	__stdcall
FindContextFromName(
					//
					//	The name cache the client wishes to use !
					//
					PNAME_CACHE_CONTEXT	pNameCache, 
					//
					//	User provides arbitrary bytes for Key to the cache item - pfnKeyCompare() used 
					//	to compare keys !
					//
					_In_reads_bytes_(cbName) IN	LPBYTE	lpbName, 
					_In_ IN	DWORD	cbName, 
					//
					//	User provides function which is called with the key once the key comparison
					//	matches the key.  This lets the user do some extra checking that they're getting 
					//	what they want.
					//
					IN	CACHE_READ_CALLBACK	pfnCallback,
					IN	LPVOID	lpvClientContext,
					//
					//	Ask the cache to evaluate the embedded security descriptor
					//	if hToken is 0 then we ignore and security descriptor data 
					//
					IN	HANDLE		hToken,
					IN	ACCESS_MASK	accessMask,
					//
					//	We have a separate mechanism for returning the FIO_CONTEXT
					//	from the cache.
					//
					OUT	struct FIO_CONTEXT**	ppContext
					) ;


//
//	Find the FIO_CONTEXT that is associated with some user name.
//
//	The function returns TRUE if the Name was found in the cache.
//	FALSE if the name was not found in the cache.
//	
//	If the function returns FALSE then the pfnCallback function will not be 
//	called.
//
//	If the function returns TRUE, ppFIOContext may return a NULL pointer, 
//	if the user passed a NULL FIO_CONTEXT to AssociateContextWithName() !
//
//
FILEHC_EXPORT
BOOL	__stdcall
FindSyncContextFromName(
					//
					//	The name cache the client wishes to use !
					//
					PNAME_CACHE_CONTEXT	pNameCache, 
					//
					//	User provides arbitrary bytes for Key to the cache item - pfnKeyCompare() used 
					//	to compare keys !
					//
					_In_reads_bytes_(cbName) IN	LPBYTE	lpbName, 
					IN	DWORD	cbName, 
					//
					//	User provides function which is called with the key once the key comparison
					//	matches the key.  This lets the user do some extra checking that they're getting 
					//	what they want.
					//
					IN	CACHE_READ_CALLBACK	pfnCallback,
					IN	LPVOID	lpvClientContext,
					//
					//	Ask the cache to evaluate the embedded security descriptor
					//	if hToken is 0 then we ignore and security descriptor data 
					//
					IN	HANDLE		hToken,
					IN	ACCESS_MASK	accessMask,
					//
					//	We have a separate mechanism for returning the FIO_CONTEXT
					//	from the cache.
					//
					OUT	struct FIO_CONTEXT**	ppContext
					) ;


//
//	Cache Associate context with name !
//	This insert a Name into the Name cache, that will find the specified FIO_CONTEXT !
//
//	If the name is already present in the cache, this will fail with GetLastError()==ERROR_DUP_NAME !
//
FILEHC_EXPORT
BOOL	__stdcall
AssociateContextWithName(	
					//
					//	The name cache the client wishes to use !
					//
					_In_ PNAME_CACHE_CONTEXT	pNameCache, 
					//
					//	User provides arbitrary bytes for the Name of the cache item.
					//
					_In_reads_bytes_(cbName) IN	LPBYTE	lpbName, 
					IN	DWORD	cbName, 
					//
					//	User may provide some arbitrary data to assoicate with the name !
					//	
					_In_ IN	LPBYTE	lpbData, 
					IN	DWORD	cbData, 
					//
					//	User may provide a self relative security descriptor to 
					//	be associated with the name !
					//
					IN	PGENERIC_MAPPING		pGenericMapping,
					_In_opt_ IN	PSECURITY_DESCRIPTOR	pSecurityDescriptor,
					//
					//	User provides the FIO_CONTEXT that the name should reference
					//
					_In_opt_ struct FIO_CONTEXT*		pContext,
					//
					//	User specifies whether they wish to keep their reference on the FIO_CONTEXT
					//
					BOOL				fKeepReference
					) ;

//
//	This function allows the user to remove a single name and all associated data
//	from the name cache.
//
FILEHC_EXPORT
BOOL
__stdcall
InvalidateName(	
					//
					//	The name cache the client wishes to use !
					//
					PNAME_CACHE_CONTEXT	pNameCache, 
					//
					//	User provides arbitrary bytes for the Name of the cache item.
					//
					_In_reads_bytes_(cbName) IN	LPBYTE	lpbName, 
					IN	DWORD	cbName
					) ;
	

//
//	End of Name Cache API's
//----------------------------------------------------------------------------------

//----------------------------------------------------------------------------------
//	DOT STUFFING API's
//

//
//  This function gets an FIO_CONTEXT with the requested state.
//  We may or may not create a new FIO_CONTEXT, if we do create one we'll stick 
//  it into the cache so it can be used again !
//  NOTE: if we have to do work, the user has the only reference to the resulting
//  FIO_CONTEXT which will go away when they call ReleaseContext() !
//
//	pContext - the original FIO_CONTEXT
//	lpstrName - the file name associated with pContext
//	fWantItDotStuffed - if TRUE the resulting FIO_CONTEXT should be dot stuffed !
//	fTerminatorIncluded - if this is TRUE the source FIO_CONTEXT contains a terminating
//	dot that we should be carefull not to stuff !
//
//	NOTE: We may return the same FIO_CONTEXT as the caller provided - in which case
//	an extra reference has been added that needs to be dropped with ReleaseContext() !
//
//
FILEHC_EXPORT
struct FIO_CONTEXT*	__stdcall
ProduceDotStuffedContext(	IN	struct FIO_CONTEXT*	pContext,
                                                 _In_ IN  LPSTR           lpstrName,
							IN  BOOL			fWantItDotStuffed // if TRUE add dots, if FALSE remove dots
							) ;

//
//	This function takes a source FIO_CONTEXT (pContextSource) and copies
//	the content into pContextDestination.
//
//	The user specifies whether the Destination FIO_CONTEXT should be dot stuffed
//	with fWantItDotStuffed, and whether the source FIO_CONTEXT includes the 
//	terminating CRLF.CRLF
//
//	The out parameter pfModified is TRUE if there were modifications when
//	Source was copied to Destination !
//
//	The function returns TRUE if successfull, FALSE otherwise !
//
FILEHC_EXPORT
BOOL	__stdcall
ProduceDotStuffedContextInContext(
							IN	struct FIO_CONTEXT*	pContextSource,
							IN	struct FIO_CONTEXT*	pContextDestination,
							IN	BOOL			fWantItDotStuffed, 
							_Out_opt_ OUT	BOOL*			pfModified
							) ;
							



//
//	Find out whether the file has a terminating 'CRLF.CRLF' sequence !
//
FILEHC_EXPORT
BOOL	__stdcall
GetIsFileDotTerminated(	IN	struct FIO_CONTEXT*	pContext ) ;

//
//	Set whether the file has a terminating 'CRLF.CRLF' sequence !
//
FILEHC_EXPORT
void	__stdcall
SetIsFileDotTerminated(	IN	struct FIO_CONTEXT*	pContext,
						IN	BOOL			fIsDotTerminated 
						) ;

//
//	Enable dot stuffing properties on the write path of the file
//	handle cache of this message !
//
//	if fEnable is FALSE then all dot stuffing behaviour is turned
//	off.
//
//	if fStripDots is TRUE the File Handle Cache will convert 
//	occurrences of "\r\n." to "\r\n" within your message.
//
//	if fStripDots is FALSE the FileHandle Cache will convert occurrences
//	of "\r\n.." to "\r\n" within your message.
//	
//
FILEHC_EXPORT
BOOL	__stdcall
SetDotStuffingOnWrites(	IN	struct FIO_CONTEXT*	pContext, 
						//
						//	fEnable == FALSE means ignore fStripDots, and writes are unmodified
						//
						IN	BOOL			fEnable,
						//
						//	fStripDots == TRUE means we remove dots that are dot stuffed, 
						//	fStripDots == FALSE means that we add dots to make the message dot stuffed
						//
						IN	BOOL			fStripDots
						) ;

#if 0 
//
//	This function temporarily disabled !
//
FILEHC_EXPORT
BOOL	__stdcall
SetDotStuffingOnReads(	IN	struct FIO_CONTEXT*	pContext,
						IN	BOOL			fEnable,
						IN	BOOL			fStripDots
						) ;
#endif

//
//	Enable dot scanning properties on the write path 
//	of the file handle cache for this file !
//
//	if fEnable is TRUE the we will examine each write
//	that goes through us to determine whether the
//	message has any occurrences of "\r\n.".
//
FILEHC_EXPORT
BOOL	__stdcall
SetDotScanningOnWrites(	IN	struct FIO_CONTEXT*	pContext, 
						IN	BOOL			fEnable
						) ;

//
//	
//	This function should be called when we have finished doing all writes to an FIO_CONTEXT
//	This function should be paired with SetDotStuffingOnWrites() and the fStripDots
//	parameter should be the same as when SetDotStuffingOnWrites() was called.
//
//	We will update the Dot Stuffing State of the FIO_CONTEXT and discard 
//	all dot stuffing memory and stuff that may have been required !
//
//	If this function call is paired with a call to SetDotScanningOnWrites() fStripDots should be TRUE !
//
FILEHC_EXPORT
void	__stdcall
CompleteDotStuffingOnWrites(	IN	struct FIO_CONTEXT*	pContext, 
								IN	BOOL			fStripDots
								) ;

//
//	This will cause us to examine each read for occurrences of 
//	"\r\n."
//
//	NOTE : the user must use ASYNC Reads for this to work - we will assert
//	if the user tries to pend any synchronous reads while we are in this state !
//
FILEHC_EXPORT
BOOL	__stdcall
SetDotScanningOnReads(	IN	struct FIO_CONTEXT*	pContext, 
						IN	BOOL			fEnable
						) ;
							

//
//	If any of the dot stuffing mechanism our turned on, 
//	this will get a count of the number of occurrences/modifications
//	have occurred.
//
//	if fReads is TRUE we get the count for occurrences on Read's
//	if fReads is FALSE we get the count for occurrences on Write's
//
//	if dot stuffing was turned off or not enabled somehow then
//	GetDotStuffState() will return FALSE.
//
//	NOTE: A NULL pfStuffed is not allowed !
//
FILEHC_EXPORT
BOOL	__stdcall
GetDotStuffState(		IN	struct FIO_CONTEXT*	pContext, 
						IN	BOOL			fReads,
						_Inout_ OUT	BOOL*			pfStuffed,
						_Out_ OUT	BOOL*			pfStoredWithDots
						) ;

//
//	In this case we always assume that the FIO_CONTEXT is not going to be dot stuffed.
//	fRequiresStuffing == TRUE indicates that it SHOULD BE stuffed.
//	fRequiresStuffing == FALSE indicates that the message does not need dot stuffing.
//
FILEHC_EXPORT
void	__stdcall
SetDotStuffState(		IN	struct FIO_CONTEXT*	pContext, 
						//
						//	fIsStuffed is only relevant when fKnown == TRUE
						//
						IN	BOOL			fKnown,		// We do know the dot stuff state
						//
						//	if fKnown is TRUE then fIsStuffed is meaningfull, when thats the case
						//	if fIsStuffed is TRUE then the message 
						//
						IN	BOOL			fRequiresStuffing// if fKnown is TRUE this arg is meaningfull
						) ;


#ifdef	__cplusplus	
}
#endif




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif	// _FILEHC_H_
