/*++

Copyright (c) 1996-2004 Microsoft Corporation

Module Name:

    prcomoem.h

Abstract:

    Interface declaration for Windows NT printer driver OEM plugins

--*/

#ifndef _PRCOMOEM_H_
#define _PRCOMOEM_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// This file has to be included after printoem.h. We also need to inlude
// objbase.h or comcat.h from sdk\inc.
//

//
// Each dll/exe must initialize the GUIDs once.If you are not using precompiled
// headers for the file(s) which initializes the GUIDs, define INITGUID before
// including objbase.h.
//

//
// Class ID for OEM rendering component. All OEM rendering plugins need to use this ID.
//
// {6d6abf26-9f38-11d1-882a-00c04fb961ec}
//

DEFINE_GUID(CLSID_OEMRENDER, 0x6d6abf26, 0x9f38, 0x11d1, 0x88, 0x2a, 0x00, 0xc0, 0x4f, 0xb9, 0x61, 0xec);

//
// Class ID for OEM UI component. All OEM UI plugins need to use this ID.
//
// {abce80d7-9f46-11d1-882a-00c04fb961ec}
//

DEFINE_GUID(CLSID_OEMUI, 0xabce80d7, 0x9f46, 0x11d1, 0x88, 0x2a, 0x00, 0xc0, 0x4f, 0xb9, 0x61, 0xec);

#if (NTDDI_VERSION >= NTDDI_VISTA)

//
// Class ID for OEM XPS Driver support.  All OEM UI plugins need to use this ID.
//
// {4E144300-5B43-4288-932A-5E4DD6D82BED}
//

DEFINE_GUID(CLSID_OEMUIMXDC, 0x4e144300, 0x5b43, 0x4288, 0x93, 0x2a, 0x5e, 0x4d, 0xd6, 0xd8, 0x2b, 0xed);

//
// Class ID for OEM Print Ticket support.  All OEM UI plugins need to use this ID.
//
// {91723892-45D2-48e2-9EC9-562379DAF992}
//

DEFINE_GUID(CLSID_OEMPTPROVIDER, 0x91723892, 0x45d2, 0x48e2, 0x9e, 0xc9, 0x56, 0x23, 0x79, 0xda, 0xf9, 0x92);

//
// Interface ID for IPrintCoreHelper Interface
//
// {A89EC53E-3905-49c6-9C1A-C0A88117FDB6}
//

DEFINE_GUID(IID_IPrintCoreHelper, 0xa89ec53e, 0x3905, 0x49c6, 0x9c, 0x1a, 0xc0, 0xa8, 0x81, 0x17, 0xfd, 0xb6);


//
// Interface ID for IPrintCoreHelperUni Interface
//
// {7E8E51D6-E5EE-4426-817B-958B9444EB79}
//

DEFINE_GUID(IID_IPrintCoreHelperUni, 0x7e8e51d6, 0xe5ee, 0x4426, 0x81, 0x7b, 0x95, 0x8b, 0x94, 0x44, 0xeb, 0x79);

//
// Interface ID for IPrintCoreHelperPS Interface
//
// {C2C14F6F-95D3-4d63-96CF-6BD9E6C907C2}
//

DEFINE_GUID(IID_IPrintCoreHelperPS, 0xc2c14f6f, 0x95d3, 0x4d63, 0x96, 0xcf, 0x6b, 0xd9, 0xe6, 0xc9, 0x7, 0xc2);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// Interface ID for IPrintCoreHelperUni2 Interface
//
// {6C8AFDFC-EAD0-4D2D-8071-9BF0175A6C3A}
//
DEFINE_GUID(IID_IPrintCoreHelperUni2, 0x6c8afdfc, 0xead0, 0x4d2d, 0x80, 0x71, 0x9b, 0xf0, 0x17, 0x5a, 0x6c, 0x3a);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

//
// Interface ID for IPrintOemCommon Interface
//
// {7f42285e-91d5-11d1-8820-00c04fb961ec}
//

DEFINE_GUID(IID_IPrintOemCommon, 0x7f42285e, 0x91d5, 0x11d1, 0x88, 0x20, 0x00, 0xc0, 0x4f, 0xb9, 0x61, 0xec);

//
// Interface ID for IPrintOemEngine Interface
//
// {63d17590-91d8-11d1-8820-00c04fb961ec}
//

DEFINE_GUID(IID_IPrintOemEngine, 0x63d17590, 0x91d8, 0x11d1, 0x88, 0x20, 0x00, 0xc0, 0x4f, 0xb9, 0x61, 0xec);

//
// Interface ID for IPrintOemUI Interface
//
// {C6A7A9D0-774C-11d1-947F-00A0C90640B8}
//

DEFINE_GUID(IID_IPrintOemUI, 0xc6a7a9d0, 0x774c, 0x11d1, 0x94, 0x7f, 0x0, 0xa0, 0xc9, 0x6, 0x40, 0xb8);

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// Interface ID for IPrintOemUI2 Interface
//
// {292515F9-B54B-489B-9275-BAB56821395E}
//

DEFINE_GUID(IID_IPrintOemUI2, 0x292515f9, 0xb54b, 0x489b, 0x92, 0x75, 0xba, 0xb5, 0x68, 0x21, 0x39, 0x5e);
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)
//
// Interface ID for IPrintOemUIMXDC
//
// {7349D725-E2C1-4dca-AFB5-C13E91BC9306}
//

DEFINE_GUID(IID_IPrintOemUIMXDC, 0x7349d725, 0xe2c1, 0x4dca, 0xaf, 0xb5, 0xc1, 0x3e, 0x91, 0xbc, 0x93, 0x6);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//
// Interface ID for IPrintOemDriverUI interface
//
// {92B05D50-78BC-11d1-9480-00A0C90640B8}
//

DEFINE_GUID(IID_IPrintOemDriverUI, 0x92b05d50, 0x78bc, 0x11d1, 0x94, 0x80, 0x0, 0xa0, 0xc9, 0x6, 0x40, 0xb8);

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// Interface ID for IPrintCoreUI2 interface
//
// {085CCFCA-3ADF-4c9e-B491-D851A6EDC997}
//

DEFINE_GUID(IID_IPrintCoreUI2, 0x85ccfca, 0x3adf, 0x4c9e, 0xb4, 0x91, 0xd8, 0x51, 0xa6, 0xed, 0xc9, 0x97);
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
// Interface ID for IPrintOemPS Interface
//
// {688342b5-8e1a-11d1-881f-00c04fb961ec}
//

DEFINE_GUID(IID_IPrintOemPS, 0x688342b5, 0x8e1a, 0x11d1, 0x88, 0x1f, 0x00, 0xc0, 0x4f, 0xb9, 0x61, 0xec);

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
// Interface ID for IPrintOemPS2 Interface
//
// {BECF7F34-51B3-46c9-8A1C-18679BD21F36}
//

DEFINE_GUID(IID_IPrintOemPS2, 0xbecf7f34, 0x51b3, 0x46c9, 0x8a, 0x1c, 0x18, 0x67, 0x9b, 0xd2, 0x1f, 0x36);

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
// Interface ID for IPrintOemDriverPS interface
//
// {d90060c7-8e1a-11d1-881f-00c04fb961ec}
//

DEFINE_GUID(IID_IPrintOemDriverPS, 0xd90060c7, 0x8e1a, 0x11d1, 0x88, 0x1f, 0x00, 0xc0, 0x4f, 0xb9, 0x61, 0xec);

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// Interface ID for IPrintCorePS2 interface
//
// {CDBB0B0B-A917-40d7-9FBF-483B3BE7EF22}

DEFINE_GUID(IID_IPrintCorePS2, 0xcdbb0b0b, 0xa917, 0x40d7, 0x9f, 0xbf, 0x48, 0x3b, 0x3b, 0xe7, 0xef, 0x22);
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
// Interface ID for IPrintOemUni interface
//
// {D67EBBF0-78BF-11d1-9480-00A0C90640B8}
//

DEFINE_GUID(IID_IPrintOemUni, 0xd67ebbf0, 0x78bf, 0x11d1, 0x94, 0x80, 0x0, 0xa0, 0xc9, 0x6, 0x40, 0xb8);

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// Interface ID for IPrintOemUni2 Interface
//
// {B91220AC-15CC-4e7a-A21E-9591F34D6F6C}
//

DEFINE_GUID(IID_IPrintOemUni2, 0xb91220ac, 0x15cc, 0x4e7a, 0xa2, 0x1e, 0x95, 0x91, 0xf3, 0x4d, 0x6f, 0x6c);
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)
//
// Interface ID for IPrintOemUni3 Interface
//
// {4E4A7747-BC76-4AED-ACC8-C76AF22AAD43}
//

DEFINE_GUID(IID_IPrintOemUni3, 0x4e4a7747, 0xbc76, 0x4aed, 0xac, 0xc8, 0xc7, 0x6a, 0xf2, 0x2a, 0xad, 0x43);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
// Interface ID for IPrintOemDriverUni interface
//
// {D67EBBF1-78BF-11d1-9480-00A0C90640B8}
//

DEFINE_GUID(IID_IPrintOemDriverUni, 0xd67ebbf1, 0x78bf, 0x11d1, 0x94, 0x80, 0x0, 0xa0, 0xc9, 0x6, 0x40, 0xb8);


#if (NTDDI_VERSION >= NTDDI_VISTA)
//
// Interface ID for IPrintOemPrintTicketProvider interface
//
// {BCF5E011-F60A-49ff-AB2F-E06F4ABDF281}
//
DEFINE_GUID(IID_IPrintOemPrintTicketProvider, 0xbcf5e011, 0xf60a, 0x49ff, 0xab, 0x2f, 0xe0, 0x6f, 0x4a, 0xbd, 0xf2, 0x81);

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#undef IUnknown

