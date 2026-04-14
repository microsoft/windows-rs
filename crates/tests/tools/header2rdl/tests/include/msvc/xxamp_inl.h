/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* xxamp_inl.h
*
* C++ AMP Library helper classes implementations.
*
* This file contains the bodies of methods declared in xxamp which rely on
* amp.h class definitions.
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#ifndef _SILENCE_AMP_DEPRECATION_WARNINGS
#error <xxamp_inl.h> is part of C++ AMP which is deprecated by Microsoft and will be REMOVED. \
You can define _AMP_SILENCE_DEPRECATION_WARNINGS to acknowledge that you have received this warning.
#endif // _SILENCE_AMP_DEPRECATION_WARNINGS

#include <xxamp.h>


namespace Concurrency
{
namespace details
{

// Projection Helpers

template <typename _T, int _R>
/*static*/ inline typename _Projection_result_type<_T,_R>::_Const_result_type
_Const_array_view_projection_helper<_T,_R>::_Project0(const array_view<const _T,_R>* _Arr_view, int _I) __GPU
{
    typename _Projection_result_type<_T,_R>::_Const_result_type _Projected_view;
    _Arr_view->_Project0(_I, _Projected_view);
    return _Projected_view;
}

template <typename _T, int _R>
/*static*/ inline typename _Projection_result_type<_T,_R>::_Result_type
_Array_view_projection_helper<_T,_R>::_Project0(const array_view<_T,_R>* _Arr_view, int _I) __GPU
{
    typename _Projection_result_type<_T,_R>::_Result_type _Projected_view;
    _Arr_view->_Project0(_I, _Projected_view);
    return _Projected_view;
}

template <typename _T>
/*static*/ inline typename _Projection_result_type<_T,1>::_Const_result_type
_Const_array_view_projection_helper<_T,1>::_Project0(const array_view<const _T,1>* _Arr_view, int _I) __GPU
{
    return _Arr_view->operator[](index<1>(_I));
}

template <typename _T>
/*static*/ inline typename _Projection_result_type<_T,1>::_Result_type
_Array_view_projection_helper<_T,1>::_Project0(const array_view<_T,1>* _Arr_view, int _I) __GPU
{
    return _Arr_view->operator[](index<1>(_I));
}

template <typename _T, int _R>
/*static*/ inline typename _Projection_result_type<_T,_R>::_Const_result_type
_Const_array_projection_helper<_T,_R>::_Project0(const array<_T, _R>* _Array, int _I) __GPU
{
    array_view<const _T,_R> _Temp(*_Array);
    return _Const_array_view_projection_helper<_T,_R>::_Project0(&_Temp, _I);
}

template <typename _T, int _R>
/*static*/ inline typename _Projection_result_type<_T,_R>::_Result_type
_Array_projection_helper<_T,_R>::_Project0(_In_ array<_T, _R>* _Array, int _I) __GPU
{
    array_view<_T,_R> _Temp(*_Array);
    return _Array_view_projection_helper<_T,_R>::_Project0(&_Temp, _I);
}

template <typename _T>
/*static*/ inline typename _Projection_result_type<_T,1>::_Const_result_type
_Const_array_projection_helper<_T,1>::_Project0(const array<_T,1>* _Array, int _I) __GPU
{
    return _Array->operator[](index<1>(_I));
}

template <typename _T>
/*static*/ inline typename _Projection_result_type<_T,1>::_Result_type
_Array_projection_helper<_T,1>::_Project0(_In_ array<_T,1>* _Array, int _I) __GPU
{
    return _Array->operator[](index<1>(_I));
}

// Calculate the extent dimensions at selected mipmap level given base extent
template<int _Rank>
inline extent<_Rank> _Get_extent_at_level_unsafe(const extent<_Rank> &_Base_extent, unsigned int _Level) __GPU
{
    static_assert(_Rank >= 3, "_Rank >= 3");
}

template<>
inline extent<1> _Get_extent_at_level_unsafe<1>(const extent<1> &_Base_extent, unsigned int _Level) __GPU
{
    extent<1> _Extent_at_level(_Base_extent);

    _Extent_at_level[0] >>= _Level;
    _Extent_at_level[0] = _Extent_at_level[0] ? _Extent_at_level[0] : 1;

    return _Extent_at_level;
}

template<>
inline extent<2> _Get_extent_at_level_unsafe<2>(const extent<2> &_Base_extent, unsigned int _Level) __GPU
{
    extent<2> _Extent_at_level(_Base_extent);

    _Extent_at_level[0] >>= _Level;
    _Extent_at_level[1] >>= _Level;
    _Extent_at_level[0] = _Extent_at_level[0] ? _Extent_at_level[0] : 1;
    _Extent_at_level[1] = _Extent_at_level[1] ? _Extent_at_level[1] : 1;

    return _Extent_at_level;
}

template<>
inline extent<3> _Get_extent_at_level_unsafe<3>(const extent<3> &_Base_extent, unsigned int _Level) __GPU
{
    extent<3> _Extent_at_level(_Base_extent);

    _Extent_at_level[0] >>= _Level;
    _Extent_at_level[1] >>= _Level;
    _Extent_at_level[2] >>= _Level;
    _Extent_at_level[0] = _Extent_at_level[0] ? _Extent_at_level[0] : 1;
    _Extent_at_level[1] = _Extent_at_level[1] ? _Extent_at_level[1] : 1;
    _Extent_at_level[2] = _Extent_at_level[2] ? _Extent_at_level[2] : 1;

    return _Extent_at_level;
}

// Calculate the extent dimensions at selected mipmap level given base extent
template<int _Rank>
inline extent<_Rank> _Get_extent_at_level(const extent<_Rank> &_Base_extent, unsigned int _Level)
{
    _Are_valid_mipmap_parameters(_Level);
    return _Get_extent_at_level_unsafe(_Base_extent, _Level);
}

} // namespace details
} // namespace concurrency
