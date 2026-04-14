/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* amp_graphics.h
*
* C++ AMP Graphics Library
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#ifndef _SILENCE_AMP_DEPRECATION_WARNINGS
#error <amp_graphics.h> is part of C++ AMP which is deprecated by Microsoft and will be REMOVED. \
You can define _SILENCE_AMP_DEPRECATION_WARNINGS to acknowledge that you have received this warning.
#endif // _SILENCE_AMP_DEPRECATION_WARNINGS

#include <amp_short_vectors.h>
#include <array>
#include <dxgiformat.h>
#include <sstream>

#define _AMP_GRAPHICS_H

#pragma warning( push )
#pragma warning( disable : 4127 ) // conditional expression is constant
#pragma warning( disable : 4996 ) // writeonly_texture_view is deprecated
#pragma warning( disable : 6326 ) // Potential comparison of a constant with another constant

namespace Concurrency
{

namespace graphics
{

namespace details
{


template<typename _Ty>
struct _Short_vector_type_traits
{
    typedef void _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = false;
    static const _Short_vector_base_type_id _Format_base_type_id = _Invalid_type;
    static const unsigned int _Num_channels = 0;
    static const unsigned int _Default_bits_per_channel = 0;
};

template<>
struct _Short_vector_type_traits<unsigned int>
{
    typedef unsigned int _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Uint_type;
    static const unsigned int _Num_channels = 1;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<uint_2>
{
    typedef uint_2::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Uint_type;
    static const unsigned int _Num_channels = 2;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<uint_3>
{
    typedef uint_3::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Uint_type;
    static const unsigned int _Num_channels = 3;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<uint_4>
{
    typedef uint_4::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
   static const _Short_vector_base_type_id _Format_base_type_id = _Uint_type;
    static const unsigned int _Num_channels = 4;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<int>
{
    typedef int _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
   static const _Short_vector_base_type_id _Format_base_type_id = _Int_type;
    static const unsigned int _Num_channels = 1;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<int_2>
{
    typedef int_2::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Int_type;
    static const unsigned int _Num_channels = 2;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<int_3>
{
    typedef int_3::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Int_type;
    static const unsigned int _Num_channels = 3;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<int_4>
{
    typedef int_4::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Int_type;
    static const unsigned int _Num_channels = 4;
    static const unsigned int _Default_bits_per_channel = 32;
};


template<>
struct _Short_vector_type_traits<float>
{
    typedef float _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Float_type;
    static const unsigned int _Num_channels = 1;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<float_2>
{
    typedef float_2::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Float_type;
    static const unsigned int _Num_channels = 2;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<float_3>
{
    typedef float_3::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Float_type;
    static const unsigned int _Num_channels = 3;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<float_4>
{
    typedef float_4::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Float_type;
    static const unsigned int _Num_channels = 4;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<unorm>
{
    typedef unorm _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Unorm_type;
    static const unsigned int _Num_channels = 1;
    static const unsigned int _Default_bits_per_channel = 16;
};

template<>
struct _Short_vector_type_traits<unorm_2>
{
    typedef unorm_2::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Unorm_type;
    static const unsigned int _Num_channels = 2;
    static const unsigned int _Default_bits_per_channel = 16;
};

template<>
struct _Short_vector_type_traits<unorm_3>
{
    typedef unorm_3::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = false;
    static const _Short_vector_base_type_id _Format_base_type_id = _Invalid_type;
    static const unsigned int _Num_channels = 0;
    static const unsigned int _Default_bits_per_channel = 0;
};

template<>
struct _Short_vector_type_traits<unorm_4>
{
    typedef unorm_4::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Unorm_type;
    static const unsigned int _Num_channels = 4;
    static const unsigned int _Default_bits_per_channel = 16;
};

template<>
struct _Short_vector_type_traits<norm>
{
    typedef norm _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Norm_type;
    static const unsigned int _Num_channels = 1;
    static const unsigned int _Default_bits_per_channel = 16;
};

template<>
struct _Short_vector_type_traits<norm_2>
{
    typedef norm_2::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Norm_type;
    static const unsigned int _Num_channels = 2;
    static const unsigned int _Default_bits_per_channel = 16;
};

template<>
struct _Short_vector_type_traits<norm_3>
{
    typedef norm_3::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = false;
    static const _Short_vector_base_type_id _Format_base_type_id = _Invalid_type;
    static const unsigned int _Num_channels = 0;
    static const unsigned int _Default_bits_per_channel = 0;
};

template<>
struct _Short_vector_type_traits<norm_4>
{
    typedef norm_4::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Norm_type;
    static const unsigned int _Num_channels = 4;
    static const unsigned int _Default_bits_per_channel = 16;
};


template<>
struct _Short_vector_type_traits<double>
{
    typedef double _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Double_type;
    static const unsigned int _Num_channels = 2;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<double_2>
{
    typedef double_2::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = true;
    static const _Short_vector_base_type_id _Format_base_type_id = _Double_type;
    static const unsigned int _Num_channels = 4;
    static const unsigned int _Default_bits_per_channel = 32;
};

template<>
struct _Short_vector_type_traits<double_3>
{
    typedef double_3::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = false;
    static const _Short_vector_base_type_id _Format_base_type_id = _Invalid_type;
    static const unsigned int _Num_channels = 0;
    static const unsigned int _Default_bits_per_channel = 0;
};

template<>
struct _Short_vector_type_traits<double_4>
{
    typedef double_4::value_type _Scalar_type;
    static const bool _Is_valid_SVT_for_texture = false;
    static const _Short_vector_base_type_id _Format_base_type_id = _Invalid_type;
    static const unsigned int _Num_channels = 0;
    static const unsigned int _Default_bits_per_channel = 0;
};

template<typename _Short_vector_type>
unsigned int _Get_default_bits_per_scalar_element()
{
    return _Short_vector_type_traits<_Short_vector_type>::_Format_base_type_id == _Double_type ?
            _Short_vector_type_traits<_Short_vector_type>::_Default_bits_per_channel * 2 :
            _Short_vector_type_traits<_Short_vector_type>::_Default_bits_per_channel;
}

template<int _Rank>
std::array<size_t, 3> _Get_dimensions(const Concurrency::extent<_Rank> & _Ext, unsigned int _Mip_offset)
{
    std::array<size_t, 3> _Arr;
    // For un-used dimensions, use value 1.
    switch((_Rank)) {
    case 1:
        _Arr[0] = static_cast<size_t>((_Ext[0] >> _Mip_offset) ? (_Ext[0] >> _Mip_offset) : 1U);
        _Arr[1] = 1;
        _Arr[2] = 1;
        break;
    case 2:
        _Arr[0] = static_cast<size_t>((_Ext[1] >> _Mip_offset) ? (_Ext[1] >> _Mip_offset) : 1U);
        _Arr[1] = static_cast<size_t>((_Ext[0] >> _Mip_offset) ? (_Ext[0] >> _Mip_offset) : 1U);
        _Arr[2] = 1;
        break;
    case 3:
        _Arr[0] = static_cast<size_t>((_Ext[2] >> _Mip_offset) ? (_Ext[2] >> _Mip_offset) : 1U);
        _Arr[1] = static_cast<size_t>((_Ext[1] >> _Mip_offset) ? (_Ext[1] >> _Mip_offset) : 1U);
        _Arr[2] = static_cast<size_t>((_Ext[0] >> _Mip_offset) ? (_Ext[0] >> _Mip_offset) : 1U);
        break;
    default:
        _ASSERTE(false);
        _Arr[0] = 1;
        _Arr[1] = 1;
        _Arr[2] = 1;
        break;
    }
    return _Arr;
}

template <int _Rank>
std::array<size_t, 3> _Get_indices(const index<_Rank> &_Idx)
{
    std::array<size_t, 3> _Arr;
        // For un-used dimensions, use value 0.
    switch((_Rank)) {
    case 1:
        _Arr[0] = static_cast<size_t>(_Idx[0]);
        _Arr[1] = 0;
        _Arr[2] = 0;
        break;
    case 2:
        _Arr[0] = static_cast<size_t>(_Idx[1]);
        _Arr[1] = static_cast<size_t>(_Idx[0]);
        _Arr[2] = 0;
        break;
    case 3:
        _Arr[0] = static_cast<size_t>(_Idx[2]);
        _Arr[1] = static_cast<size_t>(_Idx[1]);
        _Arr[2] = static_cast<size_t>(_Idx[0]);
        break;
    default:
        _ASSERTE(false);
        _Arr[0] = 0;
        _Arr[1] = 0;
        _Arr[2] = 0;
        break;
    }
    return _Arr;
}

template<int _Rank>
Concurrency::extent<_Rank> _Create_extent(size_t _Width, size_t _Height, size_t _Depth)
{
    extent<_Rank> _Ext;
    switch((_Rank)) {
    case 1:
        _Ext[0] = static_cast<int>(_Width);
        break;
    case 2:
        _Ext[0] = static_cast<int>(_Height);
        _Ext[1] = static_cast<int>(_Width);
        break;
    case 3:
        _Ext[0] = static_cast<int>(_Depth);
        _Ext[1] = static_cast<int>(_Height);
        _Ext[2] = static_cast<int>(_Width);
        break;
    default:
        _ASSERTE(false);
        break;
    }
    return _Ext;
}

// forward declaration
template <typename _Value_type, int _Rank> class _Texture_base;
template <typename _Value_type, int _Rank>
_Event _Copy_async_impl(const void * _Src, unsigned int _Src_byte_size, const _Texture_base<_Value_type, _Rank>& _Dst, const index<_Rank> &_Offset, const Concurrency::extent<_Rank> &_Copy_extent);
template <typename OutputIterator, typename _Value_type, int _Rank>
_Event _Copy_async_impl(const _Texture_base<_Value_type, _Rank> &_Src, OutputIterator _Dest_iter);

template<typename _Value_type, int _Rank>
_Event _Copy_async_impl(const _Texture_base<_Value_type, _Rank>& _Src, const index<_Rank> &_Src_offset,
                const _Texture_base<_Value_type, _Rank>& _Dst, const index<_Rank> &_Dst_offset, const extent<_Rank> &_Copy_extent);

// The base class for texture, writeonly_texture_view
template <typename _Value_type, int _Rank>
class _Texture_base
{
    static_assert(_Rank > 0 && _Rank <= 3, "texture is only supported for rank 1, 2, and 3.");
    static_assert(_Short_vector_type_traits<typename std::remove_const<_Value_type>::type>::_Is_valid_SVT_for_texture, "invalid value_type for a texture.");

    // Friends
    template<typename _T>
    friend const _Texture_descriptor& Concurrency::details::_Get_texture_descriptor(const _T& _Tex) __GPU;
    template<typename _T>
    friend _Ret_ _Texture* Concurrency::details::_Get_texture(const _T& _Tex) __CPU_ONLY;
    template<typename _Value_type, int _Rank>
    friend _Event _Copy_async_impl(const _Texture_base<_Value_type, _Rank>& _Src, const index<_Rank> &_Src_offset,
                           const _Texture_base<_Value_type, _Rank>& _Dst, const index<_Rank> &_Dst_offset, const extent<_Rank> &_Copy_extent) __CPU_ONLY;

public:
    static const int rank = _Rank;
    typedef _Value_type value_type;
    typedef typename _Short_vector_type_traits<_Value_type>::_Scalar_type scalar_type;

public:
    /// <summary>
    ///     Returns the extent that defines the shape of this texture or texture view.
    /// </summary>
    __declspec(property(get=get_extent)) Concurrency::extent<_Rank> extent;
    Concurrency::extent<_Rank> get_extent() const __GPU
    {
        return _M_extent;
    }

    /// <summary>
    ///     Returns the extent for specific mipmap level of this texture or texture view.
    /// </summary>
    /// <param>
    ///     Mipmap level for which extent should be calculated.
    /// </param>
    Concurrency::extent<_Rank> get_mipmap_extent(unsigned int _Mipmap_level) const __CPU_ONLY
    {
        if (_Mipmap_level >= this->get_mipmap_levels())
        {
            std::stringstream _Err_msg;
            _Err_msg << "Value for _Mipmap_level parameter (" << _Mipmap_level
            << ") cannot be greater than or equal to number of mipmap levels ("
            << this->get_mipmap_levels() << ") on the texture or texture view";

            throw runtime_exception(_Err_msg.str().c_str(), E_INVALIDARG);
        }
        return Concurrency::details::_Get_extent_at_level(_M_extent, _Mipmap_level);
    }

    /// <summary>
    ///     Returns the extent for specific mipmap level of this texture or texture view.
    /// </summary>
    /// <param>
    ///     Mipmap level for which extent should be calculated.
    /// </param>
    Concurrency::extent<_Rank> get_mipmap_extent(unsigned int _Mipmap_level) const __GPU_ONLY
    {
        return Concurrency::details::_Get_extent_at_level_unsafe(_M_extent, _Mipmap_level);
    }

    /// <summary>
    ///     Returns the accelerator_view where this texture or texture view is located.
    /// </summary>
    __declspec(property(get=get_accelerator_view)) Concurrency::accelerator_view accelerator_view;
    Concurrency::accelerator_view get_accelerator_view() const __CPU_ONLY
    {
        return _Get_texture()->_Get_access_on_accelerator_view();
    }

    /// <summary>
    ///     Returns the number of bits per scalar element
    /// </summary>
    __declspec(property(get=get_bits_per_scalar_element)) unsigned int bits_per_scalar_element;
    unsigned int get_bits_per_scalar_element() const __CPU_ONLY
    {
        unsigned int _Bits_per_channel = _Get_texture()->_Get_bits_per_channel();
        return _Short_vector_type_traits<_Value_type>::_Format_base_type_id == _Double_type ? _Bits_per_channel * (sizeof(double)/sizeof(int)) : _Bits_per_channel;
    }

    /// <summary>
    ///     Query how many mipmap levels are accessible by this texture (or texture view).
    /// </summary>
    /// <returns>
    ///     Returns number of mipmap levels accessible by this texture (or texture view).
    /// </returns>
    __declspec(property(get=get_mipmap_levels)) unsigned int mipmap_levels;
    unsigned int get_mipmap_levels() const __GPU
    {
        return _M_texture_descriptor._Get_view_mipmap_levels();
    }

    /// <summary>
    ///     Returns the physical data length (in bytes) that is required in order to represent
    ///     the texture on the host side with its native format.
    ///     If the texture contains multiple mipmap levels the value represents the sum of physical data length for each accessible mipmap level by this texture (or texture view).
    /// </summary>
    __declspec(property(get=get_data_length)) unsigned int data_length;
    unsigned int get_data_length() const __CPU_ONLY
    {
        return _Get_texture()->_Get_data_length(this->_Get_most_detailed_mipmap_level(), this->get_mipmap_levels());
    }

protected:
    // internal storage abstraction
    typedef Concurrency::details::_Texture_descriptor _Texture_descriptor;

    _Texture_base() __CPU_ONLY
    {
        // This default ctor is required to enable move ctor for a derived types,
        // empty _Texture_base is later initialized by move assignment operator
    }

    _Texture_base(const Concurrency::extent<_Rank>& _Ext, unsigned int _Mipmap_levels = 1) __CPU_ONLY
        : _M_extent(_Ext), _M_texture_descriptor(/*_Most_detailed_mipmap_level=*/0, _Mipmap_levels)
    {
        _Is_valid_extent(_M_extent);
        _Are_valid_mipmap_parameters(/*_Most_detailed_mipmap_level=*/0, _Mipmap_levels);

        // Validate if we can generate _Mipmap_levels number of mipmap levels given the dimensionality of the texture
        unsigned int _Max_mipmap_levels = _Get_max_mipmap_levels(_M_extent);
        if (_Mipmap_levels > _Max_mipmap_levels)
        {
            std::stringstream _Err_msg;
            _Err_msg << "The texture extent is too small to generate (" << _Mipmap_levels << ") mipmap levels, the maximum allowed is (" << _Max_mipmap_levels << ")";
            throw runtime_exception(_Err_msg.str().c_str(), E_INVALIDARG);
        }
        else if (_Mipmap_levels == 0)
        {
            // Generate full range of all mipmaps
            // e.g. 2D 10x2 texture would have: 10x2, 5x1, 2x1, 1x1 (4 mipmap levels)
            _Mipmap_levels = _Max_mipmap_levels;
        }
        _M_texture_descriptor._Set_view_mipmap_levels(_Mipmap_levels);
    }

    // shallow copy for texture_views
    _Texture_base(const _Texture_base & _Src) __GPU
        : _M_extent(_Src._M_extent), _M_texture_descriptor(_Src._M_texture_descriptor)
    {
    }

    // shallow copy for texture_views that redefine range of mipmaps
    _Texture_base(const _Texture_base & _Src, unsigned int _Most_detailed_mipmap_level, unsigned int _View_mipmap_levels) __CPU_ONLY
        : _M_extent(_Get_extent_at_level(_Src.extent, _Most_detailed_mipmap_level)), _M_texture_descriptor(_Src._M_texture_descriptor, _Src._Get_most_detailed_mipmap_level() + _Most_detailed_mipmap_level, _View_mipmap_levels)
    {
        Concurrency::details::_Is_valid_mipmap_range(_Src.get_mipmap_levels(), _Most_detailed_mipmap_level, _View_mipmap_levels);
    }

    // shallow copy for texture_views that in restrict(amp) context, the texture views can no longer redefine mipmap range,
    // but read-write texture view needs to flatten to single mipmap level when created over a texture with multiple mipmap levels.
    _Texture_base(const _Texture_base & _Src, bool _Flatten_mipmap_levels) __GPU_ONLY
        : _M_extent(_Src.extent), _M_texture_descriptor(_Src._M_texture_descriptor, /*_Most_detailed_mipmap_level=*/0, _Flatten_mipmap_levels ? /*_View_mipmap_levels=*/1 : _Src.get_mipmap_levels())
    {
    }

    // interop
    _Texture_base(const Concurrency::extent<_Rank>& _Ext, const _Texture_descriptor & _Desc) __CPU_ONLY
        : _M_extent(_Ext), _M_texture_descriptor(_Desc)
    {
        Concurrency::details::_Is_valid_extent(_M_extent);
    }

    void _Copy_to(const _Texture_base & _Dest) const __CPU_ONLY
    {
        if (!(*this == _Dest))
        {
            _ASSERTE(this->extent == _Dest.extent);
            details::_Copy_async_impl(*this, index<_Rank>(), _Dest, index<_Rank>(), _Dest.extent)._Get();
        }
    }

    bool operator==(const _Texture_base & _Other) const __CPU_ONLY
    {
        return _Other._M_extent == _M_extent && _Other._M_texture_descriptor == _M_texture_descriptor;
    }

    ~_Texture_base() __GPU
    {
    }

    _Ret_ _Texture* _Get_texture() const __CPU_ONLY
    {
        return _M_texture_descriptor._Get_texture_ptr();
    }

    unsigned int _Get_most_detailed_mipmap_level() const __GPU
    {
        return _M_texture_descriptor._Get_most_detailed_mipmap_level();
    }

    bool _Are_mipmap_levels_overlapping(const _Texture_base &_Other)  const __CPU_ONLY
    {
        return _M_texture_descriptor._Are_mipmap_levels_overlapping(&_Other._M_texture_descriptor);
    }

protected:
    Concurrency::extent<_Rank> _M_extent;
    _Texture_descriptor _M_texture_descriptor;
};

inline void _Is_valid_data_length(unsigned int _Num_elems, unsigned int _Bits_per_elem)
{
    unsigned long long _Bytes_per_elem = static_cast<unsigned long long>(_Bits_per_elem / 8U);
    unsigned long long _Total_bytes = static_cast<unsigned long long>(_Num_elems) * _Bytes_per_elem;
    if (_Total_bytes > static_cast<unsigned long long>(UINT_MAX))
    {
        throw runtime_exception("Invalid - texture data_length exceeds UINT_MAX", E_INVALIDARG);
    }
}

template<typename _Iterator>
struct _Is_iterator
{
    template<class _Uty> static auto _Fn(_Uty _Val, decltype(*_Val, ++_Val, 0)) -> std::true_type;
    template<class _Uty> static auto _Fn(_Uty _Val, ...) -> std::false_type;
    static constexpr bool value = decltype(_Fn(std::declval<_Iterator>(),0))::value;
};

} // namespace details


using Concurrency::graphics::details::_Short_vector_type_traits;

// forward declarations
template <typename _Value_type, int _Rank>
class texture;
template <typename _Value_type, int _Rank>
class writeonly_texture_view;
template <typename _Value_type, int _Rank>
class texture_view;
class sampler;

namespace direct3d
{
template<typename _Value_type, int _Rank>
texture<_Value_type, _Rank> make_texture(const Concurrency::accelerator_view &_Av, _In_ IUnknown *_D3D_texture, DXGI_FORMAT _View_format = DXGI_FORMAT_UNKNOWN) __CPU_ONLY;

sampler make_sampler(_In_ IUnknown *_D3D_sampler) __CPU_ONLY;
_Ret_ IUnknown * get_sampler(const Concurrency::accelerator_view &_Av, const sampler &_Sampler) __CPU_ONLY;

} // namespace direct3d

namespace details
{

template <typename T>
struct texture_traits
{
    static const bool is_texture = false;
    static const bool is_writable = false;
};

template <typename _Value_type, int _Rank>
struct texture_traits<texture<_Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = true;
};

template <typename _Value_type, int _Rank>
struct texture_traits<const texture<_Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = false;
};

template <typename _Value_type, int _Rank>
struct texture_traits<writeonly_texture_view<_Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = true;
};

template <typename _Value_type, int _Rank>
struct texture_traits<const writeonly_texture_view<_Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = true;
};

template <typename _Value_type, int _Rank>
struct texture_traits<texture_view<_Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = true;
};

template <typename _Value_type, int _Rank>
struct texture_traits<texture_view<const _Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = false;
};

template <typename _Value_type, int _Rank>
struct texture_traits<const texture_view<const _Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = false;
};

template <typename _Value_type, int _Rank>
struct texture_traits<const texture_view<_Value_type, _Rank>>
{
    static const bool is_texture = true;
    static const bool is_writable = true;
};

// The helper function used by ETW and copy functions to calculate number of bytes for the copy operation given input section
template <typename _Value_type, int _Rank>
unsigned int _Get_section_size(const _Texture_base<_Value_type, _Rank> &_Tex, const extent<_Rank> &_Extent)
{
    _Texture* _Tex_ptr = _Get_texture(_Tex);
    _Texture_descriptor _Tex_desc = _Get_texture_descriptor(_Tex);

    return _Tex_ptr->_Get_data_length(_Tex_desc._Get_most_detailed_mipmap_level(), _Tex_desc._Get_view_mipmap_levels(), _Get_dimensions(_Extent, /*Mip_offset=*/0).data());
}

template <typename _Input_iterator, typename _Value_type>
_Event _Copy_async_impl(_Input_iterator _First, _Input_iterator _Last,
                        _In_ _Texture *_Dst, const size_t *_Dst_offset, unsigned int _Dst_mipmap_level,
                        const size_t *_Copy_extent, const size_t *_Preferred_copy_chunk_extent = NULL)
{
    _ASSERTE(_Dst != nullptr);
    _ASSERTE(_Dst_offset != nullptr);
    _ASSERTE(_Copy_extent != nullptr);

    _ASSERTE((unsigned int)std::distance(_First, _Last) >= (_Copy_extent[0] * _Copy_extent[1] * _Copy_extent[2]));

    // The copy region should be within the bounds of the destination texture
    _ASSERTE((_Dst_offset[0] + _Copy_extent[0]) <= _Dst->_Get_width(_Dst_mipmap_level));
    _ASSERTE((_Dst_offset[1] + _Copy_extent[1]) <= _Dst->_Get_height(_Dst_mipmap_level));
    _ASSERTE((_Dst_offset[2] + _Copy_extent[2]) <= _Dst->_Get_depth(_Dst_mipmap_level));

    if ((sizeof(_Value_type) > sizeof(unsigned char)) && (_Dst->_Get_bits_per_element() != (8U * sizeof(_Value_type))))
    {
        throw runtime_exception("Iterator-based copy is not supported on textures where the size of the _Value_type is not equal to the texel size.", E_INVALIDARG);
    }

    // If the dest is accessible on the host we can perform the copy entirely on the host
    if (_Dst->_Get_host_ptr() != NULL)
    {
        // We have made sure that the three multiplications below won't cause integer overflow when creating the texture
        _ASSERTE(((_Dst->_Get_bits_per_element() * _Copy_extent[0]) % (8U * sizeof(_Value_type))) == 0);

        size_t _Row_size = (_Dst->_Get_bits_per_element() * _Copy_extent[0]) >> 3; // in bytes
        size_t _Depth_slice_size = _Row_size * _Copy_extent[1];

        size_t _Row_pitch = _Dst->_Get_row_pitch();
        size_t _Depth_pitch = _Dst->_Get_depth_pitch();
        _ASSERTE(_Row_pitch >= _Row_size);
        _ASSERTE(_Depth_pitch >= _Depth_slice_size);

        size_t _Dst_offset_in_bytes = ((_Dst_offset[0] * _Dst->_Get_bits_per_element()) >> 3) +
                                        (_Dst_offset[1] * _Row_pitch) + (_Dst_offset[2] * _Depth_pitch);

        unsigned char *_PDest = reinterpret_cast<unsigned char*>(_Dst->_Get_host_ptr()) + _Dst_offset_in_bytes;

        _Copy_data_on_host_src_iter(_Dst->_Get_rank(), _First, reinterpret_cast<_Value_type*>(_PDest),
                                    _Row_size / sizeof(_Value_type), _Copy_extent[1], _Copy_extent[2],
                                    _Row_pitch, _Depth_pitch, _Row_size / sizeof(_Value_type), _Depth_slice_size / sizeof(_Value_type));

        return _Event();
    }

    // The dest is not accessible on the host; we need to copy src to
    // a temporary staging texture and launch a copy from the staging texture
    // to the dest texture.
    _Event _Ev;

    // Determine the copy chunk extent
    std::array<size_t, 3> _Copy_chunk_extent;
    if (_Preferred_copy_chunk_extent != NULL)
    {
        std::copy(&_Preferred_copy_chunk_extent[0], &_Preferred_copy_chunk_extent[3], _Copy_chunk_extent.begin());
    }
    else
    {
        _Get_preferred_copy_chunk_extent(_Dst->_Get_rank(), _Copy_extent[0], _Copy_extent[1], _Copy_extent[2], _Dst->_Get_bits_per_element(), _Copy_chunk_extent.data());
    }

    std::array<size_t, 3> _Curr_copy_offset;
    std::copy(&_Dst_offset[0], &_Dst_offset[3], _Curr_copy_offset.begin());

    std::array<size_t, 3> _Remaining_copy_extent;
    std::copy(&_Copy_extent[0], &_Copy_extent[3], _Remaining_copy_extent.begin());

    bool _Truncated_copy = false;
    do
    {
        _Texture_ptr _Dst_staging_tex_ptr;
        std::array<size_t, 3> _Curr_copy_extent;
        _Truncated_copy = _Get_chunked_staging_texture(_Dst, _Copy_chunk_extent.data(), _Remaining_copy_extent.data(), _Curr_copy_extent.data(), &_Dst_staging_tex_ptr);


        // Now copy from the src pointer to the temp staging texture
        _Dst_staging_tex_ptr->_Map_buffer(_Write_access, true /* _Wait */);

        std::array<size_t, 3> _Dst_staging_tex_offset;
        _Dst_staging_tex_offset.fill(0);
        _Event _Temp_ev = _Copy_async_impl<_Input_iterator, _Value_type>(_First, _Last, _Dst_staging_tex_ptr,
                                                                         _Dst_staging_tex_offset.data(), /*_Dst_mipmap_level=*/0, _Curr_copy_extent.data(), _Copy_chunk_extent.data());

        // Now chain a copy from the temporary staging texture to the _Dst texture
        _Texture_ptr _Dst_tex_ptr = _Dst;
        _Temp_ev = _Temp_ev._Add_continuation(std::function<_Event()>([_Dst_staging_tex_ptr, _Dst_tex_ptr, _Curr_copy_extent,
                                                                       _Dst_staging_tex_offset, _Curr_copy_offset, _Dst_mipmap_level]() mutable -> _Event
        {
            return _Dst_staging_tex_ptr->_Copy_to_async(_Dst_tex_ptr, _Curr_copy_extent.data(), _Dst_staging_tex_offset.data(), _Curr_copy_offset.data(), /*_Src_mipmap_level=*/0, _Dst_mipmap_level);
        }));

        _Ev = _Ev._Add_event(_Temp_ev);

        // Now adjust the _Src and _Dst offsets for the remaining part of the copy
        if (_Truncated_copy)
        {
            // The offset only needs to be adjusted in the most significant dimension
            _Curr_copy_offset[_Dst->_Get_rank() - 1] += _Curr_copy_extent[_Dst->_Get_rank() - 1];
            std::advance(_First, (((_Curr_copy_extent[0] * _Dst->_Get_bits_per_element()) >> 3) / sizeof(_Value_type)) * _Curr_copy_extent[1] * _Curr_copy_extent[2]);
        }

    } while (_Truncated_copy);

    return _Ev;
}

template <typename _Output_iterator, typename _Value_type>
_Event _Copy_async_impl(_Texture *_Tex, const size_t *_Tex_offset, unsigned int _Src_mipmap_level, const size_t *_Copy_extent, _Output_iterator _First, const size_t *_Preferred_copy_chunk_extent = NULL)
{
    _ASSERTE(_Tex != nullptr);
    _ASSERTE(_Tex_offset != nullptr);
    _ASSERTE(_Copy_extent != nullptr);

    // The copy region should be within the bounds of the source texture
    _ASSERTE((_Tex_offset[0] + _Copy_extent[0]) <= _Tex->_Get_width(_Src_mipmap_level));
    _ASSERTE((_Tex_offset[1] + _Copy_extent[1]) <= _Tex->_Get_height(_Src_mipmap_level));
    _ASSERTE((_Tex_offset[2] + _Copy_extent[2]) <= _Tex->_Get_depth(_Src_mipmap_level));

    if ((sizeof(_Value_type) > sizeof(unsigned char)) && (_Tex->_Get_bits_per_element() != (8U * sizeof(_Value_type))))
    {
        throw runtime_exception("Iterator-based copy is not supported on textures where the size of the _Value_type is not equal to the texel size.", E_INVALIDARG);
    }

    // If the texture is available on the host then we can perform the copy entirely on the host
    if (_Tex->_Get_host_ptr() != nullptr)
    {
        // We have made sure that the three multiplications below won't cause integer overflow when creating the texture
        _ASSERTE(((_Tex->_Get_bits_per_element() * _Copy_extent[0]) % 8U) == 0);

        size_t _Row_size = (_Tex->_Get_bits_per_element() * _Copy_extent[0]) >> 3; // in bytes
        size_t _Depth_slice_size = _Row_size * _Copy_extent[1];

        size_t _Row_pitch = _Tex->_Get_row_pitch();
        size_t _Depth_pitch = _Tex->_Get_depth_pitch();
        _ASSERTE(_Row_pitch >= _Row_size);
        _ASSERTE(_Depth_pitch >= _Depth_slice_size);

        size_t _Tex_offset_in_bytes = ((_Tex_offset[0] * _Tex->_Get_bits_per_element()) >> 3) +
                                       (_Tex_offset[1] * _Row_pitch) + (_Tex_offset[2] * _Depth_pitch);

        unsigned char *_PTex = reinterpret_cast<unsigned char*>(_Tex->_Get_host_ptr()) + _Tex_offset_in_bytes;

        _Copy_data_on_host_dst_iter(_Tex->_Get_rank(), reinterpret_cast<_Value_type*>(_PTex), _First,
                                    _Row_size / sizeof(_Value_type), _Copy_extent[1], _Copy_extent[2],
                                    _Row_pitch, _Depth_pitch, _Row_size / sizeof(_Value_type), _Depth_slice_size / sizeof(_Value_type));

        return _Event();
    }

    // The texture is not accessible on the host; we need to copy to/from a staging
    // texture before the copy to the destination.  This is done in chunks, such that
    // we can concurrently copy from the source texture to a staging texture while
    // copying from a staging texture from a previous chunk to the destination.
    _Event _Ev;

    // Determine the copy chunk extent
    std::array<size_t, 3> _Copy_chunk_extent;
    if (_Preferred_copy_chunk_extent != nullptr)
    {
        std::copy(&_Preferred_copy_chunk_extent[0], &_Preferred_copy_chunk_extent[3], _Copy_chunk_extent.begin());
    }
    else
    {
        _Get_preferred_copy_chunk_extent(_Tex->_Get_rank(), _Copy_extent[0], _Copy_extent[1], _Copy_extent[2], _Tex->_Get_bits_per_element(), _Copy_chunk_extent.data());
    }

    std::array<size_t, 3> _Curr_copy_offset;
    std::copy(&_Tex_offset[0], &_Tex_offset[3], _Curr_copy_offset.begin());

    std::array<size_t, 3> _Remaining_copy_extent;
    std::copy(&_Copy_extent[0], &_Copy_extent[3], _Remaining_copy_extent.begin());

    bool _Truncated_copy = false;

    _Texture_ptr _Staging_tex_ptr;
    std::array<size_t, 3> _Curr_copy_extent;
    _Truncated_copy = _Get_chunked_staging_texture(_Tex, _Copy_chunk_extent.data(), _Remaining_copy_extent.data(), _Curr_copy_extent.data(), &_Staging_tex_ptr);

    // Now copy into the temp staging texture
    std::array<size_t, 3> _Staging_tex_offset;
    _Staging_tex_offset.fill(0);
    _Event _Temp_ev = _Copy_async_impl(_Tex, _Curr_copy_offset.data(), _Src_mipmap_level,
                                       _Staging_tex_ptr._Get_ptr(), _Staging_tex_offset.data(), /*_Dst_mipmap_level=*/0,
                                       _Curr_copy_extent.data(), _Copy_chunk_extent.data());
    _Ev = _Ev._Add_event(_Temp_ev);

    // If we have finished our copy, we just need to add a continuation to copy
    // from the temporary staging texture to the _Dst pointer
    if (!_Truncated_copy)
    {
        return _Ev._Add_continuation(std::function<_Event()>([_Staging_tex_ptr,
            _Curr_copy_extent, _Staging_tex_offset, _Copy_chunk_extent, _First]() mutable -> _Event
        {
            return _Copy_async_impl<_Output_iterator, _Value_type>(_Staging_tex_ptr, _Staging_tex_offset.data(), /*_Src_mipmap_level=*/0, _Curr_copy_extent.data(), _First, _Copy_chunk_extent.data());
        }));
    }
    else
    {
        // The copy was truncated. We need to recursively perform the rest of the copy
        _Texture_ptr _Tex_ptr = _Tex;
        _Curr_copy_offset[_Tex->_Get_rank() - 1] += _Curr_copy_extent[_Tex->_Get_rank() - 1];
        return _Ev._Add_continuation(std::function<_Event()>([_Staging_tex_ptr, _First, _Curr_copy_extent,
            _Staging_tex_offset, _Tex_ptr, _Curr_copy_offset, _Remaining_copy_extent, _Copy_chunk_extent, _Src_mipmap_level]() mutable -> _Event
        {
            // Initiate copying of the remaining portion
            _Output_iterator _New_dst_iter = _First;
            _Advance_output_iterator<decltype(_New_dst_iter), size_t>(_New_dst_iter, (((_Curr_copy_extent[0] * _Tex_ptr->_Get_bits_per_element()) >> 3) / sizeof(_Value_type)) * _Curr_copy_extent[1] * _Curr_copy_extent[2]);
            _Event _Ev1 = _Copy_async_impl<_Output_iterator, _Value_type>(_Tex_ptr, _Curr_copy_offset.data(), _Src_mipmap_level, _Remaining_copy_extent.data(), _New_dst_iter, _Copy_chunk_extent.data());

            // Now copy the data from the temp staging buffer to the _Dst pointer
            _Event _Ev2 = _Copy_async_impl<_Output_iterator, _Value_type>(_Staging_tex_ptr, _Staging_tex_offset.data(), /*_Src_mipmap_level=*/0, _Curr_copy_extent.data(), _First, _Copy_chunk_extent.data());

            return _Ev2._Add_event(_Ev1);
        }));
    }
}

template <typename _Value_type, int _Rank>
_Event _Copy_async_impl(const void * _Src, unsigned int _Src_byte_size, const _Texture_base<_Value_type, _Rank>& _Dst, const index<_Rank> &_Dst_offset, const extent<_Rank> &_Copy_extent)
{
    _Is_valid_section(_Dst.extent, _Dst_offset, _Copy_extent);

    if (_Dst.get_mipmap_levels() > 1)
    {
        throw runtime_exception("Invalid destination - multiple mipmap levels cannot be copied from source", E_INVALIDARG);
    }

    if (_Src_byte_size < _Get_section_size(_Dst, _Copy_extent))
    {
        if (_Dst.extent == _Copy_extent)
        {
            throw runtime_exception("Invalid _Src_byte_size argument. _Src_byte_size is smaller than the total size of _Dst.", E_INVALIDARG);
        }
        else
        {
            throw runtime_exception("Invalid _Src_byte_size argument. _Src_byte_size is smaller than the provided section of _Dst.", E_INVALIDARG);
        }
    }

    _Texture *_Dst_tex_ptr = _Get_texture(_Dst);
    std::array<size_t, 3> _Copy_extent_arr = _Get_dimensions(_Copy_extent, /*_Mip_offset=*/0);
    std::array<size_t, 3> _Dst_offset_arr = _Get_indices(_Dst_offset);
    auto _First = reinterpret_cast<const unsigned char*>(_Src);
    auto _Last = reinterpret_cast<const unsigned char*>(_Src) + _Src_byte_size;

    return _Copy_async_impl<decltype(_First), unsigned char>(_First, _Last, _Dst_tex_ptr, _Dst_offset_arr.data(), _Get_texture_descriptor(_Dst)._Get_most_detailed_mipmap_level(), _Copy_extent_arr.data());
}

template<typename _Value_type, int _Rank>
_Event _Copy_async_impl(const _Texture_base<_Value_type, _Rank>& _Src, const index<_Rank> &_Src_offset, const extent<_Rank> &_Copy_extent, _Out_ void * _Dst, unsigned int _Dst_byte_size)
{
    _Is_valid_section(_Src.extent, _Src_offset, _Copy_extent);

    if (_Src.get_mipmap_levels() > 1)
    {
        throw runtime_exception("Invalid source - multiple mipmap levels cannot be copied to destination", E_INVALIDARG);
    }

    if (_Get_section_size(_Src, _Copy_extent) > _Dst_byte_size)
    {
        if (_Src.extent == _Copy_extent)
        {
            throw runtime_exception("Invalid _Dst_byte_size argument. _Dst_byte_size is smaller than the size of _Src.", E_INVALIDARG);
        }
        else
        {
            throw runtime_exception("Invalid _Dst_byte_size argument. _Dst_byte_size is smaller than the provided section of _Src.", E_INVALIDARG);
        }
    }

    _Texture *_Src_tex_ptr = _Get_texture(_Src);
    std::array<size_t, 3> _Copy_extent_arr = _Get_dimensions(_Copy_extent, /*_Mip_offset=*/0);
    std::array<size_t, 3> _Src_offset_arr = _Get_indices(_Src_offset);

    auto _First = reinterpret_cast<unsigned char*>(_Dst);

    return _Copy_async_impl<decltype(_First), unsigned char>(_Src_tex_ptr, _Src_offset_arr.data(), _Get_texture_descriptor(_Src)._Get_most_detailed_mipmap_level(), _Copy_extent_arr.data(), _First);
}

template <typename _Output_iterator, typename _Value_type, int _Rank>
_Event _Copy_async_impl(const _Texture_base<_Value_type, _Rank> &_Src, const index<_Rank> &_Src_offset, const extent<_Rank> &_Copy_extent, _Output_iterator _Dest_iter)
{
    _Is_valid_section(_Src.extent, _Src_offset, _Copy_extent);

    if (_Src.get_mipmap_levels() > 1)
    {
        throw runtime_exception("Invalid source - multiple mipmap levels cannot be copied to destination", E_INVALIDARG);
    }

    _Texture *_Src_tex_ptr = _Get_texture(_Src);
    std::array<size_t, 3> _Copy_extent_arr = _Get_dimensions(_Copy_extent, /*_Mip_offset=*/0);
    std::array<size_t, 3> _Src_offset_arr = _Get_indices(_Src_offset);

    return _Copy_async_impl<_Output_iterator, _Value_type>(_Src_tex_ptr, _Src_offset_arr.data(), _Get_texture_descriptor(_Src)._Get_most_detailed_mipmap_level(), _Copy_extent_arr.data(), _Dest_iter);
}

template <typename _Input_iterator, typename _Value_type, int _Rank>
_Event _Copy_async_impl(_Input_iterator _First, _Input_iterator _Last, const _Texture_base<_Value_type, _Rank>& _Dst, const index<_Rank> &_Dst_offset, const extent<_Rank> &_Copy_extent)
{
    _Is_valid_section(_Dst.extent, _Dst_offset, _Copy_extent);
    if (static_cast<unsigned int>(std::distance(_First, _Last)) < _Copy_extent.size())
    {
        throw runtime_exception("Inadequate amount of data supplied through the iterators", E_INVALIDARG);
    }

    if (_Dst.get_mipmap_levels() > 1)
    {
        throw runtime_exception("Invalid destination - multiple mipmap levels cannot be copied from source", E_INVALIDARG);
    }

    std::array<size_t, 3> _Copy_extent_arr = _Get_dimensions(_Copy_extent, /*_Mip_offset=*/0);
    std::array<size_t, 3> _Dst_offset_arr = _Get_indices(_Dst_offset);

    _Texture *_Dst_tex_ptr = _Get_texture(_Dst);
    return _Copy_async_impl<_Input_iterator, _Value_type>(_First, _Last, _Dst_tex_ptr, _Dst_offset_arr.data(), _Get_texture_descriptor(_Dst)._Get_most_detailed_mipmap_level(), _Copy_extent_arr.data());
}

template<typename _Value_type, int _Rank>
_Event _Copy_async_impl(const _Texture_base<_Value_type, _Rank>& _Src, const index<_Rank> &_Src_offset,
                             const _Texture_base<_Value_type, _Rank>& _Dst, const index<_Rank> &_Dst_offset,
                             const extent<_Rank> &_Copy_extent)
{
    _Is_valid_section(_Src.extent, _Src_offset, _Copy_extent);
    _Is_valid_section(_Dst.extent, _Dst_offset, _Copy_extent);

    _Texture_descriptor _Src_tex_desc = _Get_texture_descriptor(_Src);
    _Texture_descriptor _Dst_tex_desc = _Get_texture_descriptor(_Dst);

    if (_Src_tex_desc._Get_view_mipmap_levels() != _Dst_tex_desc._Get_view_mipmap_levels())
    {
        throw runtime_exception("The source and destination textures must have the exactly the same number of mipmap levels for texture copy.", E_INVALIDARG);
    }

    bool _Is_whole_texture_copy = (_Src_offset == _Dst_offset && _Src_offset == index<_Rank>() && _Src.extent == _Dst.extent && _Src.extent == _Copy_extent);

    if (_Src_tex_desc._Get_view_mipmap_levels() > 1 && !_Is_whole_texture_copy)
    {
        throw runtime_exception("Sections are not allowed when copy involves multiple mipmap levels", E_INVALIDARG);
    }

    if (_Src_tex_desc._Are_mipmap_levels_overlapping(&_Dst_tex_desc))
    {
        throw runtime_exception("The source and destination are overlapping areas on the same texture", E_INVALIDARG);
    }

    _Texture* _Src_tex = _Get_texture(_Src);
    _Texture* _Dst_tex = _Get_texture(_Dst);

    // Formats must be identical for non-adopted textures.  Textures created through D3D interop are not subject to this test
    // to allow copy between related, but not identical, formats.  Attempting to copy between unrelated formats through interop
    // will result in exceptions in debug mode and undefined behavior in release mode.
    if (!_Src_tex->_Is_adopted() && !_Dst_tex->_Is_adopted() && (_Src_tex->_Get_texture_format() != _Dst_tex->_Get_texture_format()))
    {
        throw runtime_exception("The source and destination textures are not compatible.", E_INVALIDARG);
    }

    std::array<size_t, 3> _Src_offset_arr = _Get_indices(_Src_offset);
    std::array<size_t, 3> _Dst_offset_arr = _Get_indices(_Dst_offset);

    _Event _Copy_event;

    unsigned int _Src_most_detailed_mipmap_level = _Src_tex_desc._Get_most_detailed_mipmap_level();
    unsigned int _Dst_most_detailed_mipmap_level = _Dst_tex_desc._Get_most_detailed_mipmap_level();

    // Copy all mipmap levels from source to destination one by one.
    // Note that the offsets are not allowed therefore only dimensions need to be updated for subsequent mipmap levels
    for (unsigned int _Mip_offset = 0; _Mip_offset < _Src_tex_desc._Get_view_mipmap_levels(); ++_Mip_offset)
    {
        std::array<size_t, 3> _Copy_extent_arr = _Get_dimensions(_Copy_extent, _Mip_offset);

        auto _Step_event = _Copy_async_impl(_Src_tex, _Src_offset_arr.data(), _Src_most_detailed_mipmap_level + _Mip_offset,
                                            _Dst_tex, _Dst_offset_arr.data(), _Dst_most_detailed_mipmap_level + _Mip_offset,
                                            _Copy_extent_arr.data());

        _Copy_event = _Copy_event._Add_event(_Step_event);
    }

    return _Copy_event;
}

} // namespace details

/// <summary>
///     Copies the contents of the source texture into the destination host buffer.
/// </summary>
/// <param name="_Rank">
///     The rank of the source texture.
/// </param>
/// <param name="_Value_type">
///     The type of the elements of the source texture.
/// </param>
/// <param name="_Src">
///     The source texture or texture_view.
/// </param>
/// <param name="_Dst">
///     The destination host buffer.
/// </param>
/// <param name="_Dst_byte_size">
///     Number of bytes in the destination buffer.
/// </param>
template <typename _Src_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture, void>::type> void copy(const _Src_type &_Src, _Out_ void * _Dst, unsigned int _Dst_byte_size)
{
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                        nullptr,
                                                                        _Get_section_size(_Src, _Src.extent));

    details::_Copy_async_impl(_Src, index<_Src_type::rank>(), _Src.extent, _Dst, _Dst_byte_size)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies the contents of a section of the source texture into the destination host buffer.
/// </summary>
/// <param name="_Rank">
///     The rank of the source texture.
/// </param>
/// <param name="_Value_type">
///     The type of the elements of the source texture.
/// </param>
/// <param name="_Src">
///     The source texture or texture_view.
/// </param>
/// <param name="_Src_offset">
///     The offset into the source texture from which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
/// <param name="_Dst">
///     The destination host buffer.
/// </param>
/// <param name="_Dst_byte_size">
///     Number of bytes in the destination buffer.
/// </param>
template <typename _Src_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture, void>::type> void copy(const _Src_type &_Src, const index<_Src_type::rank> &_Src_offset, const extent<_Src_type::rank> &_Copy_extent, _Out_ void * _Dst, unsigned int _Dst_byte_size)
{
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                        nullptr,
                                                                        _Get_section_size(_Src, _Copy_extent));

    details::_Copy_async_impl(_Src, _Src_offset, _Copy_extent, _Dst, _Dst_byte_size)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}


/// <summary>
///     Copies the contents of the source host buffer into the destination texture _Dst.
/// </summary>
/// <param name="_Rank">
///     The rank of the destination texture.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture or texture_view.
/// </param>
/// <param name="_Src">
///     The source host buffer.
/// </param>
/// <param name="_Src_byte_size">
///     Number of bytes in the source buffer.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
template <typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> void copy(const void * _Src, unsigned int _Src_byte_size, _Dst_type & _Dst)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(nullptr,
                                                                        concurrency::details::_Get_texture_descriptor(_Dst),
                                                                        _Get_section_size(_Dst, _Dst.extent));

    details::_Copy_async_impl(_Src, _Src_byte_size, _Dst, index<_Dst_type::rank>(), _Dst.extent)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies the contents of the source host buffer into a section of the destination texture _Dst.
/// </summary>
/// <param name="_Dst_type">
///     The type of the destination texture or texture_view.
/// </param>
/// <param name="_Src">
///     The source host buffer.
/// </param>
/// <param name="_Src_byte_size">
///     Number of bytes in the source buffer.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
/// <param name="_Dst_offset">
///     The offset into the destination texture to which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
template <typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> void copy(const void * _Src, unsigned int _Src_byte_size, _Dst_type & _Dst,
                                        const index<_Dst_type::rank> &_Dst_offset, const extent<_Dst_type::rank> &_Copy_extent)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(nullptr,
                                                                        concurrency::details::_Get_texture_descriptor(_Dst),
                                                                        _Get_section_size(_Dst, _Copy_extent));

    details::_Copy_async_impl(_Src, _Src_byte_size, _Dst, _Dst_offset, _Copy_extent)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}


/// <summary>
///     Asynchronously copies the contents of the source texture into the destination host buffer.
/// </summary>
/// <param name="_Rank">
///     The rank of the source texture.
/// </param>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="_Src">
///     The source texture or texture_view.
/// </param>
/// <param name="_Dst">
///     The destination host buffer.
/// </param>
/// <param name="_Dst_byte_size">
///     Number of bytes in the destination buffer.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template<typename _Src_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture, void>::type> concurrency::completion_future copy_async(const _Src_type &_Src, _Out_ void * _Dst, unsigned int _Dst_byte_size)
{
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                                   nullptr,
                                                                                   _Get_section_size(_Src, _Src.extent));

    _Event _Ev = details::_Copy_async_impl(_Src, index<_Src_type::rank>(), _Src.extent, _Dst, _Dst_byte_size);

    return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies the contents of the provided section of the source texture into the destination host buffer.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="_Src">
///     The source texture or texture_view.
/// </param>
/// <param name="_Src_offset">
///     The offset into the source texture from which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
/// <param name="_Dst">
///     The destination host buffer.
/// </param>
/// <param name="_Dst_byte_size">
///     Number of bytes in the destination buffer.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template<typename _Src_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture, void>::type> concurrency::completion_future copy_async(const _Src_type &_Src, const index<_Src_type::rank> &_Src_offset, const extent<_Src_type::rank> &_Copy_extent,
                                                                       _Out_ void * _Dst, unsigned int _Dst_byte_size)
{
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                                   nullptr,
                                                                                   _Get_section_size(_Src, _Copy_extent));

