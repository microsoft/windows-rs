/* ----------------------------------------------------------------------------
Microsoft	D.T.C (Distributed Transaction Coordinator)

Copyright (C) 1995-1999 Microsoft Corporation.  All rights reserved.

@doc

@module		DTCHelp.h	|

			contains helper API for loading the DTCHelper DLL

-------------------------------------------------------------------------------
@rev 	0 	| 8th -Sep-1995	|	GaganC		| Created
----------------------------------------------------------------------------- */
#ifndef __DTCHELP_H__
#define __DTCHELP_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



//---------------------------------------------------------
//		ALL INCLUDE FILES GO HERE
//---------------------------------------------------------
#include <windows.h>


//---------------------------------------------------------
// constants / enums / typedefs / defines
//---------------------------------------------------------
#ifndef DEFINED_DTC_STATUS
typedef enum DTC_STATUS_
{
	DTC_STATUS_UNKNOWN = 0,			// Status not known
	DTC_STATUS_STARTING = 1,		// DTC is starting
	DTC_STATUS_STARTED = 2,			// DTC has started
	DTC_STATUS_PAUSING = 3,			// DTC is being paused
	DTC_STATUS_PAUSED = 4,			// DTC has been paused
	DTC_STATUS_CONTINUING = 5,		// DTC is being continued
	DTC_STATUS_STOPPING = 6,		// DTC is stopping
	DTC_STATUS_STOPPED = 7,			// DTC has stopped
	DTC_STATUS_E_CANTCONTROL = 8,	// DTC cannot be controlled in its present state
	DTC_STATUS_FAILED = 9			// DTC has failed.
} DTC_STATUS;
#define DEFINED_DTC_STATUS
#endif

typedef HRESULT  (__cdecl * DTC_GET_TRANSACTION_MANAGER)(
									/* in */ char * pszHost,
									/* in */ char * pszTmName,
									/* in */ REFIID rid,
									/* in */ DWORD	dwReserved1,
									/* in */ WORD	wcbReserved2,
									/* in */ void FAR * pvReserved2,
									/*out */ void** ppvObject )	;

typedef HRESULT  (__cdecl * DTC_GET_TRANSACTION_MANAGER_EX_A)(
									/* in */ char * i_pszHost,
									/* in */ char * i_pszTmName,
									/* in */ REFIID i_riid,
									/* in */ DWORD i_grfOptions,
									/* in */ void * i_pvConfigParams,
									/* out */ void ** o_ppvObject
									);

typedef HRESULT  (__cdecl * DTC_GET_TRANSACTION_MANAGER_EX_W)(
									/* in */ WCHAR * i_pwszHost,
									/* in */ WCHAR * i_pwszTmName,
									/* in */ REFIID i_riid,
									/* in */ DWORD i_grfOptions,
									/* in */ void * i_pvConfigParams,
									/* out */ void ** o_ppvObject
									);

typedef HRESULT	( * DTC_INSTALL_CLIENT ) ( 
									   LPTSTR i_pszRemoteTmHostName, 
									   DWORD i_dwProtocol,
									   DWORD i_dwOverwrite );

#ifndef UNICODE 

#define DTC_GET_TRANSACTION_MANAGER_EX		DTC_GET_TRANSACTION_MANAGER_EX_A
#define LoadDtcHelperEx						LoadDtcHelperExA
#define	GetDTCStatus						GetDTCStatusA
#define	StartDTC							StartDTCA
#define	StopDTC								StopDTCA

#else

#define DTC_GET_TRANSACTION_MANAGER_EX		DTC_GET_TRANSACTION_MANAGER_EX_W
#define LoadDtcHelperEx						LoadDtcHelperExW
#define	GetDTCStatus						GetDTCStatusW
#define	StartDTC							StartDTCW
#define	StopDTC								StopDTCW

#endif 



#define	DTCINSTALL_E_CLIENT_ALREADY_INSTALLED					0x0000180L

#define	DTCINSTALL_E_SERVER_ALREADY_INSTALLED					0x0000181L

//***** Install overwrite options
const	DWORD	DTC_INSTALL_OVERWRITE_CLIENT	=	0x00000001;
													// first bit from right, controls client overwrite
													// 1=Overwrite existing client install
													// 0=dont overwrite existing client install
const	DWORD	DTC_INSTALL_OVERWRITE_SERVER	=	0x00000002;
													//  second bit from right, controls server overwrite
													// 1=Overwrite existing server install
													// 0=dont overwrite existing server install