#ifdef __cplusplus
extern "C" {
#endif


#if (NTDDI_VERSION >= NTDDI_VISTA)


//
//****************************************************************************
//  IPrintCoreHelper interface
//****************************************************************************
//
// Note: This interface largely replaces / duplicates the functionality of
// IPrintCoreUI2, and extends the functionality provided by that interface
// to Unidrv-based drivers, as well as rendering plug-ins.
//
typedef struct _PRINT_FEATURE_OPTION
{
    PCSTR pszFeature;
    PCSTR pszOption;
} PRINT_FEATURE_OPTION;


#undef INTERFACE
#define INTERFACE IPrintCoreHelper
DECLARE_INTERFACE_(IPrintCoreHelper, IUnknown)
{
    //
    // IUnknown methods
    //
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintCoreHelper methods
    //

    //
    // GetOption
    //

    STDMETHOD(GetOption) (THIS_
                          _In_reads_bytes_opt_(cbSize)
                                   CONST DEVMODE    *pDevmode, OPTIONAL
                          _In_     DWORD             cbSize,
                          _In_     PCSTR             pszFeatureRequested,
                          _Outptr_ PCSTR            *ppszOption) PURE;

    //
    // SetOptions
    //

    STDMETHOD(SetOptions) (THIS_
                           IN  PDEVMODE                     pDevmode, OPTIONAL
                           IN  DWORD                        cbSize,
                           IN  BOOL                         bResolveConflicts,
                           IN  CONST PRINT_FEATURE_OPTION   pFOPairs[],
                           IN  DWORD                        cPairs,
                           OUT PDWORD                       pcPairsWritten,
                           OUT PDWORD                       pdwResult) PURE;

    //
    // EnumConstrainedOptions
    //

    STDMETHOD(EnumConstrainedOptions) (THIS_
                                       IN  CONST DEVMODE   *pDevmode, OPTIONAL
                                       IN  DWORD            cbSize,
                                       IN  PCSTR            pszFeatureKeyword,
                                       OUT PCSTR           *pConstrainedOptionList[],
                                       OUT DWORD           *pdwNumOptions) PURE;

    //
    // WhyConstrained
    //

    STDMETHOD(WhyConstrained) (THIS_
                               _In_reads_bytes_opt_(cbSize)
                                     CONST DEVMODE               *pDevmode, OPTIONAL
                               _In_  DWORD                        cbSize,
                               _In_  PCSTR                        pszFeatureKeyword,
                               _In_  PCSTR                        pszOptionKeyword,
                               _Outptr_result_buffer_(*pdwNumOptions)
                                     CONST PRINT_FEATURE_OPTION **ppFOConstraints,
                               _Out_ DWORD                       *pdwNumOptions) PURE;

    //
    // EnumFeatures
    //

    STDMETHOD(EnumFeatures) (THIS_
                             _Outptr_result_buffer_(*pdwNumFeatures)
                                   PCSTR     *pFeatureList[],
                             _Out_ DWORD     *pdwNumFeatures) PURE;

    //
    // EnumOptions
    //

    STDMETHOD(EnumOptions) (THIS_
                            _In_  PCSTR      pszFeatureKeyword,
                            _Outptr_result_buffer_(*pdwNumOptions)
                                  PCSTR     *pOptionList[],
                            _Out_ DWORD     *pdwNumOptions) PURE;

    //
    // GetFontSubstitution
    //

    STDMETHOD(GetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    OUT PCWSTR *ppszDevFontName) PURE;


    //
    // SetFontSubstitution
    //

    STDMETHOD(SetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    IN  PCWSTR  pszDevFontName) PURE;

    //
    // CreateInstanceOfMSXMLObject
    //

    STDMETHOD(CreateInstanceOfMSXMLObject) (THIS_
                                            IN  REFCLSID  rclsid,
                                            IN  LPUNKNOWN pUnkOuter,
                                            IN  DWORD     dwClsContext,
                                            IN  REFIID    riid,
                                            OUT LPVOID   *ppv) PURE;

};

//
//****************************************************************************
//  IPrintCoreHelperUni interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintCoreHelperUni
DECLARE_INTERFACE_(IPrintCoreHelperUni, IPrintCoreHelper)
{
    //
    // IUnknown methods
    //
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintCoreHelper methods
    //

    //
    // GetOption
    //

    STDMETHOD(GetOption) (THIS_
                          _In_reads_bytes_opt_(cbSize)
                                   CONST DEVMODE    *pDevmode, OPTIONAL
                          _In_     DWORD             cbSize,
                          _In_     PCSTR             pszFeatureRequested,
                          _Outptr_ PCSTR            *ppszOption) PURE;

    //
    // SetOptions
    //

    STDMETHOD(SetOptions) (THIS_
                           IN  PDEVMODE                     pDevmode, OPTIONAL
                           IN  DWORD                        cbSize,
                           IN  BOOL                         bResolveConflicts,
                           IN  CONST PRINT_FEATURE_OPTION   pFOPairs[],
                           IN  DWORD                        cPairs,
                           OUT PDWORD                       pcPairsWritten,
                           OUT PDWORD                       pdwResult) PURE;

    //
    // EnumConstrainedOptions
    //

    STDMETHOD(EnumConstrainedOptions) (THIS_
                                       IN  CONST DEVMODE   *pDevmode, OPTIONAL
                                       IN  DWORD            cbSize,
                                       IN  PCSTR            pszFeatureKeyword,
                                       OUT PCSTR           *pConstrainedOptionList[],
                                       OUT DWORD           *pdwNumOptions) PURE;

    //
    // WhyConstrained
    //

    STDMETHOD(WhyConstrained) (THIS_
                               _In_reads_bytes_opt_(cbSize)
                                     CONST DEVMODE               *pDevmode, OPTIONAL
                               _In_  DWORD                        cbSize,
                               _In_  PCSTR                        pszFeatureKeyword,
                               _In_  PCSTR                        pszOptionKeyword,
                               _Outptr_result_buffer_(*pdwNumOptions)
                                     CONST PRINT_FEATURE_OPTION **ppFOConstraints,
                               _Out_ DWORD                       *pdwNumOptions) PURE;

    //
    // EnumFeatures
    //

    STDMETHOD(EnumFeatures) (THIS_
                             _Outptr_result_buffer_(*pdwNumFeatures)
                                   PCSTR     *pFeatureList[],
                             _Out_ DWORD     *pdwNumFeatures) PURE;

    //
    // EnumOptions
    //

    STDMETHOD(EnumOptions) (THIS_
                            _In_  PCSTR      pszFeatureKeyword,
                            _Outptr_result_buffer_(*pdwNumOptions)
                                  PCSTR     *pOptionList[],
                            _Out_ DWORD     *pdwNumOptions) PURE;


    //
    // GetFontSubstitution
    //

    STDMETHOD(GetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    OUT PCWSTR *ppszDevFontName) PURE;

    //
    // SetFontSubstitution
    //

    STDMETHOD(SetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    IN  PCWSTR  pszDevFontName) PURE;

    //
    // CreateInstanceOfMSXMLObject
    //

    STDMETHOD(CreateInstanceOfMSXMLObject) (THIS_
                                            IN  REFCLSID  rclsid,
                                            IN  LPUNKNOWN pUnkOuter,
                                            IN  DWORD     dwClsContext,
                                            IN  REFIID    riid,
                                            OUT LPVOID   *ppv) PURE;

    //
    // IPrintCoreHelperUni methods
    //

    //
    // CreateGDLSnapshot
    //

    STDMETHOD(CreateGDLSnapshot) (THIS_
                                  IN  PDEVMODE   pDevmode, OPTIONAL
                                  IN  DWORD      cbSize,
                                  IN  DWORD      dwFlags,
                                  OUT LPSTREAM  *ppSnapshotStream) PURE;

    //
    // CreateDefaultGDLSnapshot
    //

    STDMETHOD(CreateDefaultGDLSnapshot) (THIS_
                                         _In_       DWORD      dwFlags,
                                         _Outptr_   LPSTREAM  *ppSnapshotStream) PURE;
};

#if (NTDDI_VERSION >= NTDDI_WIN8)

//
//****************************************************************************
//  IPrintCoreHelperUni2 interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintCoreHelperUni2
DECLARE_INTERFACE_(IPrintCoreHelperUni2, IPrintCoreHelperUni)
{
    //
    // IUnknown methods
    //
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintCoreHelper methods
    //

    //
    // GetOption
    //

    STDMETHOD(GetOption) (THIS_
                          _In_reads_bytes_opt_(cbSize)
                                   CONST DEVMODE    *pDevmode, OPTIONAL
                          _In_     DWORD             cbSize,
                          _In_     PCSTR             pszFeatureRequested,
                          _Outptr_ PCSTR            *ppszOption) PURE;

    //
    // SetOptions
    //

    STDMETHOD(SetOptions) (THIS_
                           IN  PDEVMODE                     pDevmode, OPTIONAL
                           IN  DWORD                        cbSize,
                           IN  BOOL                         bResolveConflicts,
                           IN  CONST PRINT_FEATURE_OPTION   pFOPairs[],
                           IN  DWORD                        cPairs,
                           OUT PDWORD                       pcPairsWritten,
                           OUT PDWORD                       pdwResult) PURE;

    //
    // EnumConstrainedOptions
    //

    STDMETHOD(EnumConstrainedOptions) (THIS_
                                       IN  CONST DEVMODE   *pDevmode, OPTIONAL
                                       IN  DWORD            cbSize,
                                       IN  PCSTR            pszFeatureKeyword,
                                       OUT PCSTR           *pConstrainedOptionList[],
                                       OUT DWORD           *pdwNumOptions) PURE;

    //
    // WhyConstrained
    //

    STDMETHOD(WhyConstrained) (THIS_
                               _In_reads_bytes_opt_(cbSize)
                                     CONST DEVMODE               *pDevmode, OPTIONAL
                               _In_  DWORD                        cbSize,
                               _In_  PCSTR                        pszFeatureKeyword,
                               _In_  PCSTR                        pszOptionKeyword,
                               _Outptr_result_buffer_(*pdwNumOptions)
                                     CONST PRINT_FEATURE_OPTION **ppFOConstraints,
                               _Out_ DWORD                       *pdwNumOptions) PURE;

    //
    // EnumFeatures
    //

    STDMETHOD(EnumFeatures) (THIS_
                             _Outptr_result_buffer_(*pdwNumFeatures)
                                   PCSTR     *pFeatureList[],
                             _Out_ DWORD     *pdwNumFeatures) PURE;

    //
    // EnumOptions
    //

    STDMETHOD(EnumOptions) (THIS_
                            _In_  PCSTR      pszFeatureKeyword,
                            _Outptr_result_buffer_(*pdwNumOptions)
                                  PCSTR     *pOptionList[],
                            _Out_ DWORD     *pdwNumOptions) PURE;


    //
    // GetFontSubstitution
    //

    STDMETHOD(GetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    OUT PCWSTR *ppszDevFontName) PURE;

    //
    // SetFontSubstitution
    //

    STDMETHOD(SetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    IN  PCWSTR  pszDevFontName) PURE;

    //
    // CreateInstanceOfMSXMLObject
    //

    STDMETHOD(CreateInstanceOfMSXMLObject) (THIS_
                                            IN  REFCLSID  rclsid,
                                            IN  LPUNKNOWN pUnkOuter,
                                            IN  DWORD     dwClsContext,
                                            IN  REFIID    riid,
                                            OUT LPVOID   *ppv) PURE;

    //
    // IPrintCoreHelperUni methods
    //

    //
    // CreateGDLSnapshot
    //

    STDMETHOD(CreateGDLSnapshot) (THIS_
                                  IN  PDEVMODE   pDevmode, OPTIONAL
                                  IN  DWORD      cbSize,
                                  IN  DWORD      dwFlags,
                                  OUT LPSTREAM  *ppSnapshotStream) PURE;

    //
    // CreateDefaultGDLSnapshot
    //

    STDMETHOD(CreateDefaultGDLSnapshot) (THIS_
                                         IN  DWORD      dwFlags,
                                         OUT LPSTREAM  *ppSnapshotStream) PURE;

    //
    // IPrintCoreHelperUni2 methods
    //

    //
    // GetNamedCommand
    //
    STDMETHOD(GetNamedCommand) (THIS_
        _In_reads_bytes_opt_(cbSize)
                PDEVMODE    pDevmode,
                DWORD       cbSize,
        _In_    PCWSTR      pszCommandName,
        _Outptr_result_bytebuffer_(*pcbCommandSize)
                PBYTE      *ppCommandBytes,
        _Out_   DWORD      *pcbCommandSize) PURE;
};

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

//
//****************************************************************************
//  IPrintCoreHelperPS interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintCoreHelperPS
DECLARE_INTERFACE_(IPrintCoreHelperPS, IPrintCoreHelper)
{

    //
    // IUnknown methods
    //
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintCoreHelper methods
    //

    //
    // GetOption
    //

    STDMETHOD(GetOption) (THIS_
                          _In_reads_bytes_opt_(cbSize)
                                   CONST DEVMODE    *pDevmode, OPTIONAL
                          _In_     DWORD             cbSize,
                          _In_     PCSTR             pszFeatureRequested,
                          _Outptr_ PCSTR            *ppszOption) PURE;

    //
    // SetOptions
    //

    STDMETHOD(SetOptions) (THIS_
                           IN  PDEVMODE                     pDevmode, OPTIONAL
                           IN  DWORD                        cbSize,
                           IN  BOOL                         bResolveConflicts,
                           IN  CONST PRINT_FEATURE_OPTION   pFOPairs[],
                           IN  DWORD                        cPairs,
                           OUT PDWORD                       pcPairsWritten,
                           OUT PDWORD                       pdwResult) PURE;

    //
    // EnumConstrainedOptions
    //

    STDMETHOD(EnumConstrainedOptions) (THIS_
                                       IN  CONST DEVMODE   *pDevmode, OPTIONAL
                                       IN  DWORD            cbSize,
                                       IN  PCSTR            pszFeatureKeyword,
                                       OUT PCSTR           *pConstrainedOptionList[],
                                       OUT DWORD           *pdwNumOptions) PURE;

    //
    // WhyConstrained
    //

    STDMETHOD(WhyConstrained) (THIS_
                               _In_reads_bytes_opt_(cbSize)
                                     CONST DEVMODE               *pDevmode, OPTIONAL
                               _In_  DWORD                        cbSize,
                               _In_  PCSTR                        pszFeatureKeyword,
                               _In_  PCSTR                        pszOptionKeyword,
                               _Outptr_result_buffer_(*pdwNumOptions)
                                     CONST PRINT_FEATURE_OPTION **ppFOConstraints,
                               _Out_ DWORD                       *pdwNumOptions) PURE;

    //
    // EnumFeatures
    //

    STDMETHOD(EnumFeatures) (THIS_
                             _Outptr_result_buffer_(*pdwNumFeatures)
                                   PCSTR     *pFeatureList[],
                             _Out_ DWORD     *pdwNumFeatures) PURE;

    //
    // EnumOptions
    //

    STDMETHOD(EnumOptions) (THIS_
                            _In_  PCSTR      pszFeatureKeyword,
                            _Outptr_result_buffer_(*pdwNumOptions)
                                  PCSTR     *pOptionList[],
                            _Out_ DWORD     *pdwNumOptions) PURE;

    //
    // GetFontSubstitution
    //

    STDMETHOD(GetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    OUT PCWSTR *ppszDevFontName) PURE;


    //
    // SetFontSubstitution
    //

    STDMETHOD(SetFontSubstitution) (THIS_
                                    IN  PCWSTR  pszTrueTypeFontName,
                                    IN  PCWSTR  pszDevFontName) PURE;

    //
    // CreateInstanceOfMSXMLObject
    //

    STDMETHOD(CreateInstanceOfMSXMLObject) (THIS_
                                            IN  REFCLSID  rclsid,
                                            IN  LPUNKNOWN pUnkOuter,
                                            IN  DWORD     dwClsContext,
                                            IN  REFIID    riid,
                                            OUT LPVOID   *ppv) PURE;

    //
    // IPrintCoreHelperPS methods
    //

    //
    // GetGlobalAttribute
    //

    STDMETHOD(GetGlobalAttribute) (THIS_
                                   _In_  PCSTR      pszAttribute,
                                   _Out_ PDWORD     pdwDataType,
                                   _Outptr_result_buffer_(*pcbSize)
                                         PBYTE     *ppbData,
                                   _Out_ PDWORD     pcbSize) PURE;

    //
    // GetFeatureAttribute
    //

    STDMETHOD(GetFeatureAttribute) (THIS_
                                    _In_  PCSTR      pszFeatureKeyword,
                                    _In_  PCSTR      pszAttribute,
                                    _Out_ PDWORD     pdwDataType,
                                    _Outptr_result_buffer_(*pcbSize)
                                          PBYTE     *ppbData,
                                    _Out_ PDWORD     pcbSize) PURE;

    //
    // GetOptionAttribute
    //

    STDMETHOD(GetOptionAttribute) (THIS_
                                   _In_      PCSTR      pszFeatureKeyword,
                                   _In_      PCSTR      pszOptionKeyword,
                                   _In_opt_  PCSTR      pszAttribute,
                                   _Out_     PDWORD     pdwDataType,
                                   _Outptr_result_buffer_(*pcbSize)
                                             PBYTE     *ppbData,
                                   _Out_     PDWORD     pcbSize) PURE;
};

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
//****************************************************************************
//  IPrintOemCommon interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemCommon
DECLARE_INTERFACE_(IPrintOemCommon, IUnknown)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;
};