    _Event _Ev = details::_Copy_async_impl(_Src, _Src_offset, _Copy_extent, _Dst, _Dst_byte_size);

    return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies the contents of the source host buffer into the destination texture _Dst.
/// </summary>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_Src">
///     The source host buffer.
/// </param>
/// <param name="_Src_byte_size">
///     Number of bytes in the source buffer.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> concurrency::completion_future copy_async(const void * _Src, unsigned int _Src_byte_size, _Dst_type & _Dst)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(nullptr,
                                                                                   concurrency::details::_Get_texture_descriptor(_Dst),
                                                                                   _Get_section_size(_Dst, _Dst.extent));

    _Event _Ev = details::_Copy_async_impl(_Src, _Src_byte_size, _Dst, index<_Dst_type::rank>(), _Dst.extent);

    return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies the contents of the source host buffer into a section of the destination texture _Dst.
/// </summary>
/// <param name="_Dst_type">
///     The type of the elements of the destination texture.
/// </param>
/// <param name="_Src">
///     The source host buffer.
/// </param>
/// <param name="_Src_byte_size">
///     Number of bytes in the source buffer.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
/// <param name="_Dst_offset">
///     The offset into the destination texture to which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> concurrency::completion_future copy_async(const void * _Src, unsigned int _Src_byte_size, _Dst_type & _Dst,
                                                                        const index<_Dst_type::rank> &_Dst_offset, const extent<_Dst_type::rank> &_Copy_extent)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(nullptr,
                                                                                   concurrency::details::_Get_texture_descriptor(_Dst),
                                                                                   _Get_section_size(_Dst, _Copy_extent));

