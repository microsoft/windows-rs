// Copyright (c) Microsoft Corp.
//
//  Definitions used for WinRT True Async Runtime Type Information
#pragma once
#include <wtypesbase.h>

typedef enum WinrtRttiKind 
{ 
    WinrtRttiKind_Namespace = 0, 
    WinrtRttiKind_RuntimeClass, 
    WinrtRttiKind_Interface, 
    WinrtRttiKind_Delegate, 
    WinrtRttiKind_ParameterizedInterface, 
    WinrtRttiKind_ParameterizedInterfaceInstance, 
    WinrtRttiKind_ParameterizedDelegate, 
    WinrtRttiKind_ParameterizedDelegateInstance, 
} WinrtRttiKind; 

#define RTTI_VERSION_1      1

typedef struct WinrtRtti 
{ 
    WinrtRttiKind Kind; 
    ULONG Version; 
    const struct WinrtRtti *Namespace; 
    PCWSTR Name; 
    MIDL_METHOD_PROPERTY_MAP PropertyMap; 
} WinrtRtti; 


#pragma push_macro("STRUCT_WITH_BASE_BEGIN") 
#pragma push_macro("STRUCT_WITH_BASE_END") 
#undef STRUCT_WITH_BASE_BEGIN 
#undef STRUCT_WITH_BASE_END 

#ifdef __cplusplus 
#define STRUCT_WITH_BASE_BEGIN(Struct, Base) struct Struct : public Base { 
#define STRUCT_WITH_BASE_END(Struct) }; 
#else 
#define STRUCT_WITH_BASE_BEGIN(Struct, Base) typedef struct Struct { Base _Base; 
#define STRUCT_WITH_BASE_END(Struct) } Struct; 
#endif 


STRUCT_WITH_BASE_BEGIN(WinrtRttiNamespace_V1, WinrtRtti) 
// Empty 
STRUCT_WITH_BASE_END(WinrtRttiNamespace_V1) 
typedef WinrtRttiNamespace_V1 WinrtRttiNamespace; 

STRUCT_WITH_BASE_BEGIN(WinrtRttiRuntimeClass_V1, WinrtRtti) 
const WinrtRtti *DefaultInterface; 
STRUCT_WITH_BASE_END(WinrtRttiRuntimeClass_V1) 
typedef WinrtRttiRuntimeClass_V1 WinrtRttiRuntimeClass; 


STRUCT_WITH_BASE_BEGIN(WinrtRttiInterface_V1, WinrtRtti) 
const IID *Iid; 
STRUCT_WITH_BASE_END(WinrtRttiInterface_V1) 
typedef WinrtRttiInterface_V1 WinrtRttiInterface; 


STRUCT_WITH_BASE_BEGIN(WinrtRttiDelegate_V1, WinrtRtti) 
const IID *Iid; 
STRUCT_WITH_BASE_END(WinrtRttiDelegate_V1) 
typedef WinrtRttiDelegate_V1 WinrtRttiDelegate; 


STRUCT_WITH_BASE_BEGIN(WinrtRttiParameterizedInterface_V1, WinrtRtti) 
// Empty 
STRUCT_WITH_BASE_END(WinrtRttiParameterizedInterface_V1) 
typedef WinrtRttiParameterizedInterface_V1 WinrtRttiParameterizedInterface; 


STRUCT_WITH_BASE_BEGIN(WinrtRttiParameterizedInterfaceInstance_V1, WinrtRtti) 
const IID *Iid; 
const WinrtRtti *ParameterizedInterface; 
STRUCT_WITH_BASE_END(WinrtRttiParameterizedInterfaceInstance_V1) 
typedef WinrtRttiParameterizedInterfaceInstance_V1 WinrtRttiParameterizedInterfaceInstance; 


STRUCT_WITH_BASE_BEGIN(WinrtRttiParameterizedDelegate_V1, WinrtRtti) 
// Empty 
STRUCT_WITH_BASE_END(WinrtRttiParameterizedDelegate_V1) 
typedef WinrtRttiParameterizedDelegate_V1 WinrtRttiParameterizedDelegate; 

STRUCT_WITH_BASE_BEGIN(WinrtRttiParameterizedDelegateInstance_V1, WinrtRtti) 
const IID *Iid; 
const WinrtRtti *ParameterizedDelegate; 
STRUCT_WITH_BASE_END(WinrtRttiParameterizedDelegateInstance_V1) 
typedef WinrtRttiParameterizedDelegateInstance_V1 WinrtRttiParameterizedDelegateInstance; 

#pragma pop_macro("STRUCT_WITH_BASE_BEGIN") 
#pragma pop_macro("STRUCT_WITH_BASE_END") 



// Generic property ID constants 


// 0x0000xxxx: General properties 
#define WinrtRttiPropertyId_InterfaceInterceptionInfo           0x00000000

