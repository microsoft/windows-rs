/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* amp.h
*
* C++ AMP Library
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#ifndef _SILENCE_AMP_DEPRECATION_WARNINGS
#error <amp.h> is part of C++ AMP which is deprecated by Microsoft and will be REMOVED. \
You can define _SILENCE_AMP_DEPRECATION_WARNINGS to acknowledge that you have received this warning.
#endif // _SILENCE_AMP_DEPRECATION_WARNINGS

#include <crtdbg.h>
#include <vector>
#include <iterator>
#include <future>

#include <amprt.h>
#include <xxamp.h>
#include <type_traits>

#define _AMP_H

#pragma pack(push,8)


namespace Concurrency
{

/// <summary>
/// Define an N-dimensional index point; which may also be viewed as a vector
/// based at the origin in N-space.
///
/// The index&lt;N&gt; type represents an N-dimensional vector of int which specifies
/// a unique position in an N-dimensional space.  The values in the coordinate
/// vector are ordered from most-significant to least-significant. Thus, in
/// 2-dimensional space, the index vector (5,3) represents the position at
/// row 5, column 3.
///
/// The position is relative to the origin in the N-dimensional space, and can
/// contain negative component values.
/// </summary>
///
/// <param name="_Rank">
///    The dimensionality space into which this index applies, can be any integer
///    greater than 0.
/// </param>
template <int _Rank> class index
{
public:
    _CPP_AMP_VERIFY_RANK(_Rank, index);

    template <typename _Value_type, int _Rank_other>
    friend class array;

    template <int _Rank_other, int _Element_size>
    friend class details::_Array_view_shape;

    template <int _Rank_other, int _Element_size>
    friend class details::_Array_view_base;

    static const int rank = _Rank;
    typedef int value_type;

    /// <summary>
    ///     Default constructor, initializes all elements with 0.
    /// </summary>
    index() __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAssign>::func(*this, 0);
    }

    /// <summary>
    ///     Copy Constructor.
    /// </summary>
    /// <param name="_Other">
    ///        The object to copy from
    /// </param>
    index(const index<_Rank>& _Other) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAssign>::func(*this, _Other);
    }

    /// <summary>
    ///     Constructor for index&lt;1&gt;
    /// </summary>
    /// <param name="_I">
    ///     The value for initialization
    /// </param>
    explicit index(int _I) __GPU
    {
        static_assert(_Rank == 1, "This constructor can only be used to construct an index<1> object.");
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAssign>::func(*this, _I);
    }

    /// <summary>
    ///     Constructor for index&lt;2&gt;
    /// </summary>
    /// <param name="_I0">
    ///     The index value for dimension 0
    /// </param>
    /// <param name="_I1">
    ///     The index value for dimension 1
    /// </param>
    index(int _I0, int _I1) __GPU
    {
        static_assert(_Rank == 2, "This constructor can only be used to construct an index<2> object.");
        _M_base[0] = _I0;
        _M_base[1] = _I1;
    }

    /// <summary>
    ///     Constructor for index&lt;3&gt;
    /// </summary>
    /// <param name="_I0">
    ///     The index value for dimension 0
    /// </param>
    /// <param name="_I1">
    ///     The index value for dimension 1
    /// </param>
    /// <param name="_I2">
    ///     The index value for dimension 2
    /// </param>
    index(int _I0, int _I1, int _I2) __GPU
    {
        static_assert(_Rank == 3, "This constructor can only be used to construct an index<3> object.");
        _M_base[0] = _I0;
        _M_base[1] = _I1;
        _M_base[2] = _I2;
    }

    /// <summary>
    ///     Constructs an index&lt;N&gt; with the coordinate values provided the array
    ///     of int component values.  If the coordinate array length is not N,
    ///     the behavior is undefined.
    /// </summary>
    /// <param name="_Array">
    ///     A single-dimensional array with _Rank elements.
    /// </param>
    explicit index(const int _Array[_Rank]) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAssign>::func(*this, _Array);
    }

    /// <summary>
    ///     copy-assignment operators
    /// </summary>
    index<_Rank>& operator=(const index<_Rank>& _Other) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAssign>::func(*this, _Other);
        return *this;
    }

    /// <summary>
    ///     Index operator.
    /// </summary>
    /// <param name="_Index">
    ///     An integral value between 0 and _Rank-1.
    /// </param>
    /// <returns>
    ///     The corresponding value stored at _Index.
    /// </returns>
    int operator[] (unsigned _Index) const __GPU
    {
        return _M_base[_Index];
    }

    /// <summary>
    ///     Index operator.
    /// </summary>
    /// <param name="_Index">
    ///     An integral value between 0 and _Rank-1.
    /// </param>
    /// <returns>
    ///     A reference to the corresponding value stored at _Index.
    /// </returns>
    int& operator[] (unsigned _Index) __GPU
    {
        return _M_base[_Index];
    }

    // Operations

    /// <summary>
    ///     Element-wise addition of this index with another index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The index to add
    /// </param>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator+=(const index<_Rank>& _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAddEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Element-wise subtraction of this index with another index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The index to subtract
    /// </param>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator-=(const index<_Rank>& _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opSubEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Adds an integer value to each element of this index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to add
    /// </param>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator+=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAddEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Subtracts an integer value from each element of this index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to subtract.
    /// </param>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator-=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opSubEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Multiplies each element of this index with an integer value.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to multiply.
    /// </param>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator*=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opMulEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Divides each element of this index by an integer value.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to divide by.
    /// </param>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator/=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opDivEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Modulus an integer value into each element of this index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to modulus.
    /// </param>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator%=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opModEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Pre-increments each element of this index.
    /// </summary>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator++() __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAddEq>::func(*this, 1);
        return *this;
    }

    /// <summary>
    ///     Post-increments each element of this index.
    /// </summary>
    /// <returns>
    ///     The value of the unincremented index.
    /// </returns>
    index<_Rank> operator++(int) __GPU
    {
        index<_Rank> old_Index(*this);
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opAddEq>::func(*this, 1);
        return old_Index;
    }

    /// <summary>
    ///     Pre-decrements each element of this index.
    /// </summary>
    /// <returns>
    ///     A reference to this index.
    /// </returns>
    index<_Rank>& operator--() __GPU
    {
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opSubEq>::func(*this, 1);
        return *this;
    }

    /// <summary>
    ///     Post-decrements each element of this index.
    /// </summary>
    /// <returns>
    ///     The value of the undecremented index.
    /// </returns>
    index operator--(int) __GPU
    {
        index<_Rank> old_Index(*this);
        details::_compound_assign_op_loop_helper<index<_Rank>, details::opSubEq>::func(*this, 1);
        return old_Index;
    }

private:
    template<class _Tuple_type>
    friend
    _Tuple_type details::_Create_uninitialized_tuple() __GPU;

    /// <summary>
    ///     Constructor.
    /// </summary>
    /// <param name="">
    ///     Indicates that no initialization is necessary.
    /// </param>
    index(details::_eInitializeState) __GPU {}
    //
    // implementation details - end

    int _M_base[_Rank];
};