    _Event _Ev = details::_Copy_async_impl(_Src, _Src_byte_size, _Dst, _Dst_offset, _Copy_extent);

    return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies data from the pair of source iterators into the destination texture _Dst.
/// </summary>
/// <param name="InputIterator">
///     The input iterator type.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_First">
///     The starting iterator for the copy.
/// </param>
/// <param name="_Last">
///     The ending iterator for the copy.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
template <typename InputIterator, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> void copy(InputIterator _First, InputIterator _Last, _Dst_type &_Dst)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(nullptr,
                                                                    concurrency::details::_Get_texture_descriptor(_Dst),
                                                                    _Get_section_size(_Dst, _Dst.extent));

    details::_Copy_async_impl(_First, _Last, _Dst, index<_Dst_type::rank>(), _Dst.extent)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies data from the pair of source iterators into a section of the destination texture _Dst.
/// </summary>
/// <param name="InputIterator">
///     The input iterator type.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_First">
///     The starting iterator for the copy.
/// </param>
/// <param name="_Last">
///     The ending iterator for the copy.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
/// <param name="_Dst_offset">
///     The offset into the destination texture to which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
template <typename InputIterator, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> void copy(InputIterator _First, InputIterator _Last, _Dst_type &_Dst, const index<_Dst_type::rank> &_Dst_offset, const extent<_Dst_type::rank> &_Copy_extent)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(nullptr,
                                                                    concurrency::details::_Get_texture_descriptor(_Dst),
                                                                    _Get_section_size(_Dst, _Copy_extent));

