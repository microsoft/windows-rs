/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* xxamp.h
*
* C++ AMP Library helper classes
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#ifndef _SILENCE_AMP_DEPRECATION_WARNINGS
#error <xxamp.h> is part of C++ AMP which is deprecated by Microsoft and will be REMOVED. \
You can define _SILENCE_AMP_DEPRECATION_WARNINGS to acknowledge that you have received this warning.
#endif // _SILENCE_AMP_DEPRECATION_WARNINGS

#define CPP_AMP_MAX_RANK 128
#define _CPP_AMP_VERIFY_RANK(_Rank, _Type_name) \
    static_assert(((_Rank) > 0) && ((_Rank) <= CPP_AMP_MAX_RANK), "The _Rank of " #_Type_name " should be greater than 0 and <= 128.")

// This macro is used to determine whether a specified iterator is a mutable iterator
// by ensuring that the iterator_category iterator_trait is or inherits from either the
// output_iterator_tag or forward_iterator_tag. We use a static_assert to emit a compilation
// error when the iterator does not meet this requirement. This macro is used for verifying that
// the destination iterator passed to the concurrency::copy overloads is of the right type.
#define _CPP_AMP_VERIFY_MUTABLE_ITERATOR(_Type_name) \
    static_assert((std::is_base_of<std::output_iterator_tag, typename std::iterator_traits<_Type_name>::iterator_category>::value \
                  || std::is_base_of<std::forward_iterator_tag, typename std::iterator_traits<_Type_name>::iterator_category>::value), \
                  "Invalid destination argument type to concurrency::copy.")

#pragma pack(push,8)

//=============================================================================
// Internal Intrinsic Functions used for implementing libraries
//=============================================================================
extern "C"
{
//=============================================================================
// Intrinsics that access textures
//=============================================================================
void __dp_read_texture(const void * /* pTex */,
                       _Out_ void * /* pRet */,
                       unsigned int /* x */,
                       unsigned int /* y */,
                       unsigned int /* z */,
                       unsigned int /* Miplevel */)  __GPU_ONLY;

void __dp_write_texture(_Out_ void * /* pTex */,
                        const void * /* pValue */,
                        unsigned int /* x */,
                        unsigned int /* y */,
                        unsigned int /* z */) __GPU_ONLY;

void __dp_sample_texture(const void * /* pTex */,
                         const void * /* pSampler */,
                         _Out_ void* /* pRet */,
                         float /* x */,
                         float /* y */,
                         float /* z */,
                         unsigned int /* kind: 0 - GatherRed, 1 - GatherGreen, 2 - GatherBlue, 3 - GatherAlpha, 4 - Sample */,
                         float /* LevelOfDetail */) __GPU_ONLY;

void __dp_sample_texture_predefined(const void * /* pTex */,
                                    _Out_ void * /* pRet */,
                                    float /* x */,
                                    float /* y */,
                                    float /* z */,
                                    unsigned int /* predefinedSamplerId: filter_mode << 16 | address_mode */,
                                    unsigned int /* kind: 0 - GatherRed, 1 - GatherGreen, 2 - GatherBlue, 3 - GatherAlpha, 4 - Sample */,
                                    float /* LevelOfDetail */) __GPU_ONLY;
}

namespace Concurrency
{
    // forward decls
    template <typename _Value_type, int _Rank>
    class array;

    template <typename _Value_type, int _Rank>
    class array_view;

    template <int _Dim0, int _Dim1 = 0, int _Dim2 = 0>
    class tiled_extent;

    template <int _Rank>
    class extent;

    template <int _Rank>
    class index;

namespace details
{
    // forward decls
    template <int _Rank, int _Element_size>
    class _Array_view_shape;

    template <int _Rank, int _Element_size>
    class _Array_view_base;

    // Helper class to avoid static_assertion errors on uninstantiated
    // templates.
    template <typename _T>
    struct _Falsifier
    {
        static const bool value = false;
    };

    // Helper classes for array and array_view projection
    template <typename _T, int _R>
    class _Projection_result_type
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");
        static_assert(_R >= 2, "Rank must be greater than or equal to 2");

    public:
        typedef array_view<const _T,_R-1> _Const_result_type;
        typedef array_view<_T,_R-1> _Result_type;
    };

    template <typename _T>
    class _Projection_result_type<_T,1>
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");

    public:
        typedef const _T& _Const_result_type;
        typedef _T& _Result_type;
    };

    template <typename _T, int _R>
    class _Const_array_view_projection_helper
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");
        static_assert(_R >= 2, "Rank must be greater than or equal to 2");

    public:
        static typename _Projection_result_type<_T,_R>::_Const_result_type _Project0(const array_view<const _T,_R>* _Arr_view, int _I) __GPU;
    };

    template <typename _T>
    class _Const_array_view_projection_helper<_T,1>
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");

    public:
        static typename _Projection_result_type<_T,1>::_Const_result_type _Project0(const array_view<const _T,1>* _Arr_view, int _I) __GPU;
    };

    template <typename _T, int _R>
    class _Array_view_projection_helper
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");
        static_assert(_R >= 2, "Rank must be greater than or equal to 2");

    public:
        static typename _Projection_result_type<_T,_R>::_Result_type _Project0(const array_view<_T,_R>* _Arr_view, int _I) __GPU;
    };

    template <typename _T>
    class _Array_view_projection_helper<_T,1>
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");

    public:
        static typename _Projection_result_type<_T,1>::_Result_type _Project0(const array_view<_T,1>* _Arr_view, int _I) __GPU;
    };

    template <typename _T, int _R>
    class _Const_array_projection_helper
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");
        static_assert(_R >= 2, "Rank must be greater than or equal to 2");

    public:
        static typename _Projection_result_type<_T,_R>::_Const_result_type _Project0(const array<_T,_R>* _Array, int _I) __GPU;
    };

    template <typename _T>
    class _Const_array_projection_helper<_T,1>
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");

    public:
        static typename _Projection_result_type<_T,1>::_Const_result_type _Project0(const array<_T,1>* _Array, int _I) __GPU;
    };

    template <typename _T, int _R>
    class _Array_projection_helper
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");
        static_assert(_R >= 2, "Rank must be greater than or equal to 2");

    public:
        static typename _Projection_result_type<_T,_R>::_Result_type _Project0(_In_ array<_T,_R>* _Array, int _I) __GPU;
    };

    template <typename _T>
    class _Array_projection_helper<_T,1>
    {
        static_assert(!std::is_const<_T>::value, "const _T is not supported");

    public:
        static typename _Projection_result_type<_T,1>::_Result_type _Project0(_In_ array<_T,1>* _Array, int _I) __GPU;
    };

    // leading underscore implies 'private'
    enum _eInitializeState {
        _do_not_initialize
    };

    // Helpers to create not initialized tuples
    template<class _Tuple_type>
    _Tuple_type _Create_uninitialized_tuple() __GPU
    {
        return _Tuple_type(details::_do_not_initialize);
    };


    // ========================================================
    // helpers for unroll the loops for lower rank
    // ========================================================

    // operation kinds
    enum _op_kind
    {
        // cmp op
        opEq,      // a == b
        opNeq,     // a != b
        // not op
        opNot,     // !a
        // compound assignment
        opAssign,  // a = b
        opAddEq,   // a += b;
        opSubEq,   // a -= b;
        opMulEq,   // a *= b
        opDivEq,   // a /= b
        opModEq,   // a %= b
        // arithmetic ops
        opAdd,     // c = a + b
        opSub,     // c = a - b
        opMul,     // c = a * b
        opDiv,     // c = a / b
        opMod,     // c = a % b
    };

    const static int LOOP_UNROLL_THRESHOLD = 4;

    // forward declarations
    template<typename _T, _op_kind _Kind, int _Rank = _T::rank, bool _Unroll = (_Rank > 1 && _Rank <= LOOP_UNROLL_THRESHOLD)>
    struct _cmp_op_loop_helper;

    template<typename _T, _op_kind _Kind, int _Rank = _T::rank, bool _Unroll = (_Rank > 1 && _Rank <= LOOP_UNROLL_THRESHOLD)>
    struct _compound_assign_op_loop_helper;

    template<typename _T, _op_kind _Kind, int _Rank = _T::rank, bool _Unroll = (_Rank > 1 && _Rank <= LOOP_UNROLL_THRESHOLD)>
    struct _arithmetic_op_loop_helper;

    template<typename _T, int _Rank = _T::rank, bool _Unroll = (_Rank > 1 && _Rank <= LOOP_UNROLL_THRESHOLD)>
    struct _product_helper;

    // comparison operations
    template<typename _T, _op_kind _Kind>
    struct _cmp_op_helper;

    template<typename _T>
    struct _cmp_op_helper<_T, opEq>
    {
        static bool func(const _T & a, const _T & b) __GPU { return a == b; }
    };

    template<typename _T>
    struct _cmp_op_helper<_T, opNeq>
    {
        static bool func(const _T & a, const _T & b) __GPU { return a != b; }
    };

    template<typename _T, _op_kind _Kind>
    struct _cmp_op_helper
    {
        static bool func(const _T & a, const _T & b) __GPU { static_assert(_Falsifier<_T>::value, "invalid op"); return false;}
    };

    // operations:  a op= b
    template<typename _T, _op_kind _Kind>
    struct _compound_assign_op_helper;

    template<typename _T>
    struct _compound_assign_op_helper<_T, opAssign>
    {
        static void func(_T & a, const _T & b) __GPU { a = b; }
    };

    template<typename _T>
    struct _compound_assign_op_helper<_T, opAddEq>
    {
        static void func(_T & a, const _T & b) __GPU { a += b; }
    };

    template<typename _T>
    struct _compound_assign_op_helper<_T, opSubEq>
    {
        static void func(_T & a, const _T & b) __GPU { a -= b; }
    };

    template<typename _T>
    struct _compound_assign_op_helper<_T, opMulEq>
    {
        static void func(_T & a, const _T & b) __GPU { a *= b; }
    };

    template<typename _T>
    struct _compound_assign_op_helper<_T, opDivEq>
    {
        static void func(_T & a, const _T & b) __GPU { a /= b; }
    };

    template<typename _T>
    struct _compound_assign_op_helper<_T, opModEq>
    {
        static void func(_T & a, const _T & b) __GPU { a %= b; }
    };

    template<typename _T, _op_kind _Kind>
    struct _compound_assign_op_helper
    {
        static void func(_T & a, const _T & b) __GPU { static_assert(_Falsifier<_T>::value, "invalid op"); }
    };

    // operations:  a = b + c
    template<typename _T, _op_kind _Kind>
    struct _arithmetic_op_helper;

    template<typename _T>
    struct _arithmetic_op_helper<_T, opAdd>
    {
        static void func(_T & a, const _T & b, const _T & c) __GPU { a = b + c; }
    };

    template<typename _T>
    struct _arithmetic_op_helper<_T, opSub>
    {
        static void func(_T & a, const _T & b, const _T & c) __GPU { a = b - c; }
    };

    template<typename _T>
    struct _arithmetic_op_helper<_T, opMul>
    {
        static void func(_T & a, const _T & b, const _T & c) __GPU { a = b * c; }
    };

    template<typename _T>
    struct _arithmetic_op_helper<_T, opDiv>
    {
        static void func(_T & a, const _T & b, const _T & c) __GPU { a = b / c; }
    };

    template<typename _T>
    struct _arithmetic_op_helper<_T, opMod>
    {
        static void func(_T & a, const _T & b, const _T & c) __GPU { a = b % c; }
    };

    template<typename _T, _op_kind _Kind>
    struct _arithmetic_op_helper
    {
        static void func(_T & a, const _T & b, const _T & c) __GPU { static_assert(_Falsifier<_T>::value, "invalid op"); }
    };


