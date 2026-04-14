// __msvc_heap_algorithms.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_HEAP_ALGORITHMS_HPP
#define __MSVC_HEAP_ALGORITHMS_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#include <xutility>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
template <class _RanIt, class _Ty, class _Pr>
_CONSTEXPR20 void _Push_heap_by_index(
    _RanIt _First, _Iter_diff_t<_RanIt> _Hole, _Iter_diff_t<_RanIt> _Top, _Ty&& _Val, _Pr _Pred) {
    // percolate _Hole to _Top or where _Val belongs
    using _Diff = _Iter_diff_t<_RanIt>;
    for (_Diff _Idx                                                         = (_Hole - 1) >> 1; // shift for codegen
        _Top < _Hole && _DEBUG_LT_PRED(_Pred, *(_First + _Idx), _Val); _Idx = (_Hole - 1) >> 1) { // shift for codegen
        // move _Hole up to parent
        *(_First + _Hole) = _STD move(*(_First + _Idx));
        _Hole             = _Idx;
    }

    *(_First + _Hole) = _STD forward<_Ty>(_Val); // drop _Val into final hole
}

_EXPORT_STD template <class _RanIt, class _Pr>
_CONSTEXPR20 void push_heap(_RanIt _First, _RanIt _Last, _Pr _Pred) {
    // push *(_Last - 1) onto heap at [_First, _Last - 1)
    _STD _Adl_verify_range(_First, _Last);
    const auto _UFirst = _STD _Get_unwrapped(_First);
    auto _ULast        = _STD _Get_unwrapped(_Last);
    using _Diff        = _Iter_diff_t<_RanIt>;
    _Diff _Count       = _ULast - _UFirst;
    if (2 <= _Count) {
        _Iter_value_t<_RanIt> _Val(_STD move(*--_ULast));
        _STD _Push_heap_by_index(_UFirst, --_Count, _Diff(0), _STD move(_Val), _STD _Pass_fn(_Pred));
    }
}

_EXPORT_STD template <class _RanIt>
_CONSTEXPR20 void push_heap(_RanIt _First, _RanIt _Last) {
    // push *(_Last - 1) onto heap at [_First, _Last - 1)
    _STD push_heap(_First, _Last, less<>{});
}

template <class _RanIt, class _Ty, class _Pr>
_CONSTEXPR20 void _Pop_heap_hole_by_index(
    _RanIt _First, _Iter_diff_t<_RanIt> _Hole, _Iter_diff_t<_RanIt> _Bottom, _Ty&& _Val, _Pr _Pred) {
    // percolate _Hole to _Bottom, then push _Val
    _STL_INTERNAL_CHECK(_Bottom > 0);

    using _Diff      = _Iter_diff_t<_RanIt>;
    const _Diff _Top = _Hole;
    _Diff _Idx       = _Hole;

    // Check whether _Idx can have a child before calculating that child's index, since
    // calculating the child's index can trigger integer overflows
    const _Diff _Max_sequence_non_leaf = (_Bottom - 1) >> 1; // shift for codegen
    while (_Idx < _Max_sequence_non_leaf) { // move _Hole down to larger child
        _Idx = 2 * _Idx + 2;
        if (_DEBUG_LT_PRED(_Pred, *(_First + _Idx), *(_First + (_Idx - 1)))) {
            --_Idx;
        }
        *(_First + _Hole) = _STD move(*(_First + _Idx));
        _Hole             = _Idx;
    }

    if (_Idx == _Max_sequence_non_leaf && _Bottom % 2 == 0) { // only child at bottom, move _Hole down to it
        *(_First + _Hole) = _STD move(*(_First + (_Bottom - 1)));
        _Hole             = _Bottom - 1;
    }

    _STD _Push_heap_by_index(_First, _Hole, _Top, _STD forward<_Ty>(_Val), _Pred);
}

template <class _RanIt, class _Ty, class _Pr>
_CONSTEXPR20 void _Pop_heap_hole_unchecked(_RanIt _First, _RanIt _Last, _RanIt _Dest, _Ty&& _Val, _Pr _Pred) {
    // pop *_First to *_Dest and reheap
    // precondition: _First != _Last
    // precondition: _First != _Dest
    *_Dest      = _STD move(*_First);
    using _Diff = _Iter_diff_t<_RanIt>;
    _STD _Pop_heap_hole_by_index(
        _First, static_cast<_Diff>(0), static_cast<_Diff>(_Last - _First), _STD forward<_Ty>(_Val), _Pred);
}

template <class _RanIt, class _Pr>
_CONSTEXPR20 void _Pop_heap_unchecked(_RanIt _First, _RanIt _Last, _Pr _Pred) {
    // pop *_First to *(_Last - 1) and reheap
    if (2 <= _Last - _First) {
        --_Last;
        _Iter_value_t<_RanIt> _Val(_STD move(*_Last));
        _STD _Pop_heap_hole_unchecked(_First, _Last, _Last, _STD move(_Val), _Pred);
    }
}

_EXPORT_STD template <class _RanIt, class _Pr>
_CONSTEXPR20 void pop_heap(_RanIt _First, _RanIt _Last, _Pr _Pred) {
    // pop *_First to *(_Last - 1) and reheap
    _STD _Adl_verify_range(_First, _Last);
    _STD _Pop_heap_unchecked(_STD _Get_unwrapped(_First), _STD _Get_unwrapped(_Last), _STD _Pass_fn(_Pred));
}

_EXPORT_STD template <class _RanIt>
_CONSTEXPR20 void pop_heap(_RanIt _First, _RanIt _Last) {
    // pop *_First to *(_Last - 1) and reheap
    _STD pop_heap(_First, _Last, less<>{});
}

template <class _RanIt, class _Pr>
_CONSTEXPR20 void _Make_heap_unchecked(_RanIt _First, _RanIt _Last, _Pr _Pred) {
    // make [_First, _Last) into a heap
    using _Diff   = _Iter_diff_t<_RanIt>;
    _Diff _Bottom = _Last - _First;
    for (_Diff _Hole = _Bottom >> 1; _Hole > 0;) { // shift for codegen
        // reheap top half, bottom to top
        --_Hole;
        _Iter_value_t<_RanIt> _Val(_STD move(*(_First + _Hole)));
        _STD _Pop_heap_hole_by_index(_First, _Hole, _Bottom, _STD move(_Val), _Pred);
    }
}

_EXPORT_STD template <class _RanIt, class _Pr>
_CONSTEXPR20 void make_heap(_RanIt _First, _RanIt _Last, _Pr _Pred) { // make [_First, _Last) into a heap
    _STD _Adl_verify_range(_First, _Last);
    _STD _Make_heap_unchecked(_STD _Get_unwrapped(_First), _STD _Get_unwrapped(_Last), _STD _Pass_fn(_Pred));
}

_EXPORT_STD template <class _RanIt>
_CONSTEXPR20 void make_heap(_RanIt _First, _RanIt _Last) { // make [_First, _Last) into a heap
    _STD make_heap(_First, _Last, less<>{});
}
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_HEAP_ALGORITHMS_HPP