    details::_Copy_async_impl(_First, _Last, _Dst, _Dst_offset, _Copy_extent)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies data from the source texture _Src into an output iterator.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="OutputIterator">
///     The output iterator type.
/// </param>
/// <param name="_Dst">
///     The starting iterator for the copy output.
/// </param>
template <typename _Src_type, typename OutputIterator, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && !details::texture_traits<OutputIterator>::is_texture, void>::type> void copy(const _Src_type &_Src, OutputIterator _Dst)
{
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    nullptr,
                                                                    _Get_section_size(_Src, _Src.extent));

    details::_Copy_async_impl(_Src, index<_Src_type::rank>(), _Src.extent, _Dst)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies data from a section of the source texture _Src into an output iterator.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="OutputIterator">
///     The output iterator type.
/// </param>
/// <param name="_Src_offset">
///     The offset into the source texture from which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
/// <param name="_Dst">
///     The starting iterator for the copy output.
/// </param>
template <typename _Src_type, typename OutputIterator, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && !details::texture_traits<OutputIterator>::is_texture, void>::type> void copy(const _Src_type &_Src, const index<_Src_type::rank> &_Src_offset, const extent<_Src_type::rank> &_Copy_extent, OutputIterator _Dst)
{
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    nullptr,
                                                                    _Get_section_size(_Src, _Copy_extent));

    details::_Copy_async_impl(_Src, _Src_offset, _Copy_extent, _Dst)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies data from the source texture _Src into the destination texture _Dst.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_Src">
///     The source texture from which to copy.
/// </param>
/// <param name="_Dst">
///     The destination texture into which to copy.
/// </param>
template <typename _Src_type, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && details::texture_traits<_Dst_type>::is_texture, void>::type> void copy(const _Src_type &_Src,  _Dst_type &_Dst)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");

    if (_Src.extent != _Dst.extent)
    {
        throw runtime_exception("The source and destination textures must have the exactly the same extent for whole-texture copy.", E_INVALIDARG);
    }

    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    concurrency::details::_Get_texture_descriptor(_Dst),
                                                                    _Get_section_size(_Dst, _Dst.extent));

    details::_Copy_async_impl(_Src, index<_Src_type::rank>(), _Dst, index<_Dst_type::rank>(), _Dst.extent)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies data from a section of the source texture _Src into a section of the destination texture _Dst.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_Src">
///     The source texture from which to copy.
/// </param>
/// <param name="_Src_offset">
///     The offset into the source texture from which to begin copying.
/// </param>
/// <param name="_Dst">
///     The destination texture into which to copy.
/// </param>
/// <param name="_Dst_offset">
///     The offset into the destination texture to which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
template <typename _Src_type, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && details::texture_traits<_Dst_type>::is_texture, void>::type> void copy(const _Src_type &_Src, const index<_Src_type::rank> &_Src_offset, _Dst_type &_Dst, const index<_Dst_type::rank> &_Dst_offset, const extent<_Src_type::rank> &_Copy_extent)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    concurrency::details::_Get_texture_descriptor(_Dst),
                                                                    _Get_section_size(_Src, _Copy_extent));

    details::_Copy_async_impl(_Src, _Src_offset, _Dst, _Dst_offset, _Copy_extent)._Get();

    concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Asynchronously copies data from the pair of source iterators into the destination texture _Dst.
/// </summary>
/// <param name="InputIterator">
///     The input iterator type.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_First">
///     The starting iterator for the copy.
/// </param>
/// <param name="_Last">
///     The ending iterator for the copy.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename InputIterator, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> concurrency::completion_future copy_async(InputIterator _First, InputIterator _Last, _Dst_type &_Dst)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(nullptr,
        concurrency::details::_Get_texture_descriptor(_Dst),
        _Get_section_size(_Dst, _Dst.extent));

    _Event _Ev = details::_Copy_async_impl<InputIterator, _Dst_type::value_type, _Dst_type::rank>(_First, _Last, _Dst, index<_Dst_type::rank>(), _Dst.extent);

    return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies data from the pair of source iterators into a section of the destination texture _Dst.
/// </summary>
/// <param name="InputIterator">
///     The input iterator type.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_First">
///     The starting iterator for the copy.
/// </param>
/// <param name="_Last">
///     The ending iterator for the copy.
/// </param>
/// <param name="_Dst">
///     The destination texture or texture_view.
/// </param>
/// <param name="_Dst_offset">
///     The offset into the destination texture to which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename InputIterator, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Dst_type>::is_texture, void>::type> concurrency::completion_future copy_async(InputIterator _First, InputIterator _Last, _Dst_type &_Dst,
           const index<_Dst_type::rank> &_Dst_offset, const extent<_Dst_type::rank> &_Copy_extent)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(nullptr,
                                                                                                concurrency::details::_Get_texture_descriptor(_Dst),
                                                                                                _Get_section_size(_Dst, _Copy_extent));

    _Event _Ev = details::_Copy_async_impl<InputIterator, _Dst_type::value_type, _Dst_type::rank>(_First, _Last, _Dst, _Dst_offset, _Copy_extent);

    return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies data from the source texture _Src into an output iterator.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="OutputIterator">
///     The output iterator type.
/// </param>
/// <param name="_Dst">
///     The starting iterator for the copy output.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Src_type, typename OutputIterator, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && !details::texture_traits<OutputIterator>::is_texture, void>::type> concurrency::completion_future copy_async(_Src_type &_Src, OutputIterator _Dst)
{
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    nullptr,
                                                                    _Get_section_size(_Src, _Src.extent));

    _Event _Ev = details::_Copy_async_impl(_Src, index<_Src_type::rank>(), _Src.extent, _Dst);

     return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies data from a section of the source texture _Src into an output iterator.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="OutputIterator">
///     The output iterator type.
/// </param>
/// <param name="_Src_offset">
///     The offset into the source texture from which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
/// <param name="_Dst">
///     The starting iterator for the copy output.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Src_type, typename OutputIterator, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && !details::texture_traits<OutputIterator>::is_texture, void>::type> concurrency::completion_future copy_async(_Src_type &_Src, const index<_Src_type::rank> &_Src_offset, const extent<_Src_type::rank> &_Copy_extent, OutputIterator _Dst)
{
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    nullptr,
                                                                    _Get_section_size(_Src, _Copy_extent));

    _Event _Ev = details::_Copy_async_impl(_Src, _Src_offset, _Copy_extent, _Dst);

     return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies data from the source texture _Src into the destination texture _Dst.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_Src">
///     The source texture from which to copy.
/// </param>
/// <param name="_Dst">
///     The destination texture into which to copy.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Src_type, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && details::texture_traits<_Dst_type>::is_texture, void>::type> concurrency::completion_future copy_async(_Src_type &_Src, _Dst_type &_Dst)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");

    if (_Src.extent != _Dst.extent)
    {
        throw runtime_exception("The source and destination textures must have the exactly the same extent for whole-texture copy.", E_INVALIDARG);
    }
    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    concurrency::details::_Get_texture_descriptor(_Dst),
                                                                    _Get_section_size(_Dst, _Dst.extent));

    _Event _Ev = details::_Copy_async_impl(_Src, index<_Src_type::rank>(), _Dst, index<_Dst_type::rank>(), _Dst.extent);

     return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies data from a section of the source texture _Src into the destination texture _Dst.