#pragma warning( push )
#pragma warning( disable : 4100 ) // unreferenced formal parameter
    template<typename _T1>
    struct _index_helper
    {
        template<typename _T2>
        static typename _T1::value_type func(const _T2 & a, int i) __GPU
        {
            return a[i];
        }

        static typename _T1::value_type func(typename _T1::value_type a[_T1::rank], int i) __GPU
        {
            return a[i];
        }

        static typename _T1::value_type func(typename _T1::value_type a, int i) __GPU
        {
            return a;
        }
    };
#pragma warning( pop )

    // loop unrolling helpers, unroll the loop if rank <= LOOP_UNROLL_THRESHOLD


    // helper class to unroll the loop that uses cmp ops

    // a[i] op b[i]
    template<typename _T, _op_kind _Kind>
    struct _cmp_op_loop_helper<_T, _Kind, 1, false /* _Unroll */ >
    {
        static bool func(const _T& a, const _T& b) __GPU
        {
            return _cmp_op_helper<typename _T::value_type, _Kind>::func(a[0], b[0]);
        }
    };

    template<typename _T, _op_kind _Kind, int _Rank>
    struct _cmp_op_loop_helper<_T, _Kind, _Rank, true /* _Unroll */ >
    {
        static bool func(const _T& a, const _T& b) __GPU
        {
            if (!_cmp_op_helper<typename _T::value_type, _Kind>::func(a[_Rank - 1], b[_Rank - 1]))
                return false;
            else
                return _cmp_op_loop_helper<_T, _Kind, _Rank - 1>::func(a, b);
        }
    };

    template<typename _T, _op_kind _Kind, int _Rank>
    struct _cmp_op_loop_helper<_T, _Kind, _Rank, false /* _Unroll */ >
    {
        static bool func(const _T& a, const _T& b) __GPU
        {
            for (int i = 0; i < _Rank; i++)
            {
                if (!_cmp_op_helper<typename _T::value_type, _Kind>::func(a[i], b[i]))
                    return false;
            }
            return true;
        }
    };

    // helper class for loop that uses  a op= b

    template<typename _T, _op_kind _Kind>
    struct _compound_assign_op_loop_helper<_T, _Kind, 1, false /* _Unroll */>
    {
        template<typename _T2>
        static void func(_T& a, const _T2& b) __GPU
        {
            _compound_assign_op_helper<typename _T::value_type, _Kind>::func(a[0], _index_helper<_T>::func(b, 0));
        }
    };

    template<typename _T, _op_kind _Kind, int _Rank>
    struct _compound_assign_op_loop_helper<_T, _Kind, _Rank, true /* _Unroll */>
    {
        template<typename _T2>
        static void func(_T& a, const _T2& b) __GPU
        {
            _compound_assign_op_loop_helper<_T, _Kind, _Rank - 1>::func(a, b);
            _compound_assign_op_helper<typename _T::value_type, _Kind>::func(a[_Rank - 1], _index_helper<_T>::func(b, _Rank - 1));
        }
    };

    template<_op_kind _Kind,  typename _T, int _Rank>
    struct _compound_assign_op_loop_helper<_T, _Kind, _Rank, false /* _Unroll */>
    {
        template<typename _T2>
        static void func(_T& a, const _T2& b) __GPU
        {
            for (int i = 0; i < _Rank; i++)
            {
                _compound_assign_op_helper<typename _T::value_type, _Kind>::func(a[i], _index_helper<_T>::func(b, i));
            }
        }
    };

    // specialization of _compound_assign_op_loop_helper for opAssign
    template<typename _T>
    struct _compound_assign_op_loop_helper<_T, opAssign, 1, false /* _Unroll */>
    {
        template<typename _T2>
        static void func(_T& a, const _T2& b) __GPU
        {
            a[0] = b[0];
        }

        static void func(_T& a, typename _T::value_type b) __GPU
        {
            a[0] = b;
        }
    };

    template<typename _T>
    struct _compound_assign_op_loop_helper<_T, opAssign, 2, true /* _Unroll */>
    {
        template<typename _T2>
        static void func(_T& a, const _T2& b) __GPU
        {
            a[0] = b[0];
            a[1] = b[1];
        }

        static void func(_T& a, typename _T::value_type b) __GPU
        {
            a[0] = b;
            a[1] = b;
        }
    };

    template<typename _T>
    struct _compound_assign_op_loop_helper<_T, opAssign, 3, true /* _Unroll */>
    {
        template<typename _T2>
        static void func(_T& a, const _T2& b) __GPU
        {
            a[0] = b[0];
            a[1] = b[1];
            a[2] = b[2];
        }

        static void func(_T& a, typename _T::value_type b) __GPU
        {
            a[0] = b;
            a[1] = b;
            a[2] = b;
        }
    };

    template<typename _T>
    struct _compound_assign_op_loop_helper<_T, opAssign, 4, true /* _Unroll */>
    {
        template<typename _T2>
        static void func(_T& a, const _T2& b) __GPU
        {
            a[0] = b[0];
            a[1] = b[1];
            a[2] = b[2];
            a[3] = b[3];
        }

        static void func(_T& a, typename _T::value_type b) __GPU
        {
            a[0] = b;
            a[1] = b;
            a[2] = b;
            a[3] = b;
        }
    };

    // helper class for loop that uses  a = b op c

    template<typename _T, _op_kind _Kind>
    struct _arithmetic_op_loop_helper<_T, _Kind, 1, false /* _Unroll */>
    {
        template<typename _T1, typename _T2>
        static void func(_T& a, const _T1& b, const _T2& c) __GPU
        {
            _arithmetic_op_helper<typename _T::value_type, _Kind>::func(a[0],
                                                               _index_helper<_T>::func(b, 0),
                                                               _index_helper<_T>::func(c, 0));
        }
    };

    template<typename _T, _op_kind _Kind, int _Rank>
    struct _arithmetic_op_loop_helper<_T, _Kind, _Rank, true /* _Unroll */>
    {
        template<typename _T1, typename _T2>
        static void func(_T& a, const _T1& b, const _T2& c) __GPU
        {
            _arithmetic_op_loop_helper<_T, _Kind, _Rank - 1>::func(a, b, c);
            _arithmetic_op_helper<typename _T::value_type, _Kind>::func(a[_Rank - 1],
                                                               _index_helper<_T>::func(b, _Rank - 1),
                                                               _index_helper<_T>::func(c, _Rank - 1));
        }
    };

    template<typename _T, _op_kind _Kind, int _Rank>
    struct _arithmetic_op_loop_helper<_T, _Kind, _Rank, false /* _Unroll */>
    {
        template<typename _T1, typename _T2>
        static void func(_T& a, const _T1& b, const _T2& c) __GPU
        {
            for (int i = 0; i < _Rank; i++)
            {
                _arithmetic_op_helper<typename _T::value_type, _Kind>::func(a[i],
                                                                   _index_helper<_T>::func(b, i),
                                                                   _index_helper<_T>::func(c, i));
            }
        }
    };

    // helper for unroll the loop for product operation

    template<typename _T>
    struct _product_helper<_T, 1, false /* _Unroll */>
    {
        template<typename _T1>
        static typename _T::value_type func(const _T1 & a) __GPU
        {
            return a[0];
        }
    };

    template<typename _T, int _Rank>
    struct _product_helper<_T, _Rank, true /* _Unroll */>
    {
        template<typename _T1>
        static typename _T::value_type func(const _T1 & a) __GPU
        {
            return _product_helper<_T, _Rank - 1>::func(a) * a[_Rank - 1];
        }
    };

    template<typename _T, int _Rank>
    struct _product_helper<_T, _Rank, false /* _Unroll */>
    {
        template<typename _T1>
        static typename _T::value_type func(const _T1 & a) __GPU
        {
            typename _T::value_type _e = a[0];
            for (int i = 1; i < _Rank; i++)
            {
                _e *= a[i];
            }
            return _e;
        }
    };

    template<typename _T1, int _Rank = _T1::rank>
    struct _map_index;