#ifndef KERNEL_MODE

//
// Definitions used by user interface module only.
// Make sure the macro KERNEL_MODE is not defined.
//

//
//****************************************************************************
//  IPrintOemUI interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemUI
DECLARE_INTERFACE_(IPrintOemUI, IPrintOemCommon)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemUI methods
    //

    //
    // Method for publishing Driver interface.
    //

    STDMETHOD(PublishDriverInterface) (THIS_ _In_ IUnknown *pIUnknown) PURE;


    //
    // CommonUIProp
    //

    STDMETHOD(CommonUIProp) (THIS_
            _In_ DWORD  dwMode,
            _In_ POEMCUIPPARAM   pOemCUIPParam
            )PURE;

    //
    // DocumentPropertySheets
    //

    STDMETHOD(DocumentPropertySheets) (THIS_
            _Inout_ PPROPSHEETUI_INFO   pPSUIInfo,
            _In_    LPARAM              lParam
            )PURE;

    //
    // DevicePropertySheets
    //

    STDMETHOD(DevicePropertySheets) (THIS_
            _In_ PPROPSHEETUI_INFO   pPSUIInfo,
            _In_ LPARAM              lParam
            )PURE;


    //
    // DevQueryPrintEx
    //

    STDMETHOD(DevQueryPrintEx) (THIS_
            _In_ POEMUIOBJ               poemuiobj,
            _In_ PDEVQUERYPRINT_INFO     pDQPInfo,
            _In_ PDEVMODE                pPublicDM,
            _In_ PVOID                   pOEMDM
            )PURE;

    //
    // DeviceCapabilities
    //

    STDMETHOD(DeviceCapabilities) (THIS_
            _Inout_ POEMUIOBJ   poemuiobj,
            _In_    HANDLE      hPrinter,
            _In_    PWSTR       pDeviceName,
            _In_    WORD        wCapability,
            _Out_writes_(_Inexpressible_("varies with wCapability"))
                    PVOID       pOutput,
            _In_    PDEVMODE    pPublicDM,
            _In_    PVOID       pOEMDM,
            _In_    DWORD       dwOld,
            _Out_   DWORD       *dwResult
            )PURE;

    //
    // UpgradePrinter
    //

    STDMETHOD(UpgradePrinter) (THIS_
            _In_ DWORD   dwLevel,
            _At_((PDRIVER_UPGRADE_INFO_1)pDriverUpgradeInfo, _In_)
                 PBYTE   pDriverUpgradeInfo
            )PURE;

    //
    // PrinterEvent
    //

    STDMETHOD(PrinterEvent) (THIS_
            _In_ PWSTR   pPrinterName,
            _In_ INT     iDriverEvent,
            _In_ DWORD   dwFlags,
            _In_ LPARAM  lParam
            )PURE;

    //
    // DriverEvent
    //

    STDMETHOD(DriverEvent) (THIS_
            _In_ DWORD   dwDriverEvent,
            _In_ DWORD   dwLevel,
            _In_reads_(_Inexpressible_("varies"))
                 LPBYTE  pDriverInfo,
            _In_ LPARAM  lParam
            )PURE;

    //
    // QueryColorProfile
    //

    STDMETHOD(QueryColorProfile) (THIS_
            _In_    HANDLE      hPrinter,
            _In_    POEMUIOBJ   poemuiobj,
            _In_    PDEVMODE    pPublicDM,
            _In_    PVOID       pOEMDM,
            _In_    ULONG       ulQueryMode,
            _Out_writes_(*pcbProfileData) VOID       *pvProfileData,
            _Inout_ ULONG      *pcbProfileData,
            _Out_   FLONG      *pflProfileData
            )PURE;

    //
    // FontInstallerDlgProc
    //

    STDMETHOD(FontInstallerDlgProc) (THIS_
            _In_ HWND    hWnd,
            _In_ UINT    usMsg,
            _In_ WPARAM  wParam,
            _In_ LPARAM  lParam
            )PURE;

    //
    // UpdateExternalFonts
    //

    STDMETHOD(UpdateExternalFonts) (THIS_
            _In_ HANDLE  hPrinter,
            _In_ HANDLE  hHeap,
            _In_ PWSTR   pwstrCartridges
           )PURE;
};