/// </summary>
/// <param name="_Src_type">
///     The type of the source texture.
/// </param>
/// <param name="_Dst_type">
///     The type of the destination texture.
/// </param>
/// <param name="_Src">
///     The source texture from which to copy.
/// </param>
/// <param name="_Src_offset">
///     The offset into the source texture from which to begin copying.
/// </param>
/// <param name="_Dst">
///     The destination texture into which to copy.
/// </param>
/// <param name="_Dst_offset">
///     The offset into the destination texture to which to begin copying.
/// </param>
/// <param name="_Copy_extent">
///     The extent of the texture section to copy.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Src_type, typename _Dst_type, typename = typename std::enable_if<details::texture_traits<_Src_type>::is_texture && details::texture_traits<_Dst_type>::is_texture, void>::type> concurrency::completion_future copy_async(_Src_type &_Src, const index<_Src_type::rank> &_Src_offset, _Dst_type &_Dst, const index<_Dst_type::rank> &_Dst_offset, const extent<_Src_type::rank> &_Copy_extent)
{
    static_assert(details::texture_traits<_Dst_type>::is_writable, "Destination is not a writable texture type.");

    auto _Async_op_id = concurrency::details::_Get_amp_trace()->_Launch_async_copy_event_helper(concurrency::details::_Get_texture_descriptor(_Src),
                                                                    concurrency::details::_Get_texture_descriptor(_Dst),
                                                                    _Get_section_size(_Src, _Copy_extent));

    _Event _Ev = details::_Copy_async_impl(_Src, _Src_offset, _Dst, _Dst_offset, _Copy_extent);

     return concurrency::details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     A texture is a data aggregate on an accelerator_view in the extent domain.
///     It is a collection of variables, one for each element in an extent domain.
///     Each variable holds a value corresponding to C++ primitive type (unsigned int,
///     int, float, double), or scalar type norm, or unorm (defined in concurrency::graphics),
///     or eligible short vector types defined in concurrency::graphics.
/// </summary>
/// <param name="_Value_type">
///     The type of the elements in the texture aggregates.
/// </param>
/// <param name="_Rank">
///     The _Rank of the corresponding extent domain.
/// </param>
template <typename _Value_type, int _Rank> class texture : public details::_Texture_base<_Value_type, _Rank>
{
    template<typename _Value_type, int _Rank>
    friend texture<_Value_type,_Rank> direct3d::make_texture(const Concurrency::accelerator_view &_Av, _In_ IUnknown *_D3D_texture, DXGI_FORMAT _View_format) __CPU_ONLY;

    static_assert(!std::is_const<_Value_type>::value, "const value type is not supported for texture.");

    using _Texture_base = details::_Texture_base<_Value_type, _Rank>;
public:

    /// <summary>
    ///     Construct a texture from extents.
    /// </summary>
    /// <param name="_Extent">
    ///     An extent that describes the shape of the texture.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view);
    }

    /// <summary>
    ///     Construct texture&lt;T,1&gt; with the extent _E0
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    texture(int _E0) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int) is only permissible on texture<value_type, 1>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view);
    }

    /// <summary>
    ///     Construct a texture&lt;T,2&gt; from two integer extents.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    texture(int _E0, int _E1) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int) is only permissible on texture<value_type, 2>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view);
    }

    /// <summary>
    ///     Construct a texture&lt;T,3&gt; from three integer extents.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the next-to-most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    texture(int _E0, int _E1, int _E2) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int) is only permissible on texture<value_type, 3>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view);
    }

    /// <summary>
    ///     Construct a texture from extents, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Extent">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av);
    }

    /// <summary>
    ///     Construct a staging texture from extents, bound to a specific accelerator_view
    ///     and an associated accelerator_view that is the preferred location for copying
    ///     to/from this texture.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av);
    }

    /// <summary>
    ///     Construct a texture&lt;T,1&gt; with the extent _E0, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, accelerator_view) is only permissible on texture<value_type, 1>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,1&gt; with the extent _E0, bound to a specific
    ///     accelerator_view and an associated accelerator_view that is the preferred location
    ///     for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 1>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av);
    }

    /// <summary>
    ///     Construct a texture&lt;T,2&gt; from two integer extents, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, int _E1, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, accelerator_view) is only permissible on texture<value_type, 2>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,2&gt; from two integer extents, bound to a
    ///     specific accelerator_view and an associated accelerator_view that is the
    ///     preferred location for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, int _E1, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 2>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av);
    }

    /// <summary>
    ///     Construct a texture&lt;T,3&gt; from three integer extents, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the next-to-most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, int _E1, int _E2, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, accelerator_view) is only permissible on texture<value_type, 3>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,3&gt; from three integer extents, bound to a
    ///     specific accelerator_view and an associated accelerator_view that is the preferred
    ///     location for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the next-to-most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, int _E1, int _E2, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 3>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av);
    }

    /// <summary>
    ///     Construct a texture initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(const Concurrency::extent<_Rank>& _Ext, _Input_iterator _Src_first, _Input_iterator _Src_last) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a texture&lt;T,1&gt; with the extent _E0 and from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, _Input_iterator _Src_first, _Input_iterator _Src_last) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, iterator, iterator) is only permissible on texture<value_type, 1>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a texture&lt;T,2&gt; with two integers and initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, int _E1, _Input_iterator _Src_first, _Input_iterator _Src_last) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, iterator, iterator) is only permissible on texture<value_type, 2>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Src_first, _Src_last);
    }


    /// <summary>
    ///     Construct a texture&lt;T,3&gt; with three integers and initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the next-to-most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, int _E1, int _E2, _Input_iterator _Src_first, _Input_iterator _Src_last) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, iterator, iterator) is only permissible on texture<value_type, 3>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a texture initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(const Concurrency::extent<_Rank>& _Ext, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a staging texture initialized from a pair of iterators into a container,
    ///     bound to a specific accelerator_view and an associated accelerator_view that is the
    ///     preferred location for copying to/from this texture.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(const Concurrency::extent<_Rank>& _Ext, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a texture&lt;T,1&gt; with integer _E0 and initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, iterator, iterator, accelerator_view) is only permissible on texture<value_type, 1>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,1&gt; with integer _E0 and initialized from a pair of iterators
    ///     into a container, bound to a specific accelerator_view and an associated accelerator_view that is
    ///     the preferred location for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, iterator, iterator, accelerator_view, accelerator_view) is only permissible on texture<value_type, 1>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a texture&lt;T,2&gt; with two integers and initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, int _E1, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, iterator, iterator, accelerator_view) is only permissible on texture<value_type, 2>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,2&gt; with two integers and initialized from a pair of iterators
    ///     into a container, bound to a specific accelerator_view and an associated accelerator_view that is
    ///     the preferred location for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, int _E1, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, iterator, iterator, accelerator_view, accelerator_view) is only permissible on texture<value_type, 2>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a texture&lt;T,3&gt; with three integers and initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the next-to-most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, int _E1, int _E2, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, iterator, iterator, accelerator_view) is only permissible on texture<value_type, 3>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,3&gt; with three integers and initialized from a pair of iterators
    ///     into a container, bound to a specific accelerator_view and an associated accelerator_view that is the
    ///     preferred location for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the next-to-most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///     An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    template<typename _Input_iterator,
             typename = typename std::enable_if<details::_Is_iterator<_Input_iterator>::value>::type>
    texture(int _E0, int _E1, int _E2, _Input_iterator _Src_first, _Input_iterator _Src_last, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, iterator, iterator, accelerator_view, accelerator_view) is only permissible on texture<value_type, 3>.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "texture cannot be constructed from unorm based short vectors via this constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "texture cannot be constructed from norm based short vectors via this constructor.");
        _Initialize(_Av, _Associated_av, _Src_first, _Src_last);
    }

    /// <summary>
    ///     Construct a texture from extents and specified bits per scalar element
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture from extents, specified bits per scalar element and number of mipmap levels
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Mipmap_levels">
    ///     Number of mipmap levels in the underlying texture.
    ///     If 0 is specified, the texture will have full range of mipmap levels down to smallest possible size for the given extent.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, unsigned int _Bits_per_scalar_element, unsigned int _Mipmap_levels) __CPU_ONLY
        : _Texture_base(_Ext, _Mipmap_levels)
    {
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T,1&gt; with integer _E0 and specified bits per scalar element
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(int _E0, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, unsigned int) is only permissible on texture<value_type, 1>.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T,2&gt; with two integers and specified bits per scalar element
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(int _E0, int _E1, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, unsigned int) is only permissible on texture<value_type, 2>.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T,3&gt; with three integers and specified bits per scalar element
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the next-to-most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Src_first">
    ///     A beginning iterator into the source container.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(int _E0, int _E1, int _E2, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, unsigned int) is only permissible on texture<value_type, 3>.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Bits_per_scalar_element);
    }


    /// <summary>
    ///     Construct a texture from extents and specified bits per scalar element, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        _Initialize(_Av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture from extents, specified bits per scalar element and number of mipmap levels
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Mipmap_levels">
    ///     Number of mipmap levels in the underlying texture.
    ///     If 0 is specified, the texture will have full range of mipmap levels down to smallest possible size for the given extent.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, unsigned int _Bits_per_scalar_element, unsigned int _Mipmap_levels, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(_Ext, _Mipmap_levels)
    {
        _Initialize(_Av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture from extents and specified bits per scalar element, bound to a
    ///     specific accelerator_view and an associated accelerator_view that is the preferred location
    ///     for copying to/from this texture.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        _Initialize(_Av, _Associated_av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T, 1&gt; with integer _E0 and specified bits per scalar element, bound to a specific accelerator.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, unsigned int, accelerator_view) is only permissible on texture<value_type, 1>.");
        _Initialize(_Av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T, 1&gt; with integer _E0 and specified bits per scalar element,
    ///     bound to a specific accelerator and an associated accelerator_view that is the preferred location
    ///     for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, unsigned int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 1>.");
        _Initialize(_Av, _Associated_av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T,2&gt; with two integers and specified bits per scalar element, bound to a specific accelerator.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, int _E1, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, unsigned int, accelerator_view) is only permissible on texture<value_type, 2>.");
        _Initialize(_Av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,2&gt; with two integers and specified bits per scalar element,
    ///     bound to a specific accelerator and an associated accelerator_view that is the preferred location
    ///     for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, int _E1, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, unsigned int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 2>.");
        _Initialize(_Av, _Associated_av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T,3&gt; with three integers and specified bits per scalar element, bound to a specific accelerator.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, int _E1, int _E2, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, unsigned int, accelerator_view) is only permissible on texture<value_type, 3>.");
        _Initialize(_Av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T,3&gt; with three integers and specified bits per scalar element,
    ///     bound to a specific accelerator and an associated accelerator_view that is the preferred location
    ///     for copying to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, int _E1, int _E2, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, unsigned int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 3>.");
        _Initialize(_Av, _Associated_av, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture from extents and specified bits per scalar element, initialized from a host buffer.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T,1&gt; with integer _E0 and specified bits per scalar element, initialized from a host buffer.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(int _E0, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, void *, unsigned int, unsigned int) is only permissible on texture<value_type, 1>.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T,2&gt; with two integers and specified bits per scalar element, initialized from a host buffer.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(int _E0, int _E1, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, void *, unsigned int, unsigned int) is only permissible on texture<value_type, 2>.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }


    /// <summary>
    ///     Construct a texture&lt;T,3&gt; with three integers and specified bits per scalar element, initialized from a host buffer.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    texture(int _E0, int _E1, int _E2, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, void *, unsigned int, unsigned int) is only permissible on texture<value_type, 3>.");
        _Initialize(Concurrency::details::_Select_default_accelerator().default_view, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture from extents and specified bits per scalar element, initialized from a host buffer,  bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        _Initialize(_Av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture from extents and specified bits per scalar element, initialized from a host buffer,
    ///     bound to a specific accelerator_view and an associated accelerator_view that is the preferred location for copying
    ///     to/from this texture.
    /// </summary>
    /// <param name="_Ext">
    ///     An extent that describes the shape of the texture.
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(const Concurrency::extent<_Rank>& _Ext, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(_Ext)
    {
        _Initialize(_Av, _Associated_av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T, 1&gt; with integer _E0 and specified bits per scalar element, initialized from a host buffer,  bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, void *, unsigned int, unsigned int, accelerator_view) is only permissible on texture<value_type, 1>.");
        _Initialize(_Av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T, 1&gt; with integer _E0 and specified bits per scalar element, initialized from a host buffer,
    ///     bound to a specific accelerator_view and an associated accelerator_view that is the preferred location for copying
    ///     to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "texture(int, void *, unsigned int, unsigned int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 1>.");
        _Initialize(_Av, _Associated_av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T, 2&gt; with two integers and specified bits per scalar element, initialized from a host buffer,  bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, int _E1, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, void *, unsigned int, unsigned int, accelerator_view) is only permissible on texture<value_type, 2>.");
        _Initialize(_Av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T, 2&gt; with two integers and specified bits per scalar element, initialized from a host buffer,
    ///     bound to a specific accelerator_view and an associated accelerator_view that is the preferred location for copying
    ///     to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, int _E1, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "texture(int, int, void *, unsigned int, unsigned int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 2>.");
        _Initialize(_Av, _Associated_av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture&lt;T, 3&gt; with three integers and specified bits per scalar element, initialized from a host buffer,  bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(int _E0, int _E1, int _E2, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, void *, unsigned int, unsigned int, accelerator_view) is only permissible on texture<value_type, 3>.");
        _Initialize(_Av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a staging texture&lt;T, 3&gt; with three integers and specified bits per scalar element, initialized from a host buffer,
    ///     bound to a specific accelerator_view and an associated accelerator_view that is the preferred location for copying
    ///     to/from this texture.
    /// </summary>
    /// <param name="_E0">
    ///     An integer that is the length of the most-significant dimension of this texture (depth).
    /// </param>
    /// <param name="_E1">
    ///     An integer that is the length of the least-significant dimension of this texture (height).
    /// </param>
    /// <param name="_E2">
    ///     An integer that is the length of the least-significant dimension of this texture (width).
    /// </param>
    /// <param name="_Source">
    ///     A pointer to a host buffer.
    /// </param>
    /// <param name="_Source_byte_size">
    ///     Number of bytes in the source buffer.
    /// </param>
    /// <param name="_Bits_per_scalar_element">
    ///     Number of bits per each scalar element in the underlying scalar type of the texture.
    ///     In general, supported value is 8, 16, 32, 64.
    ///     If 0 is specified, the number of bits picks defaulted value for the underlying scalar_type.
    ///     64 is only valid for double based textures
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(int _E0, int _E1, int _E2, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element, const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
        : _Texture_base(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "texture(int, int, int, void *, unsigned int, unsigned int, accelerator_view, accelerator_view) is only permissible on texture<value_type, 3>.");
        _Initialize(_Av, _Associated_av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    /// <summary>
    ///     Construct a texture from a texture_view. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The texture_view to copy from.
    /// </param>
    texture(const texture_view<_Value_type, _Rank> & _Src)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Src.accelerator_view, _Src);
    }

    /// <summary>
    ///     Construct a texture from a read-only texture_view. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The read-only texture_view to copy from.
    /// </param>
    texture(const texture_view<const _Value_type, _Rank> & _Src)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Src.accelerator_view, _Src);
    }

    /// <summary>
    ///     Construct a texture from a texture_view on another accelerator_view. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The texture_view to copy from.
    /// </param>
    /// <param name="_Acc_view">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(const texture_view<_Value_type, _Rank> & _Src, const Concurrency::accelerator_view & _Acc_view)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Acc_view, _Src);
    }

    /// <summary>
    ///     Construct a texture from a read-only texture_view on another accelerator_view. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The read-only texture_view to copy from.
    /// </param>
    /// <param name="_Acc_view">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(const texture_view<const _Value_type, _Rank> & _Src, const Concurrency::accelerator_view & _Acc_view)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Acc_view, _Src);
    }

    /// <summary>
    ///     Construct a staging texture from a texture_view on another accelerator_view. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The texture_view to copy from.
    /// </param>
    /// <param name="_Acc_view">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(const texture_view<_Value_type, _Rank> & _Src, const Concurrency::accelerator_view & _Acc_view, const Concurrency::accelerator_view& _Associated_av)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Acc_view, _Associated_av, _Src);
    }

    /// <summary>
    ///     Construct a staging texture from a read-only texture_view on another accelerator_view. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The read-only texture_view to copy from.
    /// </param>
    /// <param name="_Acc_view">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(const texture_view<const _Value_type, _Rank> & _Src, const Concurrency::accelerator_view & _Acc_view, const Concurrency::accelerator_view& _Associated_av)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Acc_view, _Associated_av, _Src);
    }

    /// <summary>
    ///     Copy constructor. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The texture to copy from.
    /// </param>
    texture(const texture & _Src)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Src.accelerator_view, _Src.associated_accelerator_view, _Src);
    }

    /// <summary>
    ///     Move constructor
    /// </summary>
    /// <param name="_Other">
    ///     The source texture to move from.
    /// </param>
    texture(texture && _Other)
    {
        *this = std::move(_Other);
    }

    /// <summary>
    ///     Copy constructor. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The texture to copy from.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    texture(const texture & _Src, const Concurrency::accelerator_view & _Av)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Av, _Src);
    }

    /// <summary>
    ///     Copy constructor. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The texture to copy from.
    /// </param>
    /// <param name="_Av">
    ///     An accelerator_view where this texture resides.
    /// </param>
    /// <param name="_Associated_av">
    ///     An accelerator_view which specifies the preferred target location for copies
    ///     to/from the texture.
    /// </param>
    texture(const texture & _Src, const Concurrency::accelerator_view & _Av, const Concurrency::accelerator_view& _Associated_av)
        : _Texture_base(_Src.extent, _Src.get_mipmap_levels())
    {
        _Initialize(_Av, _Associated_av, _Src);
    }

    /// <summary>
    ///     Copy assignment operator. Deep copy
    /// </summary>
    /// <param name="_Src">
    ///     The texture to copy from.
    /// </param>
    /// <returns>
    ///     A reference to this texture.
    /// </returns>
    texture& operator=(const texture & _Other)
    {
        if (this != &_Other)
        {
            this->_M_extent = _Other._M_extent;
            this->_M_texture_descriptor._Set_view_mipmap_levels(_Other.get_mipmap_levels());
            _Initialize(_Other.accelerator_view, _Other.associated_accelerator_view, _Other);
        }
        return *this;
    }

    /// <summary>
    ///     Move assignment operator
    /// </summary>
    /// <param name="_Other">
    ///     The source texture to move from.
    /// </param>
    /// <returns>
    ///     A reference to this texture.
    /// </returns>
    texture& operator=(texture<_Value_type, _Rank> && _Other)
    {
        if (this != &_Other)
        {
            this->_M_extent = _Other._M_extent;
            this->_M_texture_descriptor = _Other._M_texture_descriptor;

            _Other._M_texture_descriptor._M_data_ptr = NULL;
            _Other._M_texture_descriptor._Set_texture_ptr(NULL);
        }
        return *this;
    }

    /// <summary>
    ///     Copy-to, deep copy
    /// </summary>
    /// <param name="_Dest">
    ///     The destination texture to copy to.
    /// </param>
    void copy_to(texture & _Dest) const
    {
        if (this->extent != _Dest.extent)
        {
            throw runtime_exception("The source and destination textures must have the exactly the same extent.", E_INVALIDARG);
        }

        auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(*this),
                                                                            concurrency::details::_Get_texture_descriptor(_Dest),
                                                                            this->get_data_length());

        this->_Copy_to(_Dest);

        concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    /// <summary>
    ///     Copy-to, deep copy
    /// </summary>
    /// <param name="_Dest">
    ///     The destination writeonly_texture_view to copy to.
    /// </param>
    void copy_to(const writeonly_texture_view<_Value_type, _Rank> & _Dest) const
    {
        if (this->extent != _Dest.extent)
        {
            throw runtime_exception("The source and destination textures must have the exactly the same extent.", E_INVALIDARG);
        }

        auto _Span_id = concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(concurrency::details::_Get_texture_descriptor(*this),
                                                                            concurrency::details::_Get_texture_descriptor(_Dest),
                                                                            this->get_data_length());

        this->_Copy_to(_Dest);

        concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    /// <summary>
    ///     Destructor
    /// </summary>
    ~texture() __CPU_ONLY
    {
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type operator[] (const index<_Rank>& _Index) const __GPU_ONLY
    {
        typename details::_Texture_base<_Value_type, _Rank>::value_type _Tmp;
        _Texture_read_helper<index<_Rank>, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Tmp, _Index, /*_Mip_level=*/0);
        return _Tmp;
    }

    /// <summary>
    ///     Get the element value indexed by _I.
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I.
    /// </returns>
    const _Value_type operator[] (int _I0) const __GPU_ONLY
    {
        static_assert(_Rank == 1, "value_type texture::operator[](int) is only permissible on texture<value_type, 1>.");
        return (*this)[index<1>(_I0)];
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type operator() (const index<_Rank>& _Index) const __GPU_ONLY
    {
        return (*this)[_Index];
    }

    /// <summary>
    ///     Get the element value indexed by _I0
    /// </summary>
    /// <param name="_I0">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I0.
    /// </returns>
    const _Value_type operator() (int _I0) const __GPU_ONLY
    {
        static_assert(_Rank == 1, "value_type texture::operator()(int) is only permissible on texture<value_type, 1>.");
        return (*this)[index<1>(_I0)];
    }

    /// <summary>
    ///     Get the element value indexed by (_I0,_I1)
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the index
    /// </param>
    /// <param name="_I1">
    ///     The least-significant component of the index
    /// </param>
    /// <returns>
    ///     The element value indexed by (_I0,_I1)
    /// </returns>
    const _Value_type operator() (int _I0, int _I1) const __GPU_ONLY
    {
        static_assert(_Rank == 2, "value_type texture::operator()(int, int) is only permissible on texture<value_type, 2>.");
        return (*this)[index<2>(_I0, _I1)];
    }

    /// <summary>
    ///     Get the element value indexed by (_I0,_I1,_I2)
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the index
    /// </param>
    /// <param name="_I1">
    ///     The next-to-most-significant component of the index
    /// </param>
    /// <param name="_I2">
    ///     The least-significant component of the index
    /// </param>
    /// <returns>
    ///     The element value indexed by (_I0,_I1,_I2)
    /// </returns>
    const _Value_type operator() (int _I0, int _I1, int _I2) const __GPU_ONLY
    {
        static_assert(_Rank == 3, "value_type texture::operator()(int, int, int) is only permissible on texture<value_type, 3>.");
        return (*this)[index<3>(_I0, _I1, _I2)];
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type get(const index<_Rank>& _Index) const __GPU_ONLY
    {
        return (*this)[_Index];
    }

    /// <summary>
    ///     Set the element indexed by _Index with value _Value.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <param name="_Value">
    ///     The value to be set to the element indexed by _Index.
    /// </param>
    void set(const index<_Rank>& _Index, const _Value_type& _Value) __GPU_ONLY
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Num_channels == 1, "Invalid value_type for set method.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "Invalid value_type for set method.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "Invalid value_type for set method.");
        _Texture_write_helper<index<_Rank>, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Value, _Index);
    }

    /// <summary>
    ///     Returns a CPU pointer to the raw data of this texture.
    /// </summary>
    _Ret_ void* data() __CPU_ONLY
    {
        return this->_Get_texture()->_Get_host_ptr();
    }

    /// <summary>
    ///     Returns a CPU pointer to the raw data of this texture.
    /// </summary>
    const void* data() const __CPU_ONLY
    {
        return this->_Get_texture()->_Get_host_ptr();
    }

    /// <summary>
    ///     Returns the row pitch (in bytes) of a 2D or 3D staging texture on the CPU to be
    ///     used for navigating the staging texture from row to row on the CPU.
    /// </summary>
    __declspec(property(get=get_row_pitch)) unsigned int row_pitch;
    unsigned int get_row_pitch() const __CPU_ONLY
    {
        static_assert(_Rank >= 2, "row_pitch is only applicable to staging textures with rank 2 or higher.");

        if (!this->_Get_texture()->_Is_staging()) {
            throw runtime_exception("row_pitch is only applicable to staging textures.", E_INVALIDARG);
        }

        return static_cast<unsigned int>(this->_Get_texture()->_Get_row_pitch());
    }

    /// <summary>
    ///     Returns the depth pitch (in bytes) of a 3D staging texture on the CPU to be used
    ///     for navigating the staging texture from depth slice to depth slice on the CPU.
    /// </summary>
    __declspec(property(get=get_depth_pitch)) unsigned int depth_pitch;
    unsigned int get_depth_pitch() const __CPU_ONLY
    {
        static_assert(_Rank == 3, "depth_pitch is only applicable to staging textures with rank 3.");

        if (!this->_Get_texture()->_Is_staging()) {
            throw runtime_exception("depth_pitch is only applicable to staging textures.", E_INVALIDARG);
        }

        return static_cast<unsigned int>(this->_Get_texture()->_Get_depth_pitch());
    }

    /// <summary>
    ///     Returns the accelerator_view that is the preferred target where this texture can be copied.
    /// </summary>
    __declspec(property(get=get_associated_accelerator_view)) Concurrency::accelerator_view associated_accelerator_view;
    Concurrency::accelerator_view get_associated_accelerator_view() const __CPU_ONLY
    {
        return this->_Get_texture()->_Get_accelerator_view();
    }

private:
    // Private constructor used by make_texture to create a texture from D3D texture
    texture(const Concurrency::extent<_Rank> & _Ext, const _Texture_descriptor & _Descriptor)
        : details::_Texture_base<_Value_type, _Rank>(_Ext, _Descriptor)
    {
    }

    bool _Should_create_staging_texture(const Concurrency::accelerator_view &_Av, const Concurrency::accelerator_view &_Associated_av)
    {
        return (_Is_cpu_accelerator(_Av.accelerator) && !_Is_cpu_accelerator(_Associated_av.accelerator));
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av, unsigned int _Bits_per_scalar_element) __CPU_ONLY
    {
        if (_Bits_per_scalar_element != 8 && _Bits_per_scalar_element != 16 &&
            _Bits_per_scalar_element != 32 && _Bits_per_scalar_element != 64)
        {
            throw runtime_exception("Invalid _Bits_per_scalar_element argument - it can only be 8, 16, 32, or 64.", E_INVALIDARG);
        }

        // special cases for 64 and for double based textures

        if (_Bits_per_scalar_element == 64 && _Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Double_type)
        {
            throw runtime_exception("Invalid _Bits_per_scalar_element argument - 64 is only valid for texture of double based short vector types.", E_INVALIDARG);
        }

        if (_Bits_per_scalar_element != 64 && _Short_vector_type_traits<_Value_type>::_Format_base_type_id == _Double_type)
        {
            throw runtime_exception("Invalid _Bits_per_scalar_element argument - it can only be 64 for texture of double based short vector types.", E_INVALIDARG);
        }

        details::_Is_valid_data_length(this->_M_extent.size(), _Bits_per_scalar_element * _Short_vector_type_traits<_Value_type>::_Num_channels);

        // the rest of the check is done by _Texture::_Create_texture, it depends on the underlying supported DXGI formats.

        unsigned int _Bits_per_channel = _Bits_per_scalar_element;

        if (_Short_vector_type_traits<_Value_type>::_Format_base_type_id == _Double_type)
        {
            _Bits_per_channel = _Short_vector_type_traits<_Value_type>::_Default_bits_per_channel;
        }

        std::array<size_t, 3> _Dimensions = Concurrency::graphics::details::_Get_dimensions(this->_M_extent, /*_Mip_offset=*/0);

        // release the old texture first before allocating new one to avoid the chance on hitting OOM
        this->_M_texture_descriptor._Set_texture_ptr(NULL);
        _Texture_ptr _Tex_ptr = NULL;

        // See if we need to allocate a staging texture
        if (_Should_create_staging_texture(_Av, _Associated_av)) {

            if (this->_M_texture_descriptor._Get_view_mipmap_levels() > 1)
            {
                throw runtime_exception("Creating staging textures with mipmap levels > 1 is not supported", E_INVALIDARG);
            }

            _Tex_ptr = _Texture::_Create_stage_texture(
                _Associated_av, _Av, _Rank, _Dimensions[0], _Dimensions[1], _Dimensions[2], this->_M_texture_descriptor._Get_view_mipmap_levels(),
                _Short_vector_type_traits<_Value_type>::_Format_base_type_id == _Double_type ? _Uint_type : _Short_vector_type_traits<_Value_type>::_Format_base_type_id,
                _Short_vector_type_traits<_Value_type>::_Num_channels,
                _Bits_per_channel);

            // Now map the texture
            _Tex_ptr->_Map_buffer(_Write_access, true /* _Wait */);
        }
        else {
            _Tex_ptr = _Texture::_Create_texture(_Av, _Rank, _Dimensions[0], _Dimensions[1], _Dimensions[2], this->_M_texture_descriptor._Get_view_mipmap_levels(),
                _Short_vector_type_traits<_Value_type>::_Format_base_type_id == _Double_type ? _Uint_type : _Short_vector_type_traits<_Value_type>::_Format_base_type_id,
                _Short_vector_type_traits<_Value_type>::_Num_channels,
                _Bits_per_channel);
        }

        this->_M_texture_descriptor._Set_texture_ptr(_Tex_ptr);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, unsigned int _Bits_per_scalar_element) __CPU_ONLY
    {
        _Initialize(_Av, _Av, _Bits_per_scalar_element);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av) __CPU_ONLY
    {
        _Initialize(_Av, _Associated_av, Concurrency::graphics::details::_Get_default_bits_per_scalar_element<_Value_type>());
    }

    void _Initialize(const Concurrency::accelerator_view& _Av) __CPU_ONLY
    {
        _Initialize(_Av, _Av);
    }

    template<typename _Input_iterator>
    void _Initialize(const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av, _Input_iterator _Src_first, _Input_iterator _Src_last) __CPU_ONLY
    {
        _Initialize(_Av, _Associated_av);

        auto _Span_id = Concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(nullptr,
                                                                                         Concurrency::details::_Get_texture_descriptor(*this),
                                                                                         this->get_data_length());

        Concurrency::graphics::details::_Copy_async_impl(_Src_first, _Src_last, *this, index<_Rank>(), this->extent)._Get();

        Concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    template<typename _Input_iterator>
    void _Initialize(const Concurrency::accelerator_view& _Av, _Input_iterator _Src_first, _Input_iterator _Src_last) __CPU_ONLY
    {
        _Initialize(_Av, _Av, _Src_first, _Src_last);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element) __CPU_ONLY
    {
        _Initialize(_Av, _Associated_av, _Bits_per_scalar_element);
        Concurrency::graphics::copy(_Source, _Src_byte_size, *this);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const void * _Source, unsigned int _Src_byte_size, unsigned int _Bits_per_scalar_element) __CPU_ONLY
    {
        _Initialize(_Av, _Av, _Source, _Src_byte_size, _Bits_per_scalar_element);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av, const void * _Source, unsigned int _Src_byte_size) __CPU_ONLY
    {
        _Initialize(_Av, _Associated_av);
        Concurrency::graphics::copy(_Source, _Src_byte_size, *this);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const void * _Source, unsigned int _Src_byte_size) __CPU_ONLY
    {
        _Initialize(_Av, _Av, _Source, _Src_byte_size);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const Concurrency::accelerator_view& _Associated_av, const details::_Texture_base<_Value_type, _Rank> & _Src) __CPU_ONLY
    {
        if (_Src.bits_per_scalar_element != 0) // _Src is not created via interop
        {
            _Initialize(_Av, _Associated_av, _Src.bits_per_scalar_element);
        }
        else // _Src is created via interop, create a new texture with the same properties as the existing one.
        {
            _Texture_ptr _New_tex;
            if (_Should_create_staging_texture(_Av, _Associated_av))
            {
                 _New_tex = _Texture::_Clone_texture(concurrency::details::_Get_texture(_Src), _Associated_av, _Av);
            }
            else
            {
                _New_tex = _Texture::_Clone_texture(concurrency::details::_Get_texture(_Src),  _Av, _Associated_av);
            }
            this->_M_texture_descriptor._Set_texture_ptr(_New_tex);
        }

        auto _Span_id = Concurrency::details::_Get_amp_trace()->_Start_copy_event_helper(Concurrency::details::_Get_texture_descriptor(_Src),
                                                                                         Concurrency::details::_Get_texture_descriptor(*this),
                                                                                         this->get_data_length());

        Concurrency::graphics::details::_Copy_async_impl(_Src, index<_Rank>(), *this, index<_Rank>(), this->extent)._Get();

        Concurrency::details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    void _Initialize(const Concurrency::accelerator_view& _Av, const details::_Texture_base<_Value_type, _Rank> & _Src) __CPU_ONLY
    {
        _Initialize(_Av, _Av, _Src);
    }
};

/// <summary>
///     A writeonly_texture_view provides writeonly access to a texture.
/// </summary>
/// <param name="_Value_type">
///     The type of the elements in the texture aggregates.
/// </param>
/// <param name="_Rank">
///     The _Rank of the corresponding extent domain.
/// </param>
template <typename _Value_type, int _Rank> class __declspec(deprecated("writeonly_texture_view is deprecated. Please use texture_view instead.")) writeonly_texture_view : public details::_Texture_base<_Value_type, _Rank>
{
    static_assert(!std::is_const<_Value_type>::value, "const value type is not supported for writeonly_texture_view.");

    using _Texture_base = details::_Texture_base<_Value_type, _Rank>;
public:
    /// <summary>
    ///     Construct a writeonly_texture_view of a texture _Src.
    /// </summary>
    /// <param name="_Src">
    ///     The texture on which the writeonly view is created.
    /// </param>
    writeonly_texture_view(texture<_Value_type, _Rank>& _Src) __CPU_ONLY
        : _Texture_base(_Src, /*_Most_detailed_mipmap_level=*/0, /*_View_mipmap_levels=*/1)
    {
        _Texture* _Tex = this->_Get_texture();
        if ((_Tex->_Get_num_channels() == 3) && (_Tex->_Get_bits_per_channel() == 32)) {
            throw runtime_exception("writeonly_texture_view cannot be created from a 3-channel texture with 32 bits per scalar element.", E_INVALIDARG);
        }
        if (_Tex->_Is_staging()) {
            throw runtime_exception("writeonly_texture_view cannot be created from a staging texture object.", E_INVALIDARG);
        }
    }

    /// <summary>
    ///     Construct a writeonly_texture_view of a texture _Src.
    /// </summary>
    /// <param name="_Src">
    ///     The texture on which the writeonly view is created.
    /// </param>
    writeonly_texture_view(texture<_Value_type, _Rank>& _Src) __GPU_ONLY
        : _Texture_base(_Src, /*_Flatten_mipmap_levels=*/true)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Num_channels == 1, "Invalid value_type for the constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "Invalid value_type for the constructor.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "Invalid value_type for the constructor.");
    }

    /// <summary>
    ///     Construct a writeonly_texture_view from another writeonly_texture_view. Both are views of the same texture.
    /// </summary>
    /// <param name="_Src">
    ///     The writeonly_texture_view from which the current view is created.
    /// </param>
    writeonly_texture_view(const writeonly_texture_view<_Value_type, _Rank>& _Src) __GPU
        : _Texture_base(_Src)
    {
    }

    /// <summary>
    ///     Assignment operator. This writeonly_texture_view becomes a view of the same texture which _Other is a view of.
    /// </summary>
    /// <param name="_Other">
    ///     The source writeonly_texture_view.
    /// </param>
    writeonly_texture_view<_Value_type, _Rank>& operator=(const writeonly_texture_view<_Value_type, _Rank>& _Other) __GPU
    {
        if (this != &_Other)
        {
            this->_M_extent = _Other._M_extent;
            this->_M_texture_descriptor = _Other._M_texture_descriptor;
        }
        return *this;
    }

    /// <summary>
    ///     Destructor
    /// </summary>
    ~writeonly_texture_view() __GPU
    {
    }

    /// <summary>
    ///     Set the element indexed by _Index with value _Value.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <param name="_Value">
    ///     The value to be set to the element indexed by _Index.
    /// </param>
    void set(const index<_Rank>& _Index, const _Value_type& _Value) const __GPU_ONLY
    {
        _Texture_write_helper<index<_Rank>, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Value, _Index);
    }
};

/// <summary>
///     A texture_view provides read and write access to a texture.
///     Note that currently texture_view can only be used to read textures whose value type is int, unsigned int and float
///     with default 32 bit bpse. To read other texture formats, use texture_view&lt;const _Value_type, _Rank&gt;.
/// </summary>
/// <param name="_Value_type">
///     The type of the elements in the texture aggregates.
/// </param>
/// <param name="_Rank">
///     The _Rank of the corresponding extent domain.
/// </param>
template <typename _Value_type, int _Rank> class texture_view : public details::_Texture_base<_Value_type, _Rank>
{
    friend class texture_view<const _Value_type, _Rank>;

    using _Texture_base = details::_Texture_base<_Value_type, _Rank>;
public:
    /// <summary>
    ///     Construct a texture_view of a texture _Src on host.
    /// </summary>
    /// <param name="_Src">
    ///     The texture on which the texture_view is created.
    /// </param>
    /// <param name="_Mipmap_level">
    ///     The specific mipmap level on a _Src texture that this read and write texture_view should bind to.
    ///     The default value 0, binds to the top mosted detail mipmap level.
    /// </param>
    texture_view(texture<_Value_type, _Rank>& _Src, unsigned int _Mipmap_level = 0) __CPU_ONLY
        : _Texture_base(_Src, _Mipmap_level, /*_View_mipmap_levels=*/1)
    {
        if (this->_Get_texture()->_Is_staging()) {
            throw runtime_exception("texture_view cannot be created from a staging texture object.", E_INVALIDARG);
        }
    }

    /// <summary>
    ///     Construct a texture_view of a texture _Src on an accelerator.
    /// </summary>
    /// <param name="_Src">
    ///     The texture on which the texture_view is created.
    /// </param>
    texture_view(texture<_Value_type, _Rank>& _Src) __GPU_ONLY
        : _Texture_base(_Src, /*_Flatten_mipmap_levels=*/true)
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Num_channels == 1, "writable texture_view can only be created from a single-component texture on an accelerator.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "writable texture_view cannot be created from a unorm texture on an accelerator.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "writable texture_view cannot be created from a norm texture on an accelerator.");
    }

    /// <summary>
    ///     Construct a texture_view from another texture_view. Both are views of the same texture.
    /// </summary>
    /// <param name="_Other">
    ///     The source texture_view.
    /// </param>
    texture_view(const texture_view<_Value_type, _Rank>& _Other) __GPU
        : _Texture_base(_Other)
    {
    }

    /// <summary>
    ///     Assignment operator. This texture_view becomes a view of the same texture which _Other is a view of.
    /// </summary>
    /// <param name="_Other">
    ///     The source texture_view.
    /// </param>
    texture_view<_Value_type, _Rank>& operator=(const texture_view<_Value_type, _Rank>& _Other) __GPU
    {
        if (this != &_Other)
        {
            this->_M_extent = _Other._M_extent;
            this->_M_texture_descriptor = _Other._M_texture_descriptor;
        }
        return *this;
    }

    /// <summary>
    ///     Destructor
    /// </summary>
    ~texture_view() __GPU
    {
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type operator[] (const index<_Rank>& _Index) const __GPU_ONLY
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Num_channels == 1, "Read is only permissible on single-component writable texture_view.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Unorm_type, "Read is not permissible on a writable unorm texture_view.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Norm_type, "Read is not permissible on a writable norm texture_view.");

        _Value_type _Tmp;
        _Texture_read_helper<index<_Rank>, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Tmp, _Index, /*_Mip_level=*/0);
        return _Tmp;
    }

    /// <summary>
    ///     Get the element value indexed by _I0.
    /// </summary>
    /// <param name="_I0">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I0.
    /// </returns>
    const _Value_type operator[] (int _I0) const __GPU_ONLY
    {
        static_assert(_Rank == 1, "const value_type operator[](int) is only permissible on texture_view<value_type, 1>.");
        return (*this)[index<1>(_I0)];
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type operator() (const index<_Rank>& _Index) const __GPU_ONLY
    {
        return (*this)[_Index];
    }

    /// <summary>
    ///     Get the element value indexed by _I0
    /// </summary>
    /// <param name="_I0">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I0.
    /// </returns>
    const _Value_type operator() (int _I0) const __GPU_ONLY
    {
        static_assert(_Rank == 1, "const value_type operator()(int) is only permissible on texture_view<value_type, 1>.");
        return (*this)[index<1>(_I0)];
    }

    /// <summary>
    ///     Get the element value indexed by (_I0,_I1)
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the index
    /// </param>
    /// <param name="_I1">
    ///     The least-significant component of the index
    /// </param>
    /// <returns>
    ///     The element value indexed by (_I0,_I1)
    /// </returns>
    const _Value_type operator() (int _I0, int _I1) const __GPU_ONLY
    {
        static_assert(_Rank == 2, "const value_type operator()(int, int) is only permissible on texture_view<value_type, 2>.");
        return (*this)[index<2>(_I0, _I1)];
    }

    /// <summary>
    ///     Get the element value indexed by (_I0,_I1,_I2)
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the index
    /// </param>
    /// <param name="_I1">
    ///     The next-to-most-significant component of the index
    /// </param>
    /// <param name="_I2">
    ///     The least-significant component of the index
    /// </param>
    /// <returns>
    ///     The element value indexed by (_I0,_I1,_I2)
    /// </returns>
    const _Value_type operator() (int _I0, int _I1, int _I2) const __GPU_ONLY
    {
        static_assert(_Rank == 3, "const value_type operator()(int, int, int) is only permissible on texture_view<value_type, 3>.");
        return (*this)[index<3>(_I0, _I1, _I2)];
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type get(const index<_Rank>& _Index) const __GPU_ONLY
    {
        return (*this)[_Index];
    }

    /// <summary>
    ///     Set the element indexed by _Index with value _Value.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <param name="_Value">
    ///     The value to be set to the element indexed by _Index.
    /// </param>
    void set(const index<_Rank>& _Index, const _Value_type& _Value) const __GPU_ONLY
    {
        _Texture_write_helper<index<_Rank>, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Value, _Index);
    }
};

