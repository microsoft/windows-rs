//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  Component: WSDAPI - Microsoft Web Services for Devices API
// 
//  File: wsdxmldom.h
//
//  Abstract: WSDAPI XML Type Definitions
//
//--------------------------------------------------------------------------
#pragma once

// pragma once does not guard properly when included into .idl
#ifndef __WSDXMLDOM_H__
#define __WSDXMLDOM_H__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


typedef struct _WSDXML_NAMESPACE WSDXML_NAMESPACE;
typedef struct _WSDXML_NAME WSDXML_NAME;
typedef struct _WSDXML_TYPE WSDXML_TYPE;
typedef struct _WSDXML_PREFIX_MAPPING WSDXML_PREFIX_MAPPING;
typedef struct _WSDXML_ATTRIBUTE WSDXML_ATTRIBUTE;
typedef struct _WSDXML_NODE WSDXML_NODE;
typedef struct _WSDXML_ELEMENT WSDXML_ELEMENT;
typedef struct _WSDXML_TEXT WSDXML_TEXT;
typedef struct _WSDXML_ELEMENT_LIST WSDXML_ELEMENT_LIST;

struct _WSDXML_NAMESPACE
{
    const WCHAR* Uri;
    const WCHAR* PreferredPrefix;
    WSDXML_NAME* Names;
    WORD NamesCount;
    WORD Encoding;
};

struct _WSDXML_NAME
{
    WSDXML_NAMESPACE* Space;
    WCHAR* LocalName;
};

struct _WSDXML_TYPE
{
    const WCHAR* Uri;
    const BYTE* Table;
};

struct _WSDXML_PREFIX_MAPPING
{
    DWORD Refs;
    WSDXML_PREFIX_MAPPING* Next;
    WSDXML_NAMESPACE* Space;
    WCHAR* Prefix;
};

struct _WSDXML_ATTRIBUTE
{
    WSDXML_ELEMENT* Element;
    WSDXML_ATTRIBUTE* Next;
    WSDXML_NAME* Name;
    WCHAR* Value;
};

struct _WSDXML_NODE
{
    enum
    {   
        ElementType,
        TextType
    } Type;
    WSDXML_ELEMENT* Parent;
    WSDXML_NODE* Next;
};

struct _WSDXML_ELEMENT
{
    WSDXML_NODE Node;
    WSDXML_NAME* Name;
    WSDXML_ATTRIBUTE* FirstAttribute;
    WSDXML_NODE* FirstChild;
    WSDXML_PREFIX_MAPPING* PrefixMappings;
};

struct _WSDXML_TEXT
{
	WSDXML_NODE Node;
    WCHAR* Text;
};

struct _WSDXML_ELEMENT_LIST
{
    WSDXML_ELEMENT_LIST* Next;
    WSDXML_ELEMENT* Element;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //  __WSDXMLDOM_H__