#if (NTDDI_VERSION >= NTDDI_WINXP)
//
//****************************************************************************
//  IPrintOemUI2 interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemUI2
DECLARE_INTERFACE_(IPrintOemUI2, IPrintOemUI)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemUI methods
    //

    //
    // Method for publishing Driver interface.
    //

    STDMETHOD(PublishDriverInterface) (THIS_ _In_ IUnknown *pIUnknown) PURE;


    //
    // CommonUIProp
    //

    STDMETHOD(CommonUIProp) (THIS_
            _In_ DWORD  dwMode,
            _In_ POEMCUIPPARAM   pOemCUIPParam
            )PURE;

    //
    // DocumentPropertySheets
    //

    STDMETHOD(DocumentPropertySheets) (THIS_
            _Inout_ PPROPSHEETUI_INFO   pPSUIInfo,
            _In_    LPARAM              lParam
            )PURE;

    //
    // DevicePropertySheets
    //

    STDMETHOD(DevicePropertySheets) (THIS_
            _In_ PPROPSHEETUI_INFO   pPSUIInfo,
            _In_ LPARAM              lParam
            )PURE;


    //
    // DevQueryPrintEx
    //

    STDMETHOD(DevQueryPrintEx) (THIS_
            _In_ POEMUIOBJ               poemuiobj,
            _In_ PDEVQUERYPRINT_INFO     pDQPInfo,
            _In_ PDEVMODE                pPublicDM,
            _In_ PVOID                   pOEMDM
            )PURE;

    //
    // DeviceCapabilities
    //

    STDMETHOD(DeviceCapabilities) (THIS_
            _Inout_ POEMUIOBJ   poemuiobj,
            _In_    HANDLE      hPrinter,
            _In_    PWSTR       pDeviceName,
            _In_    WORD        wCapability,
            _Out_writes_(_Inexpressible_("varies with wCapability"))
                    PVOID       pOutput,
            _In_    PDEVMODE    pPublicDM,
            _In_    PVOID       pOEMDM,
            _In_    DWORD       dwOld,
            _Out_   DWORD       *dwResult
            )PURE;

    //
    // UpgradePrinter
    //

    STDMETHOD(UpgradePrinter) (THIS_
            _In_ DWORD   dwLevel,
            _At_((PDRIVER_UPGRADE_INFO_1)pDriverUpgradeInfo, _In_)
                 PBYTE   pDriverUpgradeInfo
            )PURE;

    //
    // PrinterEvent
    //

    STDMETHOD(PrinterEvent) (THIS_
            _In_ PWSTR   pPrinterName,
            _In_ INT     iDriverEvent,
            _In_ DWORD   dwFlags,
            _In_ LPARAM  lParam
            )PURE;

    //
    // DriverEvent
    //

    STDMETHOD(DriverEvent) (THIS_
            _In_ DWORD   dwDriverEvent,
            _In_ DWORD   dwLevel,
            _In_reads_(_Inexpressible_("varies"))
                 LPBYTE  pDriverInfo,
            _In_ LPARAM  lParam
            )PURE;

    //
    // QueryColorProfile
    //

    STDMETHOD(QueryColorProfile) (THIS_
            _In_    HANDLE      hPrinter,
            _In_    POEMUIOBJ   poemuiobj,
            _In_    PDEVMODE    pPublicDM,
            _In_    PVOID       pOEMDM,
            _In_    ULONG       ulQueryMode,
            _Out_writes_(*pcbProfileData) VOID       *pvProfileData,
            _Inout_ ULONG      *pcbProfileData,
            _Out_   FLONG      *pflProfileData
            )PURE;

    //
    // FontInstallerDlgProc
    //

    STDMETHOD(FontInstallerDlgProc) (THIS_
            _In_ HWND    hWnd,
            _In_ UINT    usMsg,
            _In_ WPARAM  wParam,
            _In_ LPARAM  lParam
            )PURE;

    //
    // UpdateExternalFonts
    //

    STDMETHOD(UpdateExternalFonts) (THIS_
            _In_ HANDLE  hPrinter,
            _In_ HANDLE  hHeap,
            _In_ PWSTR   pwstrCartridges
           )PURE;

    //
    // IPrintOemUI2 methods
    //

    //
    // QueryJobAttributes
    //

    STDMETHOD(QueryJobAttributes)  (THIS_
            _In_             HANDLE      hPrinter,
            _In_             PDEVMODE    pDevmode,
            _In_range_(1, 4) DWORD       dwLevel,
            _In_reads_(_Inexpressible_("varies"))
                             LPBYTE      lpAttributeInfo
           )PURE;

    //
    // Hide Standard UI
    //

    STDMETHOD(HideStandardUI)  (THIS_
            DWORD       dwMode
           )PURE;

    //
    // DocumentEvent
    //

    STDMETHOD(DocumentEvent) (THIS_
            HANDLE      hPrinter,
            HDC         hdc,
            INT         iEsc,
            ULONG       cbIn,
            PVOID       pvIn,
            ULONG       cbOut,
            PVOID       pvOut,
            PINT        piResult
           )PURE;
};

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)
//
//****************************************************************************
//  IPrintOemUIMXDC interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemUIMXDC
DECLARE_INTERFACE_(IPrintOemUIMXDC, IUnknown)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintOemUIMXDC methods
    //

    STDMETHOD(AdjustImageableArea) (THIS_
            HANDLE          hPrinter,
            DWORD           cbDevMode,
            CONST PDEVMODE  pDevMode,
            DWORD           cbOEMDM,
            CONST PVOID     pOEMDM,
            PRECTL          prclImageableArea
           )PURE;

    STDMETHOD(AdjustImageCompression) (THIS_
            HANDLE          hPrinter,
            DWORD           cbDevMode,
            CONST PDEVMODE  pDevMode,
            DWORD           cbOEMDM,
            CONST PVOID     pOEMDM,
            PLONG           pCompressionMode
            ) PURE;

    STDMETHOD(AdjustDPI) (THIS_
            HANDLE          hPrinter,
            DWORD           cbDevMode,
            CONST PDEVMODE  pDevMode,
            DWORD           cbOEMDM,
            CONST PVOID     pOEMDM,
            PLONG           pDPI
            ) PURE;

};

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTA)

#define S_DEVCAP_OUTPUT_FULL_REPLACEMENT  ((HRESULT)0x0004DC01L)

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
//****************************************************************************
//  IPrintOemDriverUI interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemDriverUI
DECLARE_INTERFACE_(IPrintOemDriverUI, IUnknown)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintOemDriverUI methods
    //

    //
    // Helper function to get driver settings
    //

    STDMETHOD(DrvGetDriverSetting) (THIS_
                        PVOID   pci,
                        PCSTR   Feature,
                        PVOID   pOutput,
                        DWORD   cbSize,
                        PDWORD  pcbNeeded,
                        PDWORD  pdwOptionsReturned
                        )PURE;

    //
    // Helper function to allow OEM plugins upgrade private registry
    // settings. This function should be called only by OEM's UpgradePrinter()
    //

    STDMETHOD(DrvUpgradeRegistrySetting) (THIS_
                        HANDLE   hPrinter,
                        PCSTR    pFeature,
                        PCSTR    pOption
                        )PURE;

    //
    // Helper function to allow OEM plugins to update the driver UI
    // settings and show constraints. This function should be called only when
    // the UI is present.
    //

    STDMETHOD(DrvUpdateUISetting) (THIS_
                        PVOID    pci,
                        PVOID    pOptItem,
                        DWORD    dwPreviousSelection,
                        DWORD    dwMode
                        )PURE;
};

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
//****************************************************************************
//  IPrintCoreUI2 interface
//
//  This is the revised core driver helper interface OEM UI plugin can call.
//  It supercedes the old helper interface IPrintOemDriverUI.
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintCoreUI2
DECLARE_INTERFACE_(IPrintCoreUI2, IPrintOemDriverUI)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintOemDriverUI methods
    //

    //
    // Helper function to get driver settings. This function is only supported
    // for UI plugins that do not fully replace core driver's standard UI.
    //

    STDMETHOD(DrvGetDriverSetting) (THIS_
                        PVOID   pci,
                        PCSTR   Feature,
                        PVOID   pOutput,
                        DWORD   cbSize,
                        PDWORD  pcbNeeded,
                        PDWORD  pdwOptionsReturned
                        )PURE;

    //
    // Helper function to allow OEM plugins upgrade private registry
    // settings. This function is supported for any UI plugins and should be
    // called only by OEM's UpgradePrinter.
    //

    STDMETHOD(DrvUpgradeRegistrySetting) (THIS_
                        HANDLE   hPrinter,
                        PCSTR    pFeature,
                        PCSTR    pOption
                        )PURE;

    //
    // Helper function to allow OEM plugins to update the driver UI settings.
    // This function is only supported for UI plugins that do not fully replace
    // core driver's standard UI. It should be called only when the UI is present.
    //

    STDMETHOD(DrvUpdateUISetting) (THIS_
                        PVOID    pci,
                        PVOID    pOptItem,
                        DWORD    dwPreviousSelection,
                        DWORD    dwMode
                        )PURE;

    //
    // IPrintCoreUI2 new methods
    //

    //
    // Following four helper functions are only supported for UI plugins that fully
    // replace core driver's standard UI. They should only be called by the UI plugin's
    // DocumentPropertySheets, DevicePropertySheets and their property sheet callback
    // functions.
    //
    // Helper function to retrieve driver's current setting as a list of
    // feature/option keyword pairs.
    //

    STDMETHOD(GetOptions) (THIS_
                           _In_        POEMUIOBJ  poemuiobj,
                           _Reserved_  DWORD      dwFlags,
                           _In_reads_bytes_opt_(cbIn)
                                       PCZZSTR      pmszFeaturesRequested,
                           _In_        DWORD      cbIn,
                           _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                       PZZSTR       pmszFeatureOptionBuf,
                           _In_        DWORD      cbSize,
                           _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                       PDWORD     pcbNeeded) PURE;

    //
    // Helper function to change driver's setting using a list of feature/option
    // keyword pairs.
    //

    STDMETHOD(SetOptions) (THIS_
                           _In_                     POEMUIOBJ  poemuiobj,
                           _In_                     DWORD      dwFlags,
                           _In_reads_bytes_(cbIn)   PCZZSTR    pmszFeatureOptionBuf,
                           _In_                     DWORD      cbIn,
                           _Out_                    PDWORD     pdwResult) PURE;

    //
    // Helper function to retrieve the option(s) of a given feature that are
    // constrained in driver's current setting.
    //

    STDMETHOD(EnumConstrainedOptions) (THIS_
                                       _In_        POEMUIOBJ  poemuiobj,
                                       _Reserved_  DWORD      dwFlags,
                                       _In_        PCSTR      pszFeatureKeyword,
                                       _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                                   PZZSTR       pmszConstrainedOptionList,
                                       _In_        DWORD      cbSize,
                                       _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                                   PDWORD     pcbNeeded) PURE;

    //
    // Helper function to retrieve a list of feature/option keyword pairs from
    // driver's current setting that conflict with the given feature/option pair.
    //

    STDMETHOD(WhyConstrained) (THIS_
                               _In_        POEMUIOBJ  poemuiobj,
                               _Reserved_  DWORD      dwFlags,
                               _In_        PCSTR      pszFeatureKeyword,
                               _In_        PCSTR      pszOptionKeyword,
                               _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                           PZZSTR     pmszReasonList,
                               _In_        DWORD      cbSize,
                               _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                           PDWORD     pcbNeeded) PURE;

    //
    // Following five helper functions are supported for any UI plugins.
    //
    // Helper function to retrieve global attribute.
    //

    STDMETHOD(GetGlobalAttribute) (THIS_
                                   _In_        POEMUIOBJ  poemuiobj,
                                   _Reserved_  DWORD      dwFlags,
                                   _In_opt_    PCSTR      pszAttribute,
                                   _Out_       PDWORD     pdwDataType,
                                   _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                               PBYTE      pbData,
                                   _In_        DWORD      cbSize,
                                   _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                               PDWORD     pcbNeeded) PURE;


    //
    // Helper function to retrieve attribute of a given feature.
    //

    STDMETHOD(GetFeatureAttribute) (THIS_
                                    _In_        POEMUIOBJ  poemuiobj,
                                    _Reserved_  DWORD      dwFlags,
                                    _In_        PCSTR      pszFeatureKeyword,
                                    _In_opt_    PCSTR      pszAttribute,
                                    _Out_       PDWORD     pdwDataType,
                                    _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                                PBYTE      pbData,
                                    _In_        DWORD      cbSize,
                                    _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                                PDWORD     pcbNeeded) PURE;

    //
    // Helper function to retrieve attribute of a given feature/option selection.
    //

    STDMETHOD(GetOptionAttribute) (THIS_
                                   _In_        POEMUIOBJ  poemuiobj,
                                   _Reserved_  DWORD      dwFlags,
                                   _In_        PCSTR      pszFeatureKeyword,
                                   _In_        PCSTR      pszOptionKeyword,
                                   _In_opt_    PCSTR      pszAttribute,
                                   _Out_       PDWORD     pdwDataType,
                                   _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                               PBYTE      pbData,
                                   _In_        DWORD      cbSize,
                                   _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                               PDWORD     pcbNeeded) PURE;

    //
    // Helper function to retrieve the list of feature keyword.
    //

    STDMETHOD(EnumFeatures) (THIS_
                             _In_        POEMUIOBJ  poemuiobj,
                             _Reserved_  DWORD      dwFlags,
                             _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                         PZZSTR       pmszFeatureList,
                             _In_        DWORD      cbSize,
                             _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                         PDWORD     pcbNeeded) PURE;

    //
    // Helper function to retrieve the list of options keyword of a given feature.
    //

    STDMETHOD(EnumOptions) (THIS_
                            _In_        POEMUIOBJ  poemuiobj,
                            _Reserved_  DWORD      dwFlags,
                            _In_        PCSTR      pszFeatureKeyword,
                            _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                        PZZSTR       pmszOptionList,
                            _In_        DWORD      cbSize,
                            _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                        PDWORD     pcbNeeded) PURE;

    //
    // Helper function to query system simulation support
    //

    STDMETHOD(QuerySimulationSupport) (THIS_
                                       _In_  HANDLE  hPrinter,
                                       _In_  DWORD   dwLevel,
                                       _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                             PBYTE   pCaps,
                                       _In_  DWORD   cbSize,
                                       _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                             PDWORD  pcbNeeded) PURE;
};

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)