/// <summary>
///    filter modes supported for texture sampling
/// </summary>
enum filter_mode
{
    filter_point    = 0,
    filter_linear   = 0x15,
    filter_unknown  = 0xFFFFFFFF,
};

/// <summary>
///    address modes supported for texture sampling
/// </summary>
enum address_mode
{
    address_wrap    = 1,
    address_mirror  = 2,
    address_clamp   = 3,
    address_border  = 4,
    address_unknown = 0xFFFFFFFF,
};

/// <summary>
///     A sampler class aggregates sampling configuration information to be used for texture sampling.
/// </summary>
class sampler
{
    friend sampler direct3d::make_sampler(_In_ IUnknown *_D3D_sampler) __CPU_ONLY;
    friend _Ret_ IUnknown * direct3d::get_sampler(const Concurrency::accelerator_view &_Av, const sampler &_Sampler) __CPU_ONLY;

    template <typename _Value_type, int _Rank>
    friend class texture_view;

public:
    /// <summary>
    ///     Constructs a sampler with default filter mode (filter_lienar, same for min, mag, mip), addressing
    ///     mode (address_clamp, same for all dimensions), and border color (float_4(0.0f, 0.0f, 0.0f, 0.0f)).
    /// </summary>
    sampler() __CPU_ONLY
        : _M_filter_mode(filter_linear),
          _M_address_mode(address_clamp),
          _M_border_color(float_4(0.0f, 0.0f, 0.0f, 0.0f))
    {
        _Initialize();
    }