//---------------------------------------------------------
//			Function Prototypes
//---------------------------------------------------------
#ifdef __cplusplus
extern "C" {
#endif
	DTC_GET_TRANSACTION_MANAGER			__cdecl LoadDtcHelper (void);
	DTC_GET_TRANSACTION_MANAGER_EX_A	__cdecl LoadDtcHelperExA (void);
	DTC_GET_TRANSACTION_MANAGER_EX_W	__cdecl LoadDtcHelperExW (void);
	void								__cdecl FreeDtcHelper (void);

	HMODULE __cdecl GetDtcLocaleResourceHandle(void);

	// Function:	Initialize
	// This function initialized all the function pointers needed to 
	// carry out other operations supported by this static library.
	// It is optional to make this call.
	// Returns - S_OK if all is fine, E_FAIL other wise.
	// Note: this function is not thread safe
	HRESULT __cdecl Initialize (void);

	// Function:	Uninitialize
	// This function unloads the dynamically loaded dlls.
	// Returns - S_OK if all is fine, E_FAIL other wise.
	// Note: this function is not thread safe
	HRESULT __cdecl Uninitialize (void);

	// Function:	GetDTCStatusW
	// Unicode version of GetDTCStatus - used to get the status of the DTC service.
	// The wszHostName parameter specifies the node on which to perform the 
	// operation.  On Windows 9x this parameter must be either NULL or the local
	// machine name.
	// Returns - the appropriate status from the enum DTC_STATUS
	DTC_STATUS __cdecl GetDTCStatusW (_In_ LPWSTR wszHostName);

	// Function:	GetDTCStatusA
	// Ansi version of GetDTCStatus - used to get the status of the DTC service
	// The szHostName parameter specifies the node on which to perform the 
	// operation.  On Windows 9x this parameter must be either NULL or the local
	// machine name.
	// Returns - the appropriate status from the enum DTC_STATUS
	DTC_STATUS __cdecl GetDTCStatusA (_In_ LPSTR szHostName);

	// Function:	StartDTCW
	// Unicode version of StartDTC - used to start the DTC service.
	// If the service is already started then the following is a no-op
	// The wszHostName parameter specifies the node on which to perform the 
	// operation.  On Windows 9x this parameter must be either NULL or the local
	// machine name.
	// Returns -	S_OK if all is ok
	// 			E_FAIL if the operation failed for some reason.
	// 			E_UNEXPECTED if an unexpected error occured.
	HRESULT __cdecl StartDTCW (_In_ LPWSTR wszHostName);

	// Function:	StartDTCA
	// Ansi version of StartDTC - used to start the DTC service.
	// If the service is already started then the following is a no-op
	// The szHostName parameter specifies the node on which to perform the 
	// operation.  On Windows 9x this parameter must be either NULL or the local
	// machine name.
	// Returns -	S_OK if all is ok
	// 			E_FAIL if the operation failed for some reason.
	// 			E_UNEXPECTED if an unexpected error occured.
	HRESULT __cdecl StartDTCA (_In_ LPSTR szHostName);

	// Function:	StopDTCW
	// Unicode version of StopDTC - used to stop the DTC service.
	// If the service is already stopped then the following is a no-op
	// The wszHostName parameter specifies the node on which to perform the 
	// operation.  On Windows 9x this parameter must be either NULL or the local
	// machine name.
	// Returns -	S_OK if all is ok
	// 			E_FAIL if the operation failed for some reason.
	// 			E_UNEXPECTED if an unexpected error occured.
	HRESULT __cdecl StopDTCW (_In_ LPWSTR wszHostName);

	// Function:	StopDTCA
	// Ansi version of StopDTC - used to stop the DTC service.
	// If the service is already stopped then the following is a no-op
	// The szHostName parameter specifies the node on which to perform the 
	// operation.  On Windows 9x this parameter must be either NULL or the local
	// machine name.
	// Returns -	S_OK if all is ok
	// 			E_FAIL if the operation failed for some reason.
	// 			E_UNEXPECTED if an unexpected error occured.
	HRESULT __cdecl StopDTCA (_In_ LPSTR szHostName);


	//  Function:	DtcInstallClient
	//  Installs the client version of DTC.
	//  Parameters:	i_pszRemoteTmHostName	- the name of the host tm,
	//			i_szProt				- the protocol to use in string format
	//									0x00000001	TCP/IP  (1)
	//									0x00000004	NetBEUI	(4)
	//										
	//			i_dwOverwrite			- overwrite previous install or not?
	//			DTC_INSTALL_OVERWRITE_CLIENT	=	0x00000001;
													// 0=Overwrite existing client install
													// 1=dont overwrite existing client install
	//			DTC_INSTALL_OVERWRITE_SERVER	=	0x00000002;
													// 0=Overwrite existing server install
													// 1=dont overwrite existing server install
	//  Returns -	S_OK if all is ok
	//			E_FAIL if the operation failed for some reason
	//  E_UNEXPECTED if an unexpected error occured
	HRESULT __cdecl DtcInstallClient(_In_ LPTSTR i_pszRemoteTmHostName, DWORD i_dwProtocol, DWORD i_dwOverwrite);
#ifdef __cplusplus
}
#endif


//---------------------------------------------------------
//			ALL ERRORS GO HERE
//---------------------------------------------------------



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __DTCHELP_H__