#if defined(PLUGIN_PRINTTICKET)

//
//****************************************************************************
//  IPrintOemPrintTicketProvider interface
//****************************************************************************
//

typedef enum tagOEMPTOPTS
{
    OEMPT_DEFAULT   = 0,
    OEMPT_NOSNAPSHOT = 0x1
} OEMPTOPTS, *POEMPTOPTS;

#undef INTERFACE
#define INTERFACE IPrintOemPrintTicketProvider
DECLARE_INTERFACE_(IPrintOemPrintTicketProvider, IUnknown)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj)PURE;
    STDMETHOD_(ULONG, AddRef) (THIS)PURE;
    STDMETHOD_(ULONG, Release) (THIS)PURE;

    //
    // IPrintOemPrintTicketProvider methods
    //

    STDMETHOD(GetSupportedVersions)( THIS_
            _In_ HANDLE hPrinter,
            _Outptr_result_buffer_(*cVersions) INT *ppVersions[],
            _Out_ INT *cVersions
            ) PURE;

    STDMETHOD(BindPrinter)( THIS_
            _In_  HANDLE           hPrinter,
                  INT              version,
            _Out_ POEMPTOPTS       pOptions,
            _Out_ INT             *cNamespaces,
            _Outptr_result_buffer_maybenull_(*cNamespaces) BSTR **ppNamespaces
            ) PURE;

    STDMETHOD(PublishPrintTicketHelperInterface)( THIS_
            _In_ IUnknown *pHelper
            ) PURE;

    STDMETHOD(QueryDeviceDefaultNamespace)( THIS_
            OUT BSTR *pbstrNamespaceUri
            ) PURE;

    STDMETHOD(ConvertPrintTicketToDevMode)( THIS_
            _In_    IXMLDOMDocument2 *pPrintTicket,
                    ULONG             cbDevmode,
            _Inout_updates_bytes_(cbDevmode) 
                    PDEVMODE          pDevmode,
                    ULONG             cbDrvPrivateSize,
            _Inout_ PVOID             pPrivateDevmode
            ) PURE;

    STDMETHOD(ConvertDevModeToPrintTicket)( THIS_
                    ULONG             cbDevmode,
            _Inout_updates_bytes_(cbDevmode) 
                    PDEVMODE          pDevmode,
                    ULONG             cbDrvPrivateSize,
            _Inout_ PVOID             pPrivateDevmode,
            _Inout_ IXMLDOMDocument2 *pPrintTicket
            ) PURE;

    STDMETHOD(CompletePrintCapabilities)( THIS_
            _In_opt_ IXMLDOMDocument2 *pPrintTicket,
            _Inout_  IXMLDOMDocument2 *pCapabilities
            ) PURE;

    STDMETHOD(ExpandIntentOptions)( THIS_
            _Inout_ IXMLDOMDocument2 *pPrintTicket
            ) PURE;

    STDMETHOD(ValidatePrintTicket)( THIS_
            _Inout_ IXMLDOMDocument2 *pPrintTicket
            ) PURE;
};

#endif // PLUGIN_PRINTTICKET

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#else
// KERNEL_MODE

//
// Definitions used by rendering module only.
// Make sure the macro KERNEL_MODE is defined.
//

//
//****************************************************************************
//  IPrintOemEngine interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemEngine
DECLARE_INTERFACE_(IPrintOemEngine, IPrintOemCommon)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
            _In_  DWORD   dwMode,
            _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
            _In_  DWORD   cbSize,
            _Out_ PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemEngine methods
    //

    //
    // Method for OEM to specify DDI hook out
    //

    STDMETHOD(EnableDriver)  (THIS_   DWORD           DriverVersion,
                                      DWORD           cbSize,
                                      PDRVENABLEDATA  pded) PURE;

    //
    // Method to notify OEM plugin that it is no longer required
    //

    STDMETHOD(DisableDriver) (THIS) PURE;

    //
    // Method for OEM to contruct its own PDEV
    //

    STDMETHOD(EnablePDEV)    (THIS_   PDEVOBJ         pdevobj,
                                      PWSTR           pPrinterName,
                                      ULONG           cPatterns,
                                      HSURF          *phsurfPatterns,
                                      ULONG           cjGdiInfo,
                                      GDIINFO        *pGdiInfo,
                                      ULONG           cjDevInfo,
                                      DEVINFO        *pDevInfo,
                                      DRVENABLEDATA  *pded,
                                      OUT PDEVOEM    *pDevOem) PURE;

    //
    // Method for OEM to free any resource associated with its PDEV
    //

    STDMETHOD(DisablePDEV)   (THIS_   PDEVOBJ         pdevobj) PURE;

    //
    // Method for OEM to transfer from old PDEV to new PDEV
    //

    STDMETHOD(ResetPDEV)     (THIS_   PDEVOBJ         pdevobjOld,
                                      PDEVOBJ         pdevobjNew) PURE;
};

