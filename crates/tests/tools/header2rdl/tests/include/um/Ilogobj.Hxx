/*++

   Copyright    (c)    1995-1999    Microsoft Corporation

   Module  Name:

      ilogobj.hxx

   Abstract:

      Logging interfaces

   Author:

       Terence Kwan    ( terryk )    18-June-1996


--*/

# ifndef _ILOGOBJ_HXX_
# define _ILOGOBJ_HXX_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// strings
//

#define NCSALOG_CLSID          TEXT("{FF16065F-DE82-11CF-BC0A-00AA006111E0}")
#define ODBCLOG_CLSID          TEXT("{FF16065B-DE82-11CF-BC0A-00AA006111E0}")
#define ASCLOG_CLSID           TEXT("{FF160657-DE82-11CF-BC0A-00AA006111E0}")
#define EXTLOG_CLSID           TEXT("{FF160663-DE82-11CF-BC0A-00AA006111E0}")

#define NCSALOGUI_CLSID        TEXT("{31DCAB85-BB3E-11d0-9299-00C04FB6678B}")
#define ODBCLOGUI_CLSID        TEXT("{31DCAB86-BB3E-11d0-9299-00C04FB6678B}")
#define ASCLOGUI_CLSID         TEXT("{31DCAB87-BB3E-11d0-9299-00C04FB6678B}")
#define EXTLOGUI_CLSID         TEXT("{31DCAB88-BB3E-11d0-9299-00C04FB6678B}")

//
// GUIDS
//

// Interface IDs

DEFINE_GUID(IID_IINETLOG_INFORMATION,   /* cc557a71-f61a-11cf-bc0f-00aa006111e0 */
    0xcc557a71,0xf61a,0x11cf,0xbc, 0x0f, 0x00, 0xaa, 0x00, 0x61, 0x11, 0xe0);

DEFINE_GUID(IID_ILogPlugin,             /* 08fd99d1-cfb6-11cf-bc03-00aa006111e0 */
    0x08fd99d1,0xcfb6,0x11cf,0xbc, 0x03, 0x00, 0xaa, 0x00, 0x61, 0x11, 0xe0);

DEFINE_GUID(IID_ILogPluginEx,           /* 3710E192-9C25-11d1-8B9A-080009DCC2FA */
    0x3710e192, 0x9c25, 0x11d1, 0x8b, 0x9a, 0x8, 0x0, 0x9, 0xdc, 0xc2, 0xfa);

// Class IDs

DEFINE_GUID(CLSID_NCSALOG,              /* ff16065F-DE82-11cf-bc0a-00aa006111e0 */
    0xff16065F,0xde82,0x11cf,0xbc, 0x0a, 0x00, 0xaa, 0x00, 0x61, 0x11, 0xe0);

DEFINE_GUID(CLSID_ODBCLOG,              /* ff16065B-DE82-11cf-bc0a-00aa006111e0 */
    0xff16065B,0xde82,0x11cf,0xbc, 0x0a, 0x00, 0xaa, 0x00, 0x61, 0x11, 0xe0);

DEFINE_GUID(CLSID_ASCLOG,               /* ff160657-DE82-11cf-bc0a-00aa006111e0 */
    0xff160657,0xde82,0x11cf,0xbc, 0x0a, 0x00, 0xaa, 0x00, 0x61, 0x11, 0xe0);

DEFINE_GUID(CLSID_EXTLOG,               /* ff160663-DE82-11cf-bc0a-00aa006111e0 */
    0xff160663,0xde82,0x11cf,0xbc, 0x0a, 0x00, 0xaa, 0x00, 0x61, 0x11, 0xe0);


// Logging UI IDs

DEFINE_GUID(IID_LOGGINGUI,              /* 31DCAB89-BB3E-11d0-9299-00C04FB6678B */
    0x31dcab89, 0xbb3e, 0x11d0, 0x92, 0x99, 0x0, 0xc0, 0x4f, 0xb6, 0x67, 0x8b);

// {FAE6E2A8-BF79-4ac6-AA58-71347C92D593}
DEFINE_GUID(IID_LOGGINGUI2, 
    0xfae6e2a8, 0xbf79, 0x4ac6, 0xaa, 0x58, 0x71, 0x34, 0x7c, 0x92, 0xd5, 0x93);

DEFINE_GUID(CLSID_NCSALOGUI,            /* 31DCAB85-BB3E-11d0-9299-00C04FB6678B */
    0x31dcab85, 0xbb3e, 0x11d0, 0x92, 0x99, 0x0, 0xc0, 0x4f, 0xb6, 0x67, 0x8b);

DEFINE_GUID(CLSID_ODBCLOGUI,            /* 31DCAB86-BB3E-11d0-9299-00C04FB6678B */
    0x31dcab86, 0xbb3e, 0x11d0, 0x92, 0x99, 0x0, 0xc0, 0x4f, 0xb6, 0x67, 0x8b);

DEFINE_GUID(CLSID_ASCLOGUI,             /* 31DCAB87-BB3E-11d0-9299-00C04FB6678B */
    0x31dcab87, 0xbb3e, 0x11d0, 0x92, 0x99, 0x0, 0xc0, 0x4f, 0xb6, 0x67, 0x8b);

