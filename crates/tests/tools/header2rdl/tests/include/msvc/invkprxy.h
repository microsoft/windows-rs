/***
* invkprxy.h - Native C++ compiler COM support - IDispatch::Invoke helpers
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
****/

#pragma once

#if !defined(_INC_INVKPRXY)
#define _INC_INVKPRXY 1

HRESULT __stdcall _com_handle_excepinfo(EXCEPINFO& excepInfo, IErrorInfo** pperrinfo);
HRESULT __cdecl _com_invoke_helper(IDispatch* pDispatch,
                                DISPID dwDispID,
                                WORD wFlags,
                                VARTYPE vtRet,
                                void* pvRet,
                                _In_opt_z_ const wchar_t* pwParamInfo,
                                va_list argList,
                                IErrorInfo** pperrinfo);
HRESULT __cdecl _com_dispatch_raw_method(IDispatch* pDispatch,
                                DISPID dwDispID,
                                WORD wFlags,
                                VARTYPE vtRet,
                                void* pvRet,
                                const wchar_t* pwParamInfo,
                                ...) throw();
HRESULT __stdcall _com_dispatch_raw_propget(IDispatch* pDispatch,
                                DISPID dwDispID,
                                VARTYPE vtProp,
                                void* pvProp) throw();
HRESULT __cdecl _com_dispatch_raw_propput(IDispatch* pDispatch,
                                DISPID dwDispID,
                                VARTYPE vtProp,
                                ...) throw();

#endif // _INC_INVKPRXY