//
//****************************************************************************
//  IPrintOemPS interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemPS
DECLARE_INTERFACE_(IPrintOemPS, IPrintOemEngine)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemEngine methods
    //

    //
    // Method for OEM to specify DDI hook out
    //

    STDMETHOD(EnableDriver)  (THIS_   DWORD           DriverVersion,
                                      DWORD           cbSize,
                                      PDRVENABLEDATA  pded) PURE;

    //
    // Method to notify OEM plugin that it is no longer required
    //

    STDMETHOD(DisableDriver) (THIS) PURE;

    //
    // Method for OEM to construct its own PDEV
    //

    STDMETHOD(EnablePDEV)    (THIS_   PDEVOBJ         pdevobj,
                                      PWSTR           pPrinterName,
                                      ULONG           cPatterns,
                                      HSURF          *phsurfPatterns,
                                      ULONG           cjGdiInfo,
                                      GDIINFO        *pGdiInfo,
                                      ULONG           cjDevInfo,
                                      DEVINFO        *pDevInfo,
                                      DRVENABLEDATA  *pded,
                                      OUT PDEVOEM    *pDevOem) PURE;

    //
    // Method for OEM to free any resource associated with its PDEV
    //

    STDMETHOD(DisablePDEV)   (THIS_   PDEVOBJ         pdevobj) PURE;

    //
    // Method for OEM to transfer from old PDEV to new PDEV
    //

    STDMETHOD(ResetPDEV)     (THIS_   PDEVOBJ         pdevobjOld,
                                      PDEVOBJ         pdevobjNew) PURE;

    //
    // IPrintOemPS methods
    //

    //
    // Method for publishing Driver interface.
    //

    STDMETHOD(PublishDriverInterface)(THIS_  _In_ IUnknown *pIUnknown) PURE;

    //
    // Method for OEM to generate output at specific injection point
    //

    STDMETHOD(Command) (THIS_   PDEVOBJ     pdevobj,
                                DWORD       dwIndex,
                                PVOID       pData,
                                DWORD       cbSize,
                                OUT DWORD   *pdwResult) PURE;
};

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
//****************************************************************************
//  IPrintOemPS2 interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemPS2
DECLARE_INTERFACE_(IPrintOemPS2, IPrintOemPS)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemEngine methods
    //

    //
    // Method for OEM to specify DDI hook out
    //

    STDMETHOD(EnableDriver)  (THIS_   DWORD           DriverVersion,
                                      DWORD           cbSize,
                                      PDRVENABLEDATA  pded) PURE;

    //
    // Method to notify OEM plugin that it is no longer required
    //

    STDMETHOD(DisableDriver) (THIS) PURE;

    //
    // Method for OEM to construct its own PDEV
    //

    STDMETHOD(EnablePDEV)    (THIS_   PDEVOBJ         pdevobj,
                                      PWSTR           pPrinterName,
                                      ULONG           cPatterns,
                                      HSURF          *phsurfPatterns,
                                      ULONG           cjGdiInfo,
                                      GDIINFO        *pGdiInfo,
                                      ULONG           cjDevInfo,
                                      DEVINFO        *pDevInfo,
                                      DRVENABLEDATA  *pded,
                                      OUT PDEVOEM    *pDevOem) PURE;

    //
    // Method for OEM to free any resource associated with its PDEV
    //

    STDMETHOD(DisablePDEV)   (THIS_   PDEVOBJ         pdevobj) PURE;

    //
    // Method for OEM to transfer from old PDEV to new PDEV
    //

    STDMETHOD(ResetPDEV)     (THIS_   PDEVOBJ         pdevobjOld,
                                      PDEVOBJ         pdevobjNew) PURE;

    //
    // IPrintOemPS methods
    //

    //
    // Method for publishing Driver interface.
    //

    STDMETHOD(PublishDriverInterface)(THIS_  _In_ IUnknown *pIUnknown) PURE;

    //
    // Method for OEM to generate output at specific injection point
    //

    STDMETHOD(Command) (THIS_   PDEVOBJ     pdevobj,
                                DWORD       dwIndex,
                                PVOID       pData,
                                DWORD       cbSize,
                                OUT DWORD   *pdwResult) PURE;

    //
    // IPrintOemPS2 methods
    //

    //
    // Method for plugin to hook out spooler's WritePrinter API so it
    // can get access to output data PostScript driver is generating
    //
    // At DrvEnablePDEV time, PostScript driver will call this function with
    // pdevobj = NULL, pBuf = NULL, cbBuffer = 0 to detect if the plugin
    // implements this function. Plugin should return S_OK to indicate it is
    // implementing this function, or return E_NOTIMPL otherwise.
    //
    // In pcbWritten, plugins should return the number of bytes written to the
    // spooler's WritePrinter function. Zero doesn't carry a special meaning,
    // errors must be reported through the returned HRESULT.
    //

    STDMETHOD(WritePrinter) (THIS_   PDEVOBJ    pdevobj,
                                     PVOID      pBuf,
                                     DWORD      cbBuffer,
                                     PDWORD     pcbWritten) PURE;

    //
    // Method for plugin to implement if it wants to be called to get the chance
    // to override some PDEV settings such as paper margins.
    // Plugins that recognize the adjustment type should return S_OK.
    // If the adjustment type is unrecognized, they should return S_FALSE
    // and not E_NOTIMPL, this code should be reserved for the COM meaning.
    // If the plugin fails the call, it should return E_FAIL.
    // The chain of plugins will be called until a plugin returns S_OK or
    // any failure code other than E_NOTIMPL, in other words, until the first
    // plugin that is designed to handle the adjustment is found.
    //

    STDMETHOD(GetPDEVAdjustment) (THIS_ PDEVOBJ    pdevobj,
                                        DWORD      dwAdjustType,
                                        PVOID      pBuf,
                                        DWORD      cbBuffer,
                                        OUT BOOL  *pbAdjustmentDone) PURE;
};

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
//****************************************************************************
//  IPrintOemDriverPS interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemDriverPS
DECLARE_INTERFACE_(IPrintOemDriverPS, IUnknown)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemDriverPS methods
    //

    //
    // Method for OEM to get driver settings
    //

    STDMETHOD(DrvGetDriverSetting) (THIS_   PVOID   pdriverobj,
                                            PCSTR   Feature,
                                            PVOID   pOutput,
                                            DWORD   cbSize,
                                            PDWORD  pcbNeeded,
                                            PDWORD  pdwOptionsReturned) PURE;

    //
    // Method for OEM to write to spooler buffer
    //

    STDMETHOD(DrvWriteSpoolBuf)(THIS_       PDEVOBJ     pdevobj,
                                            PVOID       pBuffer,
                                            DWORD       cbSize,
                                            OUT DWORD   *pdwResult) PURE;
};

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
//****************************************************************************
//  IPrintCorePS2 interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintCorePS2
DECLARE_INTERFACE_(IPrintCorePS2, IUnknown)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintCorePS2 methods
    //

    //
    // Method for OEM to write to spooler buffer.
    //

    STDMETHOD(DrvWriteSpoolBuf)(THIS_
                           IN  PDEVOBJ  pdevobj,
                           IN  PVOID    pBuffer,
                           IN  DWORD    cbSize,
                           OUT DWORD    *pdwResult) PURE;

    //
    // Helper function to retrieve driver's current setting as a list of
    // feature/option keyword pairs.
    //

    STDMETHOD(GetOptions) (THIS_
                           _In_        PDEVOBJ  pdevobj,
                           _Reserved_  DWORD      dwFlags,
                           _In_reads_bytes_opt_(cbIn)
                                       PCZZSTR      pmszFeaturesRequested,
                           _In_        DWORD      cbIn,
                           _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                       PZZSTR       pmszFeatureOptionBuf,
                           _In_        DWORD      cbSize,
                           _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                       PDWORD     pcbNeeded) PURE;

    //
    // Helper function to retrieve global attribute.
    //

    STDMETHOD(GetGlobalAttribute) (THIS_
                                   _In_        PDEVOBJ  pdevobj,
                                   _Reserved_  DWORD    dwFlags,
                                   _In_opt_    PCSTR    pszAttribute,
                                   _Out_       PDWORD   pdwDataType,
                                   _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                               PBYTE    pbData,
                                   _In_        DWORD    cbSize,
                                   _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                               PDWORD   pcbNeeded) PURE;


    //
    // Helper function to retrieve attribute of a given feature.
    //

    STDMETHOD(GetFeatureAttribute) (THIS_
                                    _In_        PDEVOBJ  pdevobj,
                                    _Reserved_  DWORD    dwFlags,
                                    _In_        PCSTR    pszFeatureKeyword,
                                    _In_opt_    PCSTR    pszAttribute,
                                    _Out_       PDWORD   pdwDataType,
                                    _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                                PBYTE    pbData,
                                    _In_        DWORD    cbSize,
                                    _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                                PDWORD   pcbNeeded) PURE;

    //
    // Helper function to retrieve attribute of a given feature/option selection.
    //

    STDMETHOD(GetOptionAttribute) (THIS_
                                   _In_        PDEVOBJ  pdevobj,
                                   _Reserved_  DWORD    dwFlags,
                                   _In_        PCSTR    pszFeatureKeyword,
                                   _In_        PCSTR    pszOptionKeyword,
                                   _In_opt_    PCSTR    pszAttribute,
                                   _Out_       PDWORD   pdwDataType,
                                   _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                               PBYTE    pbData,
                                   _In_        DWORD    cbSize,
                                   _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                               PDWORD   pcbNeeded) PURE;

    //
    // Helper function to retrieve the list of feature keyword.
    //

    STDMETHOD(EnumFeatures) (THIS_
                             _In_        PDEVOBJ  pdevobj,
                             _Reserved_  DWORD    dwFlags,
                             _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                         PZZSTR     pmszFeatureList,
                             _In_        DWORD    cbSize,
                             _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                         PDWORD   pcbNeeded) PURE;

    //
    // Helper function to retrieve the list of options keyword of a given feature.
    //

    STDMETHOD(EnumOptions) (THIS_
                            _In_        PDEVOBJ  pdevobj,
                            _Reserved_  DWORD    dwFlags,
                            _In_        PCSTR    pszFeatureKeyword,
                            _Out_writes_bytes_to_opt_(cbSize, *pcbNeeded)
                                        PSTR     pmszOptionList,
                            _In_        DWORD    cbSize,
                            _Out_ _On_failure_(_When_(return == E_OUTOFMEMORY, _Post_valid_))
                                        PDWORD   pcbNeeded) PURE;
};

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
//****************************************************************************
//  IPrintOemUni interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemUni
DECLARE_INTERFACE_(IPrintOemUni, IPrintOemEngine)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemEngine methods
    //

    //
    // Method for OEM to specify DDI hook out
    //

    STDMETHOD(EnableDriver)  (THIS_   DWORD           DriverVersion,
                                      DWORD           cbSize,
                                      PDRVENABLEDATA  pded) PURE;

    //
    // Method to notify OEM plugin that it is no longer required
    //

    STDMETHOD(DisableDriver) (THIS) PURE;

    //
    // Method for OEM to construct its own PDEV
    //

    STDMETHOD(EnablePDEV)    (THIS_   PDEVOBJ         pdevobj,
                                      PWSTR           pPrinterName,
                                      ULONG           cPatterns,
                                      HSURF          *phsurfPatterns,
                                      ULONG           cjGdiInfo,
                                      GDIINFO        *pGdiInfo,
                                      ULONG           cjDevInfo,
                                      DEVINFO        *pDevInfo,
                                      DRVENABLEDATA  *pded,
                                      OUT PDEVOEM    *pDevOem) PURE;

    //
    // Method for OEM to free any resource associated with its PDEV
    //

    STDMETHOD(DisablePDEV)   (THIS_   PDEVOBJ         pdevobj) PURE;

    //
    // Method for OEM to transfer from old PDEV to new PDEV
    //

    STDMETHOD(ResetPDEV)     (THIS_   PDEVOBJ         pdevobjOld,
                                      PDEVOBJ         pdevobjNew) PURE;

    //
    // IPrintOemUni methods
    //

    //
    // Method for publishing Driver interface.
    //

    STDMETHOD(PublishDriverInterface)(THIS_ _In_ IUnknown *pIUnknown) PURE;

    //
    // Method for getting OEM implemented methods.
    // Returns S_OK if the given method is implemented.
    // Returns S_FALSE if the given method is not implemented.
    //
    //

    STDMETHOD(GetImplementedMethod) (THIS_  PSTR    pMethodName) PURE;

    //
    // DriverDMS
    //

    STDMETHOD(DriverDMS)(THIS_  PVOID   pDevObj,
                                PVOID   pBuffer,
                                DWORD   cbSize,
                                PDWORD  pcbNeeded) PURE;

    //
    // CommandCallback
    //

    STDMETHOD(CommandCallback)(THIS_    PDEVOBJ     pdevobj,
                                        DWORD       dwCallbackID,
                                        DWORD       dwCount,
                                        PDWORD      pdwParams,
                                        OUT INT     *piResult) PURE;


    //
    // ImageProcessing
    //

    STDMETHOD(ImageProcessing)(THIS_    PDEVOBJ             pdevobj,
                                        PBYTE               pSrcBitmap,
                                        PBITMAPINFOHEADER   pBitmapInfoHeader,
                                        PBYTE               pColorTable,
                                        DWORD               dwCallbackID,
                                        PIPPARAMS           pIPParams,
                                        OUT PBYTE           *ppbResult) PURE;

    //
    // FilterGraphics
    //

    STDMETHOD(FilterGraphics) (THIS_    PDEVOBJ     pdevobj,
                                        PBYTE       pBuf,
                                        DWORD       dwLen) PURE;

    //
    // Compression
    //

    STDMETHOD(Compression)(THIS_    PDEVOBJ     pdevobj,
                                    PBYTE       pInBuf,
                                    PBYTE       pOutBuf,
                                    DWORD       dwInLen,
                                    DWORD       dwOutLen,
                                    OUT INT     *piResult) PURE;

    //
    // HalftonePattern
    //

    STDMETHOD(HalftonePattern) (THIS_   PDEVOBJ     pdevobj,
                                        PBYTE       pHTPattern,
                                        DWORD       dwHTPatternX,
                                        DWORD       dwHTPatternY,
                                        DWORD       dwHTNumPatterns,
                                        DWORD       dwCallbackID,
                                        PBYTE       pResource,
                                        DWORD       dwResourceSize) PURE;

    //
    // MemoryUsage
    //

    STDMETHOD(MemoryUsage) (THIS_   PDEVOBJ         pdevobj,
                                    POEMMEMORYUSAGE pMemoryUsage) PURE;

    //
    // TTYGetInfo
    //

    STDMETHOD(TTYGetInfo)(THIS_     PDEVOBJ     pdevobj,
                                    DWORD       dwInfoIndex,
                                    PVOID       pOutputBuf,
                                    DWORD       dwSize,
                                    DWORD       *pcbcNeeded
                                    ) PURE;
    //
    // DownloadFontheader
    //

    STDMETHOD(DownloadFontHeader)(THIS_     PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            OUT DWORD   *pdwResult) PURE;

    //
    // DownloadCharGlyph
    //

    STDMETHOD(DownloadCharGlyph)(THIS_      PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            HGLYPH      hGlyph,
                                            PDWORD      pdwWidth,
                                            OUT DWORD   *pdwResult) PURE;


    //
    // TTDownloadMethod
    //

    STDMETHOD(TTDownloadMethod)(THIS_       PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            OUT DWORD   *pdwResult) PURE;

    //
    // OutputCharStr
    //

    STDMETHOD(OutputCharStr)(THIS_      PDEVOBJ     pdevobj,
                                        PUNIFONTOBJ pUFObj,
                                        DWORD       dwType,
                                        DWORD       dwCount,
                                        PVOID       pGlyph) PURE;

    //
    // SendFontCmd
    //


    STDMETHOD(SendFontCmd)(THIS_    PDEVOBJ      pdevobj,
                                    PUNIFONTOBJ  pUFObj,
                                    PFINVOCATION pFInv) PURE;

    //
    // TextOutAsBitmap
    //

    STDMETHOD(TextOutAsBitmap)(THIS_        SURFOBJ    *pso,
                                            STROBJ     *pstro,
                                            FONTOBJ    *pfo,
                                            CLIPOBJ    *pco,
                                            RECTL      *prclExtra,
                                            RECTL      *prclOpaque,
                                            BRUSHOBJ   *pboFore,
                                            BRUSHOBJ   *pboOpaque,
                                            POINTL     *pptlOrg,
                                            MIX         mix) PURE;
};

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
//****************************************************************************
//  IPrintOemUni2 interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemUni2
DECLARE_INTERFACE_(IPrintOemUni2, IPrintOemUni)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemEngine methods
    //

    //
    // Method for OEM to specify DDI hook out
    //

    STDMETHOD(EnableDriver)  (THIS_   DWORD           DriverVersion,
                                      DWORD           cbSize,
                                      PDRVENABLEDATA  pded) PURE;

    //
    // Method to notify OEM plugin that it is no longer required
    //

    STDMETHOD(DisableDriver) (THIS) PURE;

    //
    // Method for OEM to construct its own PDEV
    //

    STDMETHOD(EnablePDEV)    (THIS_   PDEVOBJ         pdevobj,
                                      PWSTR           pPrinterName,
                                      ULONG           cPatterns,
                                      HSURF          *phsurfPatterns,
                                      ULONG           cjGdiInfo,
                                      GDIINFO        *pGdiInfo,
                                      ULONG           cjDevInfo,
                                      DEVINFO        *pDevInfo,
                                      DRVENABLEDATA  *pded,
                                      OUT PDEVOEM    *pDevOem) PURE;

    //
    // Method for OEM to free any resource associated with its PDEV
    //

    STDMETHOD(DisablePDEV)   (THIS_   PDEVOBJ         pdevobj) PURE;

    //
    // Method for OEM to transfer from old PDEV to new PDEV
    //

    STDMETHOD(ResetPDEV)     (THIS_   PDEVOBJ         pdevobjOld,
                                      PDEVOBJ         pdevobjNew) PURE;

    //
    // IPrintOemUni methods
    //

    //
    // Method for publishing Driver interface.
    //

    STDMETHOD(PublishDriverInterface)(THIS_ _In_ IUnknown *pIUnknown) PURE;

    //
    // Method for getting OEM implemented methods.
    // Returns S_OK if the given method is implemented.
    // Returns S_FALSE if the given method is not implemented.
    //
    //

    STDMETHOD(GetImplementedMethod) (THIS_  PSTR    pMethodName) PURE;

    //
    // DriverDMS
    //

    STDMETHOD(DriverDMS)(THIS_  PVOID   pDevObj,
                                PVOID   pBuffer,
                                DWORD   cbSize,
                                PDWORD  pcbNeeded) PURE;

    //
    // CommandCallback
    //

    STDMETHOD(CommandCallback)(THIS_    PDEVOBJ     pdevobj,
                                        DWORD       dwCallbackID,
                                        DWORD       dwCount,
                                        PDWORD      pdwParams,
                                        OUT INT     *piResult) PURE;


    //
    // ImageProcessing
    //

    STDMETHOD(ImageProcessing)(THIS_    PDEVOBJ             pdevobj,
                                        PBYTE               pSrcBitmap,
                                        PBITMAPINFOHEADER   pBitmapInfoHeader,
                                        PBYTE               pColorTable,
                                        DWORD               dwCallbackID,
                                        PIPPARAMS           pIPParams,
                                        OUT PBYTE           *ppbResult) PURE;

    //
    // FilterGraphics
    //

    STDMETHOD(FilterGraphics) (THIS_    PDEVOBJ     pdevobj,
                                        PBYTE       pBuf,
                                        DWORD       dwLen) PURE;

    //
    // Compression
    //

    STDMETHOD(Compression)(THIS_    PDEVOBJ     pdevobj,
                                    PBYTE       pInBuf,
                                    PBYTE       pOutBuf,
                                    DWORD       dwInLen,
                                    DWORD       dwOutLen,
                                    OUT INT     *piResult) PURE;

    //
    // HalftonePattern
    //

    STDMETHOD(HalftonePattern) (THIS_   PDEVOBJ     pdevobj,
                                        PBYTE       pHTPattern,
                                        DWORD       dwHTPatternX,
                                        DWORD       dwHTPatternY,
                                        DWORD       dwHTNumPatterns,
                                        DWORD       dwCallbackID,
                                        PBYTE       pResource,
                                        DWORD       dwResourceSize) PURE;

    //
    // MemoryUsage
    //

    STDMETHOD(MemoryUsage) (THIS_   PDEVOBJ         pdevobj,
                                    POEMMEMORYUSAGE pMemoryUsage) PURE;

    //
    // TTYGetInfo
    //

    STDMETHOD(TTYGetInfo)(THIS_     PDEVOBJ     pdevobj,
                                    DWORD       dwInfoIndex,
                                    PVOID       pOutputBuf,
                                    DWORD       dwSize,
                                    DWORD       *pcbcNeeded
                                    ) PURE;
    //
    // DownloadFontheader
    //

    STDMETHOD(DownloadFontHeader)(THIS_     PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            OUT DWORD   *pdwResult) PURE;

    //
    // DownloadCharGlyph
    //

    STDMETHOD(DownloadCharGlyph)(THIS_      PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            HGLYPH      hGlyph,
                                            PDWORD      pdwWidth,
                                            OUT DWORD   *pdwResult) PURE;


    //
    // TTDownloadMethod
    //

    STDMETHOD(TTDownloadMethod)(THIS_       PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            OUT DWORD   *pdwResult) PURE;

    //
    // OutputCharStr
    //

    STDMETHOD(OutputCharStr)(THIS_      PDEVOBJ     pdevobj,
                                        PUNIFONTOBJ pUFObj,
                                        DWORD       dwType,
                                        DWORD       dwCount,
                                        PVOID       pGlyph) PURE;

    //
    // SendFontCmd
    //


    STDMETHOD(SendFontCmd)(THIS_    PDEVOBJ      pdevobj,
                                    PUNIFONTOBJ  pUFObj,
                                    PFINVOCATION pFInv) PURE;

    //
    // TextOutAsBitmap
    //

    STDMETHOD(TextOutAsBitmap)(THIS_        SURFOBJ    *pso,
                                            STROBJ     *pstro,
                                            FONTOBJ    *pfo,
                                            CLIPOBJ    *pco,
                                            RECTL      *prclExtra,
                                            RECTL      *prclOpaque,
                                            BRUSHOBJ   *pboFore,
                                            BRUSHOBJ   *pboOpaque,
                                            POINTL     *pptlOrg,
                                            MIX         mix) PURE;

    //
    // IPrintOemUni2 methods
    //

    //
    // Method for plugin to hook out spooler's WritePrinter API so it
    // can get access to output data Universal driver is generating
    //
    // At DrvEnablePDEV time, Universal driver will call this function with
    // pdevobj = NULL, pBuf = NULL, cbBuffer = 0 to detect if the plugin
    // implements this function. Plugin should return S_OK to indicate it is
    // implementing this function, or return E_NOTIMPL otherwise.
    //
    // In pcbWritten, plugins should return the number of bytes written to the
    // spooler's WritePrinter function. Zero doesn't carry a special meaning,
    // errors must be reported through the returned HRESULT.
    //

    STDMETHOD(WritePrinter) (THIS_   PDEVOBJ    pdevobj,
                                     PVOID      pBuf,
                                     DWORD      cbBuffer,
                                     PDWORD     pcbWritten) PURE;
};

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)