    /// <summary>
    ///     Constructs a sampler with specified filter mode (same for min, mag, mip), but with default addressing
    ///     mode (address_clamp, same for all dimensions) and border color ( float_4(0.0f, 0.0f, 0.0f, 0.0f)).
    /// </summary>
    /// <param name="_Filter_mode">
    ///     The filter mode to be used in sampling.
    /// </param>
    sampler(filter_mode _Filter_mode)__CPU_ONLY
        : _M_filter_mode(_Filter_mode),
          _M_address_mode(address_clamp),
          _M_border_color(float_4(0.0f, 0.0f, 0.0f, 0.0f))
    {
        _Initialize();
    }

    /// <summary>
    ///     Constructs a sampler with default filter mode (filter_linear, same for min, mag, mip), but specified
    ///     addressing mode (same for all dimensions) and border color.
    /// </summary>
    /// <param name="_Address_mode">
    ///     The addressing mode to be used in sampling for all dimensions.
    /// </param>
    /// <param name="_Border_color">
    ///     The border color to be used if address mode is address_border. If not specified, default value is float_4(0.f, 0.f, 0.f, 0.f).
    /// </param>
    sampler(address_mode _Address_mode, float_4 _Border_color = float_4(0.0f, 0.0f, 0.0f, 0.0f)) __CPU_ONLY
        : _M_filter_mode(filter_linear),
          _M_address_mode(_Address_mode),
          _M_border_color(_Border_color)
    {
        _Initialize();
    }

    /// <summary>
    ///     Constructs a sampler with specified filter mode (same for min, mag, mip), addressing
    ///     mode (same for all dimensions) and the border color.
    /// </summary>
    /// <param name="_Filter_mode">
    ///     The filter mode to be used in sampling.
    /// </param>
    /// <param name="_Address_mode">
    ///     The addressing mode to be used in sampling for all dimensions.
    /// </param>
    /// <param name="_Border_color">
    ///     The border color to be used if address mode is address_border. If not specified, default value is float_4(0.f, 0.f, 0.f, 0.f).
    /// </param>
    sampler(filter_mode _Filter_mode, address_mode _Address_mode, float_4 _Border_color = float_4(0.0f, 0.0f, 0.0f, 0.0f)) __CPU_ONLY
        : _M_filter_mode(_Filter_mode),
          _M_address_mode(_Address_mode),
          _M_border_color(_Border_color)
    {
        _Initialize();
    }

    /// <summary>
    ///     Copy constructor.
    /// </summary>
    /// <param name="_Other">
    ///     An object of type sampler from which to initialize this new sampler.
    /// </param>
    sampler(const sampler& _Other) __GPU
        : _M_sampler_descriptor(_Other._M_sampler_descriptor),
          _M_filter_mode(_Other._M_filter_mode),
          _M_address_mode(_Other._M_address_mode),
          _M_border_color(_Other._M_border_color)
    {
    }

    /// <summary>
    ///     Move constructor.
    /// </summary>
    /// <param name="_Other">
    ///     The sampler to move from.
    /// </param>
    sampler(sampler &&_Other) __GPU
        : _M_sampler_descriptor(_Other._M_sampler_descriptor),
          _M_filter_mode(_Other._M_filter_mode),
          _M_address_mode(_Other._M_address_mode),
          _M_border_color(_Other._M_border_color)
    {
        _Other._M_sampler_descriptor._M_data_ptr = NULL;
        _Other._M_sampler_descriptor._Set_sampler_ptr(NULL);
    }

    /// <summary>
    ///     Assignment operator.
    /// </summary>
    /// <param name="_Other">
    ///     An object of type sampler from which to copy into this sampler.
    /// </param>
    /// <returns>
    ///     A reference to this sampler.
    /// </returns>
    sampler& operator=(const sampler& _Other) __GPU
    {
        if (this != &_Other)
        {
            _M_filter_mode = _Other._M_filter_mode;
            _M_address_mode = _Other._M_address_mode;
            _M_border_color = _Other._M_border_color;
            _M_sampler_descriptor = _Other._M_sampler_descriptor;
        }
        return *this;
    }

    /// <summary>
    ///     Move assignment operator.
    /// </summary>
    /// <param name="_Other">
    ///     An object of type sampler to move from.
    /// </param>
    /// <returns>
    ///     A reference to this sampler.
    /// </returns>
    sampler& operator=(sampler&& _Other) __GPU
    {
        if (this != &_Other)
        {
            _M_filter_mode = _Other._M_filter_mode;
            _M_address_mode = _Other._M_address_mode;
            _M_border_color = _Other._M_border_color;
            _M_sampler_descriptor = _Other._M_sampler_descriptor;
            _Other._M_sampler_descriptor._M_data_ptr = NULL;
            _Other._M_sampler_descriptor._Set_sampler_ptr(NULL);
        }
        return *this;
    }

    /// <summary>
    ///     Returns the sampler's filter mode
    /// </summary>
    __declspec(property(get=get_filter_mode)) Concurrency::graphics::filter_mode filter_mode;
    Concurrency::graphics::filter_mode get_filter_mode() const __GPU
    {
        return _M_filter_mode;
    }

    /// <summary>
    ///     Returns the sampler's address mode
    /// </summary>
    __declspec(property(get=get_address_mode)) Concurrency::graphics::address_mode address_mode;
    Concurrency::graphics::address_mode get_address_mode() const __GPU
    {
        return _M_address_mode;
    }

    /// <summary>
    ///     Returns the sampler's border value
    /// </summary>
    __declspec(property(get=get_border_color)) Concurrency::graphics::float_4 border_color;
    Concurrency::graphics::float_4 get_border_color() const __GPU
    {
        return _M_border_color;
    }

private:
    // internal storage abstraction
    typedef Concurrency::details::_Sampler_descriptor _Sampler_descriptor;

    // a private constructor to be used for constructing a sampler via interop.
    sampler(const _Sampler_descriptor & _Descriptor) __CPU_ONLY
        : _M_sampler_descriptor(_Descriptor),
          _M_filter_mode(filter_unknown),
          _M_address_mode (address_unknown),
          _M_border_color(float_4(0.0f, 0.0f, 0.0f, 0.0f))
    {
        // Although we could query border value from the adopted sampler, but it's not that useful
        // given that this is the only thing that we could query and when the address mode is not
        // address_border, border value is not relevant.
    }

    _Ret_ _Sampler* _Get_sampler_ptr() const __CPU_ONLY
    {
        return _M_sampler_descriptor._Get_sampler_ptr();
    }

    void _Initialize() __CPU_ONLY
    {
        // Check if the given filter_mode and address_mode are valid C++ AMP ones
        if ((_M_filter_mode != filter_point && _M_filter_mode != filter_linear) ||
           (_M_address_mode != address_wrap && _M_address_mode != address_mirror &&
            _M_address_mode != address_clamp && _M_address_mode != address_border))
        {
            throw runtime_exception("Invalid sampler configuration", E_INVALIDARG);
        }

        _Sampler_ptr samplerPtr = _Sampler::_Create(_M_filter_mode, _M_address_mode,
                            _M_border_color.r, _M_border_color.g, _M_border_color.b, _M_border_color.a);
        _M_sampler_descriptor._Set_sampler_ptr(samplerPtr);
    }

    const _Sampler_descriptor & _Get_descriptor() const __GPU_ONLY
    {
        return _M_sampler_descriptor;
    }

    _Sampler_descriptor _M_sampler_descriptor;
    Concurrency::graphics::filter_mode _M_filter_mode;
    Concurrency::graphics::address_mode _M_address_mode;
    float_4 _M_border_color;
};