#pragma warning( push )
#pragma warning( disable : 4100 ) // unreferenced formal parameter
    template<typename _T1>
    struct _map_index<_T1, 1>
    {
        template<typename _T2>
        static _T1 func(int _Flat_index, const _T2 _Base) __GPU
        {
            _T1 _index = _Create_uninitialized_tuple<_T1>();
            _index[0]   = _Flat_index;
            return _index;
        }
    };

    template<typename _T1>
    struct _map_index<_T1, 2>
    {
        template<typename _T2>
        static _T1 func(int _Flat_index, const _T2 _Base) __GPU
        {
            _T1 _index = _Create_uninitialized_tuple<_T1>();
            _index[1]   = static_cast<unsigned int>(_Flat_index) % static_cast<unsigned int>(_Base[1]);
            _index[0]   = static_cast<unsigned int>(_Flat_index) / static_cast<unsigned int>(_Base[1]);
            return _index;
        }
    };

    template<typename _T1>
    struct _map_index<_T1, 3>
    {
        template<typename _T2>
        static _T1 func(int _Flat_index, const _T2 _Base) __GPU
        {
            _T1 _index = _Create_uninitialized_tuple<_T1>();

            _index[2]   = static_cast<unsigned int>(_Flat_index) % static_cast<unsigned int>(_Base[2]);
            _Flat_index = static_cast<unsigned int>(_Flat_index) / static_cast<unsigned int>(_Base[2]);
            _index[1]   = static_cast<unsigned int>(_Flat_index) % static_cast<unsigned int>(_Base[1]);
            _index[0]   = static_cast<unsigned int>(_Flat_index) / static_cast<unsigned int>(_Base[1]);
            return _index;
        }
    };

    template<typename _T1>
    struct _map_index<_T1, 4>
    {
        template<typename _T2>
        static _T1 func(int _Flat_index, const _T2 _Base) __GPU
        {
            _T1 _index = _Create_uninitialized_tuple<_T1>();
            _index[3]   = static_cast<unsigned int>(_Flat_index) % static_cast<unsigned int>(_Base[3]);
            _Flat_index = static_cast<unsigned int>(_Flat_index) / static_cast<unsigned int>(_Base[3]);
            _index[2]   = static_cast<unsigned int>(_Flat_index) % static_cast<unsigned int>(_Base[2]);
            _Flat_index = static_cast<unsigned int>(_Flat_index) / static_cast<unsigned int>(_Base[2]);
            _index[1]   = static_cast<unsigned int>(_Flat_index) % static_cast<unsigned int>(_Base[1]);
            _index[0]   = static_cast<unsigned int>(_Flat_index) / static_cast<unsigned int>(_Base[1]);
            return _index;
        }
    };

    template<typename _T1, int _Rank>
    struct _map_index
    {
        template<typename _T2>
        static _T1 func(int _Flat_index, const _T2 _Base) __GPU
        {
            _T1 _index = _Create_uninitialized_tuple<_T1>();
            for (int i = _Rank - 1; i > 0; --i)
            {
                _index[i] = static_cast<unsigned int>(_Flat_index) % static_cast<unsigned int>(_Base[i]);
                _Flat_index = static_cast<unsigned int>(_Flat_index) / static_cast<unsigned int>(_Base[i]);
            }
            _index[0] = _Flat_index;
            return _index;
        }
    };