// 0x0001xxxx: Async pattern-specific properties 
#define WinrtRttiPropertyId_AsyncCompletedHandlerRtti           0x00010000
#define WinrtRttiPropertyId_AsyncProgressHandlerRtti            0x00010001
#define WinrtRttiPropertyId_AsyncTResultSerializationInfo       0x00010002
#define WinrtRttiPropertyId_AsyncTProgressSerializationInfo     0x00010003


// MIDL method property ID constants 


// 0x0001xxxx: Winrt async pattern-specific properties 

static const ULONG MidlMethodPropertyId_WinrtAsyncMethodName = 0x00010000; 
static const ULONG MidlMethodPropertyId_WinrtAsyncMethodOutRetvalParameterTypeInfo = 0x00010001; 


// Type names (for recognition by the channel) 

extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_NamespaceName[] = L"Windows"; 
extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Foundation_NamespaceName[] = L"Foundation"; 

extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_IAsyncAction[] = L"IAsyncAction"; 
extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_AsyncActionCompletedHandler[] = L"AsyncActionCompletedHandler"; 

extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_IAsyncOperation[] = L"IAsyncOperation`1"; 
extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_AsyncOperationCompletedHandler[] = L"AsyncOperationCompletedHandler`1"; 

extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_IAsyncOperationWithProgress[] = L"IAsyncOperationWithProgress`2"; 
extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_AsyncOperationProgressHandler[] = L"AsyncOperationProgressHandler`2"; 
extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_AsyncOperationWithProgressCompletedHandler[] = L"AsyncOperationWithProgressCompletedHandler`2"; 

extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_IAsyncActionWithProgress[] = L"IAsyncActionWithProgress`1"; 
extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_AsyncActionProgressHandler[] = L"AsyncActionProgressHandler`1"; 
extern const __declspec(selectany) _Null_terminated_ WCHAR RttiTypeName_Windows_Foundation_AsyncActionWithProgressCompletedHandler[] = L"AsyncActionWithProgressCompletedHandler`1"; 


// The RTTI information is for consumption by C only (it won't compile on C++ due to the C++
// definition of the STRUCT_WITH_BASE macros). So hide the declarations of the RTTI information
// from C++ code.
#if !defined(__cplusplus)

//  Namespace definition for "Windows" and Windows.Foundation namespaces.
__declspec(selectany) WinrtRttiNamespace const __x_Windows_Rtti =
{
    WinrtRttiKind_Namespace,
    RTTI_VERSION_1,
    NULL,
    RttiTypeName_Windows_NamespaceName,
    { 0, NULL }
};

__declspec(selectany) WinrtRttiNamespace const __x_Windows_CFoundation_Rtti =
{
    WinrtRttiKind_Namespace,
    RTTI_VERSION_1,
    &__x_Windows_Rtti._Base,
    RttiTypeName_Foundation_NamespaceName,
    { 0, NULL }
};

// Define the IIDs for IAsyncActionCompletedHandler and IAsyncAction inline so they are not a new link
// dependency for proxy/stub dlls that include RTTI. Unlike the MIDL-generated versions these are declared
// with __declspec(selectany), so use different symbol names to avoid a collision.
EXTERN_GUID(AlternateIID___x_Windows_CFoundation_CIAsyncActionCompletedHandler,
    0xa4ed5c81, 0x76c9, 0x40bd, 0x8b, 0xe6, 0xb1, 0xd9, 0x0f, 0xb2, 0x0a, 0xe7);

EXTERN_GUID(AlternateIID___x_Windows_CFoundation_CIAsyncAction,
    0x5a648006, 0x843a, 0x4da9, 0x86, 0x5b, 0x9d, 0x26, 0xe5, 0xdf, 0xad, 0x7b);

//  RTTI declaration for Windows.Foundation.IAsyncActionCompletedHandler interface
__declspec(selectany) WinrtRttiDelegate const __x_Windows_CFoundation_CAsyncActionCompletedHandler_Rtti =
{
    WinrtRttiKind_Delegate,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_AsyncActionCompletedHandler,
    { 0, NULL },
    &AlternateIID___x_Windows_CFoundation_CIAsyncActionCompletedHandler
};

//  IAsyncAction
//  RTTI declaration for Windows.Foundation.IAsyncAction interface
__declspec(selectany) MIDL_METHOD_PROPERTY const __x_Windows_CFoundation_CIAsyncAction_Rtti_Properties[] =
{
    {WinrtRttiPropertyId_AsyncCompletedHandlerRtti, (ULONG_PTR)&__x_Windows_CFoundation_CAsyncActionCompletedHandler_Rtti}
};

extern const IID IID___x_Windows_CFoundation_CIAsyncAction;
__declspec(selectany) WinrtRttiInterface const __x_Windows_CFoundation_CIAsyncAction_Rtti =
{
    WinrtRttiKind_Interface,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_IAsyncAction,
    { 1, __x_Windows_CFoundation_CIAsyncAction_Rtti_Properties },
    &AlternateIID___x_Windows_CFoundation_CIAsyncAction
};


