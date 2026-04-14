// Copyright (c) Microsoft Corporation.  All Rights Reserved.

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// {1BB05960-5FBF-11d2-A521-44DF07C10000}
DEFINE_GUID(IID_IXMLGraphBuilder, 
0x1bb05960, 0x5fbf, 0x11d2, 0xa5, 0x21, 0x44, 0xdf, 0x7, 0xc1, 0x0, 0x0);

interface IXMLGraphBuilder : IUnknown
{
    STDMETHOD(BuildFromXML) (IGraphBuilder *pGraph, IXMLElement *pxml) = 0;
    STDMETHOD(SaveToXML) (IGraphBuilder *pGraph, BSTR *pbstrxml) = 0;
    STDMETHOD(BuildFromXMLFile) (IGraphBuilder *pGraph, LPCWSTR wszFileName, LPCWSTR wszBaseURL) = 0;
};

// CLSID_XMLGraphBuilder
// {1BB05961-5FBF-11d2-A521-44DF07C10000}
DEFINE_GUID(CLSID_XMLGraphBuilder, 
0x1bb05961, 0x5fbf, 0x11d2, 0xa5, 0x21, 0x44, 0xdf, 0x7, 0xc1, 0x0, 0x0);



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