//
//****************************************************************************
//  IPrintOemUni3 interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemUni3
DECLARE_INTERFACE_(IPrintOemUni3, IPrintOemUni2)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemCommon methods
    //

    //
    // Method for getting OEM related information
    //

    STDMETHOD(GetInfo) (THIS_
         _In_   DWORD   dwMode,
         _Out_writes_bytes_to_(cbSize, *pcbNeeded) PVOID   pBuffer,
         _In_   DWORD   cbSize,
         _Out_  PDWORD  pcbNeeded) PURE;
    //
    // Method for OEM private devmode handling
    //

    STDMETHOD(DevMode) (THIS_   _In_    DWORD       dwMode,
                                _Inout_ POEMDMPARAM pOemDMParam) PURE;

    //
    // IPrintOemEngine methods
    //

    //
    // Method for OEM to specify DDI hook out
    //

    STDMETHOD(EnableDriver)  (THIS_   DWORD           DriverVersion,
                                      DWORD           cbSize,
                                      PDRVENABLEDATA  pded) PURE;

    //
    // Method to notify OEM plugin that it is no longer required
    //

    STDMETHOD(DisableDriver) (THIS) PURE;

    //
    // Method for OEM to construct its own PDEV
    //

    STDMETHOD(EnablePDEV)    (THIS_   PDEVOBJ         pdevobj,
                                      PWSTR           pPrinterName,
                                      ULONG           cPatterns,
                                      HSURF          *phsurfPatterns,
                                      ULONG           cjGdiInfo,
                                      GDIINFO        *pGdiInfo,
                                      ULONG           cjDevInfo,
                                      DEVINFO        *pDevInfo,
                                      DRVENABLEDATA  *pded,
                                      OUT PDEVOEM    *pDevOem) PURE;

    //
    // Method for OEM to free any resource associated with its PDEV
    //

    STDMETHOD(DisablePDEV)   (THIS_   PDEVOBJ         pdevobj) PURE;

    //
    // Method for OEM to transfer from old PDEV to new PDEV
    //

    STDMETHOD(ResetPDEV)     (THIS_   PDEVOBJ         pdevobjOld,
                                      PDEVOBJ         pdevobjNew) PURE;

    //
    // IPrintOemUni methods
    //

    //
    // Method for publishing Driver interface.
    //

    STDMETHOD(PublishDriverInterface)(THIS_ _In_ IUnknown *pIUnknown) PURE;

    //
    // Method for getting OEM implemented methods.
    // Returns S_OK if the given method is implemented.
    // Returns S_FALSE if the given method is not implemented.
    //
    //

    STDMETHOD(GetImplementedMethod) (THIS_  PSTR    pMethodName) PURE;

    //
    // DriverDMS
    //

    STDMETHOD(DriverDMS)(THIS_  PVOID   pDevObj,
                                PVOID   pBuffer,
                                DWORD   cbSize,
                                PDWORD  pcbNeeded) PURE;

    //
    // CommandCallback
    //

    STDMETHOD(CommandCallback)(THIS_    PDEVOBJ     pdevobj,
                                        DWORD       dwCallbackID,
                                        DWORD       dwCount,
                                        PDWORD      pdwParams,
                                        OUT INT     *piResult) PURE;


    //
    // ImageProcessing
    //

    STDMETHOD(ImageProcessing)(THIS_    PDEVOBJ             pdevobj,
                                        PBYTE               pSrcBitmap,
                                        PBITMAPINFOHEADER   pBitmapInfoHeader,
                                        PBYTE               pColorTable,
                                        DWORD               dwCallbackID,
                                        PIPPARAMS           pIPParams,
                                        OUT PBYTE           *ppbResult) PURE;

    //
    // FilterGraphics
    //

    STDMETHOD(FilterGraphics) (THIS_    PDEVOBJ     pdevobj,
                                        PBYTE       pBuf,
                                        DWORD       dwLen) PURE;

    //
    // Compression
    //

    STDMETHOD(Compression)(THIS_    PDEVOBJ     pdevobj,
                                    PBYTE       pInBuf,
                                    PBYTE       pOutBuf,
                                    DWORD       dwInLen,
                                    DWORD       dwOutLen,
                                    OUT INT     *piResult) PURE;

    //
    // HalftonePattern
    //

    STDMETHOD(HalftonePattern) (THIS_   PDEVOBJ     pdevobj,
                                        PBYTE       pHTPattern,
                                        DWORD       dwHTPatternX,
                                        DWORD       dwHTPatternY,
                                        DWORD       dwHTNumPatterns,
                                        DWORD       dwCallbackID,
                                        PBYTE       pResource,
                                        DWORD       dwResourceSize) PURE;

    //
    // MemoryUsage
    //

    STDMETHOD(MemoryUsage) (THIS_   PDEVOBJ         pdevobj,
                                    POEMMEMORYUSAGE pMemoryUsage) PURE;

    //
    // TTYGetInfo
    //

    STDMETHOD(TTYGetInfo)(THIS_     PDEVOBJ     pdevobj,
                                    DWORD       dwInfoIndex,
                                    PVOID       pOutputBuf,
                                    DWORD       dwSize,
                                    DWORD       *pcbcNeeded
                                    ) PURE;
    //
    // DownloadFontheader
    //

    STDMETHOD(DownloadFontHeader)(THIS_     PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            OUT DWORD   *pdwResult) PURE;

    //
    // DownloadCharGlyph
    //

    STDMETHOD(DownloadCharGlyph)(THIS_      PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            HGLYPH      hGlyph,
                                            PDWORD      pdwWidth,
                                            OUT DWORD   *pdwResult) PURE;


    //
    // TTDownloadMethod
    //

    STDMETHOD(TTDownloadMethod)(THIS_       PDEVOBJ     pdevobj,
                                            PUNIFONTOBJ pUFObj,
                                            OUT DWORD   *pdwResult) PURE;

    //
    // OutputCharStr
    //

    STDMETHOD(OutputCharStr)(THIS_      PDEVOBJ     pdevobj,
                                        PUNIFONTOBJ pUFObj,
                                        DWORD       dwType,
                                        DWORD       dwCount,
                                        PVOID       pGlyph) PURE;

    //
    // SendFontCmd
    //


    STDMETHOD(SendFontCmd)(THIS_    PDEVOBJ      pdevobj,
                                    PUNIFONTOBJ  pUFObj,
                                    PFINVOCATION pFInv) PURE;

    //
    // TextOutAsBitmap
    //

    STDMETHOD(TextOutAsBitmap)(THIS_        SURFOBJ    *pso,
                                            STROBJ     *pstro,
                                            FONTOBJ    *pfo,
                                            CLIPOBJ    *pco,
                                            RECTL      *prclExtra,
                                            RECTL      *prclOpaque,
                                            BRUSHOBJ   *pboFore,
                                            BRUSHOBJ   *pboOpaque,
                                            POINTL     *pptlOrg,
                                            MIX         mix) PURE;

    //
    // IPrintOemUni2 methods
    //

    //
    // Method for plugin to hook out spooler's WritePrinter API so it
    // can get access to output data Universal driver is generating
    //
    // At DrvEnablePDEV time, Universal driver will call this function with
    // pdevobj = NULL, pBuf = NULL, cbBuffer = 0 to detect if the plugin
    // implements this function. Plugin should return S_OK to indicate it is
    // implementing this function, or return E_NOTIMPL otherwise.
    //
    // In pcbWritten, plugins should return the number of bytes written to the
    // spooler's WritePrinter function. Zero doesn't carry a special meaning,
    // errors must be reported through the returned HRESULT.
    //

    STDMETHOD(WritePrinter) (THIS_   PDEVOBJ    pdevobj,
                                     PVOID      pBuf,
                                     DWORD      cbBuffer,
                                     PDWORD     pcbWritten) PURE;

    //
    // IPrintOemUni3 methods
    //

    //
    // Method for plugin to hook DownloadPattern, which is called in
    // DrvRealizeBrush to downlowad pattern to a printer.
    //

    STDMETHOD(DownloadPattern) (THIS_ PDEVOBJ  pdevobj,
                                      SURFOBJ *psoPattern,
                                      LONG     lPatternID) PURE;


    STDMETHOD(GetPDEVAdjustment) (THIS_ PDEVOBJ    pdevobj,
                                        DWORD      dwAdjustType,
                                        PVOID      pBuf,
                                        DWORD      cbBuffer,
                                        OUT BOOL  *pbAdjustmentDone) PURE;

    STDMETHOD(SetBandSize) (THIS_    PDEVOBJ    pdevobj,
                                     INT        iFormat,
                                     DWORD      dwPageWidthBytes,
                                     DWORD      dwPageHeight,
                                     DWORD      dwMaxHeight,
                                     PDWORD     pdwRequiredHeight ) PURE;

};

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
//****************************************************************************
//  IPrintOemDriverUni interface
//****************************************************************************
//