//  IAsyncOperation
//  RTTI declaration for Windows.Foundation.IAsyncOperation`1 interface.
__declspec(selectany) WinrtRttiParameterizedInterface const __x_Windows_CFoundation_CIAsyncOperation_Rtti =
{
    WinrtRttiKind_ParameterizedInterface,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_IAsyncOperation,
    { 0, NULL },
};

//  RTTI declaration for Windows.Foundation.AsyncOperationCompletedHandler
__declspec(selectany) WinrtRttiParameterizedDelegate const __x_Windows_CFoundation_CAsyncOperationCompletedHandler_Rtti =
{
    WinrtRttiKind_ParameterizedDelegate,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_AsyncOperationCompletedHandler,
    { 0, NULL },
};


// IAsyncOperationWithProgress
//  RTTI declaration for Windows.Foundation.IAsyncOperationWithProgress`2 interface.
__declspec(selectany) WinrtRttiParameterizedInterface const __x_Windows_CFoundation_CIAsyncOperationWithProgress_Rtti =
{
    WinrtRttiKind_ParameterizedInterface,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_IAsyncOperationWithProgress,
    { 0, NULL },
};

//  RTTI declaration for Windows.Foundation.AsyncOperationProgressHandler`2 interface.
__declspec(selectany) WinrtRttiParameterizedDelegate const __x_Windows_CFoundation_CAsyncOperationProgressHandler_Rtti =
{
    WinrtRttiKind_ParameterizedDelegate,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_AsyncOperationProgressHandler,
    { 0, NULL },
};

//  RTTI declaration for Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2 interface.
__declspec(selectany) WinrtRttiParameterizedDelegate const __x_Windows_CFoundation_CAsyncOperationWithProgressCompletedHandler_Rtti =
{
    WinrtRttiKind_ParameterizedDelegate,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_AsyncOperationWithProgressCompletedHandler,
    { 0, NULL },
};

//  IAsyncActionWithProgress
//  RTTI declaration for Windows.Foundation.IAsyncActionWithProgress`1 interface.
__declspec(selectany) WinrtRttiParameterizedInterface const __x_Windows_CFoundation_CIAsyncActionWithProgress_Rtti =
{
    WinrtRttiKind_ParameterizedInterface,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_IAsyncActionWithProgress,
    { 0, NULL },
};

//  RTTI declaration for Windows.Foundation.AsyncActionProgressHandler`1 interface.
__declspec(selectany) WinrtRttiParameterizedDelegate const __x_Windows_CFoundation_CAsyncActionProgressHandler_Rtti =
{
    WinrtRttiKind_ParameterizedDelegate,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_AsyncActionProgressHandler,
    { 0, NULL },
};

//  RTTI declaration for Windows.Foundation.IAsyncActionProgressHandler`1 interface.
__declspec(selectany) WinrtRttiParameterizedDelegate const __x_Windows_CFoundation_CAsyncActionWithProgressCompletedHandler_Rtti =
{
    WinrtRttiKind_ParameterizedDelegate,
    RTTI_VERSION_1,
    &__x_Windows_CFoundation_Rtti._Base,
    RttiTypeName_Windows_Foundation_AsyncActionWithProgressCompletedHandler,
    { 0, NULL },
};

#define __x_ABI_CWindows_CFoundation_CAsyncActionCompletedHandler_Rtti __x_Windows_CFoundation_CAsyncActionCompletedHandler_Rtti
#define __x_ABI_CWindows_CFoundation_CIAsyncAction_Rtti __x_Windows_CFoundation_CIAsyncAction_Rtti
#define __x_ABI_CWindows_CFoundation_CIAsyncOperation_Rtti __x_Windows_CFoundation_CIAsyncOperation_Rtti
#define __x_ABI_CWindows_CFoundation_CAsyncOperationCompletedHandler_Rtti __x_Windows_CFoundation_CAsyncOperationCompletedHandler_Rtti
#define __x_ABI_CWindows_CFoundation_CIAsyncOperationWithProgress_Rtti __x_Windows_CFoundation_CIAsyncOperationWithProgress_Rtti
#define __x_ABI_CWindows_CFoundation_CAsyncOperationProgressHandler_Rtti __x_Windows_CFoundation_CAsyncOperationProgressHandler_Rtti
#define __x_ABI_CWindows_CFoundation_CAsyncOperationWithProgressCompletedHandler_Rtti __x_Windows_CFoundation_CAsyncOperationWithProgressCompletedHandler_Rtti
#define __x_ABI_CWindows_CFoundation_CIAsyncActionWithProgress_Rtti __x_Windows_CFoundation_CIAsyncActionWithProgress_Rtti
#define __x_ABI_CWindows_CFoundation_CAsyncActionProgressHandler_Rtti __x_Windows_CFoundation_CAsyncActionProgressHandler_Rtti
#define __x_ABI_CWindows_CFoundation_CAsyncActionWithProgressCompletedHandler_Rtti __x_Windows_CFoundation_CAsyncActionWithProgressCompletedHandler_Rtti

#endif // defined(__cplusplus)