#pragma warning( pop )

    // Helper class for unrolling extent<N>::contains

#pragma warning( push )
#pragma warning( disable : 4100 ) // unreferenced formal parameter

    template<typename _EXT, typename _IDX, int _R>
    struct _contains
    {
        static bool func(const _EXT& _Ext, const _IDX& _Idx) __GPU
		{
			for (int _I=0; _I<_R; _I++)
			{
				if ((_Idx[_I] < 0) | (_Idx[_I] >= _Ext[_I]))
					return false;
			}
			return true;
		}
    };

    template<typename _EXT, typename _IDX>
    struct _contains<_EXT,_IDX,1>
    {
        static bool func(const _EXT& _Ext, const _IDX& _Idx) __GPU
		{
			return (_Idx[0] >= 0) & (_Idx[0] < _Ext[0]);
		}
    };

    template<typename _EXT, typename _IDX>
    struct _contains<_EXT,_IDX,2>
    {
        static bool func(const _EXT& _Ext, const _IDX& _Idx) __GPU
		{
			return (_Idx[0] >= 0) & (_Idx[0] < _Ext[0]) &
				   (_Idx[1] >= 0) & (_Idx[1] < _Ext[1]);
		}
    };

    template<typename _EXT, typename _IDX>
    struct _contains<_EXT,_IDX,3>
    {
        static bool func(const _EXT& _Ext, const _IDX& _Idx) __GPU
		{
			return (_Idx[0] >= 0) & (_Idx[0] < _Ext[0]) &
				   (_Idx[1] >= 0) & (_Idx[1] < _Ext[1]) &
				   (_Idx[2] >= 0) & (_Idx[2] < _Ext[2]);
		}
    };

#pragma warning( pop )

    // Helper class for loop unrolling of array projection

