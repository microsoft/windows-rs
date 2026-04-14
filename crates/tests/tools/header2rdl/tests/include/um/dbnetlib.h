// DBNETLIB.H  -This file contains Windows NT Net-Library prototypes and defines
#ifndef _DBNETLIB_H_
#define _DBNETLIB_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define NETERR		LONG	
#define TIMEINT	    USHORT
#define RETCODE   int
#define IOINT		USHORT

// Special neterr values
#define NETE_NOTSUPPORTED   109
#define NETE_TIMEOUT        -2
#define NETE_ERROR        	-1	
#define NETE_DUPLICATE      -4		
#define NETE_NOT_CONNECTED  233	
#define ENUM_SUCCESS		0
#define MORE_DATA			1
#define NET_NOT_AVAIL		2
#define NETE_NOMAP              0
#define NETE_NOMEMORY           1
#define NETE_NOACCESS           2
#define NETE_CONNBUSY           3
#define NETE_CONNBROKEN         4
#define NETE_TOOMANYCONN        5
#define NETE_SERVERNOTFOUND     6
#define NETE_NETNOTSTARTED      7
#define NETE_NORESOURCE         8
#define NETE_NETBUSY            9
#define NETE_NONETACCESS        10
#define NETE_GENERAL            11
#define NETE_CONNMODE           12
#define NETE_NAMENOTFOUND       13
#define NETE_INVALIDCONN        14
#define NETE_NETDATAERR         15
#define NETE_TOOMANYFILES       16
#define NETE_CANTCONNECT		17
#define NETE_SSLSEC             18
#define NETE_ENCRYPT_REQ        19
#define NETE_ENCRYPT_NOTSUP     20

typedef enum { NLOPT_SET_ENCRYPT, 
			   NLOPT_SET_PACKET_SIZE } OPT_REQUEST;

typedef struct _OPTSTRUCT
{
	int   iSize;
	BOOL  fEncrypt;
	OPT_REQUEST iRequest;
	DWORD dwPacketSize;
} 
OPTSTRUCT;

// Super Socket support with eventually be the only NETLIB support for
// both SQL Servers and their clients.  For now we will still include the
// original NETLIB definitions.
//

IOINT ConnectionObjectSize( void );

IOINT ConnectionRead( void *,
                                  BYTE *,
                                  IOINT,
                                  IOINT,
                                  TIMEINT,
                                  NETERR UNALIGNED * );

IOINT ConnectionWrite( void *,
                                   BYTE *,
                                   IOINT,
                                   NETERR UNALIGNED * );

IOINT ConnectionTransact( void *,
                                      BYTE *,
                                      BYTE *,
                                      IOINT,
                                      IOINT,
                                      IOINT,
                                      TIMEINT,
                                      NETERR UNALIGNED * );

IOINT ConnectionWriteOOB( void *,
                                      BYTE *,
                                      IOINT,
                                      NETERR UNALIGNED * );

RETCODE ConnectionOpen( void *,
                                    CHAR *,
                                    NETERR  * );

RETCODE ConnectionOpenW( void *,
                                     WCHAR *,
                                     NETERR * );

RETCODE ConnectionClose( void *,
                                     NETERR UNALIGNED * );

RETCODE ConnectionCheckForData( void *,
                                            LONG *,
                                            NETERR UNALIGNED * );

BOOL ConnectionError( void *,
                                  NETERR *,
                                  CHAR **,
                                  NETERR * );

BOOL ConnectionErrorW( void *,
                                   NETERR *,
                                   WCHAR **,
                                   NETERR * );

ULONG ConnectionVer(void);

ULONG ConnectionSqlVer( void * );

RETCODE ConnectionServerEnum( CHAR *,
										  IOINT,
										  IOINT UNALIGNED * );

RETCODE ConnectionServerEnumW( WCHAR *,
										   ULONG,
										   IOINT UNALIGNED * );

BOOL ConnectionOption( void * ,
                                   OPTSTRUCT * );

void ConnectionGetSvrUser( void * , 
                           char * );

HANDLE InitEnumServers( LPWSTR , 
                        LPWSTR );

BOOL GetNextEnumeration( HANDLE , 
                         BYTE * , 
                         int * );

void CloseEnumServers( HANDLE );

BOOL InitSSPIPackage (DWORD *pcbMaxMessage);

BOOL TermSSPIPackage (void);

BOOL GenClientContext (DWORD dwKey, BYTE *pIn, DWORD cbIn, BYTE *pOut, DWORD *pcbOut, BOOL *pfDone, CHAR *szUserName);

BOOL InitSession (DWORD dwKey);

BOOL TermSession (DWORD dwKey);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
