//
// vcruntime_new_debug.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// Declarations and definitions of the debug operators new and delete.
//
#pragma once

#include <vcruntime_new.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

#ifdef __cplusplus
extern "C++" {

#pragma pack(push, _CRT_PACKING)

#pragma push_macro("new")
#undef new

#ifndef _MFC_OVERRIDES_NEW

    _NODISCARD _Check_return_ _Ret_notnull_ _Post_writable_byte_size_(_Size)
    _VCRT_ALLOCATOR void* __CRTDECL operator new(
        _In_   size_t      _Size,
        _In_   int         _BlockUse,
        _In_z_ char const* _FileName,
        _In_   int         _LineNumber
        );

    _NODISCARD _Check_return_ _Ret_notnull_ _Post_writable_byte_size_(_Size)
    _VCRT_ALLOCATOR void* __CRTDECL operator new[](
        _In_   size_t      _Size,
        _In_   int         _BlockUse,
        _In_z_ char const* _FileName,
        _In_   int         _LineNumber
        );

    void __CRTDECL operator delete(
        void*       _Block,
        int         _BlockUse,
        char const* _FileName,
        int         _LineNumber
        ) noexcept;

    void __CRTDECL operator delete[](
        void*       _Block,
        int         _BlockUse,
        char const* _FileName,
        int         _LineNumber
        ) noexcept;

#endif

#pragma pop_macro("new")

#pragma pack(pop)

} // extern "C++"
#endif // __cplusplus

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
