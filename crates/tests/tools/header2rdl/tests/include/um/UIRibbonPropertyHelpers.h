//****************************************************************************
//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File:       UIRibbonPropertyHelpers.h
//
//  Contents:   Property helper functions for the Windows Ribbon Framework
//
//****************************************************************************

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <propvarutil.h>

interface IUIImage;

#ifndef TUIPROPERTYKEYDEFINED
// A version of PROPERTYKEY whose VARTYPE can be tested at compile time.
#include <pshpack8.h>
template<VARTYPE T>
struct TUIPROPERTYKEY
{ 
  GUID fmtid;
  DWORD pid;
  inline operator const PROPERTYKEY&() const { return reinterpret_cast<const PROPERTYKEY&>(*this); }
  inline const PROPERTYKEY* operator&() const { return reinterpret_cast<const PROPERTYKEY*>(this); }
};
#include <poppack.h>

C_ASSERT(sizeof(TUIPROPERTYKEY<VT_BOOL>) == sizeof(PROPERTYKEY));

#define TUIPROPERTYKEYDEFINED
#endif // TUIPROPERTYKEYDEFINED

// UIBreakCheckType is being used to do both Compilation and Runtime checking of types on PROPVARIANTs

struct UIWrongType {};

template<class A, VARTYPE B>
struct UIBreakCheckType 
{ 
    // If a PROPERTYKEY/type combination is invalid, compilation will break (it will try to cast a bool to UIWrongType)
    typedef UIWrongType _Type; 
    static bool Validate(REFPROPERTYKEY key) { UNREFERENCED_PARAMETER(key); return false; }
};

#define UI_REGISTER_TYPE( type ) \
    template<> struct UIBreakCheckType< TUIPROPERTYKEY<type>, type > \
    { \
        typedef bool _Type; \
        static bool Validate(REFPROPERTYKEY key) { UNREFERENCED_PARAMETER(key); return true; } \
    }; \
    template<> struct UIBreakCheckType< PROPERTYKEY, type > \
    { \
        typedef bool _Type; \
        /* we cannot catch at compilation time when PROPERTYKEYs are by reference, so check at runtime */ \
        static bool Validate(REFPROPERTYKEY key) { return (type) == key.pid; }\
    }; 
    
// Register known types
UI_REGISTER_TYPE( VT_BOOL )
UI_REGISTER_TYPE( VT_UI4 )
UI_REGISTER_TYPE( VT_LPWSTR )
UI_REGISTER_TYPE( VT_DECIMAL )
UI_REGISTER_TYPE( VT_UNKNOWN )
UI_REGISTER_TYPE( VT_ARRAY|VT_UNKNOWN )

//======================================================================================
// PROPVARIANT initializer helpers

template<class T>
HRESULT UIInitPropertyFromBoolean(const T& propertyKey, BOOL fVal, _Out_ PROPVARIANT* pPropVar)
{
    // If this fails to compile, it means this property is not of type VT_BOOL
    typename UIBreakCheckType<T, VT_BOOL>::_Type valid = UIBreakCheckType<T, VT_BOOL>::Validate(propertyKey);
    return valid ? InitPropVariantFromBoolean(fVal, pPropVar) : E_INVALIDARG;
}

template<class T>
HRESULT UIInitPropertyFromUInt32(const T& propertyKey, UINT ulVal, _Out_ PROPVARIANT* pPropVar)
{
    // If this fails to compile, it means this property is not of type VT_UI4
    typename UIBreakCheckType<T, VT_UI4>::_Type valid = UIBreakCheckType<T, VT_UI4>::Validate(propertyKey);
    return valid ? InitPropVariantFromUInt32(ulVal, pPropVar) : E_INVALIDARG;
}

template<class T>
HRESULT UIInitPropertyFromString(const T& propertyKey, _In_ PCWSTR psz, _Out_ PROPVARIANT* pPropVar)
{
    // If this fails to compile, it means this property is not of type VT_LPWSTR
    typename UIBreakCheckType<T, VT_LPWSTR>::_Type valid = UIBreakCheckType<T, VT_LPWSTR>::Validate(propertyKey);
    return valid ? InitPropVariantFromString(psz, pPropVar) : E_INVALIDARG;
}

template<class T>
HRESULT UIInitPropertyFromDecimal(const T& propertyKey, const DECIMAL& decValue, _Out_ PROPVARIANT* pPropVar)
{
    // If this fails to compile, it means this property is not of type VT_DECIMAL
    typename UIBreakCheckType<T, VT_DECIMAL>::_Type valid = UIBreakCheckType<T, VT_DECIMAL>::Validate(propertyKey);
    if (valid)
    {
        //  Must set decVal before vt because the two overlap.
        pPropVar->decVal = decValue;
        pPropVar->vt = VT_DECIMAL;
        return S_OK;
    }
    return E_INVALIDARG;
}

template<class T>
HRESULT UIInitPropertyFromInterface(const T& propertyKey, _In_ IUnknown* pUnk, _Out_ PROPVARIANT* pPropVar)
{
    // If this fails to compile, it means this property is not of type VT_UNKNOWN
    typename UIBreakCheckType<T, VT_UNKNOWN>::_Type valid = UIBreakCheckType<T, VT_UNKNOWN>::Validate(propertyKey);
    if (valid)
    {
        pPropVar->vt = VT_UNKNOWN;
        pPropVar->punkVal = pUnk;
        if (pUnk)
        {
            pUnk->AddRef();
        }
        return S_OK;
    }
    return E_INVALIDARG;
}