#pragma warning( push )
#pragma warning( disable : 4100 ) // unreferenced formal parameter

    template<typename _RES_EXT, typename _SRC_EXT, typename _RES_IDX, typename _SRC_IDX, int _R>
    struct _project0
    {
		static_assert(_RES_EXT::rank == _R-1, "Result extent rank must be _R-1");
		static_assert(_SRC_EXT::rank == _R, "Source extent rank must be _R");
		static_assert(_RES_IDX::rank == _R-1, "Result index rank must be _R-1");
		static_assert(_SRC_IDX::rank == _R, "Source index rank must be _R");

        static void func(_RES_EXT& _ResArrayExtent, const _SRC_EXT& _SrcArrayExtent,
                         _RES_EXT& _ResArrayMultiplier, const _SRC_EXT& _SrcArrayMultiplier,
                         _RES_IDX& _ResViewOffset, const _SRC_IDX& _SrcViewOffset,
                         _RES_EXT& _ResViewExtent, const _SRC_EXT& _SrcViewExtent) __GPU
        {
            for (int _I=0; _I<=_R-3; _I++)
            {
                _ResArrayExtent    [_I] = _SrcArrayExtent    [_I+1];
                _ResArrayMultiplier[_I] = _SrcArrayMultiplier[_I+1];
                _ResViewOffset     [_I] = _SrcViewOffset     [_I+1];
                _ResViewExtent     [_I] = _SrcViewExtent     [_I+1];
            }

            _ResArrayExtent    [_R-2] = _SrcArrayExtent    [_R-1];
            _ResViewOffset     [_R-2] = _SrcViewOffset     [_R-1];
            _ResViewExtent     [_R-2] = _SrcViewExtent     [_R-1];
        }

    };

    template<typename _RES_EXT, typename _SRC_EXT, typename _RES_IDX, typename _SRC_IDX>
    struct _project0<_RES_EXT,_SRC_EXT,_RES_IDX,_SRC_IDX,2>
    {
        static void func(_RES_EXT& _ResArrayExtent, const _SRC_EXT& _SrcArrayExtent,
                         _RES_EXT& /*_ResArrayMultiplier*/, const _SRC_EXT& /*_SrcArrayMultiplier*/,
                         _RES_IDX& _ResViewOffset, const _SRC_IDX& _SrcViewOffset,
                         _RES_EXT& _ResViewExtent, const _SRC_EXT& _SrcViewExtent) __GPU
        {
            _ResArrayExtent[0] = _SrcArrayExtent[1];
            _ResViewOffset [0] = _SrcViewOffset [1];
            _ResViewExtent [0] = _SrcViewExtent [1];
        }
    };

    template<typename _RES_EXT, typename _SRC_EXT, typename _RES_IDX, typename _SRC_IDX>
    struct _project0<_RES_EXT,_SRC_EXT,_RES_IDX,_SRC_IDX,3>
    {
        static void func(_RES_EXT& _ResArrayExtent, const _SRC_EXT& _SrcArrayExtent,
                         _RES_EXT& _ResArrayMultiplier, const _SRC_EXT& _SrcArrayMultiplier,
                         _RES_IDX& _ResViewOffset, const _SRC_IDX& _SrcViewOffset,
                         _RES_EXT& _ResViewExtent, const _SRC_EXT& _SrcViewExtent) __GPU
        {
            _ResArrayExtent    [0] = _SrcArrayExtent    [1];
            _ResArrayMultiplier[0] = _SrcArrayMultiplier[1];
            _ResViewOffset     [0] = _SrcViewOffset     [1];
            _ResViewExtent     [0] = _SrcViewExtent     [1];

            _ResArrayExtent    [1] = _SrcArrayExtent    [2];
            _ResViewOffset     [1] = _SrcViewOffset     [2];
            _ResViewExtent     [1] = _SrcViewExtent     [2];
        }

    };

#pragma warning( pop )


    // helper class for loop unrolling.
    template<typename _T1, typename _T2, int _Rank = _T2::rank>
    struct _Array_init_helper;

#pragma warning( push )
#pragma warning( disable : 4100 ) // unreferenced formal parameter
    template<typename _T1, typename _T2>
    struct _Array_init_helper<_T1, _T2, 1>
    {
        static void func(unsigned int & _Total_extent, _T1 & _Multiplier, const _T2 & _Extent) __GPU
        {
            return;
        }
    };
