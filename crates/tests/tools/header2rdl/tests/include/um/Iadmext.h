
/**********************************************************************/
/**                       Microsoft Windows NT                       **/
/**                Copyright(c) Microsoft Corp., 1997-1999           **/
/**********************************************************************/

/*
    iadmext.h

    This module contains the interface for extensions to the IISADMIN service.


    FILE HISTORY:
    7/8/97      michth      created
*/

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __iadmext_h__
#define __iadmext_h__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C"{
#endif



/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "ocidl.h"

/*
The Main Interface. All extensions must support this interface.
*/
// {51DFE970-F6F2-11d0-B9BD-00A0C922E750}
DEFINE_GUID(IID_IADMEXT, 0x51dfe970, 0xf6f2, 0x11d0, 0xb9, 0xbd, 0x0, 0xa0, 0xc9, 0x22, 0xe7, 0x50);

/*
InProcess COM Registration. All extensions must write a subkey name by the
CLSID for the above interface under this key in the Registry.
*/
#define IISADMIN_EXTENSIONS_REG_KEYA          "SOFTWARE\\Microsoft\\InetStp\\Extensions"
#define IISADMIN_EXTENSIONS_REG_KEYW          L"SOFTWARE\\Microsoft\\InetStp\\Extensions"
#define IISADMIN_EXTENSIONS_REG_KEY           TEXT("SOFTWARE\\Microsoft\\InetStp\\Extensions")

/*
DCOM Registration. CLSIDS for the DCOM interface provided by these extensions will
be written to this key and ID by IISADMIN as a multisz property.

This is intended for use by other applications which need to find out what classid's are
registered.
*/

#define IISADMIN_EXTENSIONS_CLSID_MD_KEYA      "LM/IISADMIN/EXTENSIONS/DCOMCLSIDS"
#define IISADMIN_EXTENSIONS_CLSID_MD_KEYW      L"LM/IISADMIN/EXTENSIONS/DCOMCLSIDS"
#define IISADMIN_EXTENSIONS_CLSID_MD_KEY      TEXT("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS")
#define IISADMIN_EXTENSIONS_CLSID_MD_ID       MD_IISADMIN_EXTENSIONS

#ifndef __IADMEXT_INTERFACE_DEFINED__
#define __IADMEXT_INTERFACE_DEFINED__


EXTERN_C const IID IID_IADMEXT;

#if defined(__cplusplus) && !defined(CINTERFACE)

    interface IADMEXT : public IUnknown
    {
    public:
        //
        // All methods below will be called under a thread which has called
        // CoInitializeEx(NULL, COINIT_MULTITHREADED).
        //
        // The IMSAdminBase Object will be available during all of these calls.
        //

        //
        // Initialize will be called by IISADMIN when it initializes.
        //
        virtual HRESULT STDMETHODCALLTYPE Initialize(void) = 0;

        //
        // EnumDcomCLSIDs will be called by IISADMIN when it initializes,
        // and the returned CLSIDs will be written to the metabase at
        // the path IISADMIN_EXTENSIONS_CLSID_MD_KEY.
        //
        virtual HRESULT STDMETHODCALLTYPE EnumDcomCLSIDs(
            /* [size_is][out] */ CLSID *pclsidDcom,
            /* [in] */ DWORD dwEnumIndex) = 0;

        //
        // Terminate will be called by IISADMIN when it terminates.
        //
        virtual HRESULT STDMETHODCALLTYPE Terminate(void) = 0;

    };

#else   /* C style interface */
#endif
#endif  /* __IADMEXT_INTERFACE_DEFINED__ */

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __iadmext_h__