/// <summary>
///     A texture_view&lt;const _Value_type, _Rank&gt; provides read-only access and sampling capability to a texture.
/// </summary>
/// <param name="_Value_type">
///     The type of the elements in the texture aggregates.
/// </param>
/// <param name="_Rank">
///     The _Rank of the corresponding extent domain.
/// </param>
template <typename _Value_type, int _Rank> class texture_view<const _Value_type, _Rank> : public details::_Texture_base<_Value_type, _Rank>
{
public:
    typedef const _Value_type value_type;
    typedef typename short_vector<float, _Rank>::type coordinates_type;
    typedef typename short_vector<typename details::_Texture_base<_Value_type, _Rank>::scalar_type, 4>::type gather_return_type;

    using _Texture_base = details::_Texture_base<_Value_type, _Rank>;
    /// <summary>
    ///     Construct a read-only texture_view of a texture _Src on an accelerator.
    /// </summary>
    /// <param name="_Src">
    ///     The texture on which the read-only view is created.
    /// </param>
    texture_view(const texture<_Value_type, _Rank>& _Src) __GPU_ONLY
        : _Texture_base(_Src)
    {
        // only on the gpu it is not allowed
        static_assert(_Short_vector_type_traits<_Value_type>::_Num_channels != 1, "Read-only texture_view cannot be created from single-component textures on an accelerator.");
    }

    /// <summary>
    ///     Construct a texture_view of a texture _Src on the host.
    /// </summary>
    /// <param name="_Src">
    ///     The texture on which the read-only view is created.
    /// </param>
    texture_view(const texture<_Value_type, _Rank>& _Src) __CPU_ONLY
        : _Texture_base(_Src)
    {
        if (this->_Get_texture()->_Is_staging()) {
            throw runtime_exception("Read-only texture_view cannot be created from a staging texture object.", E_INVALIDARG);
        }
    }

    /// <summary>
    ///     Construct a read-only texture_view with specific range of mipmap levels of a texture _Src on the host.
    /// </summary>
    /// <param name="_Src">
    ///     The texture on which the read-only view is created.
    /// </param>
    /// <param name="_Most_detailed_mip">
    ///     Most detailed mipmap level for the view.
    /// </param>
    /// <param name="_Mip_levels">
    ///     The number of mipmap levels accessible for the view.
    /// </param>
    texture_view(const texture<_Value_type, _Rank>& _Src, unsigned int _Most_detailed_mip, unsigned int _Mip_levels) __CPU_ONLY
        : _Texture_base(_Src, _Most_detailed_mip, _Mip_levels)
    {
        if (this->_Get_texture()->_Is_staging()) {
            throw runtime_exception("Read-only texture_view cannot be created from a staging texture object.", E_INVALIDARG);
        }
    }

    /// <summary>
    ///     Construct a read-only texture_view of a writable texture_view.
    /// </summary>
    /// <param name="_Other">
    ///     The writable texture view from which the read-only view is created.
    /// </param>
    texture_view(const texture_view<_Value_type, _Rank>& _Other) __CPU_ONLY
        : _Texture_base(_Other)
    {
    }

    /// <summary>
    ///     Construct a read-only texture_view from another read-only texture_view. Both are views of the same texture.
    /// </summary>
    /// <param name="_Other">
    ///     The source read-only texture_view.
    /// </param>
    texture_view(const texture_view<const _Value_type, _Rank>& _Other) __GPU
        : _Texture_base(_Other)
    {
    }

    /// <summary>
    ///     Construct a read-only texture_view from another read-only texture_view.
    ///      Allows narrowing down the accessible range of mipmap levels for the texture_view.
    ///      Both are views of the same texture.
    /// </summary>
    /// <param name="_Other">
    ///     The source read-only texture_view.
    /// </param>
    /// <param name="_Most_detailed_mip">
    ///     Top level mipmap for the view, relative to the input texture_view.
    /// </param>
    /// <param name="_Mip_levels">
    ///     The number of mipmap levels accessible for the view.
    /// </param>
    texture_view(const texture_view<const _Value_type, _Rank>& _Other, unsigned int _Most_detailed_mip, unsigned int _Mip_levels) __CPU_ONLY
        : _Texture_base(_Other, _Most_detailed_mip, _Mip_levels)
    {
    }

    /// <summary>
    ///     Assignment operator. This read-only texture_view becomes a view of the same texture which _Other is a view of.
    /// </summary>
    /// <param name="_Other">
    ///     The source read-only texture_view.
    /// </param>
    texture_view<const _Value_type, _Rank>& operator=(const texture_view<const _Value_type, _Rank>& _Other) __GPU
    {
        if (this != &_Other)
        {
            this->_M_extent = _Other._M_extent;
            this->_M_texture_descriptor = _Other._M_texture_descriptor;
        }
        return *this;
    }

    /// <summary>
    ///     Assignment operator from a writable texture_view.
    ///     This read-only texture_view becomes a view of the same texture which _Other is a view of.
    /// </summary>
    /// <param name="_Other">
    ///     The source writable texture_view.
    /// </param>
    texture_view<const _Value_type, _Rank>& operator=(const texture_view<_Value_type, _Rank>& _Other) __CPU_ONLY
    {
        this->_M_extent = _Other._M_extent;
        this->_M_texture_descriptor = _Other._M_texture_descriptor;
        return *this;
    }

    /// <summary>
    ///     Destructor
    /// </summary>
    ~texture_view() __GPU
    {
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type operator[] (const index<_Rank>& _Index) const __GPU_ONLY
    {
        _Value_type _Tmp;
        _Texture_read_helper<index<_Rank>, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Tmp, _Index, /*_Mip_level=*/0);
        return _Tmp;
    }

    /// <summary>
    ///     Get the element value indexed by _I.
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I.
    /// </returns>
    const _Value_type operator[] (int _I0) const __GPU_ONLY
    {
        static_assert(_Rank == 1, "value_type operator[](int) is only permissible on texture_view<value_type, 1>.");
        return (*this)[index<1>(_I0)];
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type operator() (const index<_Rank>& _Index) const __GPU_ONLY
    {
        return (*this)[_Index];
    }

    /// <summary>
    ///     Get the element value indexed by _I0
    /// </summary>
    /// <param name="_I0">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I0.
    /// </returns>
    const _Value_type operator() (int _I0) const __GPU_ONLY
    {
        static_assert(_Rank == 1, "value_type texture_view::operator()(int) is only permissible on texture_view<value_type, 1>.");
        return (*this)[index<1>(_I0)];
    }

    /// <summary>
    ///     Get the element value indexed by (_I0,_I1)
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the index
    /// </param>
    /// <param name="_I1">
    ///     The least-significant component of the index
    /// </param>
    /// <returns>
    ///     The element value indexed by (_I0,_I1)
    /// </returns>
    const _Value_type operator() (int _I0, int _I1) const __GPU_ONLY
    {
        static_assert(_Rank == 2, "value_type texture_view::operator()(int, int) is only permissible on texture_view<value_type, 2>.");
        return (*this)[index<2>(_I0, _I1)];
    }

    /// <summary>
    ///     Get the element value indexed by (_I0,_I1,_I2)
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the index
    /// </param>
    /// <param name="_I1">
    ///     The next-to-most-significant component of the index
    /// </param>
    /// <param name="_I2">
    ///     The least-significant component of the index
    /// </param>
    /// <returns>
    ///     The element value indexed by (_I0,_I1,_I2)
    /// </returns>
    const _Value_type operator() (int _I0, int _I1, int _I2) const __GPU_ONLY
    {
        static_assert(_Rank == 3, "value_type texture_view::operator()(int, int, int) is only permissible on texture_view<value_type, 3>.");
        return (*this)[index<3>(_I0, _I1, _I2)];
    }

    /// <summary>
    ///     Get the element value indexed by _Index.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <param name="_Mip_level">
    ///     The mipmap level from which we should get indexed value.
    ///     The default value 0 represents most detailed mipmap level.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index.
    /// </returns>
    const _Value_type get(const index<_Rank>& _Index, unsigned int _Mip_level = 0) const __GPU_ONLY
    {
        _Value_type _Tmp;
        _Texture_read_helper<index<_Rank>, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Tmp, _Index, _Mip_level);
        return _Tmp;
    }

    /// <summary>
    ///     Sample the texture at the given coordinates and level of detail using the specified sampling configuration.
    /// </summary>
    /// <param name="_Sampler">
    ///     The sampler that configures the sampling operation.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <param name="_Level_of_detail">
    ///     The value specifies the mipmap level to sample from.
    ///     Fractional value is used to interpolate between two mipmap levels.
    /// </param>
    /// <returns>
    ///     The interpolated value.
    /// </returns>
    const _Value_type sample(const sampler& _Sampler, const coordinates_type& _Coord, float _Level_of_detail = 0.0f) const __GPU_ONLY
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Uint_type, "sample is not allowed for uint component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Int_type, "sample is not allowed for int component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Double_type, "sample is not allowed for double component types in the texture value_type.");

        _Value_type _Tmp;
        _Texture_sample_helper<coordinates_type, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, _Sampler._Get_descriptor()._M_data_ptr, &_Tmp, _Coord, 4 /*Sampling*/, _Level_of_detail);
        return _Tmp;
    }

    /// <summary>
    ///     Sample the texture at the given coordinates and level of detail using the predefined sampling configuration.
    /// </summary>
    /// <param name="_Filter_mode">
    ///     The filter mode of the predefined sampler to be used.
    /// </param>
    /// <param name="_Address_mode">
    ///     The address mode of the predefined sampler to be used.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <param name="_Level_of_detail">
    ///     The value specifies the mipmap level to sample from.
    ///     Fractional value is used to interpolate between two mipmap levels.
    /// </param>
    /// <returns>
    ///     The interpolated value.
    /// </returns>
    template<filter_mode _Filter_mode = filter_linear, address_mode _Address_mode = address_clamp>
    const _Value_type sample(const coordinates_type& _Coord, float _Level_of_detail = 0.0f) const __GPU_ONLY
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Uint_type, "sample is not allowed for uint component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Int_type, "sample is not allowed for int component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Double_type, "sample is not allowed for double component types in the texture value_type.");
        static_assert((_Filter_mode == filter_point || _Filter_mode == filter_linear), "Invalid filter mode for sample method.");
        static_assert((_Address_mode == address_wrap || _Address_mode == address_clamp || _Address_mode == address_mirror || _Address_mode == address_border),
                                "Invalid address mode for sample method.");

        _Value_type _Tmp;
        // Predefined sampler id is constructed as filter_mode << 16 | address_mode. This is a contract between BE and runtime. Modify with caution!
        _Texture_predefined_sample_helper<coordinates_type, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Tmp, _Coord, _Filter_mode << 16 |_Address_mode, 4 /*Sampling*/, _Level_of_detail);
        return _Tmp;
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the specified sampling configuration and return the red (x) component of the four texels samples.
    /// </summary>
    /// <param name="_Sampler">
    ///     The sampler that configures the sampling operation.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the red (x) component of the 4 texel values sampled.
    /// </returns>
    const gather_return_type gather_red(const sampler& _Sampler, const coordinates_type& _Coord) const __GPU_ONLY
    {
        return _Gather(_Sampler, _Coord, 0);
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the specified sampling configuration and return the green (y) component of the four texels samples.
    /// </summary>
    /// <param name="_Sampler">
    ///     The sampler that configures the sampling operation.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the green (y) component of the 4 texel values sampled.
    /// </returns>
    const gather_return_type gather_green(const sampler& _Sampler, const coordinates_type& _Coord) const __GPU_ONLY
    {
        static_assert(1 < _Short_vector_type_traits<_Value_type>::_Num_channels, "gather_green is valid only for textures with 2 or more components in the value_type.");

        return _Gather(_Sampler, _Coord, 1);
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the specified sampling configuration and return the blue (z) component of the four texels samples.
    /// </summary>
    /// <param name="_Sampler">
    ///     The sampler that configures the sampling operation.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the blue (z) component of the 4 texel values sampled.
    /// </returns>
    const gather_return_type gather_blue(const sampler& _Sampler, const coordinates_type& _Coord) const __GPU_ONLY
    {
        static_assert(2 < _Short_vector_type_traits<_Value_type>::_Num_channels, "gather_blue is valid only for textures with 3 or more components in the value_type.");

        return _Gather(_Sampler, _Coord, 2);
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the specified sampling configuration and return the alpha (w) component of the four texels samples.
    /// </summary>
    /// <param name="_Sampler">
    ///     The sampler that configures the sampling operation.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the alpha (w) component of the 4 texel values sampled.
    /// </returns>
    const gather_return_type gather_alpha(const sampler& _Sampler, const coordinates_type& _Coord) const __GPU_ONLY
    {
        static_assert(3 < _Short_vector_type_traits<_Value_type>::_Num_channels, "gather_alpha is valid only for textures with 4 components in the value_type.");

        return _Gather(_Sampler, _Coord, 3);
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the predefined sampling configuration and return the red (x) component of the four texels samples.
    /// </summary>
    /// <param name="_Address_mode">
    ///     The address mode of the predefined sampler to be used.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the red (x) component of the 4 texel values sampled.
    /// </returns>
    template<address_mode _Address_mode = address_clamp>
    const gather_return_type gather_red(const coordinates_type& _Coord) const __GPU_ONLY
    {
        return _Gather<_Address_mode>(_Coord, 0);
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the predefined sampling configuration and return the green (y) component of the four texels samples.
    /// </summary>
    /// <param name="_Address_mode">
    ///     The address mode of the predefined sampler to be used.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the green (y)component of the 4 texel values sampled.
    /// </returns>
    template<address_mode _Address_mode = address_clamp>
    const gather_return_type gather_green(const coordinates_type& _Coord) const __GPU_ONLY
    {
        static_assert(1 < _Short_vector_type_traits<_Value_type>::_Num_channels, "gather_green is valid only for textures with 2 or more components in the value_type.");

        return _Gather<_Address_mode>(_Coord, 1);
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the predefined sampling configuration and return the blue (z) component of the four texels samples.
    /// </summary>
    /// <param name="_Address_mode">
    ///     The address mode of the predefined sampler to be used.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the blue (z) component of the 4 texel values sampled.
    /// </returns>
    template<address_mode _Address_mode = address_clamp>
    const gather_return_type gather_blue(const coordinates_type& _Coord) const __GPU_ONLY
    {
        static_assert(2 < _Short_vector_type_traits<_Value_type>::_Num_channels, "gather_blue is valid only for textures with 3 or more components in the value_type.");

        return _Gather<_Address_mode>(_Coord, 2);
    }

    /// <summary>
    ///     Sample the texture at the given coordinates using the predefined sampling configuration and return the alpha (w) component of the four texels samples.
    /// </summary>
    /// <param name="_Address_mode">
    ///     The address mode of the predefined sampler to be used.
    /// </param>
    /// <param name="_Coord">
    ///     Coordinate vector for sampling.
    /// </param>
    /// <returns>
    ///     Rank 4 short vector containing the alpha (w) component of the 4 texel values sampled.
    /// </returns>
    template<address_mode _Address_mode = address_clamp>
    const gather_return_type gather_alpha(const coordinates_type& _Coord) const __GPU_ONLY
    {
        static_assert(3 < _Short_vector_type_traits<_Value_type>::_Num_channels, "gather_alpha is valid only for textures with 4 components in the value_type.");

        return _Gather<_Address_mode>(_Coord, 3);
    }

private:
    const gather_return_type _Gather(const sampler& _Sampler, const coordinates_type& _Coord, unsigned int _Component) const __GPU_ONLY
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Uint_type, "gather is not allowed for uint component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Int_type, "gather is not allowed for int component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Double_type, "gather is not allowed for double component types in the texture value_type.");
        static_assert(_Rank == 2, "gather methods are only permissible on texture_view<value_type, 2>.");

        gather_return_type _Tmp;
        _Texture_sample_helper<coordinates_type, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, _Sampler._Get_descriptor()._M_data_ptr, &_Tmp, _Coord, _Component, /*_Level_of_detail=*/0.0f);
        return _Tmp;
    }

    template<address_mode _Address_mode>
    const gather_return_type _Gather(const coordinates_type& _Coord, unsigned int _Component) const __GPU_ONLY
    {
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Uint_type, "gather is not allowed for uint component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Int_type, "gather is not allowed for int component types in the texture value_type.");
        static_assert(_Short_vector_type_traits<_Value_type>::_Format_base_type_id != _Double_type, "gather is not allowed for double component types in the texture value_type.");
        static_assert(_Rank == 2, "gather methods are only permissible on texture_view<value_type, 2>.");
        static_assert((_Address_mode == address_wrap || _Address_mode == address_clamp || _Address_mode == address_mirror || _Address_mode == address_border),
                                "Invalid address mode for gather methods.");

        gather_return_type _Tmp;
        // Predefined sampler id is constructed as filter_mode << 16 | address_mode. This is a contract between BE and runtime. Modify with caution!
        // gather only used the address_mode of the sampler, internally we use filter_point so that the predefined sampler id scheme is same for both sample and gather.
        _Texture_predefined_sample_helper<coordinates_type, _Rank>::func(this->_M_texture_descriptor._M_data_ptr, &_Tmp, _Coord, filter_point << 16 |_Address_mode, _Component, /*_Level_of_detail=*/0.0f);
        return _Tmp;
    }
};


namespace details
{
template<int _Rank>
Concurrency::extent<_Rank> _Make_texture(const Concurrency::accelerator_view &_Av, _In_ IUnknown *_D3D_texture, _Texture_base_type_id _Id, _Inout_ _Texture ** _Tex, DXGI_FORMAT _View_format) __CPU_ONLY
{
    if (_D3D_texture == NULL)
    {
        throw runtime_exception("NULL D3D texture pointer.", E_INVALIDARG);
    }

    if (!Concurrency::details::_Is_D3D_accelerator_view(_Av)) {
        throw runtime_exception("Cannot create D3D texture on a non-D3D accelerator_view.", E_INVALIDARG);
    }

    _Texture * _Tex_ptr = _Texture::_Adopt_texture(_Rank, _Id, _D3D_texture, _Av, _View_format);
    if (_Tex_ptr->_Is_staging())
    {
        _Tex_ptr->_Map_buffer(_Write_access, true /* _Wait */);
    }
    Concurrency::extent<_Rank> _Ext = Concurrency::graphics::details::_Create_extent<_Rank>(_Tex_ptr->_Get_width(), _Tex_ptr->_Get_height(), _Tex_ptr->_Get_depth());

    _Is_valid_extent(_Ext);
    details::_Is_valid_data_length(_Ext.size(), _Tex_ptr->_Get_bits_per_element());

    *_Tex = _Tex_ptr;
    return _Ext;
}

} // namespace details

namespace direct3d
{
    /// <summary>
    ///     Get the D3D texture interface underlying a texture.
    /// </summary>
    /// <param name="_Rank">
    ///     The rank of the texture to get underlying D3D texture of.
    /// </param>
    /// <param name="_Value_type">
    ///     The type of the elements in the texture to get underlying D3D texture of.
    /// </param>
    /// <param name="_Texture">
    ///     A texture on a D3D accelerator_view for which the underlying D3D texture interface is returned.
    /// </param>
    /// <returns>
    ///     The IUnknown interface pointer corresponding to the D3D texture underlying the texture.
    /// </returns>
    template<typename _Value_type, int _Rank> _Ret_ IUnknown *get_texture(const texture<_Value_type, _Rank> &_Texture) __CPU_ONLY
    {
        return Concurrency::details::_D3D_interop::_Get_D3D_texture(Concurrency::details::_Get_texture(_Texture));
    }

    /// <summary>
    ///     Get the D3D texture interface underlying a texture viewed by a writeonly_texture_view.
    /// </summary>
    /// <param name="_Rank">
    ///     The rank of the texture to get underlying D3D texture of.
    /// </param>
    /// <param name="_Value_type">
    ///     The type of the elements in the texture to get underlying D3D texture of.
    /// </param>
    /// <param name="_Texture">
    ///     A writeonly_texture_view of a texture on a D3D accelerator_view for which the underlying D3D texture interface is returned.
    /// </param>
    /// <returns>
    ///     The IUnknown interface pointer corresponding to the D3D texture underlying the texture.
    /// </returns>
    template<typename _Value_type, int _Rank> _Ret_ IUnknown *get_texture(const writeonly_texture_view<_Value_type, _Rank> &_Texture) __CPU_ONLY
    {
        return Concurrency::details::_D3D_interop::_Get_D3D_buffer(Concurrency::details::_Get_texture(_Texture));
    }

    /// <summary>
    ///     Get the D3D texture interface underlying a texture viewed by a texture_view.
    /// </summary>
    /// <param name="_Rank">
    ///     The rank of the texture to get underlying D3D texture of.
    /// </param>
    /// <param name="_Value_type">
    ///     The type of the elements in the texture to get underlying D3D texture of.
    /// </param>
    /// <param name="_Texture">
    ///     A texture_view of a texture on a D3D accelerator_view for which the underlying D3D texture interface is returned.
    /// </param>
    /// <returns>
    ///     The IUnknown interface pointer corresponding to the D3D texture underlying the texture.
    /// </returns>
    template<typename _Value_type, int _Rank> _Ret_ IUnknown *get_texture(const texture_view<_Value_type, _Rank> &_Texture) __CPU_ONLY
    {
        return Concurrency::details::_D3D_interop::_Get_D3D_buffer(Concurrency::details::_Get_texture(_Texture));
    }

    /// <summary>
    ///     Create an texture from a D3D texture interface pointer, optionally using the specified DXGI format for all
    ///     views on this texture.
    /// </summary>
    /// <param name="_Rank">
    ///     The rank of the texture to be created from the D3D texture.
    /// </param>
    /// <param name="_Value_type">
    ///     The type of the elements of the texture to be created from the D3D texture.
    /// </param>
    /// <param name="_Av">
    ///     A D3D accelerator view on which the texture is to be created.
    /// </param>
    /// <param name="_D3D_texture">
    ///     IUnknown interface pointer of the D3D texture to create the texture from.
    /// </param>
    /// <param name="_View_format">
    ///     The DXGI format to use for views created from this texture.  Pass DXGI_FORMAT_UNKNOWN (the default)
    ///     to derive the format from the underlying format of _D3D_texture and the _Value_type of this template.
    ///     The provided format must be compatible with the underlying format of _D3D_texture.
    /// </param>
    /// <returns>
    ///     A texture using the provided D3D texture.
    /// </returns>
    template<typename _Value_type, int _Rank> texture<_Value_type, _Rank> make_texture(const Concurrency::accelerator_view &_Av, _In_ IUnknown *_D3D_texture,
                                                                                       DXGI_FORMAT _View_format /*= DXGI_FORMAT_UKNNOWN*/) __CPU_ONLY
    {
        _Texture * _Tex_ptr = NULL;
        Concurrency::extent<_Rank> _Ext = Concurrency::graphics::details::_Make_texture<_Rank>(_Av, _D3D_texture,
            _Short_vector_type_traits<_Value_type>::_Format_base_type_id == _Double_type ? _Uint_type : _Short_vector_type_traits<_Value_type>::_Format_base_type_id,
            &_Tex_ptr, _View_format);

        _ASSERTE(_Tex_ptr);
        return texture<_Value_type, _Rank>(_Ext, _Texture_descriptor(_Tex_ptr));
    }

    /// <summary>
    ///     Get the D3D sampler state interface on the given accelerator view that represents the specified sampler object.
    /// </summary>
    /// <param name="_Av">
    ///     A D3D accelerator view on which the D3D sampler state is to be created.
    /// </param>
    /// <param name="_Sampler">
    ///     A sampler object for which the underlying D3D sampler state interface is created.
    /// </param>
    /// <returns>
    ///     The IUnknown interface pointer corresponding to the D3D sampler state that represents the given sampler.
    /// </returns>
    inline _Ret_ IUnknown * get_sampler(const Concurrency::accelerator_view &_Av, const sampler &_Sampler) __CPU_ONLY
    {
        return Concurrency::details::_D3D_interop::_Get_D3D_sampler(_Av, _Sampler._Get_sampler_ptr());
    }

    /// <summary>
    ///     Create a sampler from a D3D sampler state interface pointer.
    /// </summary>
    /// <param name="_D3D_sampler">
    ///     IUnknown interface pointer of the D3D sampler state to create the sampler from.
    /// </param>
    /// <returns>
    ///     A sampler represents the provided D3D sampler state.
    /// </returns>
    inline sampler make_sampler(_In_ IUnknown *_D3D_sampler) __CPU_ONLY
    {
        return sampler(_Sampler_descriptor(_Sampler::_Create(_D3D_interop::_Get_D3D_sampler_data_ptr(_D3D_sampler))));
    }

    /// <summary>
    ///     Compares a 4-byte reference value and an 8-byte source value and
    ///     accumulates a vector of 4 sums. Each sum corresponds to the masked
    ///     sum of absolute differences of different byte alignments between
    ///     the reference value and the source value.
    /// </summary>
    /// <param name="_Reference">
    ///     The reference array of 4 bytes in one uint value
    /// </param>
    /// <param name="_Source">
    ///     The source array of 8 bytes in a vector of two uint values.
    /// </param>
    /// <param name="_Accum">
    ///     A vector of 4 values to be added to the masked sum of absolute
    ///     differences of the different byte alignments between the reference
    ///     value and the source value.
    /// </param>
    /// <returns>
    ///     Returns a vector of 4 sums. Each sum corresponds to the masked sum
    ///     of absolute differences of different byte alignments between the reference
    ///     value and the source value.
    /// </returns>
    inline uint4 msad4(uint _Reference, uint2 _Source, uint4 _Accum) __GPU_ONLY
    {
        uint4 _Tmp;
        __dp_d3d_msad4(reinterpret_cast<uint*>(&_Tmp), _Reference, _Source.x, _Source.y, _Accum.x, _Accum.y, _Accum.z, _Accum.w);
        return _Tmp;
    }
} // namespace direct3d

} //namespace graphics
} //namespace Concurrency

#pragma warning( pop )