template<class T>
HRESULT UIInitPropertyFromImage(const T& propertyKey, _In_ IUIImage* pImage, _Out_ PROPVARIANT* pPropVar)
{
    return UIInitPropertyFromInterface(propertyKey, pImage, pPropVar);
}

template<class T>
HRESULT UIInitPropertyFromIUnknownArray(const T& propertyKey, _In_ SAFEARRAY* psa, _Out_ PROPVARIANT* pPropVar)
{
    // If this fails to compile, it means this property is not of type VT_ARRAY
    typename UIBreakCheckType<T, VT_ARRAY|VT_UNKNOWN>::_Type valid = UIBreakCheckType<T, VT_ARRAY|VT_UNKNOWN>::Validate(propertyKey);
    if (valid && (psa->fFeatures & FADF_UNKNOWN) )
    {
        HRESULT hr = ::SafeArrayCopy(psa, &pPropVar->parray);
        if (SUCCEEDED(hr))
        {
            pPropVar->vt = VT_ARRAY|VT_UNKNOWN;
        }
        return hr;
    }
    return E_INVALIDARG;
}

//======================================================================================
// Extract value from PROPVARIANT

template<class T>
HRESULT UIPropertyToBoolean(const T& propertyKey, REFPROPVARIANT propvarIn, _Out_ BOOL *pfRet)
{
    // If this fails to compile, it means this property is not of type VT_BOOL
    typename UIBreakCheckType<T, VT_BOOL>::_Type valid = UIBreakCheckType<T, VT_BOOL>::Validate(propertyKey);
    return valid ? PropVariantToBoolean(propvarIn, pfRet) : E_INVALIDARG;
}

template<class T>
HRESULT UIPropertyToUInt32(const T& propertyKey, REFPROPVARIANT propvarIn, _Out_ UINT *pulVal)
{
    // If this fails to compile, it means this property is not of type VT_UI4
    typename UIBreakCheckType<T, VT_UI4>::_Type valid = UIBreakCheckType<T, VT_UI4>::Validate(propertyKey);
    return valid ? PropVariantToUInt32(propvarIn, pulVal) : E_INVALIDARG;
}

// Allocate a new string and return it in ppszOut
template<class T>
HRESULT UIPropertyToStringAlloc(const T& propertyKey, REFPROPVARIANT propvarIn, _Outptr_ PWSTR *ppszOut)
{
    // If this fails to compile, it means this property is not of type VT_LPWSTR
    typename UIBreakCheckType<T, VT_LPWSTR>::_Type valid = UIBreakCheckType<T, VT_LPWSTR>::Validate(propertyKey);
    return valid ? PropVariantToStringAlloc(propvarIn, ppszOut) : E_INVALIDARG;
}

template<class T>
HRESULT UIPropertyToDecimal(const T& propertyKey, REFPROPVARIANT propvarIn, _Out_ DECIMAL *pDecValue)
{
    // If this fails to compile, it means this property is not of type VT_DECIMAL
    typename UIBreakCheckType<T, VT_DECIMAL>::_Type valid = UIBreakCheckType<T, VT_DECIMAL>::Validate(propertyKey);
    if (valid && propvarIn.vt == VT_DECIMAL)
    {
        *pDecValue = propvarIn.decVal;
        return S_OK;
    }
    return E_INVALIDARG;
}

template<class T, class TInterface>
HRESULT UIPropertyToInterface(const T& propertyKey, REFPROPVARIANT propvarIn, _Outptr_ TInterface** ppObj)
{
    *ppObj = NULL;
    // If this fails to compile, it means this property is not of type VT_UNKNOWN
    typename UIBreakCheckType<T, VT_UNKNOWN>::_Type valid = UIBreakCheckType<T, VT_UNKNOWN>::Validate(propertyKey);
    if (valid && propvarIn.vt == VT_UNKNOWN)
    {
        if (propvarIn.punkVal)
        {
            return propvarIn.punkVal->QueryInterface(__uuidof(TInterface), (void**)ppObj);
        }
        return S_OK;
    }
    return E_INVALIDARG;
}

template<class T>
HRESULT UIPropertyToImage(const T& propertyKey, REFPROPVARIANT propvarIn, _Outptr_ IUIImage** ppImage)
{
    return UIPropertyToInterface(propertyKey, propvarIn, ppImage);
}

template<class T>
HRESULT UIPropertyToIUnknownArrayAlloc(const T& propertyKey, REFPROPVARIANT propvarIn, _Outptr_ SAFEARRAY** ppsa)
{
    // If this fails to compile, it means this property is not of type VT_ARRAY
    typename UIBreakCheckType<T, VT_ARRAY|VT_UNKNOWN>::_Type valid = UIBreakCheckType<T, VT_ARRAY|VT_UNKNOWN>::Validate(propertyKey);
    if (valid && propvarIn.vt == (VT_ARRAY|VT_UNKNOWN) )
    {
        return ::SafeArrayCopy(propvarIn.parray, ppsa);
    }
    return E_INVALIDARG;
}

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