#undef INTERFACE
#define INTERFACE IPrintOemDriverUni
DECLARE_INTERFACE_(IPrintOemDriverUni, IUnknown)
{
    //
    // IUnknown methods
    //

    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;

    //
    // IPrintOemDriverUni methods
    //

    //
    // Function to get driver settings
    //

    STDMETHOD(DrvGetDriverSetting) (THIS_   PVOID   pdriverobj,
                                            PCSTR   Feature,
                                            PVOID   pOutput,
                                            DWORD   cbSize,
                                            PDWORD  pcbNeeded,
                                            PDWORD  pdwOptionsReturned) PURE;

    //
    // Common to both Unidrv & Pscript
    //

    STDMETHOD(DrvWriteSpoolBuf)(THIS_       PDEVOBJ     pdevobj,
                                            PVOID       pBuffer,
                                            DWORD       cbSize,
                                            OUT DWORD   *pdwResult) PURE;

    //
    // Unidrv specific XMoveTo and YMoveTo. Returns E_NOT_IMPL in Pscript
    //

    STDMETHOD(DrvXMoveTo)(THIS_     PDEVOBJ     pdevobj,
                                    INT         x,
                                    DWORD       dwFlags,
                                    OUT INT     *piResult) PURE;

    STDMETHOD(DrvYMoveTo)(THIS_     PDEVOBJ     pdevobj,
                                    INT         y,
                                    DWORD       dwFlags,
                                    OUT INT     *piResult) PURE;
    //
    // Unidrv specific. To get the standard variable value.
    //

    STDMETHOD(DrvGetStandardVariable)(THIS_     PDEVOBJ     pdevobj,
                                                DWORD       dwIndex,
                                                PVOID       pBuffer,
                                                DWORD       cbSize,
                                                PDWORD      pcbNeeded) PURE;

    //
    // Unidrv specific.  To Provide OEM plugins access to GPD data.
    //

    STDMETHOD (DrvGetGPDData)(THIS_  PDEVOBJ     pdevobj,
                                     DWORD       dwType,     // Type of the data
                                     PVOID         pInputData,   // reserved. Should be set to 0
                                     PVOID          pBuffer,     // Caller allocated Buffer to be copied
                                     DWORD       cbSize,     // Size of the buffer
                                     PDWORD      pcbNeeded   // New Size of the buffer if needed.
                             ) PURE;


    //
    // Unidrv specific. To do the TextOut.
    //

    STDMETHOD(DrvUniTextOut)(THIS_    SURFOBJ    *pso,
                                      STROBJ     *pstro,
                                      FONTOBJ    *pfo,
                                      CLIPOBJ    *pco,
                                      RECTL      *prclExtra,
                                      RECTL      *prclOpaque,
                                      BRUSHOBJ   *pboFore,
                                      BRUSHOBJ   *pboOpaque,
                                      POINTL     *pptlBrushOrg,
                                      MIX         mix) PURE;

    //
    //   Warning!!!  new method!!  must place at end of
    //   interface - else major incompatibility with previous oem plugins
    //

    STDMETHOD(DrvWriteAbortBuf)(THIS_       PDEVOBJ     pdevobj,
                                            PVOID       pBuffer,
                                            DWORD       cbSize,
                                            DWORD       dwWait  //  pause data transmission for this many millisecs.
                               ) PURE;
};

#endif  // !KERNEL_MODE

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // !_PRCOMOEM_H_