#pragma warning( pop )

    template<typename _T1, typename _T2>
    struct _Array_init_helper<_T1, _T2, 2>
    {
        static void func(unsigned int & _Total_extent, _T1 & _Multiplier, const _T2 & _Extent) __GPU
        {
            _Multiplier[0] = _Total_extent;
            _Total_extent *= _Extent[0];
        }
    };

    template<typename _T1, typename _T2>
    struct _Array_init_helper<_T1, _T2, 3>
    {
        static void func(unsigned int & _Total_extent, _T1 & _Multiplier, const _T2 & _Extent) __GPU
        {
            _Multiplier[1] = _Total_extent;
            _Total_extent *= _Extent[1];
            _Multiplier[0] = _Total_extent;
            _Total_extent *= _Extent[0];
        }
    };

    template<typename _T1, typename _T2>
    struct _Array_init_helper<_T1, _T2, 4>
    {
        static void func(unsigned int & _Total_extent, _T1 & _Multiplier, const _T2 & _Extent) __GPU
        {
            _Multiplier[2] = _Total_extent;
            _Total_extent *= _Extent[2];
            _Multiplier[1] = _Total_extent;
            _Total_extent *= _Extent[1];
            _Multiplier[0] = _Total_extent;
            _Total_extent *= _Extent[0];
        }
    };

    template<typename _T1, typename _T2, int _Rank>
    struct _Array_init_helper
    {
        static void func(unsigned int & _Total_extent, _T1 & _Multiplier, const _T2 & _Extent) __GPU
        {
            _Multiplier[_Rank-2] = _Total_extent;
            for (int i = _Rank-2; i >= 1; --i) {
                _Total_extent *= _Extent[i];
                _Multiplier[i-1] = _Total_extent;
            }
            _Total_extent *= _Extent[0];
        }
    };

    template<int _Rank, typename _T1, typename _T2>
    struct _Array_flatten_helper;

    template<typename _T1, typename _T2>
    struct _Array_flatten_helper<1, _T1, _T2>
    {
        static _T2 func(const _T1 * /*_Multiplier*/, const _T2 *_Index) __GPU
        {
            return _Index[0];
        }
    };

    template<typename _T1, typename _T2>
    struct _Array_flatten_helper<2, _T1, _T2>
    {
        static _T2 func(const _T1 *_Multiplier, const _T2 *_Index) __GPU
        {
            return ((_Multiplier[0] * _Index[0]) + _Index[1]);
        }
    };

    template<typename _T1, typename _T2>
    struct _Array_flatten_helper<3, _T1, _T2>
    {
        static _T2 func(const _T1 *_Multiplier, const _T2 *_Index) __GPU
        {
            return ((_Multiplier[0] * _Index[0]) + (_Multiplier[1] * _Index[1]) + _Index[2]);
        }
    };

    template<int _Rank, typename _T1, typename _T2>
    struct _Array_flatten_helper
    {
        static _T2 func(const _T1 *_Multiplier, const _T2 *_Index) __GPU
        {
            _T2 _Offset = _Index[_Rank - 1];
            for (int _I = 0; _I < (_Rank - 1); _I++)
            {
                _Offset += (_Multiplier[_I] * _Index[_I]);
            }
            return _Offset;
        }
    };


    template<typename _T, int _Rank>
    struct _Texture_read_helper;

    // rank == 1
    template<typename _T>
    struct _Texture_read_helper<_T, 1>
    {
        static void func(const void * _Tex_data, _Out_ void * _Val, const _T & _Index, unsigned int _Mip_level) __GPU_ONLY
        {
            __dp_read_texture(_Tex_data, _Val, static_cast<unsigned int>(_Index[0]), 1, 1, _Mip_level);
        }
    };

    template<typename _T>
    struct _Texture_read_helper<_T, 2>
    {
        static void func(const void * _Tex_data, _Out_ void * _Val, const _T & _Index, unsigned int _Mip_level) __GPU_ONLY
        {
            __dp_read_texture(_Tex_data, _Val, static_cast<unsigned int>(_Index[1]), static_cast<unsigned int>(_Index[0]), 1, _Mip_level);
        }
    };

    template<typename _T>
    struct _Texture_read_helper<_T, 3>
    {
        static void func(const void * _Tex_data, _Out_ void * _Val, const _T & _Index, unsigned int _Mip_level) __GPU_ONLY
        {
            __dp_read_texture(_Tex_data, _Val, static_cast<unsigned int>(_Index[2]), static_cast<unsigned int>(_Index[1]), static_cast<unsigned int>(_Index[0]), _Mip_level);
        }
    };

    template<typename _T, int _Rank>
    struct _Texture_write_helper;

    // rank == 1
    template<typename _T>
    struct _Texture_write_helper<_T, 1>
    {
        static void func(_Out_ void * _Tex_data, const void * _Ret, const _T & _Index) __GPU_ONLY
        {
            __dp_write_texture(_Tex_data, _Ret, static_cast<unsigned int>(_Index[0]), 1, 1);
        }
    };

    template<typename _T>
    struct _Texture_write_helper<_T, 2>
    {
        static void func(_Out_ void * _Tex_data, const void * _Ret, const _T & _Index) __GPU_ONLY
        {
            __dp_write_texture(_Tex_data, _Ret, static_cast<unsigned int>(_Index[1]), static_cast<unsigned int>(_Index[0]), 1);
        }
    };

    template<typename _T>
    struct _Texture_write_helper<_T, 3>
    {
        static void func(_Out_ void * _Tex_data, const void * _Ret, const _T & _Index) __GPU_ONLY
        {
            __dp_write_texture(_Tex_data, _Ret, static_cast<unsigned int>(_Index[2]), static_cast<unsigned int>(_Index[1]), static_cast<unsigned int>(_Index[0]));
        }
    };

    template<typename _T, int _Rank>
    struct _Texture_sample_helper;

    // rank == 1
    template<typename _T>
    struct _Texture_sample_helper<_T, 1>
    {
        static void func(const void * _Tex_data, const void* _Sampler, _Out_ void * _Val, const _T & _Coord, unsigned int _Kind, float _Level_of_detail) __GPU_ONLY
        {
            __dp_sample_texture(_Tex_data, _Sampler, _Val, _Coord, 1.0f, 1.0f, _Kind, _Level_of_detail);
        }
    };

    template<typename _T>
    struct _Texture_sample_helper<_T, 2>
    {
        static void func(const void * _Tex_data, const void* _Sampler, _Out_ void * _Val, const _T & _Coord, unsigned int _Kind, float _Level_of_detail) __GPU_ONLY
        {
            __dp_sample_texture(_Tex_data, _Sampler, _Val, _Coord.x, _Coord.y, 1.0f, _Kind, _Level_of_detail);
        }
    };

    template<typename _T>
    struct _Texture_sample_helper<_T, 3>
    {
        static void func(const void * _Tex_data, const void* _Sampler, _Out_ void * _Val, const _T & _Coord, unsigned int _Kind, float _Level_of_detail) __GPU_ONLY
        {
            __dp_sample_texture(_Tex_data, _Sampler, _Val, _Coord.x, _Coord.y, _Coord.z, _Kind, _Level_of_detail);
        }
    };

    template<typename _T, int _Rank>
    struct _Texture_predefined_sample_helper;

    // rank == 1
    template<typename _T>
    struct _Texture_predefined_sample_helper<_T, 1>
    {
        static void func(const void * _Tex_data, _Out_ void * _Val, const _T & _Coord, unsigned int _Predefined_sampler_id, unsigned int _Kind, float _Level_of_detail) __GPU_ONLY
        {
            __dp_sample_texture_predefined(_Tex_data, _Val, _Coord, 1.0f, 1.0f, _Predefined_sampler_id, _Kind, _Level_of_detail);
        }
    };

    template<typename _T>
    struct _Texture_predefined_sample_helper<_T, 2>
    {
        static void func(const void * _Tex_data, _Out_ void * _Val, const _T & _Coord, unsigned int _Predefined_sampler_id,unsigned  int _Kind, float _Level_of_detail) __GPU_ONLY
        {
            __dp_sample_texture_predefined(_Tex_data, _Val, _Coord.x, _Coord.y, 1.0f, _Predefined_sampler_id, _Kind, _Level_of_detail);
        }
    };

    template<typename _T>
    struct _Texture_predefined_sample_helper<_T, 3>
    {
        static void func(const void * _Tex_data, _Out_ void * _Val, const _T & _Coord, unsigned int _Predefined_sampler_id, unsigned int _Kind, float _Level_of_detail) __GPU_ONLY
        {
            __dp_sample_texture_predefined(_Tex_data, _Val, _Coord.x, _Coord.y, _Coord.z, _Predefined_sampler_id, _Kind, _Level_of_detail);
        }
    };


    // Retrieve these fields from arrays
    template <typename _Array_type>
    const _Buffer_descriptor& _Get_buffer_descriptor(const _Array_type& _Array) __GPU
    {
        return _Array._M_buffer_descriptor;
    }

    template <typename _Array_type>
    _Ret_ _Ubiquitous_buffer* _Get_buffer(const _Array_type& _Array) __CPU_ONLY
    {
        return _Array._Get_buffer();
    }

    // Helper method to access the private _Get_access method of the array
    // object which gets access to the underlying buffer on the array's accelerator_view
    // by synchronizing any pending modifications on other accelerator_views made through
    // array_views on the array
    template <typename _Array_type>
    _Event _Get_access_async(const _Array_type& _Array, _Access_mode _Mode, _Buffer_ptr &_Buf_ptr) __CPU_ONLY
    {
        return _Array._Get_access_async(_Mode, _Buf_ptr);
    }

    // Helper method to obtain a unique ID for identifying the data source
    // underlying the array/array_view argument
    template <typename _Array_type>
    void *_Get_datasource_identity(const _Array_type &_Array)
    {
        return _Get_buffer_descriptor(_Array)._Get_buffer_ptr()._Get_ptr();
    }

    // Retrieve these fields from textures
    template <typename _Texture_type>
    const _Texture_descriptor& _Get_texture_descriptor(const _Texture_type& _Tex) __GPU
    {
        return _Tex._M_texture_descriptor;
    }

    template <typename _Texture_type>
    _Ret_ _Texture* _Get_texture(const _Texture_type& _Tex) __CPU_ONLY
    {
        return _Tex._Get_texture();
    }

    template<int _Rank, template <int> class _T>
    static void _Is_positive(const _T<_Rank> &_Tuple) __CPU_ONLY
    {
        bool valid = true;
        for (int i = 0; i < _Rank; ++i)
        {
            if (_Tuple[i] <= 0) {
                valid = false;
                break;
            }
        }

        if (!valid) {
            throw runtime_exception("Invalid - values for each dimension must be > 0", E_INVALIDARG);
        }
    }

    // The GPU version is a no-op for since there is no good exception-like mechanism on GPU
    template<int _Rank, template <int> class _T>
    static void _Is_positive(const _T<_Rank> &/*_Tuple*/) __GPU_ONLY
    {
    }


    template<int _Rank, template <int> class _T>
    static void _Is_nonnegative(const _T<_Rank> &_Tuple) __CPU_ONLY
    {
        bool valid = true;
        for (int i = 0; i < _Rank; ++i)
        {
            if (_Tuple[i] < 0) {
                valid = false;
                break;
            }
        }

        if (!valid) {
            throw runtime_exception("Invalid - values for each dimension must be >= 0", E_INVALIDARG);
        }
    }

    // The GPU version is a no-op for since there is no good exception-like mechanism on GPU
    template<int _Rank, template <int> class _T>
    static void _Is_nonnegative(const _T<_Rank> & /*_Tuple*/) __GPU_ONLY
    {
    }

    template<int _Rank, template <int> class _T1, template <int> class _T2>
    static void _Is_valid_section(
        const _T2<_Rank>& _Base_extent,
        const _T1<_Rank>& _Section_origin,
        const _T2<_Rank>& _Section_extent) __CPU_ONLY
    {
        _Is_nonnegative(_Section_origin);
        _Is_positive(_Section_extent);

        for (int i = 0; i < _Rank; ++i)
        {
            if ((_Section_origin[i] + _Section_extent[i]) > _Base_extent[i]) {
                throw runtime_exception("the specified section index and extent are out of bound", E_INVALIDARG);
            }
        }
    }

    template<int _Rank, template <int> class _T1, template <int> class _T2>
    static void _Is_valid_section(
        const _T2<_Rank>& /*_Base_extent*/,
        const _T1<_Rank>& /*_Section_origin*/,
        const _T2<_Rank>& /*_Section_extent*/) __GPU_ONLY
    {
    };

    template<int _Rank, template <int> class _T1>
    static void _Is_valid_projection(int _I, const _T1<_Rank>& _Base_extent) __CPU_ONLY
    {
        if ((_I < 0) || (_I >= _Base_extent[0])) {
            throw runtime_exception("the specified projection index is out of bound", E_INVALIDARG);
        }
    }

    template<int _Rank, template <int> class _T1>
    static void _Is_valid_projection(int /*_I*/, const _T1<_Rank>& /*_Base_extent*/) __GPU_ONLY
    {
    }


    // An extent is valid if the value in each dimension is > 0 and the
    // size of the extent does not exceed UINT_MAX
    // Important: This needs to be revisited whenever we change the return
    // type of extent.size()
    template<int _Rank, template <int> class _T>
    static void _Is_valid_extent(const _T<_Rank> &_Tuple) __CPU_ONLY
    {
        _Is_positive(_Tuple);

        bool totalSizeValid = true;
        unsigned long long totalSize = (unsigned long long)_Tuple[0];
#pragma warning( push )
#pragma warning( disable : 6294 )
        for (int i = 1; i < _Rank; ++i)
        {
            totalSize *= (unsigned long long)_Tuple[i];
            if (totalSize > UINT_MAX) {
                totalSizeValid = false;
                break;
            }
        }
#pragma warning( pop )

        if (!totalSizeValid) {
            throw runtime_exception("Invalid - extent size exceeds UINT_MAX", E_INVALIDARG);
        }
    }

    // The GPU version is a no-op for since there is no good exception-like mechanism on GPU
    template<int _Rank, template <int> class _T>
    static void _Is_valid_extent(const _T<_Rank> & /*_Tuple*/) __GPU_ONLY
    {
    }

    template<int _Rank>
    inline unsigned int _Get_max_mipmap_levels(const extent<_Rank> &_Extent)
    {
        unsigned int _Mipmap_levels = 0;

        // Find the largest dimension
        unsigned int _Max_dim = static_cast<unsigned int>(_Extent[0]);
        for(int _I=1; _I<_Rank; ++_I)
        {
            _Max_dim = static_cast<unsigned int>(_Extent[_I]) > _Max_dim ? static_cast<unsigned int>(_Extent[_I]) : _Max_dim;
        }

        // Find out how many times we can divide it by 2
        while(_Max_dim > 0)
        {
            _Mipmap_levels++;
            _Max_dim >>= 1;
        }

        return _Mipmap_levels;
    }

    inline void _Are_valid_mipmap_parameters(unsigned int _Most_detailed_mipmap_level, unsigned int _Mipmap_levels = 0)
    {
        if (_Most_detailed_mipmap_level >= 32)
        {
            throw runtime_exception("The most detailed mipmap level cannot be greater than or equal to 32", E_INVALIDARG);
        }

        if (_Mipmap_levels > 32)
        {
            throw runtime_exception("The number of mipmap levels cannot be greater than 32", E_INVALIDARG);
        }
    }

    template<int _Rank>
    inline extent<_Rank> _Get_extent_at_level_unsafe(const extent<_Rank> &_Base_extent, unsigned int _Level) __GPU;

    template<int _Rank>
    inline extent<_Rank> _Get_extent_at_level(const extent<_Rank> &_Base_extent, unsigned int _Level);

    // Validate most detailed level and mipmap levels of the new view, given number of mipmap levels on the source
    inline void _Is_valid_mipmap_range(unsigned int _Src_view_mipmap_levels, unsigned int _Dst_most_detailed_level, unsigned int _Dst_view_mipmap_levels)
    {
        _Are_valid_mipmap_parameters(_Dst_most_detailed_level, _Dst_view_mipmap_levels);

        if (_Dst_view_mipmap_levels == 0 || _Src_view_mipmap_levels < _Dst_most_detailed_level + _Dst_view_mipmap_levels)
        {
            throw runtime_exception("Invalid texture mipmap range", E_INVALIDARG);
        }
    }

    // _Parallel_for_each declarations used by the Concurrency::parallel_for_each functions
    template <int _Rank, typename _Kernel_type>
    void _Parallel_for_each(_In_ _Host_Scheduling_info *_Sch_info, extent<_Rank> _Compute_domain, const _Kernel_type &_F);

    template <int _Dim0, int _Dim1, int _Dim2, typename _Kernel_type>
    void _Parallel_for_each(_In_ _Host_Scheduling_info *_Sch_info, tiled_extent<_Dim0, _Dim1, _Dim2> _Compute_domain, const _Kernel_type &_F);

    template <int _Dim0, int _Dim1, typename _Kernel_type>
    void _Parallel_for_each(_In_ _Host_Scheduling_info *_Sch_info, tiled_extent<_Dim0, _Dim1> _Compute_domain, const _Kernel_type &_F);

    template <int _Dim0, typename _Kernel_type>
    void _Parallel_for_each(_In_ _Host_Scheduling_info *_Sch_info, tiled_extent<_Dim0> _Compute_domain, const _Kernel_type &_F);

} // namespace Concurrency::details

} // namespace Concurrency