/// <summary>
///   The extent&lt;N&gt; type represents an N-dimensional vector of int which specifies
///   the bounds of an N-dimensional space with an origin of 0.  The values in the
///   coordinate vector are ordered from most-significant to least-significant.
///   Thus, in 2-dimensional space, the extent vector (5,3) represents a space
///   with 5 rows and 3 columns.
///
///   All components of an extent must be non-negative.
///    E.g.
///      extent&lt;3&gt; domain(2, 3, 4);
///    represents all points
///      index&lt;3&gt; _Index;
///    such that
///      0 &lt;= _Index[0] &lt; 2;
///      0 &lt;= _Index[1] &lt; 3;
///      0 &lt;= _Index[2] &lt; 4;
/// </summary>
/// <param name="_Rank">
///    The _Rank or the dimensionality of the index space.
/// </param>
template <int _Rank> class extent
{
public:
    _CPP_AMP_VERIFY_RANK(_Rank, extent);

    template <typename _Value_type, int _Rank_other>
    friend class array;

    template <int _Rank_other, int _Element_size>
    friend class details::_Array_view_shape;

    template <int _Rank_other, int _Element_size>
    friend class details::_Array_view_base;

    static const int rank = _Rank;
    typedef int value_type;


    /// <summary>
    ///     Default constructor. The value at each dimension is initialized to zero.
    /// </summary>
    extent() __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAssign>::func(*this, 0);
    }

    /// <summary>
    ///     Copy constructor. Constructs a new extent from the supplied argument _Other.
    /// </summary>
    /// <param name="_Other">
    ///     The extent instance to be copied from .
    /// </param>
    extent(const extent<_Rank>& _Other) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAssign>::func(*this, _Other);
    }

    /// <summary>
    ///     Constructor for extent&lt;1&gt;.
    /// </summary>
    /// <param name="_I">
    ///     The value for initialization
    /// </param>
    explicit extent(int _I) __GPU
    {
        static_assert(_Rank == 1, "This constructor can only be used to construct an extent<1> object.");
        _M_base[0] = _I;
    }

    /// <summary>
    ///     Constructor for extent&lt;2&gt;
    /// </summary>
    /// <param name="_I0">
    ///     The extent value for dimension 0
    /// </param>
    /// <param name="_I1">
    ///     The extent value for dimension 1
    /// </param>
    extent(int _I0, int _I1) __GPU
    {
        static_assert(_Rank == 2, "This constructor can only be used to construct an extent<2> object.");
        _M_base[0] = _I0;
        _M_base[1] = _I1;
    }

    /// <summary>
    ///     Constructor for extent&lt;3&gt;
    /// </summary>
    /// <param name="_I0">
    ///     The extent value for dimension 0
    /// </param>
    /// <param name="_I1">
    ///     The extent value for dimension 1
    /// </param>
    /// <param name="_I2">
    ///     The extent value for dimension 2
    /// </param>
    extent(int _I0, int _I1, int _I2) __GPU
    {
        static_assert(_Rank == 3, "This constructor can only be used to construct an extent<3> object.");
        _M_base[0] = _I0;
        _M_base[1] = _I1;
        _M_base[2] = _I2;
    }

    /// <summary>
    ///     Constructs an extent with the coordinate values provided the array
    ///     of int component values.  If the coordinate array length is not N,
    ///     the behavior is undefined.
    /// </summary>
    /// <param name="_Array">
    ///     A single-dimensional array with _Rank elements.
    /// </param>
    explicit extent(const int _Array[_Rank]) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAssign>::func(*this, _Array);
    }

    /// <summary>
    ///     copy-assignment operator
    /// </summary>
    extent<_Rank>& operator=(const extent<_Rank>& _Other) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAssign>::func(*this, _Other);
        return *this;
    }

    /// <summary>
    ///     Index operator.
    /// </summary>
    /// <param name="_Index">
    ///     An integral value between 0 and _Rank-1.
    /// </param>
    /// <returns>
    ///     The corresponding value stored at _Index.
    /// </returns>
    int operator[] (unsigned int _Index) const __GPU
    {
        return _M_base[_Index];
    }

    /// <summary>
    ///     Index operators.
    /// </summary>
    /// <param name="_Index">
    ///     An integral value between 0 and _Rank-1.
    /// </param>
    /// <returns>
    ///     A reference to the value stored at _Index.
    /// </returns>
    int& operator[] (unsigned int _Index) __GPU
    {
        return _M_base[_Index];
    }

    /// <summary>
    ///     Returns the total linear size of this extent (in units of elements).
    /// </summary>
    unsigned int size() const __GPU
    {
        return static_cast<unsigned int>(_product_helper<extent<_Rank>>::func(_M_base));
    }

    /// <summary>
    ///     Tests whether the index "_Index" is properly contained within this extent.
    /// </summary>
    bool contains(const index<rank>& _Index) const __GPU
    {
        return details::_contains<extent<rank>, index<rank>, rank>::func(*this, _Index);
    }

    /// <summary>
    ///     Produces a tiled_extent object with the tile extents given by _Dim0.
    /// </summary>
    template <int _Dim0> tiled_extent<_Dim0> tile() const __GPU
    {
        static_assert(rank == 1, "One-dimensional tile() method only available on extent<1>");
        static_assert(_Dim0>0, "All tile dimensions must be positive");

        return tiled_extent<_Dim0>(*this);
    }

    /// <summary>
    ///     Produces a tiled_extent object with the tile extents given by _Dim0, _Dim1
    /// </summary>
    template <int _Dim0, int _Dim1> tiled_extent<_Dim0, _Dim1> tile() const __GPU
    {
        static_assert(rank == 2, "Two-dimensional tile() method only available on extent<2>");
        static_assert(_Dim0>0 && _Dim1>0, "All tile dimensions must be positive");

        return tiled_extent<_Dim0, _Dim1>(*this);
    }

    /// <summary>
    ///     Produces a tiled_extent object with the tile extents given by _Dim0, _Dim1, _Dim2.
    /// </summary>
    template <int _Dim0, int _Dim1, int _Dim2> tiled_extent<_Dim0, _Dim1, _Dim2> tile() const __GPU
    {
        static_assert(rank == 3, "Three-dimensional tile() method only available on extent<3>");
        static_assert(_Dim0>0 && _Dim1>0 && _Dim2>0, "All tile dimensions must be positive");

        return tiled_extent<_Dim0, _Dim1, _Dim2>(*this);
    }

    // Operations

    /// <summary>
    ///     Element-wise addition of this extent with an index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The index to add to this extent
    /// </param>
    /// <returns>
    ///     A new extent with the result of the computation.
    /// </returns>
    extent<_Rank> operator+(const index<_Rank>& _Rhs) const __GPU
    {
        extent<_Rank> new_extent(details::_do_not_initialize);
        details::_arithmetic_op_loop_helper<extent<_Rank>, details::opAdd>::func(new_extent, *this, _Rhs);
        return new_extent;
    }

    /// <summary>
    ///     Element-wise subtraction of this extent with an index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The index to subtract from this extent
    /// </param>
    /// <returns>
    ///     A new extent with the result of the computation.
    /// </returns>
    extent<_Rank> operator-(const index<_Rank>& _Rhs) const __GPU
    {
        extent<_Rank> new_extent(details::_do_not_initialize);
        details::_arithmetic_op_loop_helper<extent<_Rank>, details::opSub>::func(new_extent, *this, _Rhs);
        return new_extent;
    }

    /// <summary>
    ///     Element-wise addition of this extent with another extent.
    /// </summary>
    /// <param name="_Rhs">
    ///     The extent to add
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator+=(const extent<_Rank>& _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAddEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Element-wise subtraction of this extent with another extent.
    /// </summary>
    /// <param name="_Rhs">
    ///     The extent to subtract
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator-=(const extent<_Rank>& _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opSubEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Element-wise addition of this extent with an index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The index to add
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator+=(const index<_Rank>& _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAddEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Element-wise subtraction of this extent with an index.
    /// </summary>
    /// <param name="_Rhs">
    ///     The index to subtract
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator-=(const index<_Rank>& _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opSubEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Adds an integer value to each element of this extent.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to add to this extent
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator+=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAddEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Subtracts an integer value from each element of this extent.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to subtract from this extent
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator-=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opSubEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Multiplies an integer value to each element of this extent.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to multiply into this extent
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator*=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opMulEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Divides an integer value into each element of this extent.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to divide into this extent
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator/=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opDivEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Modulus an integer value from each element of this extent.
    /// </summary>
    /// <param name="_Rhs">
    ///     The integer value to modulo this extent
    /// </param>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator%=(int _Rhs) __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opModEq>::func(*this, _Rhs);
        return *this;
    }

    /// <summary>
    ///     Pre-increments each element of this extent.
    /// </summary>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator++() __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAddEq>::func(*this, 1);
        return *this;
    }

    /// <summary>
    ///     Post-increments each element of this extent.
    /// </summary>
    /// <returns>
    ///     The value of the unincremented extent.
    /// </returns>
    extent<_Rank> operator++(int) __GPU
    {
        extent<_Rank> old_extent(*this);
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opAddEq>::func(*this, 1);
        return old_extent;
    }

    /// <summary>
    ///     Pre-decrements each element of this extent.
    /// </summary>
    /// <returns>
    ///     A reference to this extent.
    /// </returns>
    extent<_Rank>& operator--() __GPU
    {
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opSubEq>::func(*this, 1);
        return *this;
    }

    /// <summary>
    ///     Post-decrements each element of this extent.
    /// </summary>
    /// <returns>
    ///     The value of the undecremented extent.
    /// </returns>
    extent<_Rank> operator--(int) __GPU
    {
        extent<_Rank> old_extent(*this);
        details::_compound_assign_op_loop_helper<extent<_Rank>, details::opSubEq>::func(*this, 1);
        return old_extent;
    }

    // implementation details (compiler helpers) - begin

    // Index mapping for simple zero-based extent domain.
    index<_Rank> _map_index(const index<_Rank>& _Index) const __GPU {
        return _Index;
    }

private:
    template<class _Tuple_type>
    friend
    _Tuple_type details::_Create_uninitialized_tuple() __GPU;
    /// <summary>
    ///     Constructor.
    /// </summary>
    /// <param name="">
    ///     Indicates that no initialization is necessary.
    /// </param>
    extent(details::_eInitializeState) __GPU {}

    // the store
    int _M_base[_Rank];
};

namespace details
{
    template <typename T> struct _Is_extent_or_index : std::false_type { };

    template <int N>
    struct _Is_extent_or_index<index<N>> : std::true_type { };

    template <int N>
    struct _Is_extent_or_index<extent<N>> : std::true_type { };
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, bool>::type
operator==(const _Tuple_type<_Rank>& _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    return details::_cmp_op_loop_helper<_Tuple_type<_Rank>, details::opEq>::func(_Lhs, _Rhs);
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, bool>::type
operator!=(const _Tuple_type<_Rank>& _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    return !details::_cmp_op_loop_helper<_Tuple_type<_Rank>, details::opEq>::func(_Lhs, _Rhs);
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator+(const _Tuple_type<_Rank>& _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opAdd>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator-(const _Tuple_type<_Rank>& _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opSub>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator+(const _Tuple_type<_Rank>& _Lhs, typename _Tuple_type<_Rank>::value_type _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opAdd>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator+(typename _Tuple_type<_Rank>::value_type _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opAdd>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator-(const _Tuple_type<_Rank>& _Lhs, typename _Tuple_type<_Rank>::value_type _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opSub>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator-(typename _Tuple_type<_Rank>::value_type _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opSub>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator*(const _Tuple_type<_Rank>& _Lhs, typename _Tuple_type<_Rank>::value_type _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opMul>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator*(typename _Tuple_type<_Rank>::value_type _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opMul>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator/(const _Tuple_type<_Rank>& _Lhs, typename _Tuple_type<_Rank>::value_type _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opDiv>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator/(typename _Tuple_type<_Rank>::value_type _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opDiv>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator%(const _Tuple_type<_Rank>& _Lhs, typename _Tuple_type<_Rank>::value_type _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opMod>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}

template <int _Rank, template <int> class _Tuple_type>
typename std::enable_if<details::_Is_extent_or_index<_Tuple_type<_Rank>>::value, _Tuple_type<_Rank>>::type
operator%(typename _Tuple_type<_Rank>::value_type _Lhs, const _Tuple_type<_Rank>& _Rhs) __GPU
{
    _Tuple_type<_Rank> new_Tuple = details::_Create_uninitialized_tuple<_Tuple_type<_Rank>>();
    details::_arithmetic_op_loop_helper<_Tuple_type<_Rank>, opMod>::func(new_Tuple, _Lhs, _Rhs);
    return new_Tuple;
}
/// <summary>
///   The tile_barrier class is a capability class that is only creatable by
///   the system, and passed to a tiled parallel_for_each lambda as part of
///   the tiled_index parameter. It provides wait methods whose purpose is
///   to synchronize execution of threads running within the thread
///   group (tile).
/// </summary>
class tile_barrier
{
public:
    /// <summary>
    ///     Copy Constructor. The tile_barrier class does not have a public
    ///     default constructor or assignment operator, only copy-constructor.
    /// </summary>
    /// <param name="_Other">
    ///     The tile_barrier instance to be copied from.
    /// </param>
#pragma warning( suppress : 4100 ) // unreferenced formal parameter
    tile_barrier(const tile_barrier& _Other) __GPU {}

    /// <summary>
    ///     Blocks execution of all threads in a tile until all threads in the tile have reached this call.
    ///     Ensures that memory accesses are visible to other threads in the thread tile, and are executed according to program order
    /// </summary>
    void wait() const __GPU_ONLY
    {
        __dp_d3d_all_memory_fence_with_tile_barrier();
    }

    /// <summary>
    ///     Blocks execution of all threads in a tile until all threads in the tile have reached this call.
    ///     Ensures that memory accesses are visible to other threads in the thread tile, and are executed according to program order
    /// </summary>
    void wait_with_all_memory_fence() const __GPU_ONLY
    {
        __dp_d3d_all_memory_fence_with_tile_barrier();
    }

    /// <summary>
    ///     Blocks execution of all threads in a tile until all threads in the tile have reached this call.
    ///     Ensures that global memory accesses are visible to other threads in the thread tile, and are executed according to program order
    /// </summary>
    void wait_with_global_memory_fence() const __GPU_ONLY
    {
        __dp_d3d_device_memory_fence_with_tile_barrier();
    }

    /// <summary>
    ///     Blocks execution of all threads in a tile until all threads in the tile have reached this call.
    ///     Ensures that tile_static memory accesses are visible to other threads in the thread tile, and are executed according to program order
    /// </summary>
    void wait_with_tile_static_memory_fence() const __GPU_ONLY
    {
        __dp_d3d_tile_static_memory_fence_with_tile_barrier();
    }
};

/// <summary>
///     A _Tiled_index_base is the base class of all three kinds of tiled_index to
///     share the common code.
/// </summary>
template <int _Rank> class _Tiled_index_base
{
public:

    _CPP_AMP_VERIFY_RANK(_Rank, tiled_index);

    static const int rank = _Rank;

    /// <summary>
    ///     An index that represents the global index within an extent.
    /// </summary>
    const index<rank> global;

    /// <summary>
    ///     An index that represents the relative index within the current tile of a tiled_extent.
    /// </summary>
    const index<rank> local;

    /// <summary>
    ///     An index that represents the coordinates of the current tile of a tiled_extent.
    /// </summary>
    const index<rank> tile;

    /// <summary>
    ///     An index that represents the global coordinates of the origin of the current tile within a tiled_extent.
    /// </summary>
    const index<rank> tile_origin;

    /// <summary>
    ///     An object which represents a barrier within the current tile of threads.
    /// </summary>
    const tile_barrier barrier;

    /// <summary>
    ///     A Constructor that initializes data members using the given values.
    /// </summary>
    /// <param name="_Global">
    ///     The global index to be copied from
    /// </param>
    /// <param name="_Local">
    ///     The local index to be copied from
    /// </param>
    /// <param name="_Tile">
    ///     The tile index to be copied from
    /// </param>
    /// <param name="_Tile_origin">
    ///     The tile origin to be copied from
    /// </param>
    /// <param name="_Barrier">
    ///     The barrier to be copied from
    /// </param>
    _Tiled_index_base(const index<rank>& _Global,
                     const index<rank>& _Local,
                     const index<rank>& _Tile,
                     const index<rank>& _Tile_origin,
                     const tile_barrier& _Barrier) __GPU
    : global(_Global), local(_Local), tile(_Tile), tile_origin(_Tile_origin), barrier(_Barrier)
    {}

    /// <summary>
    ///     Copy Constructor.
    /// </summary>
    /// <param name="_Other">
    ///     The tile_index instance to be copied from .
    /// </param>
    _Tiled_index_base(const _Tiled_index_base& _Other) __GPU
    : global(_Other.global),
      local(_Other.local),
      tile(_Other.tile),
      tile_origin(_Other.tile_origin),
      barrier(_Other.barrier)
    {}

    /// <summary>
    ///     Implicit conversion operator that converts a tiled_index into an index.
    ///     The implicit conversion converts to the .global index member.
    /// </summary>
    operator const index<rank>() const __GPU
    {
        return global;
    }

private:
    _Tiled_index_base& operator=(const _Tiled_index_base&) __GPU;
};

/// <summary>
///     A tiled_index is a set of indices of 1 to 3 dimensions which have been
///     subdivided into 1-, 2-, or 3-dimensional tiles in a tiled_extent.  It has
///     three specialized forms:  tiled_index&lt;_Dim0&gt;, tiled_index&lt;_Dim0, _Dim1&gt;, and
///     tiled_index&lt;_Dim0, _Dim1, _Dim2&gt;, where _Dim0-2 specify the length of the tile along
///     the each dimension, with _Dim0 being the most-significant dimension and _Dim2
///     being the least-significant.
/// </summary>
template <int _Dim0, int _Dim1 = 0, int _Dim2 = 0> class tiled_index : public _Tiled_index_base<3>
{
public:
    /// <summary>
    ///     A Constructor that initializes data members using the given values.
    /// </summary>
    /// <param name="_Global">
    ///     The global index to be copied from
    /// </param>
    /// <param name="_Local">
    ///     The local index to be copied from
    /// </param>
    /// <param name="_Tile">
    ///     The tile index to be copied from
    /// </param>
    /// <param name="_Tile_origin">
    ///     The tile origin to be copied from
    /// </param>
    /// <param name="_Barrier">
    ///     The barrier to be copied from
    /// </param>
    tiled_index(const index<rank>& _Global,
                const index<rank>& _Local,
                const index<rank>& _Tile,
                const index<rank>& _Tile_origin,
                const tile_barrier& _Barrier) __GPU
    : _Tiled_index_base(_Global, _Local, _Tile, _Tile_origin, _Barrier)
    {}

    /// <summary>
    ///     Copy Constructor.
    /// </summary>
    /// <param name="_Other">
    ///     The tile_index instance to be copied from .
    /// </param>
    tiled_index(const tiled_index& _Other) __GPU
    : _Tiled_index_base(_Other)
    {}

    /// <summary>
    ///     Returns an instance of an extent that captures the values of the tiled_index
    ///     template arguments _Dim0, _Dim1, _Dim2
    /// </summary>
    __declspec(property(get=get_tile_extent)) extent<rank> tile_extent;
    extent<rank> get_tile_extent() const __GPU { return extent<rank>(_Dim0, _Dim1, _Dim2); }

    /// <summary>
    ///     These constants allow access to the template arguments of tiled_index.
    /// </summary>
    static const int tile_dim0 = _Dim0;
    static const int tile_dim1 = _Dim1;
    static const int tile_dim2 = _Dim2;

private:
    tiled_index& operator=(const tiled_index&) __GPU;
};

template <int _Dim0, int _Dim1>
class tiled_index<_Dim0, _Dim1, 0> : public _Tiled_index_base<2>
{
public:
    /// <summary>
    ///     A Constructor that initializes data members using the given values.
    /// </summary>
    /// <param name="_Global">
    ///     The global index to be copied from
    /// </param>
    /// <param name="_Local">
    ///     The local index to be copied from
    /// </param>
    /// <param name="_Tile">
    ///     The tile index to be copied from
    /// </param>
    /// <param name="_Tile_origin">
    ///     The tile origin to be copied from
    /// </param>
    /// <param name="_Barrier">
    ///     The barrier to be copied from
    /// </param>
    tiled_index(const index<rank>& _Global,
                const index<rank>& _Local,
                const index<rank>& _Tile,
                const index<rank>& _Tile_origin,
                const tile_barrier& _Barrier) __GPU
    : _Tiled_index_base(_Global, _Local, _Tile, _Tile_origin, _Barrier)
    {}

    /// <summary>
    ///     Copy Constructor.
    /// </summary>
    /// <param name="_Other">
    ///     The tile_index instance to be copied from .
    /// </param>
    tiled_index(const tiled_index& _Other) __GPU
    : _Tiled_index_base(_Other)
    {}

    /// <summary>
    ///     Returns an instance of an extent that captures the values of the tiled_index
    ///     template arguments _Dim0, _Dim1
    /// </summary>
    __declspec(property(get=get_tile_extent)) extent<rank> tile_extent;
    extent<rank> get_tile_extent() const __GPU { return extent<rank>(_Dim0, _Dim1); }

    /// <summary>
    ///     These constants allow access to the template arguments of tiled_index.
    /// </summary>
    static const int tile_dim0 = _Dim0;
    static const int tile_dim1 = _Dim1;

private:
    tiled_index& operator=(const tiled_index&) __GPU;
};

template <int _Dim0>
class tiled_index<_Dim0, 0, 0> : public _Tiled_index_base<1>
{
public:
    /// <summary>
    ///     A Constructor that initializes data members using the given values.
    /// </summary>
    /// <param name="_Global">
    ///     The global index to be copied from
    /// </param>
    /// <param name="_Local">
    ///     The local index to be copied from
    /// </param>
    /// <param name="_Tile">
    ///     The tile index to be copied from
    /// </param>
    /// <param name="_Tile_origin">
    ///     The tile origin to be copied from
    /// </param>
    /// <param name="_Barrier">
    ///     The barrier to be copied from
    /// </param>
    tiled_index(const index<rank>& _Global,
                const index<rank>& _Local,
                const index<rank>& _Tile,
                const index<rank>& _Tile_origin,
                const tile_barrier& _Barrier) __GPU
    : _Tiled_index_base(_Global, _Local, _Tile, _Tile_origin, _Barrier)
    {}

    /// <summary>
    ///     Copy Constructor.
    /// </summary>
    /// <param name="_Other">
    ///     The tile_index instance to be copied from .
    /// </param>
    tiled_index(const tiled_index& _Other) __GPU
    : _Tiled_index_base(_Other)
    {}

    /// <summary>
    ///     Returns an instance of an extent that captures the values of the tiled_index
    ///     template argument _Dim0
    /// </summary>
    __declspec(property(get=get_tile_extent)) extent<rank> tile_extent;
    extent<rank> get_tile_extent() const __GPU { return extent<rank>(_Dim0); }

    /// <summary>
    ///     These constants allow access to the template arguments of tiled_index.
    /// </summary>
    static const int tile_dim0 = _Dim0;

private:
    tiled_index& operator=(const tiled_index&) __GPU;
};


/// <summary>
///     A tiled_extent is an extent of 1 to 3 dimensions which also subdivides the extent space into
///     1-, 2-, or 3-dimensional tiles. It has three specialized forms:  tiled_extent&lt;_Dim0&gt;,
///     tiled_extent&lt;_Dim0,_Dim1&gt;, and tiled_extent&lt;_Dim0,_Dim1,_Dim2&gt;, where _Dim0-2 specify the length of the tile
///     along each dimension, with _Dim0 being the most-significant dimension and _Dim2 being the
///     least-significant.
/// </summary>
template <int _Dim0, int _Dim1 /*=0*/, int _Dim2 /*=0*/> class tiled_extent : public Concurrency::extent<3>
{
public:

    static_assert(_Dim0>0, "_Dim0 must be positive");
    static_assert(_Dim1>0, "_Dim1 must be positive");
    static_assert(_Dim2>0, "_Dim2 must be positive");

    /// <summary>
    ///     Default constructor.
    /// </summary>
    tiled_extent() __GPU {}

    /// <summary>
    ///     Constructs a new tiled_extent from the supplied extent.
    /// </summary>
    tiled_extent(const Concurrency::extent<rank>& _Other) __GPU : Concurrency::extent<rank>(_Other)
    {}

    /// <summary>
    ///     Copy constructor. Constructs a new tiled_extent from the supplied argument "_Other".
    /// </summary>
    tiled_extent(const tiled_extent& _Other) __GPU : Concurrency::extent<rank>(_Other)
    {}

    /// <summary>
    ///     copy-assignment operator
    /// </summary>
    tiled_extent& operator=(const tiled_extent& _Other) __GPU
    {
        Concurrency::extent<rank>::operator=(_Other);
        return *this;
    }

    /// <summary>
    ///     Returns an instance of an extent that captures the values of the tiled_extent
    ///     template arguments _Dim0, _Dim1, _Dim2.
    /// </summary>
    __declspec(property(get=get_tile_extent)) Concurrency::extent<rank> tile_extent;
    Concurrency::extent<rank> get_tile_extent() const __GPU
    {
        return Concurrency::extent<rank>(_Dim0, _Dim1, _Dim2);
    }

    /// <summary>
    ///     Returns a new tiled_extent with extents adjusted up to be evenly divisible by the tile dimensions.
    /// </summary>
    tiled_extent pad() const __GPU
    {
        Concurrency::extent<rank> _New_extent(((static_cast<unsigned int>((*this)[0]) + _Dim0 - 1)/_Dim0) * _Dim0,
			                                  ((static_cast<unsigned int>((*this)[1]) + _Dim1 - 1)/_Dim1) * _Dim1,
											  ((static_cast<unsigned int>((*this)[2]) + _Dim2 - 1)/_Dim2) * _Dim2);

        return tiled_extent<_Dim0,_Dim1,_Dim2>(_New_extent);
    }

    /// <summary>
    ///     Returns a new tiled_extent with extents adjusted down to be evenly divisible by the tile dimensions.
    /// </summary>
    tiled_extent truncate() const __GPU
    {
        Concurrency::extent<rank> _New_extent(((*this)[0]/_Dim0) * _Dim0, ((*this)[1]/_Dim1) * _Dim1, ((*this)[2]/_Dim2) * _Dim2);
        return tiled_extent<_Dim0,_Dim1,_Dim2>(_New_extent);
    }

    /// <summary>
    ///     These constants allow access to the template arguments of tiled_extent.
    /// </summary>
    static const int tile_dim0 = _Dim0;
    static const int tile_dim1 = _Dim1;
    static const int tile_dim2 = _Dim2;

    // implementation details (compiler helpers) - begin

    // Given the local index, the tile index, the global index, in the 0-based domain that
    // has same extents as 'this', and a barrier object, return a tiled_index<_Dim0, _Dim1, _Dim2> into
    // the 'this' tiled_extent domain.
    tiled_index<_Dim0, _Dim1, _Dim2> _map_index(const index<rank>& _Local, const index<rank>& _Tile, const index<rank>& _Global, tile_barrier& _Barrier) const __GPU
    {
        index<rank> _Tile_origin = details::_Create_uninitialized_tuple<index<rank>>();
        details::_arithmetic_op_loop_helper<index<rank>, details::opMul>::func(_Tile_origin, _Tile, tile_extent);
        return tiled_index<_Dim0, _Dim1, _Dim2>(_Global, _Local, _Tile, _Tile_origin, _Barrier);
    }
    // implementation details (compiler helpers) - end
};


template <int _Dim0, int _Dim1>
class tiled_extent<_Dim0, _Dim1, 0> : public Concurrency::extent<2>
{
public:

    static_assert(_Dim0>0, "_Dim0 must be positive");
    static_assert(_Dim1>0, "_Dim1 must be positive");

    /// <summary>
    ///     Default constructor.
    /// </summary>
    tiled_extent() __GPU {}

    /// <summary>
    ///     Constructs a new tiled_extent from the supplied extent.
    /// </summary>
    tiled_extent(const Concurrency::extent<rank>& _Other) __GPU : Concurrency::extent<rank>(_Other)
    {}

    /// <summary>
    ///     Copy constructor. Constructs a new tiled_extent from the supplied argument "_Other".
    /// </summary>
    tiled_extent(const tiled_extent& _Other) __GPU : Concurrency::extent<rank>(_Other)
    {}

    /// <summary>
    ///     copy-assignment operator
    /// </summary>
    tiled_extent& operator=(const tiled_extent& _Other) __GPU
    {
        Concurrency::extent<rank>::operator=(_Other);
        return *this;
    }

    /// <summary>
    ///     Returns an instance of an extent that captures the values of the tiled_extent
    ///     template arguments _Dim0, _Dim1.
    /// </summary>
    __declspec(property(get=get_tile_extent)) Concurrency::extent<rank> tile_extent;
    Concurrency::extent<rank> get_tile_extent() const __GPU
    {
        return Concurrency::extent<rank>(_Dim0, _Dim1);
    }

    /// <summary>
    ///     Returns a new tiled_extent with extents adjusted up to be evenly divisible by the tile dimensions.
    /// </summary>
    tiled_extent pad() const __GPU
    {
        Concurrency::extent<rank> _New_extent(((static_cast<unsigned int>((*this)[0]) + _Dim0 - 1)/_Dim0) * _Dim0,
			                                  ((static_cast<unsigned int>((*this)[1]) + _Dim1 - 1)/_Dim1) * _Dim1);
        return tiled_extent<_Dim0,_Dim1>(_New_extent);
    }

    /// <summary>
    ///     Returns a new tiled_extent with extents adjusted down to be evenly divisible by the tile dimensions.
    /// </summary>
    tiled_extent truncate() const __GPU
    {
        Concurrency::extent<rank> _New_extent(((*this)[0]/_Dim0) * _Dim0, ((*this)[1]/_Dim1) * _Dim1);
        return tiled_extent<_Dim0,_Dim1>(_New_extent);
    }

    /// <summary>
    ///     These constants allow access to the template arguments of tiled_extent.
    /// </summary>
    static const int tile_dim0 = _Dim0;
    static const int tile_dim1 = _Dim1;

    // implementation details (compiler helpers) - begin

    // Given the local index, the tile index, the global index, in the 0-based domain that
    // has same extents as 'this', and a barrier object, return a tiled_index<_Dim0, _Dim1> into
    // the 'this' tiled_extent domain.
    tiled_index<_Dim0, _Dim1> _map_index(const index<rank>& _Local, const index<rank>& _Tile, const index<rank>& _Global, tile_barrier& _Barrier) const __GPU
    {
        index<rank> _Tile_origin = details::_Create_uninitialized_tuple<index<rank>>();
        details::_arithmetic_op_loop_helper<index<rank>, details::opMul>::func(_Tile_origin, _Tile, tile_extent);
        return tiled_index<_Dim0, _Dim1>(_Global, _Local, _Tile, _Tile_origin, _Barrier);
    }
    // implementation details (compiler helpers) - end
};

template <int _Dim0>
class tiled_extent<_Dim0, 0, 0> : public Concurrency::extent<1>
{
public:

    static_assert(_Dim0>0, "_Dim0 must be positive");

    /// <summary>
    ///     Default constructor.
    /// </summary>
    tiled_extent() __GPU {}

    /// <summary>
    ///     Constructs a new tiled_extent from the supplied extent.
    /// </summary>
    tiled_extent(const Concurrency::extent<rank>& _Other) __GPU : Concurrency::extent<rank>(_Other)
    {}

    /// <summary>
    ///     Copy constructor. Constructs a new tiled_extent from the supplied argument "_Other".
    /// </summary>
    tiled_extent(const tiled_extent& _Other) __GPU : Concurrency::extent<rank>(_Other)
    {}

    /// <summary>
    ///     copy-assignment operator
    /// </summary>
    tiled_extent& operator=(const tiled_extent& _Other) __GPU
    {
        Concurrency::extent<rank>::operator=(_Other);
        return *this;
    }

    /// <summary>
    ///     Returns an instance of an extent that captures the values of the tiled_extent
    ///     template argument _Dim0.
    /// </summary>
    __declspec(property(get=get_tile_extent)) Concurrency::extent<rank> tile_extent;
    Concurrency::extent<rank> get_tile_extent() const __GPU
    {
        return Concurrency::extent<rank>(_Dim0);
    }

    /// <summary>
    ///     Returns a new tiled_extent with extents adjusted up to be evenly divisible by the tile dimensions.
    /// </summary>
    tiled_extent pad() const __GPU
    {
        Concurrency::extent<rank> _New_extent(((static_cast<unsigned int>((*this)[0]) + _Dim0 - 1)/_Dim0) * _Dim0);
        return tiled_extent<_Dim0>(_New_extent);
    }

    /// <summary>
    ///     Returns a new tiled_extent with extents adjusted down to be evenly divisible by the tile dimensions.
    /// </summary>
    tiled_extent truncate() const __GPU
    {
        Concurrency::extent<rank> _New_extent(((*this)[0]/_Dim0) * _Dim0);
        return tiled_extent<_Dim0>(_New_extent);
    }

    /// <summary>
    ///     These constants allow access to the template arguments of tiled_extent.
    /// </summary>
    static const int tile_dim0 = _Dim0;

    // implementation details (compiler helpers) - begin

    // Given the local index, the tile index, the global index, in the 0-based domain that
    // has same extents as 'this', and a barrier object, return a tiled_index<_Dim0> into
    // the 'this' tiled_extent domain.
    tiled_index<_Dim0> _map_index(const index<rank>& _Local, const index<rank>& _Tile, const index<rank>& _Global, tile_barrier& _Barrier) const __GPU
    {
        index<rank> _Tile_origin = details::_Create_uninitialized_tuple<index<rank>>();
        details::_arithmetic_op_loop_helper<index<rank>, details::opMul>::func(_Tile_origin, _Tile, tile_extent);
        return tiled_index<_Dim0>(_Global, _Local, _Tile, _Tile_origin, _Barrier);
    }
};

namespace details
{

template <int _Old_element_size, int _New_element_size>
int  _Calculate_reinterpreted_size(int _Old_size) __GPU_ONLY
{
    int _Total_size = _Old_element_size * _Old_size;
    int _New_size = (_Total_size + _New_element_size - 1)/ _New_element_size;

    return _New_size;
}


template <int _Old_element_size, int _New_element_size>
int  _Calculate_reinterpreted_size(int _Old_size) __CPU_ONLY
{
    int _Total_size = _Old_element_size * _Old_size;
    int _New_size = (_Total_size + _New_element_size - 1)/ _New_element_size;

    if (_New_size * _New_element_size > _Total_size)
        throw runtime_exception("Element type of reinterpret_as does not evenly divide into extent", E_INVALIDARG);

    return _New_size;
}


// This class defines the shape of an array view and provides
// the functionality of translating dimensional indices into
// flat offsets into the underlying buffer
template <int _Rank, int _Element_size /* in number of ints */>
class _Array_view_shape
{
    typedef _Array_flatten_helper<_Rank, typename Concurrency::extent<_Rank>::value_type, typename Concurrency::index<_Rank>::value_type> _Flatten_helper;
    friend class _Array_view_shape<_Rank+1, _Element_size>;

public:
    /// <summary>
    ///     The extent of this array or view.
    /// </summary>
    __declspec(property(get=get_extent)) Concurrency::extent<_Rank> extent;
    Concurrency::extent<_Rank> get_extent() const __GPU
    {
        return _M_view_extent;
    }

    ~_Array_view_shape() __GPU {}

protected:
    int _Base_linear_offset() const __GPU
    {
        return (_M_total_linear_offset - (_Element_size * _Flatten_helper::func(_M_array_multiplier._M_base, _M_view_offset._M_base)));
    }

    _Array_view_shape(const _Array_view_shape& _Other) __GPU
        :
        _M_array_extent(_Other._M_array_extent),
        _M_array_multiplier(_Other._M_array_multiplier),
        _M_view_offset(_Other._M_view_offset),
        _M_total_linear_offset(_Other._M_total_linear_offset),
        _M_view_extent(_Other._M_view_extent)
    {
    }

    // For "section"
    _Array_view_shape(const _Array_view_shape& _Other, const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __GPU
        :
        _M_array_extent(_Other._M_array_extent),
        _M_array_multiplier(_Other._M_array_multiplier),
        _M_view_offset(_Other._M_view_offset + _Section_origin),
        _M_view_extent(_Section_extent)
    {
        _Is_valid_section(_Other._M_view_extent, _Section_origin, _Section_extent);

        _M_total_linear_offset = _Other._Base_linear_offset() + (_Element_size * _Flatten_helper::func(_M_array_multiplier._M_base, _M_view_offset._M_base));
    }

    _Array_view_shape(int _Base_linear_offset, const Concurrency::extent<_Rank>& _Array_extent) __GPU
        :
        _M_array_extent(_Array_extent),
        _M_view_offset(index<_Rank>()),
        _M_total_linear_offset(_Base_linear_offset),
        _M_view_extent(_Array_extent)
    {
        _Initialize_multiplier();
    }

    _Array_view_shape(int _Base_linear_offset, const Concurrency::extent<_Rank>& _Array_extent,
                      const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __GPU
        :
        _M_array_extent(_Array_extent),
        _M_view_offset(_Section_origin),
        _M_total_linear_offset(_Base_linear_offset),
        _M_view_extent(_Section_extent)
    {
        _Is_valid_section(_Array_extent, _Section_origin, _Section_extent);

        _Initialize_multiplier();
        _M_total_linear_offset += (_Element_size * _Flatten_helper::func(_M_array_multiplier._M_base, _M_view_offset._M_base));
    }

    _Array_view_shape& operator=(const _Array_view_shape &_Other) __GPU
    {
        _M_array_extent        = _Other._M_array_extent;
        _M_array_multiplier    = _Other._M_array_multiplier;
        _M_view_offset         = _Other._M_view_offset;
        _M_total_linear_offset = _Other._M_total_linear_offset;
        _M_view_extent         = _Other._M_view_extent;
        return *this;
    }

    void _Project0(int _I, _Array_view_shape<_Rank-1,_Element_size>& _Projected_shape) const __GPU
    {
        static_assert(_Rank > 1, "Projection is only supported on array_views with a rank of 2 or higher");

        _Is_valid_projection(_I, this->_M_view_extent);

        typedef Concurrency::extent<_Rank-1> _RES_EXT;
        typedef Concurrency::extent<_Rank> _SRC_EXT;
        typedef Concurrency::index<_Rank-1> _RES_IDX;
        typedef Concurrency::index<_Rank> _SRC_IDX;
        details::_project0<_RES_EXT, _SRC_EXT, _RES_IDX, _SRC_IDX, _Rank>::func(
                         _Projected_shape._M_array_extent, this->_M_array_extent,
                         _Projected_shape._M_array_multiplier, this->_M_array_multiplier,
                         _Projected_shape._M_view_offset, this->_M_view_offset,
                         _Projected_shape._M_view_extent, this->_M_view_extent);

        _Projected_shape._M_total_linear_offset = _M_total_linear_offset + (_Element_size * _I * _M_array_multiplier[0]);
    }

    _Array_view_shape() __GPU
        : _M_array_extent(details::_do_not_initialize),  _M_array_multiplier(details::_do_not_initialize),
        _M_view_offset(details::_do_not_initialize), _M_view_extent(details::_do_not_initialize)
    {
    }

private:

    void _Initialize_multiplier() __GPU
    {
        details::_Is_valid_extent(_M_array_extent);
        unsigned int _Ext = _M_array_extent[_Rank-1];
        details::_Array_init_helper<Concurrency::extent<_Rank>, Concurrency::extent<_Rank>>::func(_Ext, _M_array_multiplier, _M_array_extent);
    }

protected:
    Concurrency::extent<_Rank>   _M_array_extent;
    Concurrency::extent<_Rank>   _M_array_multiplier;
    Concurrency::index<_Rank>    _M_view_offset;
    int                          _M_total_linear_offset; // in number of units
    Concurrency::extent<_Rank>   _M_view_extent;
};

template <int _Rank, int _Element_size>
class _Array_view_base : public _Array_view_shape<_Rank,_Element_size /* in number of ints */>
{
    typedef _Array_flatten_helper<_Rank, typename Concurrency::extent<_Rank>::value_type, typename Concurrency::index<_Rank>::value_type> _Flatten_helper;

    template <int _R, int _S>
    friend class _Array_view_base;

public:

    typedef details::_Buffer_descriptor _Buffer_descriptor;

    ~_Array_view_base() __GPU
    {
        // Unregister the view; Do not throw exception
        _Unregister(false);
    }

protected:

    _Array_view_base() __GPU {}

    _Array_view_base(const _Buffer_descriptor& _Buffer_desc, const _Array_view_shape<_Rank, _Element_size>& _Shape) __GPU
        :
        _M_buffer_descriptor(_Buffer_desc),
        _Array_view_shape<_Rank, _Element_size>(_Shape)
    {
        // Register the view
        _Register();
    }

    _Array_view_base(const _Array_view_base& _Other) __GPU
        :
        _M_buffer_descriptor(_Other._M_buffer_descriptor),
        _Array_view_shape<_Rank, _Element_size>(_Other)
    {
        // Register the view
        _Register_copy(_Other);

        // update this buffer descriptor in case _Register_copy was late and missed the update opportunity.
        _M_buffer_descriptor = _Other._M_buffer_descriptor;
    }

    _Array_view_base(const _Array_view_base& _Other, const Concurrency::extent<_Rank>& _Array_extent) __GPU
        :
        _M_buffer_descriptor(_Other._M_buffer_descriptor),
        _Array_view_shape<_Rank, _Element_size>(_Other._Base_linear_offset(), _Array_extent)
    {
        // Register the view
        _Register();
    }

    _Array_view_base(const _Array_view_base& _Other, const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __GPU
        :
        _M_buffer_descriptor(_Other._M_buffer_descriptor),
        _Array_view_shape<_Rank, _Element_size>(_Other, _Section_origin, _Section_extent)
    {
        // Register the view
        _Register();
    }

    _Array_view_base(const _Buffer_descriptor& _Buffer_desc, const Concurrency::extent<_Rank>& _Array_extent) __GPU
        :
        _M_buffer_descriptor(_Buffer_desc),
        _Array_view_shape<_Rank, _Element_size>(0,_Array_extent)
    {
        // Register the view
        _Register();
    }

    _Array_view_base(const _Buffer_descriptor& _Buffer_desc, int _Base_linear_offset, const Concurrency::extent<_Rank>& _Array_extent) __GPU
        :
        _M_buffer_descriptor(_Buffer_desc),
        _Array_view_shape<_Rank, _Element_size>(_Base_linear_offset,_Array_extent)
    {
        // Register the view
        _Register();
    }

    _Array_view_base(
        const _Buffer_descriptor& _Buffer_desc,
        int _Base_linear_offset,
        const Concurrency::extent<_Rank>& _Array_extent,
        const Concurrency::index<_Rank>& _View_offset,
        const Concurrency::extent<_Rank>& _View_extent
        ) __CPU_ONLY
        :
        _M_buffer_descriptor(_Buffer_desc),
        _Array_view_shape<_Rank, _Element_size>(_Base_linear_offset,_Array_extent,_View_offset,_View_extent)
    {
        // Register the view
        _Register(_Buffer_desc._Get_view_key());
    }

    _Array_view_base(
        const _Buffer_descriptor& _Buffer_desc,
        int _Base_linear_offset,
        const Concurrency::extent<_Rank>& _Array_extent,
        const Concurrency::index<_Rank>& _View_offset,
        const Concurrency::extent<_Rank>& _View_extent
        ) __GPU_ONLY
        :
        _M_buffer_descriptor(_Buffer_desc),
        _Array_view_shape<_Rank, _Element_size>(_Base_linear_offset,_Array_extent,_View_offset,_View_extent)
    {
        // Register the view
        _Register();
    }

    _Array_view_base(const _Buffer_descriptor& _Buffer_desc, const Concurrency::extent<_Rank>& _Array_extent,
                     const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __GPU
        :
        _M_buffer_descriptor(_Buffer_desc),
        _Array_view_shape<_Rank, _Element_size>(0,_Array_extent,_Section_origin,_Section_extent)
    {
        // Register the view
        _Register();
    }

    _Array_view_base(const Concurrency::extent<_Rank>& _Array_extent) __CPU_ONLY
        :
        _Array_view_shape<_Rank, _Element_size>(0,_Array_extent)
    {
        _Ubiquitous_buffer_ptr _PUBuf = _Ubiquitous_buffer::_Create_ubiquitous_buffer(_Array_extent.size(), _Element_size * sizeof(int));
        _M_buffer_descriptor = _Buffer_descriptor(NULL, _PUBuf, _No_access, _No_access);

        // Register the view
        _Register();
    }

    _Array_view_base(_In_ void * _Data, const Concurrency::extent<_Rank>& _Array_extent) __CPU_ONLY
        :
        _Array_view_shape<_Rank, _Element_size>(0,_Array_extent)
    {
        if (_Data == NULL) {
            throw runtime_exception("Invalid pointer argument (NULL) to array_view constructor", E_INVALIDARG);
        }

        _Buffer_ptr _PBuf = _Buffer::_Create_buffer(_Data, accelerator(accelerator::cpu_accelerator).default_view, _Array_extent.size(), _Element_size * sizeof(int));
        _Ubiquitous_buffer_ptr _PUBuf = _Ubiquitous_buffer::_Create_ubiquitous_buffer(_PBuf);
        _M_buffer_descriptor = _Buffer_descriptor(_Data, _PUBuf, _Read_write_access, _Read_write_access);

        // Register the view
        _Register();
    }

    _Array_view_base(_In_ void * _Data, const Concurrency::extent<_Rank>& _Array_extent) __GPU_ONLY
        :
        _Array_view_shape<_Rank, _Element_size>(0,_Array_extent), _M_buffer_descriptor(_Data, NULL, _Read_write_access, _Read_write_access)
    {
    }

    _Array_view_base(const void * _Data, const Concurrency::extent<_Rank>& _Array_extent) __CPU_ONLY
        :
        _Array_view_shape<_Rank, _Element_size>(0,_Array_extent)
    {
        if (_Data == NULL) {
            throw runtime_exception("Invalid pointer argument (NULL) to array_view constructor", E_INVALIDARG);
        }

        _Buffer_ptr _PBuf = _Buffer::_Create_buffer(const_cast<void*>(_Data), accelerator(accelerator::cpu_accelerator).default_view, _Array_extent.size(), _Element_size * sizeof(int));
        _Ubiquitous_buffer_ptr _PUBuf = _Ubiquitous_buffer::_Create_ubiquitous_buffer(_PBuf);
        _M_buffer_descriptor = _Buffer_descriptor(const_cast<void*>(_Data), _PUBuf, _Read_access, _Read_access);

        // Register the view
        _Register();
    }

    _Array_view_base(const void * _Data, const Concurrency::extent<_Rank>& _Array_extent) __GPU_ONLY
        :
#pragma warning( push )
#pragma warning( disable : 4880 )
        // Casting away constness in amp restricted scope might result in
        // undefined behavior, therefore, the compiler will report a level 1 warning
        // for it. But the following const_cast is harmless thus we are suppressing
        // this warning just for this line.
        _Array_view_shape<_Rank, _Element_size>(0,_Array_extent), _M_buffer_descriptor(const_cast<void*>(_Data), NULL, _Read_access, _Read_access)
#pragma warning( pop )
    {
    }

    _Array_view_base& operator=(const _Array_view_base &_Other) __GPU
    {
        if (this != &_Other)
        {
            // Unregister the current view
            _Unregister();

            _M_buffer_descriptor = _Other._M_buffer_descriptor;
            _Array_view_shape<_Rank, _Element_size>::operator=(_Other);

            // Register the new view
            _Register_copy(_Other);

            // update this buffer descriptor in case _Register_copy was late and missed the update opportunity.
            _M_buffer_descriptor = _Other._M_buffer_descriptor;
        }

        return *this;
    }

    _Ret_ void * _Access(const index<_Rank>& _Index) const __GPU
    {
        int * _Ptr = reinterpret_cast<int *>(_M_buffer_descriptor._M_data_ptr);
        return &_Ptr[this->_M_total_linear_offset + (_Element_size * _Flatten_helper::func(this->_M_array_multiplier._M_base, _Index._M_base))];
    }

    _Ret_ void * _Access(_Access_mode _Requested_mode, const index<_Rank>& _Index) const __CPU_ONLY
    {
        // Refresh the data ptr if we do not have requested access
        if ((_M_buffer_descriptor._M_curr_cpu_access_mode & _Requested_mode) != _Requested_mode) {
            _M_buffer_descriptor._Get_CPU_access(_Requested_mode);
        }

        return _Access(_Index);
    }

    _Ret_ void * _Access(_Access_mode /*_Requested_mode*/, const index<_Rank>& _Index) const __GPU_ONLY
    {
        return _Access(_Index);
    }

    _Array_view_base _Section(const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) const __GPU
    {
        auto _View = _Array_view_base(*this, _Section_origin, _Section_extent);

        // Register the constructed view with the section buffer view shape
        _View._Register(_Array_view_base::_Create_section_buffer_shape(this->_M_buffer_descriptor, _Section_origin, _Section_extent));

        return _View;
    }

    _Array_view_base _Section(const index<_Rank>& _Idx) const __GPU
    {
        return _Section(_Idx, this->extent - _Idx);
    }

    void _Project0(int _I, _Array_view_base<_Rank-1,_Element_size>& _Projected_view) const __GPU
    {
        _Projected_view._M_buffer_descriptor = this->_M_buffer_descriptor;
        _Array_view_shape<_Rank, _Element_size>::_Project0(_I, _Projected_view);

        // Register the constructed view with the projection buffer view shape
        _Projected_view._Register(_Array_view_base::_Create_projection_buffer_shape(this->_M_buffer_descriptor, 0, _I));
    }

    template <int _New_element_size>
    _Array_view_base<_Rank,_New_element_size> _Reinterpret_as() const __GPU
    {
        static_assert(_Rank==1, "reinterpret_as is only permissible on array views of rank 1");
        int _New_size = _Calculate_reinterpreted_size<_Element_size,_New_element_size>(this->_M_view_extent.size());
        return _Array_view_base<_Rank,_New_element_size>(this->_M_buffer_descriptor,
                                                         this->_M_total_linear_offset,
                                                         Concurrency::extent<_Rank>(_New_size));
    }

    template <int _New_rank>
    _Array_view_base<_New_rank, _Element_size> _View_as(const Concurrency::extent<_New_rank>& _View_extent) const __GPU
    {
        static_assert(_Rank==1, "view_as is only permissible on array views of rank 1");
        return _Array_view_base<_New_rank, _Element_size>(this->_M_buffer_descriptor,
                                                          this->_M_total_linear_offset,
                                                          _View_extent,
                                                          index<_New_rank>(),
                                                          _View_extent);
    }

    _Ret_ _View_shape* _Create_buffer_view_shape() const __CPU_ONLY
    {
        unsigned int bufElemSize = static_cast<unsigned int>(_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_buffer_elem_size());
        unsigned int elemSize = _Element_size * sizeof(int);

        size_t linearOffsetInBytes = this->_Base_linear_offset() * sizeof(int);

        size_t baseLSDExtentInBytes = this->_M_array_extent[_Rank - 1];
        baseLSDExtentInBytes *= elemSize;

        size_t viewLSDOffsetInBytes = this->_M_view_offset[_Rank - 1];
        viewLSDOffsetInBytes *= elemSize;

        size_t viewLSDExtentInBytes = this->_M_view_extent[_Rank - 1];
        viewLSDExtentInBytes *= elemSize;

        // The base array extent, view extent, and view offset must be compatible with the underlying
        // buffer's element size
        if (((linearOffsetInBytes % bufElemSize) != 0) ||
            ((baseLSDExtentInBytes % bufElemSize) != 0) ||
            ((viewLSDOffsetInBytes % bufElemSize) != 0) ||
            ((viewLSDExtentInBytes % bufElemSize) != 0))
        {
            throw runtime_exception("The array_view base extent, view offset and/or view extent is incompatible with the underlying buffer", E_FAIL);
        }

        // The shape to be passed to the underlying buffer for registration must be in terms of
        // the element size of the buffer
        _ASSERTE((linearOffsetInBytes / bufElemSize) <= UINT_MAX);
        unsigned int linearOffset = static_cast<unsigned int>(linearOffsetInBytes / bufElemSize);

        unsigned int baseExtent[_Rank];
        unsigned int viewOffset[_Rank];
        unsigned int viewExtent[_Rank];
#pragma warning( push )
#pragma warning( disable : 6294 )
#pragma warning( disable : 6201 ) //  Index '-1' is out of valid index range '0' to '0' for possibly stack allocated buffer 'baseExtent'.
        for (int i = 0; i < _Rank - 1; ++i) {
            baseExtent[i] = this->_M_array_extent[i];
            viewOffset[i] = this->_M_view_offset[i];
            viewExtent[i] = this->_M_view_extent[i];
        }
#pragma warning( pop )

        // The extent in the least significant dimension needs to be adjusted for
        // difference in element size between the buffer and ourselves
        _ASSERTE((baseLSDExtentInBytes / bufElemSize) <= UINT_MAX);
        baseExtent[_Rank - 1] = static_cast<unsigned int>(baseLSDExtentInBytes / bufElemSize);

        _ASSERTE((viewLSDOffsetInBytes / bufElemSize) <= UINT_MAX);
        viewOffset[_Rank - 1] = static_cast<unsigned int>(viewLSDOffsetInBytes / bufElemSize);

        _ASSERTE((viewLSDExtentInBytes / bufElemSize) <= UINT_MAX);
        viewExtent[_Rank - 1] = static_cast<unsigned int>(viewLSDExtentInBytes / bufElemSize);

        return _View_shape::_Create_view_shape(_Rank, linearOffset, baseExtent, viewOffset, viewExtent);
    }

protected:

    // Underlying storage
    _Buffer_descriptor _M_buffer_descriptor;

private:

    void _Register(_In_opt_ const _View_key _Source_view_key = nullptr) __CPU_ONLY
    {
        _M_buffer_descriptor._Get_buffer_ptr()->_Register_view(_M_buffer_descriptor._Get_view_key(),
                                                               accelerator(accelerator::cpu_accelerator).default_view,
                                                               _Create_buffer_view_shape(),
                                                               _Source_view_key);

        if (_M_buffer_descriptor._M_curr_cpu_access_mode != _No_access)
        {
            _Buffer_ptr _PBuf;
            _Get_access_async(_M_buffer_descriptor._Get_view_key(),
                              accelerator(accelerator::cpu_accelerator).default_view,
                              _M_buffer_descriptor._M_curr_cpu_access_mode,
                              _PBuf)._Get();

            _M_buffer_descriptor._M_data_ptr = _PBuf->_Get_host_ptr();
        }
    }

    void _Register_copy(const _Array_view_base &_Other) __CPU_ONLY
    {
        _M_buffer_descriptor._Get_buffer_ptr()->_Register_view_copy(_M_buffer_descriptor._Get_view_key(), _Other._M_buffer_descriptor._Get_view_key());
    }

    void _Register(_In_ void* _Shape) __CPU_ONLY
    {
        if (_Shape == NULL) {
            return;
        }

        // Unregister and register with the right shape
        _Unregister();

        _M_buffer_descriptor._Get_buffer_ptr()->_Register_view(_M_buffer_descriptor._Get_view_key(),
                                                               accelerator(accelerator::cpu_accelerator).default_view,
                                                               reinterpret_cast<_View_shape*>(_Shape));

        if (_M_buffer_descriptor._M_curr_cpu_access_mode != _No_access)
        {
            _Buffer_ptr _PBuf;
            _Get_access_async(_M_buffer_descriptor._Get_view_key(),
                              accelerator(accelerator::cpu_accelerator).default_view,
                              _M_buffer_descriptor._M_curr_cpu_access_mode,
                              _PBuf)._Get();

            _M_buffer_descriptor._M_data_ptr = _PBuf->_Get_host_ptr();
        }
    }

    void _Unregister(bool _Throw_exception = true) __CPU_ONLY
    {
        if (!_Throw_exception && (std::current_exception() == nullptr)) {
            _Throw_exception = true;
        }

        try
        {
            _M_buffer_descriptor._Get_buffer_ptr()->_Unregister_view(_M_buffer_descriptor._Get_view_key());
        }
        catch(...)
        {
            if (_Throw_exception) {
                throw;
            }
        }
    }

    static _Ret_ void* _Create_projection_buffer_shape(const _Buffer_descriptor& _Descriptor, unsigned int _Dim, int _Dim_offset) __CPU_ONLY
    {
        _View_shape* _Base_shape = _Get_buffer_view_shape(_Descriptor);

        std::vector<unsigned int> _New_view_extent(_Base_shape->_Get_rank());
        std::vector<unsigned int> _New_view_offset(_Base_shape->_Get_rank());
        bool *_New_projection_info = new bool[_Base_shape->_Get_rank()];
        for (unsigned int _I = 0; _I < _Base_shape->_Get_rank(); ++_I)
        {
            _New_view_extent[_I] = _Base_shape->_Get_view_extent()[_I];
            _New_view_offset[_I] = _Base_shape->_Get_view_offset()[_I];
            _New_projection_info[_I] = _Base_shape->_Get_projection_info()[_I];
        }

        // The _Dim'th non-projected dimension needs to be found
        unsigned int _UnProjectedDimCount = 0;
        for (unsigned int _I = 0; _I < _Base_shape->_Get_rank(); ++_I)
        {
            if (_Base_shape->_Get_projection_info()[_I]) {
                continue;
            }

            if (_UnProjectedDimCount == _Dim) {
                _New_view_extent[_I] = 1;
                _New_view_offset[_I] += _Dim_offset;
                _New_projection_info[_I] = true;
                break;
            }
            else {
                _UnProjectedDimCount++;
            }
        }

        auto _PView_shape = _View_shape::_Create_view_shape(_Base_shape->_Get_rank(),
                                                            _Base_shape->_Get_linear_offset(),
                                                            _Base_shape->_Get_base_extent(),
                                                            _New_view_offset.data(),
                                                            _New_view_extent.data(),
                                                            _New_projection_info);

        delete [] _New_projection_info;

        return _PView_shape;
    }

    static _Ret_ void* _Create_section_buffer_shape(const _Buffer_descriptor& _Descriptor,
                                              const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __CPU_ONLY
    {
        _View_shape* _Base_shape = _Get_buffer_view_shape(_Descriptor);
        if (_Base_shape->_Get_rank() == _Rank) {
            return NULL;
        }

        std::vector<unsigned int> _New_view_extent(_Base_shape->_Get_rank());
        std::vector<unsigned int> _New_view_offset(_Base_shape->_Get_rank());
        unsigned int _I = 0, _J = 0;
        while (_I < _Base_shape->_Get_rank())
        {
            if (_Base_shape->_Get_projection_info()[_I])
            {
                _New_view_extent[_I] = _Base_shape->_Get_view_extent()[_I];
                _New_view_offset[_I] = _Base_shape->_Get_view_offset()[_I];
            }
            else
            {
                // If _J is the least significant dimension, then we need to adjust the
                // offset and extent for the underlying buffer's element size
                if (_J == (_Rank - 1))
                {
                    unsigned int bufElemSize = static_cast<unsigned int>(_Descriptor._Get_buffer_ptr()->_Get_master_buffer_elem_size());
                    unsigned int elemSize = _Element_size * sizeof(int);

                    size_t sectionLSDOriginInBytes = _Section_origin[_J];
                    sectionLSDOriginInBytes *= elemSize;

                    size_t sectionLSDExtentInBytes = _Section_extent[_J];
                    sectionLSDExtentInBytes *= elemSize;

                    // The section offset and extent must be compatible with the underlying
                    // buffer's element size
                    if (((sectionLSDOriginInBytes % bufElemSize) != 0) ||
                        ((sectionLSDExtentInBytes % bufElemSize) != 0))
                    {
                        throw runtime_exception("The array_view section origin and/or extent is incompatible with the underlying buffer", E_FAIL);
                    }

                    // The extent in the least significant dimension needs to be adjusted for
                    // difference in element size between the buffer and ourselves
                    _ASSERTE((sectionLSDOriginInBytes / bufElemSize) <= UINT_MAX);
                    _New_view_offset[_I] = _Base_shape->_Get_view_offset()[_I] + static_cast<unsigned int>(sectionLSDOriginInBytes / bufElemSize);

                    _ASSERTE((sectionLSDExtentInBytes / bufElemSize) <= UINT_MAX);
                    _New_view_extent[_I] = static_cast<unsigned int>(sectionLSDExtentInBytes / bufElemSize);
                }
                else
                {
                    _New_view_extent[_I] = _Section_extent[_J];
                    _New_view_offset[_I] = _Base_shape->_Get_view_offset()[_I] + _Section_origin[_J];
                }

                _J++;
            }

            _I++;
        }

        _ASSERTE(_J == _Rank);

        return _View_shape::_Create_view_shape(_Base_shape->_Get_rank(),
                                               _Base_shape->_Get_linear_offset(),
                                               _Base_shape->_Get_base_extent(),
                                               _New_view_offset.data(),
                                               _New_view_extent.data(),
                                               _Base_shape->_Get_projection_info());
    }

    void _Register() __GPU_ONLY {}

    void _Register_copy(const _Array_view_base &/*_Other*/) __GPU_ONLY
    {
    }

    void _Register(_In_ void* /*_Shape*/) __GPU_ONLY
    {
    }

    void _Unregister(bool /*_Throw_exception*/ = true) __GPU_ONLY
    {
    }

    static _Ret_ void* _Create_projection_buffer_shape(const _Buffer_descriptor& /*_Descriptor*/, int /*_Dim*/, int /*_I*/) __GPU_ONLY
    {
        return NULL;
    }

    static _Ret_ void* _Create_section_buffer_shape(const _Buffer_descriptor& /*_Descriptor*/, const Concurrency::index<_Rank>& /*_Section_origin*/, const Concurrency::extent<_Rank>& /*_Section_extent*/) __GPU_ONLY
    {
        return NULL;
    }
};

template<typename _Container>
struct _Is_container
{
    template<class _Uty> static auto _Fn(_Uty _Val, decltype(_Val.size(), _Val.data(), 0)) -> std::true_type;
    template<class _Uty> static auto _Fn(_Uty _Val, ...) -> std::false_type;
    typedef decltype(_Fn(std::declval<_Container>(),0)) type;
};

} // namespace details


/// <summary>
///     An array_view is an N-dimensional view over data held in another container (such as array&lt;T,N&gt;
///     or other container.  It exposes an indexing interface congruent to that of array&lt;T,N&gt;).
/// </summary>
/// <param name="_Rank">
///     The number of dimensions of this array_view.
/// </param>
/// <param name="_Value_type">
///     The type of the element.
/// </param>
template <typename _Value_type, int _Rank = 1> class array_view : public _Array_view_base<_Rank, sizeof(_Value_type)/sizeof(int)>
{
    typedef _Array_view_base<_Rank, sizeof(_Value_type)/sizeof(int)> _Base;

    _CPP_AMP_VERIFY_RANK(_Rank, array_view);
    static_assert(0 == (sizeof(_Value_type) % sizeof(int)), "only value types whose size is a multiple of the size of an integer are allowed in array views");

    friend class details::_Array_view_projection_helper<_Value_type,_Rank>;
    friend class details::_Array_view_projection_helper<_Value_type,_Rank+1>;

    friend class array_view<_Value_type, _Rank>;
    friend class array_view<const _Value_type, _Rank>;

    friend class array_view<_Value_type, _Rank+1>;
    friend class array_view<const _Value_type, _Rank+1>;

    template <typename _T, int _R>
    friend class array;

    friend const typename _Base::_Buffer_descriptor& details::_Get_buffer_descriptor<array_view<_Value_type, _Rank>>(const array_view<_Value_type, _Rank>& _Array) __GPU;

public:
    static const int rank = _Rank;
    typedef typename _Value_type value_type;


    /// <summary>
    ///     Destroys this array_view and reclaims resources.
    /// </summary>
    ~array_view() __GPU {}

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src array.  The extent of the
    ///   array_view is that of the _Src array, and the origin of the array view is at zero.
    /// </summary>
    /// <param name="_Src">
    ///   An array which contains the data that this array_view is bound to.
    /// </param>
    array_view(array<_Value_type,_Rank>& _Src) __GPU
        : _Base(_Get_buffer_descriptor(_Src), _Src.extent)
    {
        _Initialize();
    }

    /// <summary>
    ///   Copy constructor. Shallow copy.
    /// </summary>
    array_view(const array_view& _Other) __GPU
        : _Base(_Other)
    {
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is not bound to a data source.
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    explicit array_view(const Concurrency::extent<_Rank>& _Extent) __CPU_ONLY
        :_Base(_Extent)
    {
        _Initialize(_Extent.size(), true);
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> array_view(const Concurrency::extent<_Rank>& _Extent, _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(),_Extent)
    {
        static_assert( std::is_same<decltype(_Src.data()), _Value_type*>::value, "container element type and array view element type must match");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    /// <param name="_Src">
    ///   A pointer to the source data this array_view will bind to.  If the number of elements pointed to
    ///   by _Src is less than the size of _Extent, undefined behavior results.
    /// </param>
    array_view(const Concurrency::extent<_Rank>& _Extent, _Value_type * _Src) __GPU
        :_Base(_Src,_Extent)
    {
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is not bound to a data source.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array_view.
    /// </param>
    explicit array_view(int _E0) __CPU_ONLY
        :_Base(Concurrency::extent<1>(_E0))
    {
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize(this->get_extent().size(), true);
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    ////  The length of the array_view is the same as the length of the container
    /// </summary>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> explicit array_view(_Container& _Src, typename std::enable_if<details::_Is_container<_Container>::type::value,void **>::type = 0) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<1>(static_cast<int>(_Src.size())))
    {
        if (_Src.size() > INT_MAX) {
            throw runtime_exception("Invalid _Src container argument - _Src size is greater than INT_MAX", E_INVALIDARG);
        }
        static_assert( std::is_same<decltype(_Src.data()), _Value_type*>::value, "container element type and array view element type must match");
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> explicit array_view(int _E0, _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<1>(_E0))
    {
        static_assert( std::is_same<decltype(_Src.data()), _Value_type*>::value, "container element type and array view element type must match");
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is not bound to a data source.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    explicit array_view(int _E0, int _E1) __CPU_ONLY
        :_Base(Concurrency::extent<2>(_E0,_E1))
    {
        static_assert(_Rank == 2, "rank must be 2");
        _Initialize(this->get_extent().size(), true);
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> explicit array_view(int _E0, int _E1, _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<2>(_E0,_E1))
    {
        static_assert( std::is_same<decltype(_Src.data()), _Value_type*>::value, "container element type and array view element type must match");
        static_assert(_Rank == 2, "rank must be 2");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is not bound to a data source.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    explicit array_view(int _E0, int _E1, int _E2) __CPU_ONLY
        :_Base(Concurrency::extent<3>(_E0,_E1,_E2))
    {
        static_assert(_Rank == 3, "rank must be 3");
        _Initialize(this->get_extent().size(), true);
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> explicit array_view(int _E0, int _E1, int _E2, _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<3>(_E0,_E1,_E2))
    {
        static_assert( std::is_same<decltype(_Src.data()), _Value_type*>::value, "container element type and array view element type must match");
        static_assert(_Rank == 3, "rank must be 3");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to. If the number of elements pointed to
    ///   by _Src is less than _E0, undefined behavior results.
    /// </param>
    explicit array_view(int _E0, _In_ _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<1>(_E0))
    {
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the array _Src.
    /// </summary>
    /// <param name="_Src">
    ///   An array which contains the data that this array_view is bound to.
    /// </param>
    template <int _Size> explicit array_view(_In_ _Value_type (&_Src) [_Size]) __GPU
        :_Base(_Src, Concurrency::extent<1>(_Size))
    {
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.  If the number of elements pointed to
    ///   by _Src is less than _E0*_E1, undefined behavior results.
    /// </param>
    explicit array_view(int _E0, int _E1, _In_ _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<2>(_E0,_E1))
    {
        static_assert(_Rank == 2, "rank must be 2");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.  If the number of elements pointed to
    ///   by _Src is less than _E0*_E1*_E2, undefined behavior results.
    /// </param>
    explicit array_view(int _E0, int _E1, int _E2, _In_ _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<3>(_E0,_E1,_E2))
    {
        static_assert(_Rank == 3, "rank must be 3");
        _Initialize();
    }

    /// <summary>
    ///   Copy Assignment operator. Shallow copy.
    /// </summary>
    array_view& operator=(const array_view& _Other) __GPU
    {
        _Base::operator=(_Other);
        return *this;
    }

    /// <summary>
    ///   Copies elements from this array_view to the destination array.
    /// </summary>
    void copy_to(array<_Value_type,_Rank>& _Dest) const __CPU_ONLY
    {
        copy(*this,_Dest);
    }

    /// <summary>
    ///   Copies elements from this array_view to the destination array_view.
    /// </summary>
    void copy_to(const array_view<_Value_type,_Rank>& _Dest) const __CPU_ONLY
    {
        copy(*this,_Dest);
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array_view.  If the array_view rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Result_type operator[] (int _I) const __GPU
    {
        return details::_Array_view_projection_helper<_Value_type,_Rank>::_Project0(this, _I);
    }

    /// <summary>
    ///     Get a reference to the element indexed by _Index. Unlike the other indexing operators for accessing the
    ///     array_view on the CPU, this method does not implicitly synchronize this array_view's contents to the CPU.
    ///     After accessing the array_view on a remote location or performing a copy operation involving this array_view
    ///     users are responsible to explicitly synchronize the array_view to the CPU before calling this method.
    ///     Failure to do so results in undefined behavior.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     Reference to the element indexed by _Index
    /// </returns>
    _Value_type& get_ref(const index<_Rank>& _Index) const __GPU
    {
        void *_Ptr = this->_Access(_Index);
        return *reinterpret_cast<value_type*>(_Ptr);
    }

    /// <summary>
    ///     Get the element value indexed by _I
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I
    /// </returns>
    _Value_type& operator[] (const index<_Rank>& _Index) const __GPU
    {
        return this->operator()(_Index);
    }

    /// <summary>
    ///     Get the element value indexed by _I
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I
    /// </returns>
    _Value_type& operator() (const index<_Rank>& _Index) const __GPU
    {
        void * _Ptr = this->_Access(_Read_write_access, _Index);
        return *reinterpret_cast<_Value_type*>(_Ptr);
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array_view.  If the array_view rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Result_type operator() (int _I) const __GPU
    {
        return details::_Array_view_projection_helper<_Value_type,_Rank>::_Project0(this, _I);
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
    _Value_type& operator() (int _I0, int _I1) const __GPU
    {
        static_assert(_Rank == 2, "value_type& array_view::operator()(int,int) is only permissible on array_view<T, 2>");
        return this->operator()(index<2>(_I0,_I1));
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
    _Value_type& operator() (int _I0, int _I1, int _I2) const __GPU
    {
        static_assert(_Rank == 3, "value_type& array_view::operator()(int,int,int) is only permissible on array_view<T, 3>");
        return this->operator()(index<3>(_I0,_I1,_I2));
    }

    /// <summary>
    ///     Produces a subsection of the source array_view at the given origin and extent.
    /// </summary>
    /// <param name="_Section_origin">
    ///     The origin of the section.
    /// </param>
    /// <param name="_Section_extent">
    ///     The extent of the section
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) const __GPU
    {
        return _Convert<_Value_type>(this->_Section(_Section_origin, _Section_extent));
    }

    /// <summary>
    ///     Produces a subsection of the source array_view with origin specified by an index, with
    ///     an extent of (this-&gt;extent - _Idx).
    /// </summary>
    /// <param name="_Idx">
    ///     The index that specifies the origin of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(const Concurrency::index<_Rank>& _Idx) const __GPU
    {
        return section(_Idx, this->extent - _Idx);
    }

    /// <summary>
    ///     Produces a subsection of the source array_view with origin of zero, with
    ///     an extent of _Ext.
    /// </summary>
    /// <param name="_Ext">
    ///     The extent of this section
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(const Concurrency::extent<_Rank>& _Ext) const __GPU
    {
        return section(Concurrency::index<_Rank>(), _Ext);
    }

    /// <summary>
    ///     Produces a one-dimensional subsection of the source array_view with origin specified by the index
    ///     components _I0, with extent _E0.
    /// </summary>
    /// <param name="_I0">
    ///     The origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(int _I0, int _E0) const __GPU
    {
        static_assert(_Rank == 1, "rank must be 1");
        return section(Concurrency::index<1>(_I0), Concurrency::extent<1>(_E0));
    }

    /// <summary>
    ///     Produces a two-dimensional subsection of the source array_view with origin specified by the index
    ///     components (_I0,_I1), with extent (_E0,_E1).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(int _I0, int _I1, int _E0, int _E1) const __GPU
    {
        static_assert(_Rank == 2, "rank must be 2");
        return section(Concurrency::index<2>(_I0,_I1), Concurrency::extent<2>(_E0,_E1));
    }

    /// <summary>
    ///     Produces a three-dimensional subsection of the source array_view with origin specified by the index
    ///     components (_I0,_I1,_I2), with extent (_E0,_E1,_E2).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The next-to-most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I2">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The next-to-most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E2">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(int _I0, int _I1, int _I2, int _E0, int _E1, int _E2) const __GPU
    {
        static_assert(_Rank == 3, "rank must be 3");
        return section(Concurrency::index<3>(_I0,_I1,_I2), Concurrency::extent<3>(_E0,_E1,_E2));
    }

    /// <summary>
    ///     Produces a (possibly unsafe) reinterpretation of this array_view that is linear and with
    ///     a different element type. The size of _Value_type2 must evenly divide into the size of
    ///     this array.
    /// </summary>
    /// <returns>
    ///     A linear array_view with a reinterpreted element type.
    /// </returns>
    template <typename _Value_type2> array_view<_Value_type2, _Rank> reinterpret_as() const __GPU
    {
        return _Convert<_Value_type2>(this->template _Reinterpret_as<sizeof(_Value_type2)/sizeof(int)>());
    }

    /// <summary>
    ///     Produces an array_view of a different rank over this array_view's data.
    /// </summary>
    /// <param name="_View_extent">
    ///     The reshaping extent.
    /// </param>
    /// <returns>
    ///     A reshaped array_view.
    /// </returns>
    template <int _New_rank> array_view<_Value_type,_New_rank> view_as(const Concurrency::extent<_New_rank>& _View_extent) const __GPU
    {
        return _Convert<_Value_type>(this->_View_as(_View_extent));
    }

    /// <summary>
    ///     Returns a pointer to the raw data of this array_view.
    /// </summary>
    _Ret_ _Value_type* data() const __GPU
    {
        static_assert(_Rank == 1, "array_view::data() is only permissible on array_view<T, 1>");
        return &this->operator[](index<_Rank>());
    }

    /// <summary>
    ///     Informs the array_view that its bound memory has been modified outside
    ///     the array_view interface.  This will render all cached information stale.
    /// </summary>
    void refresh() const __CPU_ONLY
    {
        // If the array_view corresponds to a ubiquitous buffer with no data source,
        // then refresh is a no-op
        if (!this->_M_buffer_descriptor._Get_buffer_ptr()->_Has_data_source()) {
            return;
        }

        _Buffer_ptr _PBuf;
        _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view(), _Write_access, _PBuf)._Get();
    }

    /// <summary>
    ///     Asynchronously synchronizes any modifications made to "this" array_view to the specified accelerator_view.
    /// </summary>
    /// <param name="_Accl_view">
    ///     The target accelerator_view to synchronize to.
    /// </param>
    /// <param name="_Access_type">
    ///     The desired access_type on the target accelerator_view.
    ///     This parameter has a default value of access_type_read.
    /// </param>
    /// <returns>
    ///     A future upon which to wait for the operation to complete.
    /// </returns>
    concurrency::completion_future synchronize_to_async(const accelerator_view& _Accl_view, access_type _Access_type = access_type_read) const __CPU_ONLY
    {
        auto _Async_op_id = details::_Get_amp_trace()->_Launch_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;
        _Event _Ev;

        if (_Access_type != access_type_none) {
            _Ev = _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), _Accl_view, _Get_synchronize_access_mode(_Access_type), _PBuf);
        }

        return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
    }

    /// <summary>
    ///     Asynchronously synchronizes any modifications made to "this" array_view to its source data.
    /// </summary>
    /// <param name="_Access_type">
    ///     The desired access_type on the target accelerator_view.
    ///     This parameter has a default value of access_type_read.
    /// </param>
    /// <returns>
    ///     A future upon which to wait for the operation to complete.
    /// </returns>
    concurrency::completion_future synchronize_async(access_type _Access_type = access_type_read) const __CPU_ONLY
    {
        auto _Async_op_id = details::_Get_amp_trace()->_Launch_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;
        _Event _Ev;

        // If the array_view corresponds to a ubiquitous buffer with no data source, then synchronize is a no-op
        if ((_Access_type != access_type_none) && this->_M_buffer_descriptor._Get_buffer_ptr()->_Has_data_source())
        {
            _Ev = _Get_access_async(this->_M_buffer_descriptor._Get_view_key(),
                                    this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view(),
                                    _Get_synchronize_access_mode(_Access_type),
                                    _PBuf);
        }

        return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
    }

    /// <summary>
    ///     Synchronizes any modifications made to "this" array_view to the specified accelerator_view.
    /// </summary>
    /// <param name="_Accl_view">
    ///     The target accelerator_view to synchronize to.
    /// </param>
    /// <param name="_Access_type">
    ///     The desired access_type on the target accelerator_view.
    ///     This parameter has a default value of access_type_read.
    /// </param>
    void synchronize_to(const accelerator_view& _Accl_view, access_type _Access_type = access_type_read) const __CPU_ONLY
    {
        auto _Span_id = details::_Get_amp_trace()->_Start_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;

        if (_Access_type != access_type_none) {
            _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), _Accl_view, _Get_synchronize_access_mode(_Access_type), _PBuf)._Get();
        }

        details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    /// <summary>
    ///     Synchronizes any modifications made to "this" array_view to its source data.
    /// </summary>
    /// <param name="_Access_type">
    ///     The desired access_type on the target accelerator_view.
    ///     This parameter has a default value of access_type_read.
    /// </param>
    void synchronize(access_type _Access_type = access_type_read) const __CPU_ONLY
    {
        auto _Span_id = details::_Get_amp_trace()->_Start_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;

        // If the array_view corresponds to a ubiquitous buffer with no data source, then synchronize is a no-op
        if ((_Access_type != access_type_none) && this->_M_buffer_descriptor._Get_buffer_ptr()->_Has_data_source())
        {
            _Get_access_async(this->_M_buffer_descriptor._Get_view_key(),
                              this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view(),
                              _Get_synchronize_access_mode(_Access_type),
                              _PBuf)._Get();
        }

        details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    /// <summary>
    ///     Discards the current data underlying this view. This is an optimization
    ///     hint to the runtime used to avoid copying the current contents of the view to a target
    ///     accelerator_view that it is accessed on, and its use is recommended if the existing
    ///     content is not needed. This method is only available in a restrict(cpu) context and
    ///     cannot be used in a restrict(amp) context.
    /// </summary>
    void discard_data() const __CPU_ONLY
    {
        this->_M_buffer_descriptor._Get_buffer_ptr()->_Discard(this->_M_buffer_descriptor._Get_view_key());
    }

    /// <summary>
    ///     Returns the accelerator_view where the data source of the array_view is located.
    ///     If the array_view does not have a data source, this API throws a runtime_exception
    /// </summary>
    accelerator_view get_source_accelerator_view() const
    {
        if (this->_M_buffer_descriptor._Get_buffer_ptr()->_Has_data_source()) {
            return this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view();
        }
        else {
            throw runtime_exception("Cannot query source accelerator_view for an array_view without a data source.", E_INVALIDARG);
        }
    }

    __declspec(property(get=get_source_accelerator_view)) accelerator_view source_accelerator_view;

private:
    template <typename _T, int _R>
    static array_view<_T,_R> _Convert(const _Array_view_base<_R,sizeof(_T)/sizeof(int)>& _Other) __GPU
    {
        static_assert(sizeof(array_view<_T,_R>) == sizeof(_Array_view_base<_R,sizeof(_T)/sizeof(int)>), "ASSERT FAILURE: implementation relies on binary conversion between the two");
        return (*reinterpret_cast<const array_view<_T,_R>*>(&_Other));
    }

    void _Project0(int _I, array_view<_Value_type, _Rank-1> &_Projected_view) const __GPU
    {
        _Base::_Project0(_I, _Projected_view);
        _Projected_view._Initialize();
    }

    array_view() __GPU {}

    array_view(const array_view& _Other, const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __GPU
        :_Base(_Other, _Section_origin, _Section_extent)
    {
        _Initialize();
    }

    array_view(typename _Base::_Buffer_descriptor& _Src_buffer, const Concurrency::extent<_Rank>& _Extent) __GPU
        :_Base(_Src_buffer,_Extent)
    {
        _Initialize();
    }

    void _Initialize() __GPU
    {
        // Set the type access mode
        this->_M_buffer_descriptor._M_type_access_mode = _Read_write_access;
    }

    void _Initialize(size_t _Src_data_size, bool _Discard_data = false) __CPU_ONLY
    {
        // Ensure that the _Src_data_size is at least as big as the size
        // of the array_view
        if (_Src_data_size < this->extent.size()) {
            throw runtime_exception("Invalid _Src container argument - _Src size is less than the size of the array_view.", E_INVALIDARG);
        }

        _Initialize();

        if (_Discard_data) {
            discard_data();
        }
    }

};  // class array_view<T,R>

// array_view<const T,R>
template <typename _Value_type, int _Rank>
class array_view<const _Value_type, _Rank> : public _Array_view_base<_Rank, sizeof(_Value_type)/sizeof(int)>
{
    _CPP_AMP_VERIFY_RANK(_Rank, array_view);
    static_assert(0 == (sizeof(_Value_type) % sizeof(int)), "only value types whose size is a multiple of the size of an integer are allowed in array views");

    typedef _Array_view_base<_Rank, sizeof(_Value_type)/sizeof(int)> _Base;

    friend class details::_Const_array_view_projection_helper<_Value_type,_Rank>;
    friend class details::_Const_array_view_projection_helper<_Value_type,_Rank+1>;

    friend class array_view<_Value_type, _Rank>;
    friend class array_view<const _Value_type, _Rank>;

    friend class array_view<_Value_type, _Rank+1>;
    friend class array_view<const _Value_type, _Rank+1>;

    friend const typename _Base::_Buffer_descriptor& details::_Get_buffer_descriptor<array_view<const _Value_type, _Rank>>(const array_view<const _Value_type, _Rank>& _Array) __GPU;

public:
    static const int rank = _Rank;
    typedef typename const _Value_type value_type;

    /// <summary>
    ///     Destroys this array_view and reclaims resources.
    /// </summary>
    ~array_view() __GPU {}

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src array.  The extent of the
    ///   array_view is that of the _Src array, and the origin of the array view is at zero.
    /// </summary>
    /// <param name="_Src">
    ///   An array which contains the data that this array_view is bound to.
    /// </param>
    array_view(const array<_Value_type,_Rank>& _Src) __GPU
        :_Base(_Get_buffer_descriptor(_Src), _Src.extent)
    {
        _Initialize();
    }

    /// <summary>
    ///   Copy constructor. Shallow copy.
    /// </summary>
    array_view(const array_view<_Value_type,_Rank>& _Src) __GPU
        :_Base(_Src)
    {
        _Initialize();
    }

    /// <summary>
    ///   Copy constructor. Shallow copy.
    /// </summary>
    array_view(const array_view<const _Value_type,_Rank>& _Src) __GPU
        :_Base(_Src)
    {
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> array_view(const Concurrency::extent<_Rank>& _Extent, const _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(),_Extent)
    {
        static_assert( std::is_same<typename std::remove_const<typename std::remove_reference<decltype(*_Src.data())>::type>::type, _Value_type>::value, "container element type and array view element type must match");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container;
    ////  The length of the array_view is the same as the length of the container
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> explicit array_view(const _Container& _Src, typename std::enable_if<details::_Is_container<_Container>::type::value,void **>::type = 0) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<1>(static_cast<int>(_Src.size())))
    {
        if (_Src.size() > INT_MAX) {
            throw runtime_exception("Invalid _Src container argument - _Src size is greater than INT_MAX", E_INVALIDARG);
        }
        static_assert( std::is_same<decltype(_Src.data()), const _Value_type*>::value, "container element type and array view element type must match");
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> array_view(const Concurrency::extent<_Rank>& _Extent, _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(),_Extent)
    {
        static_assert( std::is_same<typename std::remove_const<typename std::remove_reference<decltype(*_Src.data())>::type>::type, _Value_type>::value, "container element type and array view element type must match");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    /// <param name="_Src">
    ///   A pointer to the source data this array_view will bind to.  If the number of elements pointed to
    ///   by _Src is less than the size of _Extent, undefined behavior results.
    /// </param>
    array_view(const Concurrency::extent<_Rank>& _Extent, const _Value_type * _Src) __GPU
        :_Base(_Src,_Extent)
    {
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_Extent">
    ///   The extent of this array view.
    /// </param>
    /// <param name="_Src">
    ///   A pointer to the source data this array_view will bind to.  If the number of elements pointed to
    ///   by _Src is less than the size of _Extent, undefined behavior results.
    /// </param>
    array_view(const Concurrency::extent<_Rank>& _Extent, _In_ _Value_type * _Src) __GPU
        :_Base(_Src,_Extent)
    {
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> array_view(int _E0, const _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<1>(_E0))
    {
        static_assert( std::is_same<typename std::remove_const<typename std::remove_reference<decltype(*_Src.data())>::type>::type, _Value_type>::value, "container element type and array view element type must match");
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container;
    ////  The length of the array_view is the same as the length of the container
    /// </summary>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <int _Size> explicit array_view(const _In_ _Value_type (&_Src) [_Size]) __GPU
        :_Base(_Src, Concurrency::extent<1>(_Size))
    {
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> array_view(int _E0, int _E1, const _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<2>(_E0,_E1))
    {
        static_assert( std::is_same<typename std::remove_const<typename std::remove_reference<decltype(*_Src.data())>::type>::type, _Value_type>::value, "container element type and array view element type must match");
        static_assert(_Rank == 2, "rank must be 2");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data contained in the _Src container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.
    /// </param>
    template <typename _Container> array_view(int _E0, int _E1, int _E2, const _Container& _Src) __CPU_ONLY
        :_Base(_Src.data(), Concurrency::extent<3>(_E0,_E1,_E2))
    {
        static_assert( std::is_same<typename std::remove_const<typename std::remove_reference<decltype(*_Src.data())>::type>::type, _Value_type>::value, "container element type and array view element type must match");
        static_assert(_Rank == 3, "rank must be 3");
        _Initialize(_Src.size());
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to. If the number of elements pointed to
    ///   by _Src is less than _E0, undefined behavior results.
    /// </param>
    array_view(int _E0, const _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<1>(_E0))
    {
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.  If the number of elements pointed to
    ///   by _Src is less than _E0*_E1, undefined behavior results.
    /// </param>
    array_view(int _E0, int _E1, const _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<2>(_E0,_E1))
    {
        static_assert(_Rank == 2, "rank must be 2");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.  If the number of elements pointed to
    ///   by _Src is less than _E0*_E1*_E2, undefined behavior results.
    /// </param>
    array_view(int _E0, int _E1, int _E2, const _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<3>(_E0,_E1,_E2))
    {
        static_assert(_Rank == 3, "rank must be 3");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to. If the number of elements pointed to
    ///   by _Src is less than _E0, undefined behavior results.
    /// </param>
    array_view(int _E0, _In_ _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<1>(_E0))
    {
        static_assert(_Rank == 1, "rank must be 1");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.  If the number of elements pointed to
    ///   by _Src is less than _E0*_E1, undefined behavior results.
    /// </param>
    array_view(int _E0, int _E1, _In_ _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<2>(_E0,_E1))
    {
        static_assert(_Rank == 2, "rank must be 2");
        _Initialize();
    }

    /// <summary>
    ///   Construct an array_view which is bound to the data pointed to by _Src.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array_view.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array_view.
    /// </param>
    /// <param name="_Src">
    ///   A container which contains the data that this array_view is bound to.  If the number of elements pointed to
    ///   by _Src is less than _E0*_E1*_E2, undefined behavior results.
    /// </param>
    array_view(int _E0, int _E1, int _E2, _In_ _Value_type * _Src) __GPU
        :_Base(_Src, Concurrency::extent<3>(_E0,_E1,_E2))
    {
        static_assert(_Rank == 3, "rank must be 3");
        _Initialize();
    }

    /// <summary>
    ///   Copy Assignment operator. Shallow copy.
    /// </summary>
    array_view& operator=(const array_view& _Other) __GPU
    {
        _Base::operator=(_Other);
        return *this;
    }

    /// <summary>
    ///   Copy Assignment operator. Shallow copy.
    /// </summary>
    array_view& operator=(const array_view<_Value_type, _Rank>& _Other) __GPU
    {
        _Base::operator=(_Other);
        return *this;
    }

    /// <summary>
    ///   Copies elements from this array_view to the destination array.
    /// </summary>
    void copy_to(array<_Value_type,_Rank>& _Dest) const __CPU_ONLY
    {
        copy(*this,_Dest);
    }

    /// <summary>
    ///   Copies elements from this array_view to the destination array_view.
    /// </summary>
    void copy_to(const array_view<_Value_type,_Rank>& _Dest) const __CPU_ONLY
    {
        copy(*this,_Dest);
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array_view.  If the array_view rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Const_result_type operator[] (int _I) const __GPU
    {
        return details::_Const_array_view_projection_helper<_Value_type,_Rank>::_Project0(this, _I);
    }

    /// <summary>
    ///     Get a reference to the element indexed by _Index. Unlike the other indexing operators for accessing the
    ///     array_view on the CPU, this method does not implicitly synchronize this array_view's contents to the CPU.
    ///     After accessing the array_view on a remote location or performing a copy operation involving this array_view
    ///     users are responsible to explicitly synchronize the array_view to the CPU before calling this method.
    ///     Failure to do so results in undefined behavior.
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     Reference to the element indexed by _Index
    /// </returns>
    const _Value_type& get_ref(const index<_Rank>& _Index) const __GPU
    {
        void *_Ptr = this->_Access(_Index);
        return *reinterpret_cast<value_type*>(_Ptr);
    }

    /// <summary>
    ///     Get the element value indexed by _I
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I
    /// </returns>
    const _Value_type& operator[] (const index<_Rank>& _Index) const __GPU
    {
        return this->operator()(_Index);
    }

    /// <summary>
    ///     Get the element value indexed by _I
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I
    /// </returns>
    const _Value_type& operator() (const index<_Rank>& _Index) const __GPU
    {
        void * _Ptr = this->_Access(_Read_access, _Index);
        return *reinterpret_cast<value_type*>(_Ptr);
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array_view.  If the array_view rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Const_result_type operator() (int _I) const __GPU
    {
        return details::_Const_array_view_projection_helper<_Value_type,_Rank>::_Project0(this, _I);
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
    const _Value_type& operator() (int _I0, int _I1) const __GPU
    {
        static_assert(_Rank == 2, "value_type& array_view::operator()(int,int) is only permissible on array_view<T, 2>");
        return this->operator()(index<2>(_I0,_I1));
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
    const _Value_type& operator() (int _I0, int _I1, int _I2) const __GPU
    {
        static_assert(_Rank == 3, "value_type& array_view::operator()(int,int,int) is only permissible on array_view<T, 3>");
        return this->operator()(index<3>(_I0,_I1,_I2));
    }

    /// <summary>
    ///     Produces a subsection of the source array_view at the given origin and extent.
    /// </summary>
    /// <param name="_Section_origin">
    ///     The origin of the section.
    /// </param>
    /// <param name="_Section_extent">
    ///     The extent of the section
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) const __GPU
    {
        return _Convert<_Value_type>(this->_Section(_Section_origin, _Section_extent));
    }

    /// <summary>
    ///     Produces a subsection of the source array_view with origin of zero, with
    ///     an extent of _Ext.
    /// </summary>
    /// <param name="_Ext">
    ///     The extent of this section
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(const Concurrency::extent<_Rank>& _Ext) const __GPU
    {
        return section(Concurrency::index<_Rank>(), _Ext);
    }

    /// <summary>
    ///     Produces a subsection of the source array_view with origin specified by an index, with
    ///     an extent of (this-&gt;extent - _Idx).
    /// </summary>
    /// <param name="_Idx">
    ///     The index that specifies the origin of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(const Concurrency::index<_Rank>& _Idx) const __GPU
    {
        return section(_Idx, this->extent - _Idx);
    }

    /// <summary>
    ///     Produces a one-dimensional subsection of the source array_view with origin specified by the index
    ///     components _I0, with extent _E0.
    /// </summary>
    /// <param name="_I0">
    ///     The origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(int _I0, int _E0) const __GPU
    {
        static_assert(_Rank == 1, "rank must be 1");
        return section(Concurrency::index<1>(_I0), Concurrency::extent<1>(_E0));
    }

    /// <summary>
    ///     Produces a two-dimensional subsection of the source array_view with origin specified by the index
    ///     components (_I0,_I1), with extent (_E0,_E1).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(int _I0, int _I1, int _E0, int _E1) const __GPU
    {
        static_assert(_Rank == 2, "rank must be 2");
        return section(Concurrency::index<2>(_I0,_I1), Concurrency::extent<2>(_E0,_E1));
    }

    /// <summary>
    ///     Produces a three-dimensional subsection of the source array_view with origin specified by the index
    ///     components (_I0,_I1,_I2), with extent (_E0,_E1,_E2).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The next-to-most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I2">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The next-to-most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E2">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view section(int _I0, int _I1, int _I2, int _E0, int _E1, int _E2) const __GPU
    {
        static_assert(_Rank == 3, "rank must be 3");
        return section(Concurrency::index<3>(_I0,_I1,_I2), Concurrency::extent<3>(_E0,_E1,_E2));
    }

    /// <summary>
    ///     Produces a (possibly unsafe) reinterpretation of this array_view that is linear and with
    ///     a different element type. The size of _Value_type2 must evenly divide into the size of
    ///     this array_view.
    /// </summary>
    /// <returns>
    ///     A linear array_view with a reinterpreted element type.
    /// </returns>
    template <typename _Value_type2> array_view<const _Value_type2, _Rank> reinterpret_as() const __GPU
    {
        return _Convert<_Value_type2>(this->template _Reinterpret_as<sizeof(_Value_type2)/sizeof(int)>());
    }

    /// <summary>
    ///     Produces an array_view of a different rank over this array_view's data.
    /// </summary>
    /// <param name="_View_extent">
    ///     The reshaping extent.
    /// </param>
    /// <returns>
    ///     A reshaped array_view.
    /// </returns>
    template <int _New_rank> array_view<const _Value_type,_New_rank> view_as(const Concurrency::extent<_New_rank>& _View_extent) const __GPU
    {
        return _Convert<_Value_type>(this->_View_as(_View_extent));
    }

    /// <summary>
    ///     Returns a pointer to the raw data of this array_view.
    /// </summary>
    const _Value_type* data() const __GPU
    {
        static_assert(_Rank == 1, "array_view::data() is only permissible on array_view<T, 1>");
        return &this->operator[](index<_Rank>());
    }

    /// <summary>
    ///     Informs the array_view that its bound memory has been modified outside
    ///     the array_view interface.  This will render all cached information stale.
    /// </summary>
    void refresh() const __CPU_ONLY
    {
        _Buffer_ptr _PBuf;
        _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view(), _Write_access, _PBuf)._Get();
    }

    /// <summary>
    ///     Asynchronously synchronizes any modifications made to "this" array_view to the specified accelerator_view.
    /// </summary>
    /// <param name="_Accl_view">
    ///     The target accelerator_view to synchronize to.
    /// </param>
    /// <returns>
    ///     A future upon which to wait for the operation to complete.
    /// </returns>
    concurrency::completion_future synchronize_to_async(const accelerator_view& _Accl_view) const __CPU_ONLY
    {
        auto _Async_op_id = details::_Get_amp_trace()->_Launch_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;
        _Event _Ev;

        _Ev = _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), _Accl_view, _Read_access, _PBuf);

        return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
    }

    /// <summary>
    ///     Asynchronously synchronizes any modifications made to "this" array_view to its source data.
    /// </summary>
    /// <returns>
    ///     A future upon which to wait for the operation to complete.
    /// </returns>
    concurrency::completion_future synchronize_async() const __CPU_ONLY
    {
        auto _Async_op_id = details::_Get_amp_trace()->_Launch_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;
        _Event _Ev;

        // If the array_view corresponds to a ubiquitous buffer with no data source,
        // then synchronize is a no-op
        if (this->_M_buffer_descriptor._Get_buffer_ptr()->_Has_data_source()) {
            _Ev = _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view(), _Read_access, _PBuf);
        }

        return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
    }

    /// <summary>
    ///     Synchronizes any modifications made to "this" array_view to the specified accelerator_view.
    /// </summary>
    /// <param name="_Accl_view">
    ///     The target accelerator_view to synchronize to.
    /// </param>
    void synchronize_to(const accelerator_view& _Accl_view) const __CPU_ONLY
    {
        auto _Span_id = details::_Get_amp_trace()->_Start_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;

        _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), _Accl_view, _Read_access, _PBuf)._Get();

        details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    /// <summary>
    ///     Synchronizes any modifications made to "this" array_view to its source data.
    /// </summary>
    void synchronize() const __CPU_ONLY
    {
        auto _Span_id = details::_Get_amp_trace()->_Start_array_view_synchronize_event_helper(this->_M_buffer_descriptor);

        _Buffer_ptr _PBuf;

        // If the array_view corresponds to a ubiquitous buffer with no data source,
        // then synchronize is a no-op
        if (this->_M_buffer_descriptor._Get_buffer_ptr()->_Has_data_source()) {
            _Get_access_async(this->_M_buffer_descriptor._Get_view_key(), this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view(), _Read_access, _PBuf)._Get();
        }

        details::_Get_amp_trace()->_Write_end_event(_Span_id);
    }

    /// <summary>
    ///     Returns the accelerator_view where the data source of the array_view is located.
    ///     If the array_view does not have a data source, this API throws a runtime_exception
    /// </summary>
    accelerator_view get_source_accelerator_view() const
    {
        if (this->_M_buffer_descriptor._Get_buffer_ptr()->_Has_data_source()) {
            return this->_M_buffer_descriptor._Get_buffer_ptr()->_Get_master_accelerator_view();
        }
        else {
            throw runtime_exception("Cannot query source accelerator_view for an array_view without a data source.", E_INVALIDARG);
        }
    }

    __declspec(property(get=get_source_accelerator_view)) accelerator_view source_accelerator_view;

private:
    template <typename _T, int _R>
    static array_view<const _T,_R> _Convert(const _Array_view_base<_R,sizeof(_T)/sizeof(int)>& _Other) __GPU
    {
        static_assert(sizeof(array_view<const _T,_R>) == sizeof(_Array_view_base<_R,sizeof(_T)/sizeof(int)>), "ASSERT FAILURE: implementation relies on binary conversion between the two");
        return (*reinterpret_cast<const array_view<const _T,_R>*>(&_Other));
    }

    void _Project0(int _I, array_view<const _Value_type, _Rank-1> &_Projected_view) const __GPU
    {
        _Base::_Project0(_I, _Projected_view);
        _Projected_view._Initialize();
    }

    array_view() __GPU {}

    array_view(const array_view& _Other, const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __GPU
        :
        _Base(_Other, _Section_origin, _Section_extent)
    {
        _Initialize();
    }

    void _Initialize() __GPU
    {
        // Set the type access mode
        this->_M_buffer_descriptor._M_type_access_mode = _Read_access;
    }

    void _Initialize(size_t _Src_data_size) __CPU_ONLY
    {
        // Ensure that the _Src_data_size is at least as big as the size
        // of the array_view
        if (_Src_data_size < this->extent.size()) {
            throw runtime_exception("Invalid _Src container argument - _Src size is less than the size of the array_view.", E_INVALIDARG);
        }

        _Initialize();
    }

};  // class array_view<const T,R>

// Forward declarations for copy functions
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array<_Value_type,_Rank>& _Src, array<_Value_type,_Rank>& _Dest);
template <typename _Value_type, int _Rank> void copy(const array<_Value_type,_Rank>& _Src, array<_Value_type,_Rank>& _Dest);
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, InputIterator _SrcLast, array<_Value_type, _Rank> &_Dest);
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, InputIterator _SrcLast, array<_Value_type, _Rank> &_Dest);
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, array<_Value_type, _Rank> &_Dest);
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, array<_Value_type, _Rank> &_Dest);
template <typename OutputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array<_Value_type, _Rank> &_Src, OutputIterator _DestIter);
template <typename OutputIterator, typename _Value_type, int _Rank> void copy(const array<_Value_type, _Rank> &_Src, OutputIterator _DestIter);
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> void copy(const array<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<const _Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> void copy(const array_view<const _Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<_Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> void copy(const array_view<_Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<const _Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> void copy(const array_view<const _Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest);
template <typename _Value_type, int _Rank> void copy(const array_view<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest);
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, InputIterator _SrcLast, const array_view<_Value_type, _Rank> &_Dest);
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, const array_view<_Value_type, _Rank> &_Dest);
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, InputIterator _SrcLast, const array_view<_Value_type, _Rank> &_Dest);
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, const array_view<_Value_type, _Rank> &_Dest);
template <typename OutputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<_Value_type, _Rank> &_Src, OutputIterator _DestIter);
template <typename OutputIterator, typename _Value_type, int _Rank> void copy(const array_view<_Value_type, _Rank> &_Src, OutputIterator _DestIter);

namespace direct3d
{
    template<typename _Value_type, int _Rank>
    array<_Value_type, _Rank> make_array(const Concurrency::extent<_Rank> &_Extent, const Concurrency::accelerator_view &_Av, _In_ IUnknown *_D3D_buffer) __CPU_ONLY;
}

/// <summary>
///     An array is a multi-dimensional data aggregate on a accelerator_view.
/// </summary>
/// <param name="_Rank">
///     The dimensionality of this array.
/// </param>
/// <param name="_Value_type">
///     The type of the elements in the array.
/// </param>
template <typename _Value_type, int _Rank = 1> class array
{
    // internal storage abstraction
    typedef details::_Buffer_descriptor _Buffer_descriptor;
    typedef _Array_flatten_helper<_Rank, typename Concurrency::extent<_Rank>::value_type, typename Concurrency::index<_Rank>::value_type> _Flatten_helper;

    _CPP_AMP_VERIFY_RANK(_Rank, array);
    static_assert(!std::is_const<_Value_type>::value, "array<const _Value_type> is not supported");
    static_assert(0 == (sizeof(_Value_type) % sizeof(int)), "only value types whose size is a multiple of the size of an integer are allowed in array");

    // Friends
    friend array<_Value_type,_Rank> direct3d::make_array<_Value_type, _Rank>(const Concurrency::extent<_Rank> &_Extent, const Concurrency::accelerator_view &_Av, _In_ IUnknown *_D3D_buffer) __CPU_ONLY;
    friend const _Buffer_descriptor& details::_Get_buffer_descriptor<array<_Value_type,_Rank>>(const array<_Value_type,_Rank>& _Array) __GPU;
    friend _Ret_ _Ubiquitous_buffer* details::_Get_buffer<array<_Value_type,_Rank>>(const array<_Value_type,_Rank>& _Array) __CPU_ONLY;
    friend _Event details::_Get_access_async<array<_Value_type,_Rank>>(const array<_Value_type,_Rank>& _Array, _Access_mode _Mode, _Buffer_ptr &_Buf_ptr) __CPU_ONLY;

    public:
    static const int rank = _Rank;
    typedef typename _Value_type value_type;

    /// <summary>
    ///   Construct an array from extents
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    explicit array(const Concurrency::extent<_Rank> & _Extent) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _Initialize(details::_Select_default_accelerator().default_view, access_type_auto);
    }

    /// <summary>
    ///   Construct array&lt;T,1&gt; with the extent _E0
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    explicit array(int _E0) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int) is only permissible on array<T, 1>");
        _Initialize(details::_Select_default_accelerator().default_view, access_type_auto);
    }

    /// <summary>
    ///   Construct an array&lt;T,2&gt; from two integer extents.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    explicit array(int _E0, int _E1) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int) is only permissible on array<T, 2>");
        _Initialize(details::_Select_default_accelerator().default_view, access_type_auto);
    }

    /// <summary>
    ///   Construct an array&lt;T,3&gt; from three integer extents.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    explicit array(int _E0, int _E1, int _E2) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int) is only permissible on array<T, 3>");
        _Initialize(details::_Select_default_accelerator().default_view, access_type_auto);
    }

    /// <summary>
    ///   Construct an array from extents, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    array(const Concurrency::extent<_Rank>& _Extent, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _Initialize(_Av, _Cpu_access_type);
    }

    /// <summary>
    ///    Construct array&lt;T,1&gt; with the extent _E0, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    array(int _E0, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, accelerator_view) is only permissible on array<T, 1>");
        _Initialize(_Av, _Cpu_access_type);
    }

    /// <summary>
    ///    Construct an array&lt;T,2&gt; from two integer extents, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    array(int _E0, int _E1, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, accelerator_view) is only permissible on array<T, 2>");
        _Initialize(_Av, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array&lt;T,3&gt; from three integer extents, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    array(int _E0, int _E1, int _E2, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, accelerator_view) is only permissible on array<T, 3>");
        _Initialize(_Av, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    array(const Concurrency::extent<_Rank>& _Extent, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _Initialize(_Av, _Associated_Av);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    array(int _E0, accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, accelerator_view, accelerator_view) is only permissible on array<T, 1>");
        _Initialize(_Av, _Associated_Av);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    array(int _E0, int _E1, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, accelerator_view, accelerator_view) is only permissible on array<T, 2>");
        _Initialize(_Av, _Associated_Av);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    array(int _E0, int _E1, int _E2, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, accelerator_view, accelerator_view) is only permissible on array<T, 3>");
        _Initialize(_Av, _Associated_Av);
    }

    /// <summary>
    ///   Construct an array initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    template <typename _InputIterator> array(const Concurrency::extent<_Rank>& _Extent, _InputIterator _Src_first, _InputIterator _Src_last) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    template <typename _InputIterator> array(const Concurrency::extent<_Rank>& _Extent, _InputIterator _Src_first) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    template <typename _InputIterator> array(int _E0, _InputIterator _Src_first, _InputIterator _Src_last) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, iterator, iterator) is only permissible on array<T, 1>");
        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    template <typename _InputIterator> array(int _E0, _InputIterator _Src_first) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, iterator) is only permissible on array<T, 1>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, _InputIterator _Src_first, _InputIterator _Src_last) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, iterator, iterator) is only permissible on array<T, 2>");
        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, _InputIterator _Src_first) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, iterator) is only permissible on array<T, 2>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, int _E2, _InputIterator _Src_first, _InputIterator _Src_last) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, iterator, iterator) is only permissible on array<T, 3>");
        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, int _E2, _InputIterator _Src_first) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, iterator) is only permissible on array<T, 3>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(details::_Select_default_accelerator().default_view, _Src_first, _Src_last, access_type_auto);
    }

    /// <summary>
    ///   Construct an array initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(const Concurrency::extent<_Rank>& _Extent, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(const Concurrency::extent<_Rank>& _Extent, _InputIterator _Src_first, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(int _E0, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, iterator, iterator) is only permissible on array<T, 1>");
        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(int _E0, _InputIterator _Src_first, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, iterator) is only permissible on array<T, 1>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, iterator, iterator) is only permissible on array<T, 2>");
        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, _InputIterator _Src_first, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, iterator) is only permissible on array<T, 2>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array initialized from a pair of iterators into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, int _E2, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, iterator, iterator) is only permissible on array<T, 3>");
        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct an array initialized from an iterator into a container, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, int _E2, _InputIterator _Src_first, Concurrency::accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, iterator) is only permissible on array<T, 3>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Src_first, _Src_last, _Cpu_access_type);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(const Concurrency::extent<_Rank>& _Extent, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from an iterator into a container.
    /// </summary>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(const Concurrency::extent<_Rank>& _Extent, _InputIterator _Src_first, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(_Extent)
    {
        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(int _E0, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, iterator, iterator, accelerator_view, accelerator_view) is only permissible on array<T, 1>");
        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from an iterator into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(int _E0, _InputIterator _Src_first, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av)
        : _M_extent(Concurrency::extent<_Rank>(_E0))
    {
        static_assert(_Rank == 1, "array(int, iterator, accelerator_view, accelerator_view) is only permissible on array<T, 1>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, iterator, iterator, accelerator_view, accelerator_view) is only permissible on array<T, 2>");
        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from an iterator into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, _InputIterator _Src_first, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1))
    {
        static_assert(_Rank == 2, "array(int, int, iterator, accelerator_view, accelerator_view) is only permissible on array<T, 2>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from a pair of iterators into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container.
    /// </param>
    /// <param name="_Src_last">
    ///   An ending iterator into the source container.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, int _E2, _InputIterator _Src_first, _InputIterator _Src_last, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, iterator, iterator, accelerator_view, accelerator_view) is only permissible on array<T, 3>");
        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_view, initialized from an iterator into a container.
    /// </summary>
    /// <param name="_E0">
    ///   An integer that is the length of the most-significant dimension of this array.
    /// </param>
    /// <param name="_E1">
    ///   An integer that is the length of the next-to-most-significant dimension of this array.
    /// </param>
    /// <param name="_E2">
    ///   An integer that is the length of the least-significant dimension of this array.
    /// </param>
    /// <param name="_Src_first">
    ///   A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
    ///   than this-&gt;extent.size(), undefined behavior results.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   An accelerator_view which specifies the preferred target location of the array.
    /// </param>
    template <typename _InputIterator> array(int _E0, int _E1, int _E2, _InputIterator _Src_first, Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
        : _M_extent(Concurrency::extent<_Rank>(_E0, _E1, _E2))
    {
        static_assert(_Rank == 3, "array(int, int, int, iterator, accelerator_view, accelerator_view) is only permissible on array<T, 3>");

        _InputIterator _Src_last = _Src_first;
        std::advance(_Src_last, this->extent.size());

        _Initialize(_Av, _Associated_Av, _Src_first, _Src_last);
    }

    /// <summary>
    ///   Construct an array initialized from an array_view.
    /// </summary>
    /// <param name="_Src">
    ///   An array_view to copy from.
    /// </param>
    explicit array(const array_view<const _Value_type,_Rank>& _Src) __CPU_ONLY
        :_M_extent(_Src.extent)
    {
        _Initialize(details::_Select_default_accelerator().default_view, access_type_auto);
        Concurrency::copy(_Src,*this);
    }

    /// <summary>
    ///   Construct an array initialized from an array_view, bound to a specific accelerator_view.
    /// </summary>
    /// <param name="_Src">
    ///   An array_view to copy from.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view where this array resides.
    /// </param>
    /// <param name="_Cpu_access_type">
    ///   The desired access_type for the array on the CPU. This
    ///   parameter has a default value of access_type_auto leaving the
    ///   CPU access_type determination to the runtime. The actual
    ///   CPU access_type for the array can be queried using the
    ///   get_cpu_access_type method.
    /// </param>
    array(const array_view<const _Value_type,_Rank>& _Src, accelerator_view _Av, access_type _Cpu_access_type = access_type_auto) __CPU_ONLY
        :_M_extent(_Src.extent)
    {
        _Initialize(_Av, _Cpu_access_type);
        Concurrency::copy(_Src,*this);
    }

    /// <summary>
    ///   Construct a staging array between two associated accelerator_views, initialized from an array_view.
    /// </summary>
    /// <param name="_Src">
    ///   An array_view to copy from.
    /// </param>
    /// <param name="_Av">
    ///   An accelerator_view which specifies the location of the array.
    /// </param>
    /// <param name="_Associated_Av">
    ///   The accelerator_view that is associated with _Av.
    /// </param>
    array(const array_view<const _Value_type,_Rank>& _Src, accelerator_view _Av, accelerator_view _Associated_Av) __CPU_ONLY
        :_M_extent(_Src.extent)
    {
        _Initialize(_Av, _Associated_Av);
        Concurrency::copy(_Src,*this);
    }

    /// <summary>
    ///   Copy constructor. Deep copy.
    /// </summary>
    array(const array& _Other) __CPU_ONLY
        : _M_extent(_Other._M_extent)
    {
        _Initialize(_Other.accelerator_view, _Other.associated_accelerator_view);
        Concurrency::copy(_Other, *this);
    }

    /// <summary>
    ///   Move constructor.
    /// </summary>
    array(array && _Other) __CPU_ONLY
        : _M_extent(_Other._M_extent), _M_multiplier(_Other._M_multiplier)
        , _M_buffer_descriptor(_Other._M_buffer_descriptor)
    {
        // Register this
        this->_Register_copy(_Other);

        // Release the _Other array
        _Other._Unregister();
        _Other._M_buffer_descriptor._M_data_ptr = NULL;
        _Other._M_buffer_descriptor._Set_buffer_ptr(NULL);
    }

    /// <summary>
    ///   Copy Assignment operator. Deep copy.
    /// </summary>
    array & operator= (const array & _Other) __CPU_ONLY
    {
        if (this != &_Other)
        {
            // First unregister myself from the current buffer
            _Unregister();

            _M_extent = _Other._M_extent;
            _Initialize(_Other.accelerator_view, _Other.associated_accelerator_view);
            Concurrency::copy(_Other, *this);
        }
        return *this;
    }

    /// <summary>
    ///   Move Assignment operator.
    /// </summary>
    array & operator= (array && _Other) __CPU_ONLY
    {
        if (this != &_Other)
        {
            // First unregister myself from the current buffer
            _Unregister();

            _M_extent = _Other._M_extent;
            _M_multiplier = _Other._M_multiplier;
            _M_buffer_descriptor = _Other._M_buffer_descriptor;
            this->_Register_copy(_Other);

            // Release the _Other array
            _Other._Unregister();
            _Other._M_buffer_descriptor._M_data_ptr = NULL;
            _Other._M_buffer_descriptor._Set_buffer_ptr(NULL);
        }
        return *this;
    }

    /// <summary>
    ///   Assignment operator from an array_view
    /// </summary>
    array& operator=(const array_view<const _Value_type,_Rank>& _Src) __CPU_ONLY
    {
        Concurrency::copy(_Src,*this);
        return *this;
    }

    /// <summary>
    ///   Copies elements from this array to the destination array.
    /// </summary>
    void copy_to(array<_Value_type,_Rank>& _Dest) const __CPU_ONLY
    {
        Concurrency::copy(*this, _Dest);
    }

    /// <summary>
    ///   Copies elements from this array to the destination array_view.
    /// </summary>
    void copy_to(const array_view<_Value_type,_Rank>& _Dest) const __CPU_ONLY
    {
        Concurrency::copy(*this,_Dest);
    }

    /// <summary>
    ///     Returns the extent that defines the shape of this array.
    /// </summary>
    __declspec(property(get=get_extent)) Concurrency::extent<_Rank> extent;
    Concurrency::extent<_Rank> get_extent() const __GPU
    {
        return _M_extent;
    }

    /// <summary>
    ///     Returns the accelerator_view where this array is located.
    /// </summary>
    __declspec(property(get=get_accelerator_view)) Concurrency::accelerator_view accelerator_view;
    Concurrency::accelerator_view get_accelerator_view() const __CPU_ONLY
    {
        return _Get_buffer()->_Get_master_buffer()->_Get_access_on_accelerator_view();
    }

    /// <summary>
    ///     Returns the accelerator_view that is the preferred target where this array can be copied.
    /// </summary>
    __declspec(property(get=get_associated_accelerator_view)) Concurrency::accelerator_view associated_accelerator_view;
    Concurrency::accelerator_view get_associated_accelerator_view() const __CPU_ONLY
    {
        return _Get_buffer()->_Get_master_buffer()->_Get_accelerator_view();
    }

    /// <summary>
    ///     Returns the CPU access_type allowed for this array.
    /// </summary>
    __declspec(property(get=get_cpu_access_type)) access_type cpu_access_type;
    access_type get_cpu_access_type() const __CPU_ONLY
    {
        return _Get_buffer()->_Get_master_buffer()->_Get_allowed_host_access_type();
    }

    /// <summary>
    ///     Get the element value indexed by _I
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I
    /// </returns>
    _Value_type& operator[] (const index<_Rank>& _Index) __GPU
    {
        // Refresh the data ptr if needed
        _Refresh_data_ptr(_Read_write_access);

        _Value_type * _Ptr = reinterpret_cast<_Value_type *>(_M_buffer_descriptor._M_data_ptr);
        return _Ptr[_Flatten_helper::func(_M_multiplier._M_base, _Index._M_base)];
    }

    /// <summary>
    ///     Get the element value indexed by _I
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I
    /// </returns>
    const _Value_type& operator[] (const index<_Rank>& _Index) const __GPU
    {
        // Refresh the data ptr if needed
#pragma warning( push )
#pragma warning( disable : 4880 )
        // Casting away constness in amp restricted scope might result in
        // undefined behavior, therefore, the compiler will report a level 1 warning
        // for it. But the following const_cast is harmless thus we are suppressing
        // this warning just for this line.
        const_cast<array*>(this)->_Refresh_data_ptr(_Read_access);
#pragma warning( pop )

        _Value_type * _Ptr = reinterpret_cast<_Value_type *>(_M_buffer_descriptor._M_data_ptr);
        return _Ptr[_Flatten_helper::func(_M_multiplier._M_base, _Index._M_base)];
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array.  If the array rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Result_type operator[](int _I)  __GPU
    {
        return details::_Array_projection_helper<_Value_type,_Rank>::_Project0(this,_I);
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array.  If the array rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Const_result_type operator[](int _I)  const __GPU
    {
        return details::_Const_array_projection_helper<_Value_type,_Rank>::_Project0(this,_I);
    }

    /// <summary>
    ///     Get the element value indexed by _I
    /// </summary>
    /// <param name="_I">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _I
    /// </returns>
    _Value_type& operator() (const index<_Rank>& _Index) __GPU
    {
        return this->operator[](_Index);
    }

    /// <summary>
    ///     Get the element value indexed by _Index
    /// </summary>
    /// <param name="_Index">
    ///     The index.
    /// </param>
    /// <returns>
    ///     The element value indexed by _Index
    /// </returns>
    const _Value_type& operator() (const index<_Rank>& _Index) const __GPU
    {
        return this->operator[](_Index);
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
    _Value_type& operator() (int _I0, int _I1) __GPU
    {
        static_assert(_Rank == 2, "value_type& array::operator()(int, int) is only permissible on array<T, 2>");
        return this->operator[](index<2>(_I0, _I1));
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
    const _Value_type& operator() (int _I0, int _I1) const __GPU
    {
        static_assert(_Rank == 2, "const value_type& array::operator()(int, int) is only permissible on array<T, 2>");
        return this->operator[](index<2>(_I0, _I1));
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
    _Value_type& operator() (int _I0, int _I1, int _I2) __GPU
    {
        static_assert(_Rank == 3, "value_type& array::operator()(int, int, int) is only permissible on array<T, 3>");
        return this->operator[](index<3>(_I0, _I1, _I2));
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
    const _Value_type& operator() (int _I0, int _I1, int _I2) const __GPU
    {
        static_assert(_Rank == 3, "const value_type& array::operator()(int, int, int) const is only permissible on array<T, 3>");
        return this->operator[](index<3>(_I0, _I1, _I2));
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array.  If the array rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Result_type operator()(int _I)  __GPU
    {
        return details::_Array_projection_helper<_Value_type,_Rank>::_Project0(this,_I);
    }

    /// <summary>
    ///     Projects the most-significant dimension of this array.  If the array rank is 1, this
    ///     produces a single element; otherwise it produces an array_view with one fewer dimensions.
    /// </summary>
    /// <param name="_I">
    ///     The most-significant index component
    /// </param>
    /// <returns>
    ///     The element at index component _I, or an array_view projected on the most-significant dimension.
    /// </returns>
    typename details::_Projection_result_type<_Value_type,_Rank>::_Const_result_type operator()(int _I)  const __GPU
    {
        return details::_Const_array_projection_helper<_Value_type,_Rank>::_Project0(this,_I);
    }

    /// <summary>
    ///     Produces a subsection of the source array at the given origin and extent.
    /// </summary>
    /// <param name="_Section_origin">
    ///     The origin of the section.
    /// </param>
    /// <param name="_Section_extent">
    ///     The extent of the section
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<_Value_type,_Rank> section(const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) __GPU
    {
        array_view<_Value_type,_Rank> _T1(*this);
        return _T1.section(_Section_origin, _Section_extent);
    }

    /// <summary>
    ///     Produces a subsection of the source array at the given origin and extent.
    /// </summary>
    /// <param name="_Section_origin">
    ///     The origin of the section.
    /// </param>
    /// <param name="_Section_extent">
    ///     The extent of the section
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<const _Value_type,_Rank> section(const Concurrency::index<_Rank>& _Section_origin, const Concurrency::extent<_Rank>& _Section_extent) const __GPU
    {
        array_view<const _Value_type,_Rank> _T1(*this);
        return _T1.section(_Section_origin, _Section_extent);
    }

    /// <summary>
    ///     Produces a subsection of the source array_view with origin of zero, with
    ///     an extent of _Ext.
    /// </summary>
    /// <param name="_Ext">
    ///     The extent of this section
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view<_Value_type,_Rank> section(const Concurrency::extent<_Rank>& _Ext) __GPU
    {
        return section(Concurrency::index<_Rank>(), _Ext);
    }

    /// <summary>
    ///     Produces a subsection of the source array_view with origin of zero, with
    ///     an extent of _Ext.
    /// </summary>
    /// <param name="_Ext">
    ///     The extent of this section
    /// </param>
    /// <returns>
    ///     A subsection of the array_view.
    /// </returns>
    array_view<const _Value_type,_Rank> section(const Concurrency::extent<_Rank>& _Ext) const __GPU
    {
        return section(Concurrency::index<_Rank>(), _Ext);
    }

    /// <summary>
    ///     Produces a subsection of the source array with origin specified by an index, with
    ///     an extent of (this-&gt;extent - _Idx).
    /// </summary>
    /// <param name="_Idx">
    ///     The index that specifies the origin of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<_Value_type,_Rank> section(const index<_Rank>& _Idx) __GPU
    {
        array_view<_Value_type,_Rank> _T1(*this);
        return _T1.section(_Idx);
    }

    /// <summary>
    ///     Produces a subsection of the source array with origin specified by an index, with
    ///     an extent of (this-&gt;extent - _Idx).
    /// </summary>
    /// <param name="_Idx">
    ///     The index that specifies the origin of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<const _Value_type,_Rank> section(const index<_Rank>& _Idx) const __GPU
    {
        array_view<const _Value_type,_Rank> _T1(*this);
        return _T1.section(_Idx);
    }

    /// <summary>
    ///     Produces a one-dimensional subsection of the source array with origin specified by the index
    ///     components _I0, with extent _E0.
    /// </summary>
    /// <param name="_I0">
    ///     The origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<_Value_type,1> section(int _I0, int _E0) __GPU
    {
        array_view<_Value_type,_Rank> _T1(*this);
        return _T1.section(_I0,_E0);
    }

    /// <summary>
    ///     Produces a one-dimensional subsection of the source array with origin specified by the index
    ///     components _I0, with extent _E0.
    /// </summary>
    /// <param name="_I0">
    ///     The origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<const _Value_type,1> section(int _I0, int _E0) const __GPU
    {
        array_view<const _Value_type,_Rank> _T1(*this);
        return _T1.section(_I0,_E0);
    }

    /// <summary>
    ///     Produces a two-dimensional subsection of the source array with origin specified by the index
    ///     components (_I0,_I1), with extent (_E0,_E1).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<_Value_type,2> section(int _I0, int _I1, int _E0, int _E1) __GPU
    {
        array_view<_Value_type,_Rank> _T1(*this);
        return _T1.section(_I0,_I1,_E0,_E1);
    }

    /// <summary>
    ///     Produces a two-dimensional subsection of the source array with origin specified by the index
    ///     components (_I0,_I1), with extent (_E0,_E1).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<const _Value_type,2> section(int _I0, int _I1, int _E0, int _E1) const __GPU
    {
        array_view<const _Value_type,_Rank> _T1(*this);
        return _T1.section(_I0,_I1,_E0,_E1);
    }

    /// <summary>
    ///     Produces a three-dimensional subsection of the source array with origin specified by the index
    ///     components (_I0,_I1,_I2), with extent (_E0,_E1,_E2).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The next-to-most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I2">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The next-to-most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E2">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<_Value_type,3> section(int _I0, int _I1, int _I2, int _E0, int _E1, int _E2) __GPU
    {
        array_view<_Value_type,_Rank> _T1(*this);
        return _T1.section(_I0,_I1,_I2,_E0,_E1,_E2);
    }

    /// <summary>
    ///     Produces a three-dimensional subsection of the source array with origin specified by the index
    ///     components (_I0,_I1,_I2), with extent (_E0,_E1,_E2).
    /// </summary>
    /// <param name="_I0">
    ///     The most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I1">
    ///     The next-to-most-significant component of the origin of this section.
    /// </param>
    /// <param name="_I2">
    ///     The least-significant component of the origin of this section.
    /// </param>
    /// <param name="_E0">
    ///     The most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E1">
    ///     The next-to-most-significant component of the extent of this section.
    /// </param>
    /// <param name="_E2">
    ///     The least-significant component of the extent of this section.
    /// </param>
    /// <returns>
    ///     A subsection of the array.
    /// </returns>
    array_view<const _Value_type,3> section(int _I0, int _I1, int _I2, int _E0, int _E1, int _E2) const __GPU
    {
        array_view<const _Value_type,_Rank> _T1(*this);
        return _T1.section(_I0,_I1,_I2,_E0,_E1,_E2);
    }

    /// <summary>
    ///     Produces a (possibly unsafe) reinterpretation of this array that is linear and with
    ///     a different element type.
    /// </summary>
    /// <returns>
    ///     A linear array_view with a reinterpreted element type.
    /// </returns>
    template <typename _Value_type2> array_view<_Value_type2,1> reinterpret_as() __GPU
    {
        return array_view<_Value_type,1>(_M_buffer_descriptor, Concurrency::extent<1>(extent.size())).template reinterpret_as<_Value_type2>();
    }

    /// <summary>
    ///     Produces a (possibly unsafe) reinterpretation of this array that is linear and with
    ///     a different element type.
    /// </summary>
    /// <returns>
    ///     A linear array_view with a reinterpreted element type.
    /// </returns>
    template <typename _Value_type2> array_view<const _Value_type2,1> reinterpret_as() const __GPU
    {
#pragma warning( push )
#pragma warning( disable : 4880 )
        // Casting away constness in amp restricted scope might result in
        // undefined behavior, therefore, the compiler will report a level 1 warning
        // for it. But the following const_cast is harmless thus we are suppressing
        // this warning just for this line.
        return const_cast<array*>(this)->reinterpret_as<_Value_type2>();
#pragma warning( pop )
    }

    /// <summary>
    ///     Produces an array_view of a different rank over this array's data.
    /// </summary>
    /// <param name="_View_extent">
    ///     The reshaping extent.
    /// </param>
    /// <returns>
    ///     A reshaped array_view.
    /// </returns>
    template <int _New_rank> array_view<_Value_type,_New_rank> view_as(const Concurrency::extent<_New_rank>& _View_extent) __GPU
    {
        return array_view<_Value_type,_New_rank>(_M_buffer_descriptor, _View_extent);
    }

    /// <summary>
    ///     Produces an array_view of a different rank over this array's data.
    /// </summary>
    /// <param name="_View_extent">
    ///     The reshaping extent.
    /// </param>
    /// <returns>
    ///     A reshaped array_view.
    /// </returns>
    template <int _New_rank> array_view<const _Value_type,_New_rank> view_as(const Concurrency::extent<_New_rank>& _View_extent) const __GPU
    {
#pragma warning( push )
#pragma warning( disable : 4880 )
        // Casting away constness in amp restricted scope might result in
        // undefined behavior, therefore, the compiler will report a level 1 warning
        // for it. But the following const_cast is harmless thus we are suppressing
        // this warning just for this line.
        return const_cast<array*>(this)->view_as<_New_rank>(_View_extent);
#pragma warning( pop )
    }

    /// <summary>
    ///     Implicitly converts this array into a vector by copying.
    /// </summary>
    operator std::vector<_Value_type>() const __CPU_ONLY
    {
        std::vector<_Value_type> _return_vector(extent.size());
        Concurrency::copy(*this, _return_vector.begin());

        return _return_vector;
    }

    /// <summary>
    ///     Returns a pointer to the raw data of this array.
    /// </summary>
    _Ret_ _Value_type* data() __GPU
    {
        _Refresh_data_ptr(_Read_write_access, false /* _Exception */);
        return reinterpret_cast<_Value_type*>(_M_buffer_descriptor._M_data_ptr);
    }

    /// <summary>
    ///     Returns a pointer to the raw data of this array.
    /// </summary>
    const _Value_type* data() const __GPU
    {
#pragma warning( push )
#pragma warning( disable : 4880 )
        // Casting away constness in amp restricted scope might result in
        // undefined behavior, therefore, the compiler will report a level 1 warning
        // for it. But the following const_cast is harmless thus we are suppressing
        // this warning just for this line.
        const_cast<array*>(this)->_Refresh_data_ptr(_Read_access, false /* _Exception */);
#pragma warning( pop )
        return reinterpret_cast<const _Value_type*>(_M_buffer_descriptor._M_data_ptr);
    }

    /// <summary>
    ///     Destroys this array and reclaims resources.
    /// </summary>
    ~array() __CPU_ONLY noexcept(false)
    {
        bool _Can_throw = (std::current_exception() == nullptr);

        // Destructor should not throw if we are already processing
        // an exception and another exception will result in termination
        try {
            _Unregister();
        }
        catch(...)
        {
            if (_Can_throw) {
                throw;
            }
        }
    }

private:

    // No default constructor
    array() __CPU_ONLY;

    // Private constructor used by direct3d::make_array
    array(const Concurrency::extent<_Rank>& _Extent, _Buffer_descriptor _Buffer_descriptor)
        : _M_extent(_Extent), _M_buffer_descriptor(_Buffer_descriptor)
    {
        _Initialize();

        // Register this
        this->_Register();
    }

    // Initialize
    unsigned int _Initialize() __CPU_ONLY
    {
        details::_Is_valid_extent(_M_extent);

        // Arrays always have a type access mode of '_Is_array_mode'
        // This is the mechanism for differentiating between arrays and array_views by the runtime
        _M_buffer_descriptor._M_type_access_mode = _Is_array_mode;
        unsigned int totalExtent = _M_extent[_Rank-1];
        details::_Array_init_helper<Concurrency::extent<_Rank>, Concurrency::extent<_Rank>>::func(totalExtent, _M_multiplier, _M_extent);

        return totalExtent;
    }

    // Initialize and allocate on specified accelerator_view
    void _Initialize(Concurrency::accelerator_view _Av, access_type _Cpu_access_type) __CPU_ONLY
    {
        unsigned int totalExtent = _Initialize();
        // release the existing buffer if any before allocation new one
        _M_buffer_descriptor._Set_buffer_ptr(NULL);

        _Buffer_ptr _PBuf = _Buffer::_Create_buffer(_Av, _Av, totalExtent, sizeof(_Value_type), false /* _Is_temp */, _Cpu_access_type);

        _M_buffer_descriptor._Set_buffer_ptr(_Ubiquitous_buffer::_Create_ubiquitous_buffer(_PBuf));
        _Register();
    }

    // Initialize and allocate on specified accelerator_view and copy specified data
    template <typename _InputIterator>
    void _Initialize(Concurrency::accelerator_view _Av, _InputIterator _Src_first, _InputIterator _Src_last, access_type _Cpu_access_type) __CPU_ONLY
    {
        _Initialize(_Av, _Cpu_access_type);
        copy(_Src_first, _Src_last, *this);
    }

    // Initialize and allocate on specified accelerator_views
    void _Initialize(Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av) __CPU_ONLY
    {
        unsigned int totalExtent = _Initialize();

        // Staging arrays can only be created if the accelerator_view is on the cpu_accelerator
        _Buffer_ptr _PBuf = NULL;

        // release the existing buffer if any before allocation new one
        _M_buffer_descriptor._Set_buffer_ptr(NULL);

        if (_Is_cpu_accelerator(_Av.accelerator))
        {
            // If the accelerator _Associated_Av supports zero-copy and the default cpu access type
            // for the accelerator is access_type_read_write, create a zero-copy buffer instead of a
            // staging buffer
            if (_Associated_Av.accelerator.supports_cpu_shared_memory && (_Get_recommended_buffer_host_access_mode(_Associated_Av) == _Read_write_access)) {
                _PBuf = _Buffer::_Create_buffer(_Associated_Av, _Av, totalExtent, sizeof(_Value_type), false /* _Is_temp */, access_type_read_write);
            }
            else {
                _PBuf = _Buffer::_Create_stage_buffer(_Associated_Av, _Av, totalExtent, sizeof(_Value_type));
            }

            _PBuf->_Map_buffer(_Read_write_access, true /* _Wait */);
        }
        else
        {
            _PBuf = _Buffer::_Create_buffer(_Av, _Av, totalExtent, sizeof(_Value_type), false /* _Is_temp */, access_type_auto);
        }

        _M_buffer_descriptor._Set_buffer_ptr(_Ubiquitous_buffer::_Create_ubiquitous_buffer(_PBuf));
        _Register();
    }

    // Initialize and allocate on specified accelerator_views
    template <typename _InputIterator>
    void _Initialize(Concurrency::accelerator_view _Av, Concurrency::accelerator_view _Associated_Av, _InputIterator _Src_first, _InputIterator _Src_last) __CPU_ONLY
    {
        _Initialize(_Av, _Associated_Av);
        copy(_Src_first, _Src_last, *this);
    }

    void _Register() __CPU_ONLY
    {
        Concurrency::accelerator_view cpuAv = _Is_cpu_accelerator(this->accelerator_view.accelerator) ?
                                               this->accelerator_view : accelerator(accelerator::cpu_accelerator).default_view;
        _M_buffer_descriptor._Get_buffer_ptr()->_Register_view(_M_buffer_descriptor._Get_view_key(), cpuAv, _Create_buffer_view_shape());

        _M_buffer_descriptor._Get_buffer_ptr()->_Discard(_M_buffer_descriptor._Get_view_key());

        // If the array is on the CPU accelerator then we will ensure that the descriptor
        // indicates CPU access
        if (_Is_cpu_accelerator(this->accelerator_view.accelerator))
        {
            _Buffer_ptr _PBuf = NULL;
            this->_Get_access_async(_Read_write_access, _PBuf, false)._Get();
        }
    }

    void _Register_copy(const array &_Other) __CPU_ONLY
    {
        _M_buffer_descriptor._Get_buffer_ptr()->_Register_view_copy(_M_buffer_descriptor._Get_view_key(), _Other._M_buffer_descriptor._Get_view_key());
    }

    void _Unregister() __CPU_ONLY
    {
        // No need to unregister if the array was moved causing the buffer ptr to be set to NULL
        if (_M_buffer_descriptor._Get_buffer_ptr() != NULL) {
            _M_buffer_descriptor._Get_buffer_ptr()->_Unregister_view(_M_buffer_descriptor._Get_view_key());
        }
    }

    _Ret_ _Ubiquitous_buffer* _Get_buffer() const __CPU_ONLY
    {
        return _M_buffer_descriptor._Get_buffer_ptr();
    }

    _Event _Get_access_async(_Access_mode _Mode, _Buffer_ptr &_Buf_ptr, bool _Zero_copy_cpu_access = false) const __CPU_ONLY
    {
        _ASSERTE(!_Zero_copy_cpu_access || (_Get_buffer()->_Get_master_buffer()->_Get_allowed_host_access_mode() != _No_access));

        _Buffer_ptr _PBuf;
        Concurrency::accelerator_view _Access_av = _Zero_copy_cpu_access ? accelerator(accelerator::cpu_accelerator).default_view : this->accelerator_view;
        _Event _Ev = details::_Get_access_async(_M_buffer_descriptor._Get_view_key(),
                                                _Access_av,
                                                _Mode, _PBuf);
        _Buf_ptr = _PBuf;

        if (_Is_cpu_accelerator(_Access_av.accelerator)) {
            _Ev = _Ev._Add_continuation(std::function<_Event()>([_PBuf, this]() mutable -> _Event {
                const_cast<array*>(this)->_M_buffer_descriptor._M_data_ptr = _PBuf->_Get_host_ptr();
                return _Event();
            }));
        }

        return _Ev;
    }

    _Ret_ _View_shape* _Create_buffer_view_shape() const
    {
        _ASSERTE(_Get_buffer()->_Get_master_buffer_elem_size() == sizeof(_Value_type));

        unsigned int _ZeroOffset[_Rank] = {0};
        unsigned int _View_extent[_Rank];
        for(int i=0; i<_Rank; ++i)
        {
            _View_extent[i] = static_cast<unsigned int>(this->_M_extent[i]);
        }
        return _View_shape::_Create_view_shape(static_cast<unsigned int>(_Rank), 0, &_View_extent[0], &_ZeroOffset[0], &_View_extent[0]);
    }

    bool _Has_cpu_access() const __CPU_ONLY
    {
        return (_Get_buffer()->_Get_master_buffer()->_Get_allowed_host_access_mode() != _No_access);
    }

    void _Refresh_data_ptr(_Access_mode _Requested_mode, bool _Exception = true) __CPU_ONLY
    {
        _ASSERTE(_Is_valid_access_mode(_Requested_mode));

        // For an array that has CPU access, the maximum CPU access allowed is that allowed by
        // the underlying _Buffer allocation
        _Requested_mode = static_cast<_Access_mode>(_Requested_mode & _Get_buffer()->_Get_master_buffer()->_Get_allowed_host_access_mode());

        // Refresh the data ptr if we do not have requested access
        if ((_Requested_mode == _No_access) || ((_M_buffer_descriptor._M_curr_cpu_access_mode & _Requested_mode) != _Requested_mode))
        {
            if (_Has_cpu_access() && (_Requested_mode != _No_access))
            {
                auto _Span_id = details::_Get_amp_trace()->_Start_array_view_synchronize_event_helper(_M_buffer_descriptor);
                _Buffer_ptr _PBuf;
                bool _Zero_copy_cpu_access = !_Is_cpu_accelerator(this->accelerator_view.accelerator);
                this->_Get_access_async(_Requested_mode, _PBuf, _Zero_copy_cpu_access)._Get();
                details::_Get_amp_trace()->_Write_end_event(_Span_id);
            }
            else
            {
                if (_Exception)
                {
                    if (!_Has_cpu_access()) {
                        throw runtime_exception("The array is not accessible on CPU.", E_FAIL);
                    }
                    else {
                        throw runtime_exception("The array is not accessible for reading on CPU.", E_FAIL);
                    }
                }
            }
        }
    }

    void _Refresh_data_ptr(_Access_mode /*_Requested_mode*/, bool /*_Exception*/ = true) __GPU_ONLY
    {
    }

private:
    // Data members

    Concurrency::extent<_Rank> _M_extent;

    // Descriptor of the buffer underlying the array
    _Buffer_descriptor _M_buffer_descriptor;

    // The vector used for index calculation.
    Concurrency::extent<_Rank> _M_multiplier;
};

namespace details
{
template <typename _Value_type, int _Rank>
_Event _Copy_async_impl(const array<_Value_type,_Rank>& _Src, array<_Value_type,_Rank>& _Dest)
{
    if (_Src.extent.size() > _Dest.extent.size())
    {
        throw runtime_exception("Invalid _Src argument. _Src size exceeds total size of the _Dest.", E_INVALIDARG);
    }

    // We can obliterate the existing content of dest if it is about to be totally overwritten
    _Access_mode _Dest_access_mode = (_Src.extent.size() == _Dest.extent.size()) ? _Write_access : _Read_write_access;

    _Buffer_ptr _PBufSrc, _PBufDest;
    _Event _Ev = _Get_access_async(_Src, _Read_access, _PBufSrc);
    _Ev = _Ev._Add_event(_Get_access_async(_Dest, _Dest_access_mode, _PBufDest));
    size_t _NumElemsToCopy = (_Src.extent.size() * sizeof(_Value_type)) / _PBufSrc->_Get_elem_size();
    return _Ev._Add_continuation(std::function<_Event()>([_PBufSrc, _PBufDest, _NumElemsToCopy]() mutable -> _Event {
        return details::_Copy_impl(_PBufSrc, 0, _PBufDest, 0, _NumElemsToCopy);
    }));
}

template <typename InputIterator, typename _Value_type, int _Rank>
_Event _Copy_async_impl(InputIterator _SrcFirst, InputIterator _SrcLast, array<_Value_type, _Rank> &_Dest)
{
    size_t _NumElemsToCopy = std::distance(_SrcFirst, _SrcLast);
    // We can obliterate the existing content of dest if it is about to be totally overwritten
    _Access_mode _Dest_access_mode = (_NumElemsToCopy == _Dest.extent.size()) ? _Write_access : _Read_write_access;
    _Buffer_ptr _PDestBuf;
    _Event _Ev = _Get_access_async(_Dest, _Dest_access_mode, _PDestBuf);

    return _Ev._Add_continuation(std::function<_Event()>([_SrcFirst, _SrcLast, _PDestBuf, _NumElemsToCopy]() mutable -> _Event {
        return details::_Copy_impl<InputIterator, _Value_type>(_SrcFirst, _SrcLast, _NumElemsToCopy, _PDestBuf, 0);
    }));
}

template <typename OutputIterator, typename _Value_type, int _Rank>
_Event _Copy_async_impl(const array<_Value_type, _Rank> &_Src, OutputIterator _DestIter)
{
    _Buffer_ptr _PSrcBuf;
    _Event _Ev = _Get_access_async(_Src, _Read_access, _PSrcBuf);
    size_t _NumElemsToCopy = (_Src.extent.size() * sizeof(_Value_type)) / _PSrcBuf->_Get_elem_size();
    return _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _NumElemsToCopy, _DestIter]() mutable -> _Event {
        return details::_Copy_impl<OutputIterator, _Value_type>(_PSrcBuf, 0, _NumElemsToCopy, _DestIter);
    }));
}

template <typename _Value_type, int _Rank>
_Event _Copy_async_impl(const array<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    const _Buffer_descriptor &_SrcBufDesc = _Get_buffer_descriptor(_Src);
    const _Buffer_descriptor &_DestBufDesc = _Get_buffer_descriptor(_Dest);
    if (_SrcBufDesc._Get_buffer_ptr() == _DestBufDesc._Get_buffer_ptr()) {
        throw runtime_exception("Cannot copy between overlapping regions of the same buffer.", E_INVALIDARG);
    }

    _Buffer_ptr _PSrcBuf, _PDestBuf;
    _Event _Ev = _Get_access_async(_Src, _Read_access, _PSrcBuf);

    // The source accelerator_view is driven by array's main location,
    // therefore we can pass nullptr to avoid unnecessary computation
    auto _AccelInfo = _Get_src_dest_accelerator_view(nullptr, &_DestBufDesc);

    _Ev = _Ev._Add_event(_Get_access_async(_DestBufDesc._Get_view_key(), _AccelInfo.second, _Write_access, _PDestBuf));
    _View_shape_ptr _PSrcShape = _Get_buffer_view_shape(_SrcBufDesc);
    _View_shape_ptr _PDestShape = _Get_buffer_view_shape(_DestBufDesc);
    return _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _PSrcShape, _PDestBuf, _PDestShape]() mutable -> _Event {
        return details::_Copy_impl(_PSrcBuf, _PSrcShape, _PDestBuf, _PDestShape);
    }));
}

template <typename _Value_type, int _Rank>
_Event _Copy_async_impl(const array_view<const _Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest)
{
    const _Buffer_descriptor &_SrcBufDesc = _Get_buffer_descriptor(_Src);
    const _Buffer_descriptor &_DestBufDesc = _Get_buffer_descriptor(_Dest);
    if (_SrcBufDesc._Get_buffer_ptr() == _DestBufDesc._Get_buffer_ptr()) {
        throw runtime_exception("Cannot copy between overlapping regions of the same buffer.", E_INVALIDARG);
    }

    auto _AccelInfo = _Get_src_dest_accelerator_view(&_SrcBufDesc, &_DestBufDesc);

    _Buffer_ptr _PSrcBuf, _PDestBuf;
    _Event _Ev = _Get_access_async(_SrcBufDesc._Get_view_key(), _AccelInfo.first, _Read_access, _PSrcBuf);
    _Ev = _Ev._Add_event(_Get_access_async(_Dest, _Write_access, _PDestBuf));
    _View_shape_ptr _PSrcShape = _Get_buffer_view_shape(_SrcBufDesc);
    _View_shape_ptr _PDestShape = _Get_buffer_view_shape(_DestBufDesc);
    return _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _PSrcShape, _PDestBuf, _PDestShape]() mutable -> _Event {
        return details::_Copy_impl(_PSrcBuf, _PSrcShape, _PDestBuf, _PDestShape);
    }));
}

template <typename _Value_type, int _Rank>
_Event _Copy_async_impl(const array_view<const _Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    const _Buffer_descriptor &_SrcBufDesc = _Get_buffer_descriptor(_Src);
    const _Buffer_descriptor &_DestBufDesc = _Get_buffer_descriptor(_Dest);
    _View_shape_ptr _PSrcShape = _Get_buffer_view_shape(_SrcBufDesc);
    _View_shape_ptr _PDestShape = _Get_buffer_view_shape(_DestBufDesc);
    if ((_SrcBufDesc._Get_buffer_ptr() == _DestBufDesc._Get_buffer_ptr()) && _PSrcShape->_Overlaps(_PDestShape)) {
        throw runtime_exception("Cannot copy between overlapping regions of the same buffer.", E_INVALIDARG);
    }

    auto _AccelInfo = _Get_src_dest_accelerator_view(&_SrcBufDesc, &_DestBufDesc);

    _Buffer_ptr _PSrcBuf, _PDestBuf;
    _Event _Ev = _Get_access_async(_SrcBufDesc._Get_view_key(), _AccelInfo.first, _Read_access, _PSrcBuf);
    _Ev = _Ev._Add_event(_Get_access_async(_DestBufDesc._Get_view_key(), _AccelInfo.second, _Write_access, _PDestBuf));
    return _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _PSrcShape, _PDestBuf, _PDestShape]() mutable -> _Event {
        return details::_Copy_impl(_PSrcBuf, _PSrcShape, _PDestBuf, _PDestShape);
    }));
}

template <typename InputIterator, typename _Value_type, int _Rank>
_Event _Copy_async_impl(InputIterator _SrcFirst, InputIterator _SrcLast, const array_view<_Value_type, _Rank> &_Dest)
{
    static_assert(!std::is_const<_Value_type>::value, "Cannot copy to array_view<const _Value_type, _Rank>.");

    size_t _Src_size = std::distance(_SrcFirst, _SrcLast);

    // Source cannot be greater than destination
    if (_Src_size > _Dest.extent.size())
    {
        throw runtime_exception("Number of elements in range between [_SrcFirst, _SrcLast) exceeds total size of the _Dest.", E_INVALIDARG);
    }

#pragma warning( push )
#pragma warning( disable : 4127 ) // Disable warning about constant conditional expression
    // Higher ranks need to have as many elements as in _Dest array_view
    if ((_Rank > 1) && (_Src_size != _Dest.extent.size()))
    {
        throw runtime_exception("For _Rank > 1 the number of elements in range between [_SrcFirst, _SrcLast) has to be equal to total size of the _Dest.", E_INVALIDARG);
    }
#pragma warning( pop )

    // We can obliterate the existing content of dest if it is about to be totally overwritten
    _Access_mode _Dest_access_mode = (_Src_size == _Dest.extent.size()) ? _Write_access : _Read_write_access;

    // Get read-write access for array_view on cpu_accelerator and take underlying pointer to data
    const _Buffer_descriptor &_DestBufDesc = _Get_buffer_descriptor(_Dest);

    auto _AccelInfo = _Get_src_dest_accelerator_view(nullptr, &_DestBufDesc);

    _Buffer_ptr _PDestBuf;
    _Event _Ev = _Get_access_async(_DestBufDesc._Get_view_key(), _AccelInfo.second, _Dest_access_mode, _PDestBuf);

    _View_shape_ptr _Dst_shape = _Get_buffer_view_shape(_DestBufDesc);

    // If the _Dst shape is linear then perform a linear copy
    unsigned int _Dst_linear_offset, _Dst_linear_size;
    if (_Dst_shape->_Is_view_linear(_Dst_linear_offset, _Dst_linear_size))
    {
        _Ev = _Ev._Add_continuation(std::function<_Event()>([_PDestBuf, _SrcFirst, _SrcLast, _Src_size, _Dst_linear_offset]() mutable -> _Event {
            return details::_Copy_impl<InputIterator, _Value_type>(_SrcFirst, _SrcLast, _Src_size, _PDestBuf, _Dst_linear_offset);
        }));
    }
    else
    {
        _View_shape_ptr _Reinterpreted_dst_shape = _Create_reinterpreted_shape(_Dst_shape, _PDestBuf->_Get_elem_size(), sizeof(_Value_type));

        // Source has as many elements as in destination, reshape source to match destination shape
        std::vector<unsigned int> _Src_offset(_Reinterpreted_dst_shape->_Get_rank(), 0);
        _View_shape_ptr _Src_shape = details::_View_shape::_Create_view_shape(_Reinterpreted_dst_shape->_Get_rank(), 0 /* linear offset*/,
                                                                              _Reinterpreted_dst_shape->_Get_view_extent(), _Src_offset.data(),
                                                                              _Reinterpreted_dst_shape->_Get_view_extent());

        _Ev = _Ev._Add_continuation(std::function<_Event()>([_PDestBuf, _SrcFirst, _Src_shape, _Dst_shape]() mutable -> _Event {
            return details::_Copy_impl<InputIterator, _Value_type>(_SrcFirst, _Src_shape, _PDestBuf, _Dst_shape);
        }));
    }

    return _Ev;
}

template <typename OutputIterator, typename _Value_type, int _Rank>
_Event _Copy_async_impl(const array_view<_Value_type, _Rank> &_Src, OutputIterator _DestIter)
{
    // Caller is responsible for passing valid _DestIter

    // Get read access for array_view on cpu_accelerator and take underlying pointer to data
    const _Buffer_descriptor &_SrcBufDesc = _Get_buffer_descriptor(_Src);

    auto _AccelInfo = _Get_src_dest_accelerator_view(&_SrcBufDesc, nullptr);

    _Buffer_ptr _PSrcBuf;
    _Event _Ev = _Get_access_async(_SrcBufDesc._Get_view_key(), _AccelInfo.first, _Read_access, _PSrcBuf);

    // Get source shape
    _View_shape_ptr _Src_shape = _Get_buffer_view_shape(_SrcBufDesc);

    // If the _Src_shape is linear then perform a linear copy
    unsigned int _Src_linear_offset, _Src_linear_size;
    if (_Src_shape->_Is_view_linear(_Src_linear_offset, _Src_linear_size))
    {
        _Ev = _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _Src_linear_offset, _Src_linear_size, _DestIter]() mutable -> _Event {
            return details::_Copy_impl<OutputIterator, _Value_type>(_PSrcBuf, _Src_linear_offset, _Src_linear_size, _DestIter);
        }));
    }
    else
    {
        _View_shape_ptr _Reinterpreted_src_shape = _Create_reinterpreted_shape(_Src_shape, _PSrcBuf->_Get_elem_size(), sizeof(_Value_type));

        // Valid destination should have space for as many elements as in source array_view, reshape to match source view shape
        std::vector<unsigned int> _Dst_offset(_Reinterpreted_src_shape->_Get_rank(), 0);
        _View_shape_ptr _Dst_shape = details::_View_shape::_Create_view_shape(_Reinterpreted_src_shape->_Get_rank(), 0 /* linear offset*/,
                                                                              _Reinterpreted_src_shape->_Get_view_extent(), _Dst_offset.data(),
                                                                              _Reinterpreted_src_shape->_Get_view_extent());

        _Ev = _Ev._Add_continuation(std::function<_Event()>([_PSrcBuf, _Src_shape, _DestIter, _Dst_shape]() mutable -> _Event {
            return details::_Copy_impl<OutputIterator, _Value_type>(_PSrcBuf, _Src_shape, _DestIter, _Dst_shape);
        }));
    }

    return _Ev;
}

}

/// <summary>
///     Asynchronously copies the contents of the source array into the destination array.
/// </summary>
/// <param name="_Src">
///     The source array.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array<_Value_type,_Rank>& _Src, array<_Value_type,_Rank>& _Dest)
{
    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                                   details::_Get_buffer_descriptor(_Dest),
                                                                                   sizeof(_Value_type) * _Src.extent.size());

    auto _Ev = _Copy_async_impl(_Src, _Dest);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies the contents of the source array into the destination array.
/// </summary>
/// <param name="_Src">
///     The source array.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
template <typename _Value_type, int _Rank> void copy(const array<_Value_type,_Rank>& _Src, array<_Value_type,_Rank>& _Dest)
{
    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                        details::_Get_buffer_descriptor(_Dest),
                                                                        sizeof(_Value_type) * _Src.extent.size());

    _Copy_async_impl(_Src, _Dest)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Asynchronously copies the elements in the range [_SrcFirst, _SrcLast) into the destination array.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container.
/// </param>
/// <param name="_SrcLast">
///     An ending iterator into the source container.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, InputIterator _SrcLast, array<_Value_type, _Rank> &_Dest)
{
    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(nullptr,
                                                                                   details::_Get_buffer_descriptor(_Dest),
                                                                                   sizeof(_Value_type) * std::distance(_SrcFirst, _SrcLast));

    _Event _Ev = _Copy_async_impl(_SrcFirst, _SrcLast, _Dest);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies the elements in the range [_SrcFirst, _SrcLast) into the destination array.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container.
/// </param>
/// <param name="_SrcLast">
///     An ending iterator into the source container.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, InputIterator _SrcLast, array<_Value_type, _Rank> &_Dest)
{
    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(nullptr,
                                                                        details::_Get_buffer_descriptor(_Dest),
                                                                        sizeof(_Value_type) * std::distance(_SrcFirst, _SrcLast));

    _Copy_async_impl(_SrcFirst, _SrcLast, _Dest)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Asynchronously copies the elements beginning at _SrcFirst into the destination array.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
///     than _Dest.extent.size(), undefined behavior results.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, array<_Value_type, _Rank> &_Dest)
{
    InputIterator _SrcLast = _SrcFirst;
    std::advance(_SrcLast, _Dest.extent.size());
    return copy_async(_SrcFirst, _SrcLast, _Dest);
}

/// <summary>
///     Copies the elements beginning at _SrcFirst into the destination array.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
///     than _Dest.extent.size(), undefined behavior results.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, array<_Value_type, _Rank> &_Dest)
{
    InputIterator _SrcLast = _SrcFirst;
    std::advance(_SrcLast, _Dest.extent.size());
    copy(_SrcFirst, _SrcLast, _Dest);
}

/// <summary>
///     Asynchronously copies the contents of the array into the destination beginning at _DestIter.
/// </summary>
/// <param name="_Src">
///     The source array.
/// </param>
/// <param name="_DestIter">
///     An output iterator to the beginning position at destination.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename OutputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array<_Value_type, _Rank> &_Src, OutputIterator _DestIter)
{
    _CPP_AMP_VERIFY_MUTABLE_ITERATOR(OutputIterator);

    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                                   nullptr,
                                                                                   sizeof(_Value_type) * _Src.extent.size());
    _Event _Ev = _Copy_async_impl(_Src, _DestIter);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies the contents of the array into the destination beginning at _DestIter.
/// </summary>
/// <param name="_Src">
///     The source array.
/// </param>
/// <param name="_DestIter">
///     An output iterator to the beginning position at destination.
/// </param>
template <typename OutputIterator, typename _Value_type, int _Rank> void copy(const array<_Value_type, _Rank> &_Src, OutputIterator _DestIter)
{
    _CPP_AMP_VERIFY_MUTABLE_ITERATOR(OutputIterator);

    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                        nullptr,
                                                                        sizeof(_Value_type) * _Src.extent.size());

    _Copy_async_impl(_Src, _DestIter)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Asynchronously copies the contents of the source array into the destination array_view.
/// </summary>
/// <param name="_Src">
///     The source array.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                                   details::_Get_buffer_descriptor(_Dest),
                                                                                   sizeof(_Value_type) * _Src.extent.size());

    _Event _Ev = _Copy_async_impl(_Src, _Dest);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies the contents of the source array into the destination array_view.
/// </summary>
/// <param name="_Src">
///     The source array.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
template <typename _Value_type, int _Rank> void copy(const array<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                        details::_Get_buffer_descriptor(_Dest),
                                                                        sizeof(_Value_type) * _Src.extent.size());

    _Copy_async_impl(_Src, _Dest)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Asynchronously copies the contents of the source array_view into the destination array.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<const _Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest)
{
    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                                   details::_Get_buffer_descriptor(_Dest),
                                                                                   sizeof(_Value_type) * _Src.extent.size());

    _Event _Ev = _Copy_async_impl(_Src, _Dest);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies the contents of the source array_view into the destination array.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
template <typename _Value_type, int _Rank> void copy(const array_view<const _Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest)
{
    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                        details::_Get_buffer_descriptor(_Dest),
                                                                        sizeof(_Value_type) * _Src.extent.size());

    _Copy_async_impl(_Src, _Dest)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Asynchronously copies the contents of the source array_view into the destination array.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<_Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest)
{
    return copy_async<_Value_type, _Rank>(array_view<const _Value_type, _Rank>(_Src), _Dest);
}

/// <summary>
///     Copies the contents of the source array_view into the destination array.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array.
/// </param>
template <typename _Value_type, int _Rank> void copy(const array_view<_Value_type, _Rank>& _Src, array<_Value_type, _Rank>& _Dest)
{
    copy<_Value_type, _Rank>(array_view<const _Value_type, _Rank>(_Src), _Dest);
}

/// <summary>
///     Asynchronously copies the contents of the source array_view into the destination array_view.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<const _Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                                   details::_Get_buffer_descriptor(_Dest),
                                                                                   sizeof(_Value_type) * _Src.extent.size());

    _Event _Ev = _Copy_async_impl(_Src, _Dest);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies the contents of the source array_view into the destination array_view.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
template <typename _Value_type, int _Rank> void copy(const array_view<const _Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                        details::_Get_buffer_descriptor(_Dest),
                                                                        sizeof(_Value_type) * _Src.extent.size());

    _Copy_async_impl(_Src, _Dest)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Asynchronously copies the contents of the source array_view into the destination array_view.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    return copy_async<_Value_type, _Rank>(array_view<const _Value_type, _Rank>(_Src), _Dest);
}

/// <summary>
///     Copies the contents of the source array_view into the destination array_view.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
template <typename _Value_type, int _Rank> void copy(const array_view<_Value_type, _Rank>& _Src, const array_view<_Value_type, _Rank>& _Dest)
{
    copy<_Value_type, _Rank>(array_view<const _Value_type, _Rank>(_Src), _Dest);
}

/// <summary>
///     Asynchronously copies the elements in the range [_SrcFirst, _SrcLast) into the destination array_view.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container.
/// </param>
/// <param name="_SrcLast">
///     An ending iterator into the source container.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, InputIterator _SrcLast, const array_view<_Value_type, _Rank> &_Dest)
{
    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(nullptr,
                                                                                   details::_Get_buffer_descriptor(_Dest),
                                                                                   sizeof(_Value_type) * std::distance(_SrcFirst, _SrcLast));

    _Event _Ev = _Copy_async_impl(_SrcFirst, _SrcLast, _Dest);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Asynchronously copies the elements beginning at _SrcFirst into the destination array_view.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
///     than _Dest.extent.size(), undefined behavior results.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename InputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(InputIterator _SrcFirst, const array_view<_Value_type, _Rank> &_Dest)
{
    InputIterator _SrcLast = _SrcFirst;
    std::advance(_SrcLast, _Dest.extent.size());
    return copy_async(_SrcFirst, _SrcLast, _Dest);
}

/// <summary>
///     Copies the elements in the range [_SrcFirst, _SrcLast) into the destination array_view.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container.
/// </param>
/// <param name="_SrcLast">
///     An ending iterator into the source container.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, InputIterator _SrcLast, const array_view<_Value_type, _Rank> &_Dest)
{
    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(nullptr,
                                                                        details::_Get_buffer_descriptor(_Dest),
                                                                        sizeof(_Value_type) * std::distance(_SrcFirst, _SrcLast));

    _Copy_async_impl(_SrcFirst, _SrcLast, _Dest)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

/// <summary>
///     Copies the contents of an STL container into the destination array_view.
/// </summary>
/// <param name="_SrcFirst">
///     A beginning iterator into the source container; if the number of available container elements starting at this iterator position is less
///     than _Dest.extent.size(), undefined behavior results.
/// </param>
/// <param name="_Dest">
///     The destination array_view.
/// </param>
template <typename InputIterator, typename _Value_type, int _Rank> void copy(InputIterator _SrcFirst, const array_view<_Value_type, _Rank> &_Dest)
{
    InputIterator _SrcLast = _SrcFirst;
    std::advance(_SrcLast, _Dest.extent.size());
    copy(_SrcFirst, _SrcLast, _Dest);
}

/// <summary>
///     Asynchronously copies the contents of the array_view into the destination beginning at _DestIter.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_DestIter">
///     An output iterator to the beginning position at destination.
/// </param>
/// <returns>
///     A future upon which to wait for the operation to complete.
/// </returns>
template <typename OutputIterator, typename _Value_type, int _Rank> concurrency::completion_future copy_async(const array_view<_Value_type, _Rank> &_Src, OutputIterator _DestIter)
{
    _CPP_AMP_VERIFY_MUTABLE_ITERATOR(OutputIterator);

    // Caller is responsible for passing valid _DestIter
    auto _Async_op_id = details::_Get_amp_trace()->_Launch_async_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                                   nullptr,
                                                                                   sizeof(_Value_type) * _Src.extent.size());

    _Event _Ev = _Copy_async_impl(_Src, _DestIter);

    return details::_Get_amp_trace()->_Start_async_op_wait_event_helper(_Async_op_id, _Ev);
}

/// <summary>
///     Copies the contents of the array_view into the destination beginning at _DestIter.
/// </summary>
/// <param name="_Src">
///     The source array_view.
/// </param>
/// <param name="_DestIter">
///     An output iterator to the beginning position at destination.
/// </param>
template <typename OutputIterator, typename _Value_type, int _Rank> void copy(const array_view<_Value_type, _Rank> &_Src, OutputIterator _DestIter)
{
    _CPP_AMP_VERIFY_MUTABLE_ITERATOR(OutputIterator);

    auto _Span_id = details::_Get_amp_trace()->_Start_copy_event_helper(details::_Get_buffer_descriptor(_Src),
                                                                        nullptr,
                                                                        sizeof(_Value_type) * _Src.extent.size());

    _Copy_async_impl(_Src, _DestIter)._Get();

    details::_Get_amp_trace()->_Write_end_event(_Span_id);
}

// Namespace for Direct3D specific functionality
namespace direct3d
{
    /// <summary>
    ///     Get the D3D buffer interface underlying an array.
    /// </summary>
    /// <param name="_Rank">
    ///     The rank of the array to get underlying D3D buffer of.
    /// </param>
    /// <param name="_Value_type">
    ///     The type of the elements in the array to get underlying D3D buffer of.
    /// </param>
    /// <param name="_Array">
    ///     A array on a D3D accelerator_view for which the underlying D3D buffer interface is returned.
    /// </param>
    /// <returns>
    ///     The IUnknown interface pointer corresponding to the D3D buffer underlying the array.
    /// </returns>
    template<typename _Value_type, int _Rank> _Ret_ IUnknown *get_buffer(const array<_Value_type, _Rank> &_Array) __CPU_ONLY
    {
        _Buffer_ptr _PBuf;
        _Get_access_async(_Array, _Read_write_access, _PBuf)._Get();
        return details::_D3D_interop::_Get_D3D_buffer(_PBuf);
    }

    /// <summary>
    ///     Create an array from a D3D buffer interface pointer.
    /// </summary>
    /// <param name="_Rank">
    ///     The rank of the array to be created from the D3D buffer.
    /// </param>
    /// <param name="_Value_type">
    ///     The type of the elements of the array to be created from the D3D buffer.
    /// </param>
    /// <param name="_Extent">
    ///   An extent that describes the shape of the array aggregate.
    /// </param>
    /// <param name="_Av">
    ///   A D3D accelerator_view on which the array is to be created.
    /// </param>
    /// <param name="_D3D_buffer">
    ///     IUnknown interface pointer of the D3D buffer to create the array from.
    /// </param>
    /// <returns>
    ///     A array created using the provided D3D buffer.
    /// </returns>
    template<typename _Value_type, int _Rank> array<_Value_type, _Rank> make_array(const Concurrency::extent<_Rank> &_Extent, const Concurrency::accelerator_view &_Av, _In_ IUnknown *_D3D_buffer) __CPU_ONLY
    {
        details::_Is_valid_extent(_Extent);

        if (_D3D_buffer == NULL)
        {
            throw runtime_exception("NULL D3D buffer pointer.", E_INVALIDARG);
        }

        if (!details::_Is_D3D_accelerator_view(_Av))
        {
            throw runtime_exception("Cannot create D3D buffer on a non-D3D accelerator_view.", E_INVALIDARG);
        }

        _Ubiquitous_buffer_ptr _PBuf = _Ubiquitous_buffer::_Create_ubiquitous_buffer(_Buffer::_Create_buffer(_D3D_buffer, _Av, _Extent.size(), sizeof(_Value_type)));
        return array<_Value_type, _Rank>(_Extent, _Buffer_descriptor(_PBuf->_Get_master_buffer()->_Get_host_ptr(), _PBuf, _Is_array_mode, _Read_write_access));
    }

} // namespace Concurrency::direct3d

//=============================================================================
// Atomic Operation Library
//=============================================================================

#define AS_UINT_PTR(p) reinterpret_cast<unsigned int *>(p)
#define AS_UINT(v)     *(reinterpret_cast<unsigned int *>(&(v)))
#define AS_INT(v)      *(reinterpret_cast<int *>(&(v)))
#define AS_FLOAT(v)    *(reinterpret_cast<float *>(&(v)))

/// <summary>
///     Performs an atomic addition of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be added to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_add(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    unsigned int _Ret;
    _Ret = __dp_d3d_interlocked_add(AS_UINT_PTR(_Dest), AS_UINT(_Value));
    return AS_INT(_Ret);
}

/// <summary>
///     Performs an atomic addition of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be added to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_add(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_add(_Dest, _Value);
}

/// <summary>
///     Performs an atomic subtraction of _Value from the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be subtracted from the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_sub(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    unsigned int _Ret;
    int _Neg = -_Value;
    _Ret = __dp_d3d_interlocked_add(AS_UINT_PTR(_Dest), AS_UINT(_Neg));
    return AS_INT(_Ret);
}

/// <summary>
///     Performs an atomic subtraction of _Value from the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be subtracted from the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>

inline unsigned int atomic_fetch_sub(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
#pragma warning( push )
#pragma warning( disable : 4146 )
    // Warning 4146: unary minus operator applied to unsigned type, result
    // still unsigned.
    //
    // This is what we want here. The resulted unsigned value have the
    // right binary representation for achieving subtraction
    return __dp_d3d_interlocked_add(_Dest, (-_Value));
#pragma warning( pop )
}


/// <summary>
///     Performs an atomic increment to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_inc(_Inout_ int * _Dest) __GPU_ONLY
{
    unsigned int _Ret;
    _Ret = __dp_d3d_interlocked_add(AS_UINT_PTR(_Dest), 1U);
    return AS_INT(_Ret);
}

/// <summary>
///     Performs an atomic increment to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_inc(_Inout_ unsigned int * _Dest) __GPU_ONLY
{
    return __dp_d3d_interlocked_add(_Dest, 1U);
}

/// <summary>
///     Performs an atomic decrement to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_dec(_Inout_ int * _Dest) __GPU_ONLY
{
#pragma warning( push )
#pragma warning( disable : 4146 )
    // Warning 4146: unary minus operator applied to unsigned type, result
    // still unsigned.
    unsigned int _Ret;
    _Ret = __dp_d3d_interlocked_add(AS_UINT_PTR(_Dest), (-(1U)));
    return AS_INT(_Ret);
#pragma warning( pop )
}

/// <summary>
///     Performs an atomic decrement to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_dec(_Inout_ unsigned int * _Dest) __GPU_ONLY
{
#pragma warning( push )
#pragma warning( disable : 4146 )
    // Warning 4146: unary minus operator applied to unsigned type, result
    // still unsigned.
    return __dp_d3d_interlocked_add(_Dest, (-(1U)));
#pragma warning( pop )
}

/// <summary>
///     Sets the value of location pointed to by _Dest to _Value as an atomic operation
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be set to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_exchange(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    unsigned int _Ret = __dp_d3d_interlocked_exchange(AS_UINT_PTR(_Dest), AS_UINT(_Value));
    return AS_INT(_Ret);
}

/// <summary>
///     Sets the value of location pointed to by _Dest to _Value as an atomic operation
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be set to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_exchange(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_exchange(_Dest, _Value);
}

/// <summary>
///     Sets the value of location pointed to by _Dest to _Value as an atomic operation
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be set to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline float atomic_exchange(_Inout_ float * _Dest, float _Value) __GPU_ONLY
{
    unsigned int _Ret = __dp_d3d_interlocked_exchange(AS_UINT_PTR(_Dest), AS_UINT(_Value));
    return AS_FLOAT(_Ret);
}

/// <summary>
///     Atomically, compares the value pointed to by _Dest for equality with that pointed to by _Expected_value,
///     and if true, returns true and replaces the value with _Value, and if false, returns false and updates the value
///     pointed to by _Expected_value with the value pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Expected_value">
///     Pointer to the value being compared to the value pointed to by _Dest. If the comparison is unsuccessful,
///     the value is updated with the value pointed to by _Dest
/// </param>
/// <param name="_Value">
///     The value to be stored to the location pointed to by _Dest if the comparison is successful
/// </param>
/// <returns>
///     If the operation is successful, return true. Otherwise, false
/// </returns>
inline bool atomic_compare_exchange(_Inout_ int * _Dest, _Inout_ int * _Expected_value, int _Value) __GPU_ONLY
{
    int _Old = *_Expected_value;
    unsigned int _Ret = __dp_d3d_interlocked_compare_exchange(AS_UINT_PTR(_Dest), AS_UINT(_Value), AS_UINT(_Old));
    if (_Ret == AS_UINT(_Old))
    {
        return true;
    }
    else
    {
        *_Expected_value = AS_INT(_Ret);
        return false;
    }
}

/// <summary>
///     Atomically, compares the value pointed to by _Dest for equality with that pointed to by _Expected_value,
///     and if true, returns true and replaces the value with _Value, and if false, returns false and updates the value
///     pointed to by _Expected_value with the value pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Expected_value">
///     Pointer to the value being compared to the value pointed to by _Dest. If the comparison is unsuccessful,
///     the value is updated with the value pointed to by _Dest
/// </param>
/// <param name="_Value">
///     The value to be stored to the location pointed to by _Dest if the comparison is successful
/// </param>
/// <returns>
///     If the operation is successful, return true. Otherwise, false
/// </returns>
inline bool atomic_compare_exchange(_Inout_ unsigned int * _Dest, _Inout_ unsigned int * _Expected_value, unsigned int _Value) __GPU_ONLY
{
    unsigned int _Old = *_Expected_value;
    unsigned int _Ret = __dp_d3d_interlocked_compare_exchange(_Dest, _Value, _Old);
    if (_Ret == _Old)
    {
        return true;
    }
    else
    {
        *_Expected_value = _Ret;
        return false;
    }
}

/// <summary>
///     Atomically computes the maximum of _Value and the value of the memory location pointed to
///     by _Dest, and stores the maximum value to the memory location
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be compared to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_max(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_max_int(_Dest, _Value);
}

/// <summary>
///     Atomically computes the maximum of _Value and the value of the memory location pointed to
///     by _Dest, and stores the maximum value to the memory location
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be compared to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_max(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_max_uint(_Dest, _Value);
}


/// <summary>
///     Atomically computes the minimum of _Value and the value of the memory location pointed to
///     by _Dest, and stores the minimum value to the memory location
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be compared to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_min(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_min_int(_Dest, _Value);
}

/// <summary>
///     Atomically computes the minimum of _Value and the value of the memory location pointed to
///     by _Dest, and stores the minimum value to the memory location
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to be compared to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_min(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_min_uint(_Dest, _Value);
}

/// <summary>
///     Performs an atomic bitwise and operation of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to bitwise and to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_and(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    unsigned int _Ret;
    _Ret = __dp_d3d_interlocked_and(AS_UINT_PTR(_Dest), AS_UINT(_Value));
    return AS_INT(_Ret);
}

/// <summary>
///     Performs an atomic bitwise and operation of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to bitwise and to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_and(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_and(_Dest, _Value);
}


/// <summary>
///     Performs an atomic bitwise or operation of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to bitwise or to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_or(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    unsigned int _Ret;
    _Ret = __dp_d3d_interlocked_or(AS_UINT_PTR(_Dest), AS_UINT(_Value));
    return AS_INT(_Ret);
}

/// <summary>
///     Performs an atomic bitwise or operation of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to bitwise or to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_or(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_or(_Dest, _Value);
}

/// <summary>
///     Performs an atomic bitwise xor operation of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to bitwise xor to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline int atomic_fetch_xor(_Inout_ int * _Dest, int _Value) __GPU_ONLY
{
    unsigned int _Ret;
    _Ret = __dp_d3d_interlocked_xor(AS_UINT_PTR(_Dest), AS_UINT(_Value));
    return AS_INT(_Ret);
}

/// <summary>
///     Performs an atomic bitwise xor operation of _Value to the memory location pointed to by _Dest
/// </summary>
/// <param name="_Dest">
///     Pointer to the destination location
/// </param>
/// <param name="_Value">
///     The value to bitwise xor to the location pointed to by _Dest
/// </param>
/// <returns>
///     The original value of the location pointed to by _Dest
/// </returns>
inline unsigned int atomic_fetch_xor(_Inout_ unsigned int * _Dest, unsigned int _Value) __GPU_ONLY
{
    return __dp_d3d_interlocked_xor(_Dest, _Value);
}

//=============================================================================
// parallel_for_each
//=============================================================================

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain on an accelerator_view.
///     The accelerator_view is determined from the arrays and/or array_views captured by the kernel function,
///     or if no accelerator_view can be derived, the default is chosen.
/// </summary>
/// <param name="_Compute_domain">
///     An extent which represents the set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "index&lt;_Rank&gt;" which performs the parallel computation.
/// </param>
template <int _Rank, typename _Kernel_type> void parallel_for_each(const extent<_Rank>& _Compute_domain, const _Kernel_type &_Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {accelerator::get_auto_selection_view()};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain that has been tiled into 3-dimensional
///     regions.  The accelerator is determined from the arrays and/or array_views captured by the kernel function,
///     or if no accelerator can be derived, the default is chosen.
/// </summary>
/// <param name="_Compute_domain">
///     A tiled_extent&lt;_Dim0,_Dim1,_Dim2&gt; which represents the tiled set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "tiled_index&lt;_Dim0,_Dim1,_Dim2&gt;" which performs the parallel computation.
/// </param>
template <int _Dim0, int _Dim1, int _Dim2, typename _Kernel_type> void parallel_for_each(const tiled_extent<_Dim0, _Dim1, _Dim2>& _Compute_domain, const _Kernel_type& _Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {accelerator::get_auto_selection_view()};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain that has been tiled into 2-dimensional
///     regions.  The accelerator is determined from the arrays and/or array_views captured by the kernel function,
///     or if no accelerator can be derived, the default is chosen.
/// </summary>
/// <param name="_Compute_domain">
///     A tiled_extent&lt;_Dim0,_Dim1&gt; which represents the tiled set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "tiled_index&lt;_Dim0,_Dim1&gt;" which performs the parallel computation.
/// </param>
template <int _Dim0, int _Dim1, typename _Kernel_type> void parallel_for_each(const tiled_extent<_Dim0, _Dim1>& _Compute_domain, const _Kernel_type& _Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {accelerator::get_auto_selection_view()};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain that has been tiled into 1-dimensional
///     regions.  The accelerator is determined from the arrays and/or array_views captured by the kernel function,
///     or if no accelerator can be derived, the default is chosen.
/// </summary>
/// <param name="_Compute_domain">
///     A tiled_extent&lt;_Dim0&gt; which represents the tiled set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "tiled_index&lt;_Dim0&gt;" which performs the parallel computation.
/// </param>
template <int _Dim0, typename _Kernel_type> void parallel_for_each(const tiled_extent<_Dim0>& _Compute_domain, const _Kernel_type& _Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {accelerator::get_auto_selection_view()};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain on an accelerator.
/// </summary>
/// <param name="_Accl_view">
///     The accelerator_view upon which to run this parallel computation.
/// </param>
/// <param name="_Compute_domain">
///     An extent which represents the set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "index&lt;_Rank&gt;" which performs the parallel computation.
/// </param>
template <int _Rank, typename _Kernel_type> void parallel_for_each(const accelerator_view& _Accl_view, const extent<_Rank>& _Compute_domain, const _Kernel_type& _Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {_Accl_view};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain that has been tiled into 3-dimensional
///     regions.
/// </summary>
/// <param name="_Accl_view">
///     The accelerator_view upon which to run this parallel computation.
/// </param>
/// <param name="_Compute_domain">
///     A tiled_extent&lt;_Dim0,_Dim1,_Dim2&gt; which represents the tiled set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "tiled_index&lt;_Dim0,_Dim1,_Dim2&gt;" which performs the parallel computation.
/// </param>
template <int _Dim0, int _Dim1, int _Dim2, typename _Kernel_type> void parallel_for_each(const accelerator_view& _Accl_view, const tiled_extent<_Dim0, _Dim1, _Dim2>& _Compute_domain, const _Kernel_type& _Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {_Accl_view};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain that has been tiled into 2-dimensional
///     regions.
/// </summary>
/// <param name="_Accl_view">
///     The accelerator_view upon which to run this parallel computation.
/// </param>
/// <param name="_Compute_domain">
///     A tiled_extent&lt;_Dim0,_Dim1&gt; which represents the tiled set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "tiled_index&lt;_Dim0,_Dim1&gt;" which performs the parallel computation.
/// </param>
template <int _Dim0, int _Dim1, typename _Kernel_type> void parallel_for_each(const accelerator_view& _Accl_view, const tiled_extent<_Dim0, _Dim1>& _Compute_domain, const _Kernel_type& _Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {_Accl_view};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}

/// <summary>
///     Invokes a parallel computation of a kernel function over a compute domain that has been tiled into 1-dimensional
///     regions.
/// </summary>
/// <param name="_Accl_view">
///     The accelerator_view upon which to run this parallel computation.
/// </param>
/// <param name="_Compute_domain">
///     A tiled_extent&lt;_Dim0&gt; which represents the tiled set of indices that form the compute domain.
/// </param>
/// <param name="_Kernel">
///     A function object that takes an argument of type "tiled_index&lt;_Dim0&gt;" which performs the parallel computation.
/// </param>
template <int _Dim0, typename _Kernel_type> void parallel_for_each(const accelerator_view& _Accl_view, const tiled_extent<_Dim0>& _Compute_domain, const _Kernel_type& _Kernel)
{
    _Host_Scheduling_info _SchedulingInfo = {_Accl_view};
    details::_Parallel_for_each(&_SchedulingInfo, _Compute_domain, _Kernel);
}



//=============================================================================

extern "C"
{

// Debugging intrinsics
void direct3d_abort() __GPU_ONLY;
void direct3d_errorf(const char *, ...) __GPU_ONLY;
void direct3d_printf(const char *, ...) __GPU_ONLY;

}

//////////////////////////////////////////////////////////////////////
/// Memory fences and tile barriers

#pragma warning( push )
#pragma warning( disable : 4100 ) // unreferenced formal parameter

/// <summary>
///     Ensures that memory accesses are visible to other threads in the thread tile, and are executed according to program order
/// </summary>
/// <param name="_Barrier">
///     A tile_barrier object
/// </param>
inline void all_memory_fence(const tile_barrier & _Barrier) __GPU_ONLY
{
    __dp_d3d_all_memory_fence();
}

/// <summary>
///     Ensures that global memory accesses are visible to other threads in the thread tile, and are executed according to program order
/// </summary>
/// <param name="_Barrier">
///     A tile_barrier object
/// </param>
inline void global_memory_fence(const tile_barrier & _Barrier) __GPU_ONLY
{
    __dp_d3d_device_memory_fence();
}

/// <summary>
///     Ensures that tile_static memory accesses are visible to other threads in the thread tile, and are executed according to program order
/// </summary>
/// <param name="_Barrier">
///     A tile_barrier object
/// </param>
inline void tile_static_memory_fence(const tile_barrier & _Barrier) __GPU_ONLY
{
    __dp_d3d_tile_static_memory_fence();
}

#pragma warning( pop )



namespace direct3d
{

/// <summary>
///     Returns the absolute value of the argument
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <returns>
///     Returns the absolute value of the argument
/// </returns>
inline int abs(int _X) __GPU_ONLY
{
    return __dp_d3d_absi(_X);
}

/// <summary>
///     Clamps _X to the specified _Min and _Max range
/// </summary>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <param name="_Min">
///     Floating-point value
/// </param>
/// <param name="_Max">
///     Floating-point value
/// </param>
/// <returns>
///     Returns the clamped value of _X
/// </returns>
inline float clamp(float _X, float _Min, float _Max) __GPU_ONLY
{
    return __dp_d3d_clampf(_X, _Min, _Max);
}

/// <summary>
///     Clamps _X to the specified _Min and _Max range
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <param name="_Min">
///     Integer value
/// </param>
/// <param name="_Max">
///     Integer value
/// </param>
/// <returns>
///     Returns the clamped value of _X
/// </returns>
inline int clamp(int _X, int _Min, int _Max) __GPU_ONLY
{
    return __dp_d3d_clampi(_X, _Min, _Max);
}

/// <summary>
///     Counts the number of set bits in _X
/// </summary>
/// <param name="_X">
///     Unsigned integer value
/// </param>
/// <returns>
///     Returns the number of set bits in _X
/// </returns>
inline unsigned int countbits(unsigned int _X) __GPU_ONLY
{
    return __dp_d3d_countbitsu(_X);
}

/// <summary>
///     Gets the location of the first set bit in _X, starting from the highest order bit and working downward
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <returns>
///     Returns The location of the first set bit
/// </returns>
inline int firstbithigh(int _X) __GPU_ONLY
{
    return __dp_d3d_firstbithighi(_X);
}

/// <summary>
///     Gets the location of the first set bit in _X, starting from the lowest order bit and working upward
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <returns>
///     Returns The location of the first set bit
/// </returns>
inline int firstbitlow(int _X) __GPU_ONLY
{
    return __dp_d3d_firstbitlowi(_X);
}

/// <summary>
///     Determine the maximum numeric value of the arguments
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <param name="_Y">
///     Integer value
/// </param>
/// <returns>
///     Return the maximum numeric value of the arguments
/// </returns>
inline int imax(int _X, int _Y) __GPU_ONLY
{
    return __dp_d3d_maxi(_X, _Y);
}

/// <summary>
///     Determine the minimum numeric value of the arguments
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <param name="_Y">
///     Integer value
/// </param>
/// <returns>
///     Return the minimum numeric value of the arguments
/// </returns>
inline int imin(int _X, int _Y) __GPU_ONLY
{
    return __dp_d3d_mini(_X, _Y);
}

/// <summary>
///     Determine the maximum numeric value of the arguments
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <param name="_Y">
///     Integer value
/// </param>
/// <returns>
///     Return the maximum numeric value of the arguments
/// </returns>
inline unsigned int umax(unsigned int _X, unsigned int _Y) __GPU_ONLY
{
    return __dp_d3d_maxu(_X, _Y);
}

/// <summary>
///     Determine the minimum numeric value of the arguments
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <param name="_Y">
///     Integer value
/// </param>
/// <returns>
///     Return the minimum numeric value of the arguments
/// </returns>
inline unsigned int umin(unsigned int _X, unsigned int _Y) __GPU_ONLY
{
    return __dp_d3d_minu(_X, _Y);
}

/// <summary>
///     Performs an arithmetic multiply/add operation on three arguments: _X * _Y + _Z
/// </summary>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <param name="_Y">
///     Floating-point value
/// </param>
/// <param name="_Z">
///     Floating-point value
/// </param>
/// <returns>
///     Returns _X * _Y + _Z
/// </returns>
inline float mad(float _X, float _Y, float _Z) __GPU_ONLY
{
    return __dp_d3d_madf(_X, _Y, _Z);
}

/// <summary>
///     Performs an arithmetic multiply/add operation on three arguments: _X * _Y + _Z
/// </summary>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <param name="_Y">
///     Floating-point value
/// </param>
/// <param name="_Z">
///     Floating-point value
/// </param>
/// <returns>
///     Returns _X * _Y + _Z
/// </returns>
inline double mad(double _X, double _Y, double _Z) __GPU_ONLY
{
    return __dp_d3d_madd(_X, _Y, _Z);
}

/// <summary>
///     Performs an arithmetic multiply/add operation on three arguments: _X * _Y + _Z
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <param name="_Y">
///     Integer value
/// </param>
/// <param name="_Z">
///     Integer value
/// </param>
/// <returns>
///     Returns _X * _Y + _Z
/// </returns>
inline int mad(int _X, int _Y, int _Z) __GPU_ONLY
{
    return __dp_d3d_madi(_X, _Y, _Z);
}

/// <summary>
///     Performs an arithmetic multiply/add operation on three arguments: _X * _Y + _Z
/// </summary>
/// <param name="_X">
///     Unsigned integer value
/// </param>
/// <param name="_Y">
///     Unsigned integer value
/// </param>
/// <param name="_Z">
///     Unsigned integer value
/// </param>
/// <returns>
///     Returns _X * _Y + _Z
/// </returns>
inline unsigned int mad(unsigned int _X, unsigned int _Y, unsigned int _Z) __GPU_ONLY
{
    return __dp_d3d_madu(_X, _Y, _Z);
}

/// <summary>
///     Generates a random value using the Perlin noise algorithm
/// </summary>
/// <param name="_X">
///     Floating-point value from which to generate Perlin noise
/// </param>
/// <returns>
///     Returns The Perlin noise value within a range between -1 and 1
/// </returns>
inline float noise(float _X) __GPU_ONLY
{
    return __dp_d3d_noisef(_X);
}

/// <summary>
///     Converts _X from degrees to radians
/// </summary>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <returns>
///     Returns _X converted from degrees to radians
/// </returns>
inline float radians(float _X) __GPU_ONLY
{
    return __dp_d3d_radiansf(_X);
}

/// <summary>
///     Calculates a fast, approximate reciprocal of the argument
/// </summary>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <returns>
///     Returns a fast, approximate reciprocal of the argument
/// </returns>
inline float rcp(float _X) __GPU_ONLY
{
    return __dp_d3d_rcpf(_X);
}

/// <summary>
///     Reverses the order of the bits in _X
/// </summary>
/// <param name="_X">
///     Unsigned integer value
/// </param>
/// <returns>
///     Returns the value with the bit order reversed in _X
/// </returns>
inline unsigned int reversebits(unsigned int _X) __GPU_ONLY
{
    return __dp_d3d_reversebitsu(_X);
}

/// <summary>
///     Clamps _X within the range of 0 to 1
/// </summary>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <returns>
///     Returns _X clamped within the range of 0 to 1
/// </returns>
inline float saturate(float _X) __GPU_ONLY
{
    return __dp_d3d_saturatef(_X);
}

/// <summary>
///     Returns the sign of the argument
/// </summary>
/// <param name="_X">
///     Integer value
/// </param>
/// <returns>
///     Returns the sign of the argument
/// </returns>
inline int sign(int _X) __GPU_ONLY
{
    return __dp_d3d_signi(_X);
}

/// <summary>
///     Returns a smooth Hermite interpolation between 0 and 1, if _X is in the range [_Min, _Max].
/// </summary>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <param name="_Min">
///     Floating-point value
/// </param>
/// <param name="_Max">
///     Floating-point value
/// </param>
/// <returns>
///     Returns 0 if _X is less than _Min; 1 if _X is greater than _Max; otherwise, a value between 0 and 1 if _X is in the range [_Min, _Max]
/// </returns>
inline float smoothstep(float _Min, float _Max, float _X) __GPU_ONLY
{
    return __dp_d3d_smoothstepf(_Min, _Max, _X);
}

/// <summary>
///     Compares two values, returning 0 or 1 based on which value is greater
/// </summary>
/// <param name="_Y">
///     Floating-point value
/// </param>
/// <param name="_X">
///     Floating-point value
/// </param>
/// <returns>
///     Returns 1 if the _X is greater than or equal to _Y; otherwise, 0
/// </returns>
inline float step(float _Y, float _X) __GPU_ONLY
{
    return __dp_d3d_stepf(_Y, _X);
}

} // namespace Concurrency::direct3d

} // namespace Concurrency

#include <xxamp_inl.h>

namespace concurrency = Concurrency;

#pragma pack(pop)
// End of file