DEFINE_GUID(CLSID_EXTLOGUI,             /* 31DCAB88-BB3E-11d0-9299-00C04FB6678B */
    0x31dcab88, 0xbb3e, 0x11d0, 0x92, 0x99, 0x0, 0xc0, 0x4f, 0xb6, 0x67, 0x8b);

//
// Unused IDs
//
DEFINE_GUID(IID_ICLAPI_CLIENT,          /* 08fd99d1-cfb6-11cf-bc03-00aa006111e0 */
    0x08fd99d1,0xcfb6,0x11cf,0xbc, 0x03, 0x00, 0xaa, 0x00, 0x61, 0x11, 0xe0);

DEFINE_GUID(CLSID_InetLogInformation,   /* a1f89741-f619-11cf-bc0f-00aa006111e0 */
    0xa1f89741, 0xf619, 0x11cf, 0xbc, 0xf, 0x0, 0xaa, 0x0, 0x61, 0x11, 0xe0);

//
// Logging Interface exposed by IIS
//

class IInetLogInformation : public IUnknown {

    public:
        virtual LPSTR STDMETHODCALLTYPE
        GetSiteName(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszSiteName,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetComputerName(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszComputerName,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetClientHostName(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszClientHostName,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetClientUserName(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszClientUserName,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetServerAddress(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszServerIPAddress,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetOperation(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszOperation,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetTarget(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszTarget,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetParameters(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszParameters,
            IN PDWORD   pcbSize
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetExtraHTTPHeaders(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszHTTPHeaders,
            IN PDWORD   pcbSize
            ) = 0;

        virtual DWORD STDMETHODCALLTYPE
        GetTimeForProcessing(
            VOID
            ) = 0;

        virtual DWORD STDMETHODCALLTYPE
        GetBytesSent(
            VOID
            ) = 0;

        virtual DWORD STDMETHODCALLTYPE
        GetBytesRecvd(
            VOID
            ) = 0;

        virtual DWORD STDMETHODCALLTYPE
        GetWin32Status(
            VOID
            ) = 0;

        virtual DWORD STDMETHODCALLTYPE
        GetProtocolStatus(
            VOID
            ) = 0;

        virtual DWORD STDMETHODCALLTYPE
        GetPortNumber(
            VOID
            ) = 0;

        virtual LPSTR STDMETHODCALLTYPE
        GetVersionString(
            _Inout_updates_bytes_opt_(*pcbSize) PCHAR    pszVersionString,
            IN PDWORD   pcbSize
            ) = 0;


};


//
// Log Plugin Interface implemented by logging plugins and called by IIS. 
//

class ILogPlugin  : public IUnknown
{
    public:

    virtual HRESULT STDMETHODCALLTYPE
    InitializeLog(
        IN LPCSTR SiteName,
        IN LPCSTR MetabasePath,
        _In_reads_bytes_(sizeof(IMDCOM)) PCHAR pvIMDCOM ) = 0;

    virtual HRESULT STDMETHODCALLTYPE
    TerminateLog( VOID ) = 0;

    virtual HRESULT STDMETHODCALLTYPE
    LogInformation( IInetLogInformation *pLogObj ) = 0;

    virtual HRESULT STDMETHODCALLTYPE
    SetConfig( IN DWORD cbSize, PBYTE Log ) = 0;

    virtual HRESULT STDMETHODCALLTYPE
    GetConfig( IN DWORD cbSize, PBYTE Log ) = 0;

    virtual HRESULT STDMETHODCALLTYPE
    QueryExtraLoggingFields(PDWORD cbSize, _Out_writes_(*cbSize) PCHAR szParameters) = 0;
};

//
// Log plugin UI Interface used by Admin
//

class ILogUIPlugin  : public IUnknown
{
    public:
    
    virtual HRESULT STDMETHODCALLTYPE
    OnProperties( _In_ OLECHAR* pocMachineName, _In_ OLECHAR* pocMetabasePath ) = 0;
};

class ILogUIPlugin2  : public ILogUIPlugin
{
public:
    virtual HRESULT STDMETHODCALLTYPE
    OnPropertiesEx(
        _In_ OLECHAR * pocMachineName, 
        _In_ OLECHAR * pocMetabasePath,
        _In_ OLECHAR * pocUserName,
        _In_ OLECHAR * pocUserPassword) = 0;
};

//
// Extended Log Plugin Interface implemented by logging plugins and called by
// IIS to support advanced/generic logging.
//

#ifndef _LOGTYPE_H_

typedef struct _CUSTOM_LOG_DATA
{
    LPCSTR  szPropertyPath;
    PVOID   pData;
    
} CUSTOM_LOG_DATA, *PCUSTOM_LOG_DATA;

#endif

class ILogPluginEx : public ILogPlugin
{
    public:
    
    virtual HRESULT STDMETHODCALLTYPE
    LogCustomInformation( 
            IN  DWORD               cCount, 
            IN  PCUSTOM_LOG_DATA    pCustomLogData,
            _In_  LPSTR               szHeaderSuffix
            ) = 0;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

# endif // _ILOGOBJ_HXX_