//=============================================================================
// Internal Intrinsic Functions used for implementing libraries
//=============================================================================

extern "C"
{

//=============================================================================
// Intrinsics that maps to D3D HLSL intrinsics
//=============================================================================
int __dp_d3d_absi(int) __GPU_ONLY;
void __dp_d3d_all_memory_fence() __GPU_ONLY;
void __dp_d3d_all_memory_fence_with_tile_barrier() __GPU_ONLY;
float __dp_d3d_clampf(float, float, float) __GPU_ONLY;
int __dp_d3d_clampi(int, int, int) __GPU_ONLY;
unsigned int __dp_d3d_countbitsu(unsigned int) __GPU_ONLY;
void __dp_d3d_device_memory_fence() __GPU_ONLY;
void __dp_d3d_device_memory_fence_with_tile_barrier() __GPU_ONLY;
int __dp_d3d_firstbithighi(int) __GPU_ONLY;
int __dp_d3d_firstbitlowi(int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_add(_Inout_ unsigned int *, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_and(_Inout_ unsigned int *, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_exchange(_Inout_ unsigned int *, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_or(_Inout_ unsigned int *, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_xor(_Inout_ unsigned int *, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_compare_exchange(_Inout_ unsigned int *, unsigned int, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_max_uint(_Inout_ unsigned int *, unsigned int) __GPU_ONLY;
int __dp_d3d_interlocked_max_int(_Inout_ int *, int) __GPU_ONLY;
unsigned int __dp_d3d_interlocked_min_uint(_Inout_ unsigned int *, unsigned int) __GPU_ONLY;
int __dp_d3d_interlocked_min_int(_Inout_ int *, int) __GPU_ONLY;
float __dp_d3d_madf(float, float, float) __GPU_ONLY;
double __dp_d3d_madd(double, double, double) __GPU_ONLY;
int __dp_d3d_madi(int, int, int) __GPU_ONLY;
int __dp_d3d_mini(int, int) __GPU_ONLY;
int __dp_d3d_maxi(int, int) __GPU_ONLY;
unsigned int __dp_d3d_minu(unsigned int, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_maxu(unsigned int, unsigned int) __GPU_ONLY;
unsigned int __dp_d3d_madu(unsigned int, unsigned int, unsigned int) __GPU_ONLY;
float __dp_d3d_noisef(float) __GPU_ONLY;
float __dp_d3d_radiansf(float) __GPU_ONLY;
float __dp_d3d_rcpf(float) __GPU_ONLY;
unsigned int __dp_d3d_reversebitsu(unsigned int) __GPU_ONLY;
float __dp_d3d_saturatef (float) __GPU_ONLY;
int __dp_d3d_signi(int) __GPU_ONLY;
float __dp_d3d_smoothstepf(float, float, float) __GPU_ONLY;
float __dp_d3d_stepf(float, float) __GPU_ONLY;
void __dp_d3d_tile_static_memory_fence() __GPU_ONLY;
void __dp_d3d_tile_static_memory_fence_with_tile_barrier() __GPU_ONLY;
void __dp_d3d_msad4(_Out_ unsigned int * /* pointer to the return value */,
                    unsigned int /* reference */,
                    unsigned int /* source.x */,
                    unsigned int /* source.y */,
                    unsigned int /* accum.x */,
                    unsigned int /* accum.y */,
                    unsigned int /* accum.z */,
                    unsigned int /* accum.w */) __GPU_ONLY;

//=============================================================================
// Intrinsics that serves as FE/BE interface
//=============================================================================

// C++ AMP stub only internal intrinsics
void __dp_stub_info(unsigned int /* x */,
                    unsigned int /* y */,
                    unsigned int /* z */,
                    unsigned int /* group forall? */) __GPU_ONLY;

_Ret_ void * __dp_get_buffer_info(bool /* read write? */,
                            unsigned int /* unique id */) __GPU_ONLY;

_Ret_ void * __dp_get_texture_info(bool /* read write? */,
                             unsigned int, /* rank */
                             unsigned int, /* base type: 0 - int, 1 - uint, 2 - float*/
                             unsigned int, /* num of channels */
                             unsigned int /* unique id */) __GPU_ONLY;

_Ret_ void * __dp_get_sampler_info(unsigned int /* unique id */) __GPU_ONLY;

void __dp_init_entry_var(_Out_ unsigned int * /* pointer to the entry symbol */,
                         unsigned int /* constant buffer id */,
                         unsigned int /* start pos */,
                         unsigned int /* end pos */) __GPU_ONLY;

void __dp_entry_var_ptr_info(unsigned int /* pos of a ptr */) __GPU_ONLY;

void __dp_const_buffer_info(unsigned int /* unique id */, unsigned int /* size */) __GPU_ONLY;

unsigned int __dp_read_const_buffer(unsigned int /* const buffer id */, unsigned int /* index */) __GPU_ONLY;

unsigned int __dp_get_physical_id(
        unsigned int /* 0 - gid, 1 - tid, 2 - dtid */,
        unsigned int /* 0 - x, 1 - y, 2 - z */) __GPU_ONLY;

// This intrinsic is used to aid line number debug info generation for C++ AMP kernels
void __dp_no_source_stub() __GPU_ONLY;

// This intrinsic is used to pass the call site info
void __dp_call_site_info(const char *, int) __GPU_ONLY;
}

namespace Concurrency
{
    namespace details
    {
        // This function is used to aid line number debug info generation for C++ AMP kernels
        // and is called by the C++ AMP FE in the generated kernel_stub for parallel_for_each calls.
        inline void __dp_no_source_func() __GPU_ONLY
        {
            __dp_no_source_stub();
        }
    }
}

namespace concurrency = Concurrency;

#pragma pack(pop)
