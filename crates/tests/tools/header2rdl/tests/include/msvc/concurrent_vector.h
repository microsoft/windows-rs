/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
* Microsoft would like to acknowledge that this concurrency data structure implementation
* is based on the Intel implementation of its Threading Building Blocks ("Intel Material").
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* concurrent_vector.h
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

/*
    Intel Material Copyright 2005-2008 Intel Corporation.  All Rights Reserved.
*/

#pragma once

#include <crtdefs.h>
#include <memory>
#include <iterator>
#include <limits>
#include <algorithm>
#include <cstring>
#include <crtdbg.h>
#include <concrt.h>

#define _PPL_CONTAINER

#if !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64))
    #error ERROR: Concurrency Runtime is supported only on X64, X86, ARM, and ARM64 architectures.
#endif  /* !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64)) */

#if defined (_M_CEE)
    #error ERROR: Concurrency Runtime is not supported when compiling /clr.
#endif  /* defined (_M_CEE) */

#pragma pack(push,_CRT_PACKING)
#pragma warning (push)
#pragma warning (disable: 4510 4512 4610)  // disable warnings for compiler unable to generate constructor
#pragma push_macro("new")
#undef new

/// <summary>
///     The <c>Concurrency</c> namespace provides classes and functions that give you access to the Concurrency Runtime,
///     a concurrent programming framework for C++. For more information, see <see cref="Concurrency Runtime"/>.
/// </summary>
/**/
namespace Concurrency
{

template<typename _Ty, class _Ax = ::std::allocator<_Ty>>
class concurrent_vector;

namespace details
{

    // Bad allocation marker.
    #define _BAD_ALLOC_MARKER reinterpret_cast<void*>(63)

    // Base class of concurrent vector implementation.
    class _Concurrent_vector_base_v4
    {
    protected:

        // Basic types declarations.
        typedef size_t _Segment_index_t;
        typedef size_t _Size_type;

        // Size constants
        static constexpr _Segment_index_t _Default_initial_segments = 1; // 2 initial items

        // Number of slots for segment's pointers inside the class
        static constexpr _Segment_index_t _Pointers_per_short_table = 3; // to fit into 8 words of entire structure
        static constexpr _Segment_index_t _Pointers_per_long_table = sizeof(_Segment_index_t) * 8; // one segment per bit

        // Segment pointer. Can be zero-initialized.
        struct _Segment_t
        {
            void* _My_array;
        };

        // Data fields

        // allocator function pointer
        void* (__cdecl *_My_vector_allocator_ptr)(_Concurrent_vector_base_v4 &, size_t);

        // embedded storage of segment pointers
        _Segment_t _My_storage[_Pointers_per_short_table];

        // Methods

        _Concurrent_vector_base_v4()
        {
            _My_early_size = 0;
            _My_first_block = 0; // here is not _Default_initial_segments
            _My_vector_allocator_ptr = nullptr;
            for( _Segment_index_t _I = 0; _I < _Pointers_per_short_table; _I++)
                _My_storage[_I]._My_array = nullptr;
            _My_segment = _My_storage;
        }
        _CONCRTIMP ~_Concurrent_vector_base_v4();

        _CONCRTIMP static _Segment_index_t __cdecl _Segment_index_of( _Size_type _Index );

        static _Segment_index_t _Segment_base( _Segment_index_t _K )
        {
            return (_Segment_index_t(1)<<_K & ~_Segment_index_t(1));
        }

        static _Segment_index_t _Segment_base_index_of( _Segment_index_t &_Index )
        {
            _Segment_index_t _K = _Segment_index_of( _Index );
            _Index -= _Segment_base(_K);
            return _K;
        }

        static _Size_type _Segment_size( _Segment_index_t _K )
        {
            return _Segment_index_t(1)<<_K; // fake value for _K==0
        }

        // An operation on an n-element array starting at begin.
        typedef void (__cdecl *_My_internal_array_op1)(void* _Begin, _Size_type _N );

        // An operation on n-element destination array and n-element source array.
        typedef void (__cdecl *_My_internal_array_op2)(void* _Dst, const void* _Src, _Size_type _N );

        // Internal structure for shrink_to_fit().
        struct _Internal_segments_table
        {
            _Segment_index_t _First_block;
            void* _Table[_Pointers_per_long_table];
        };

        _CONCRTIMP void _Internal_reserve( _Size_type _N, _Size_type _Element_size, _Size_type _Max_size );
        _CONCRTIMP _Size_type _Internal_capacity() const;
        void _Internal_grow( _Size_type _Start, _Size_type _Finish, _Size_type _Element_size, _My_internal_array_op2 _Init, const void *_Src );
        _Size_type _Internal_grow_segment( const _Size_type _Start, _Size_type _Finish, _Size_type _Element_size, _Segment_t** _PPSegment, _Size_type* _PSegStart, _Size_type* _PSegFinish );
        _CONCRTIMP _Size_type _Internal_grow_by( _Size_type _Delta, _Size_type _Element_size, _My_internal_array_op2 _Init, const void *_Src );
        _CONCRTIMP void* _Internal_push_back( _Size_type _Element_size, _Size_type& _Index );
        _CONCRTIMP _Segment_index_t _Internal_clear( _My_internal_array_op1 _Destroy );
        void _Internal_truncate( _Size_type _Old_size, _Size_type _New_size, _Size_type _Element_size, _My_internal_array_op1 _Destroy);
        _CONCRTIMP void* _Internal_compact( _Size_type _Element_size, void *_Table, _My_internal_array_op1 _Destroy, _My_internal_array_op2 _Copy );
        _CONCRTIMP void _Internal_copy( const _Concurrent_vector_base_v4& _Src, _Size_type _Element_size, _My_internal_array_op2 _Copy );
        _CONCRTIMP void _Internal_assign( const _Concurrent_vector_base_v4& _Src, _Size_type _Element_size,
                              _My_internal_array_op1 _Destroy, _My_internal_array_op2 _Assign, _My_internal_array_op2 _Copy );
        _CONCRTIMP void _Internal_throw_exception(_Size_type) const;
        _CONCRTIMP void _Internal_swap(_Concurrent_vector_base_v4&);

        _CONCRTIMP void _Internal_resize( _Size_type _New_size, _Size_type _Element_size, _Size_type _Max_size, _My_internal_array_op1 _Destroy, _My_internal_array_op2 _Init, const void* _Src);
        _CONCRTIMP _Size_type _Internal_grow_to_at_least_with_result( _Size_type _New_size, _Size_type _Element_size, _My_internal_array_op2 _Init, const void *_Src );

        // Count of segments in the first block.
        _Subatomic<_Size_type> _My_first_block;

        // Requested size of vector.
        _Subatomic<_Size_type> _My_early_size;

        // Pointer to the segments table.
        _Subatomic<_Segment_t*> _My_segment;


    private:
        // Private functionality.
        class _Helper;
        friend class _Helper;
    };

    typedef _Concurrent_vector_base_v4 _Concurrent_vector_base;

    // Meets requirements of a forward iterator for STL.*/
    /** _Value is either the _Ty or const _Ty type of the container. */
    template<typename _Container, typename _Value>
    class _Vector_iterator
    {
        // concurrent_vector over which we are iterating.
        _Container* _My_vector;

        // Index into the vector.
        size_t _My_index;

        // Caches _My_vector-&gt;_Internal_subscript(_My_index)
        /** NULL if cached value is not available */
        mutable _Value* _My_item;

        template<typename _C, typename _Ty>
        friend _Vector_iterator<_C,_Ty> operator+( ptrdiff_t _Offset, const _Vector_iterator<_C,_Ty>& _Vec );

        template<typename _C, typename _Ty, typename _U>
        friend bool operator==( const _Vector_iterator<_C,_Ty>&, const _Vector_iterator<_C,_U>& );

        template<typename _C, typename _Ty, typename _U>
        friend bool operator<( const _Vector_iterator<_C,_Ty>&, const _Vector_iterator<_C,_U>& );

        template<typename _C, typename _Ty, typename _U>
        friend ptrdiff_t operator-( const _Vector_iterator<_C,_Ty>&, const _Vector_iterator<_C,_U>& );

        template<typename _C, typename _U>
        friend class ::Concurrency::details::_Vector_iterator;

        template<typename _Ty, class _Ax>
        friend class ::Concurrency::concurrent_vector;

        _Vector_iterator( const _Container& _Vec, size_t _Index, void* _Ptr = nullptr )
            : _My_vector(const_cast<_Container*>(&_Vec)),
              _My_index(_Index),
              _My_item(static_cast<_Value*>(_Ptr))
        {
        }

    public:
        // Default constructor
        _Vector_iterator()
            : _My_vector(nullptr), _My_index(~size_t(0)), _My_item(nullptr)
        {
        }

        _Vector_iterator( const _Vector_iterator<_Container,typename _Container::value_type>& _Other )
            : _My_vector(_Other._My_vector),
              _My_index(_Other._My_index),
              _My_item(_Other._My_item)
        {
        }

        _Vector_iterator operator+( ptrdiff_t _Offset ) const
        {
            return _Vector_iterator( *_My_vector, _My_index+_Offset );
        }
        _Vector_iterator& operator+=( ptrdiff_t _Offset )
        {
            _My_index+=_Offset;
            _My_item = nullptr;
            return *this;
        }
        _Vector_iterator operator-( ptrdiff_t _Offset ) const
        {
            return _Vector_iterator( *_My_vector, _My_index-_Offset );
        }
        _Vector_iterator& operator-=( ptrdiff_t _Offset )
        {
            _My_index-=_Offset;
            _My_item = nullptr;
            return *this;
        }
        _Value& operator*() const
        {
            _Value* _Item = _My_item;
            if( !_Item )
                _Item = _My_item = &_My_vector->_Internal_subscript(_My_index);
            _CONCRT_ASSERT( _Item==&_My_vector->_Internal_subscript(_My_index));  // corrupt cache
            return *_Item;
        }
        _Value& operator[]( ptrdiff_t _K ) const
        {
            return _My_vector->_Internal_subscript(_My_index+_K);
        }
        _Value* operator->() const
        {
            return &operator*();
        }

        // Pre increment
        _Vector_iterator& operator++()
        {
            size_t _K = ++_My_index;
            if( _My_item )
            {
                // Following test uses 2's-complement wizardry.
                if( (_K& (_K-2))==0 )
                {
                    // _K is a power of two that is at least _K-2.
                    _My_item= nullptr;
                }
                else
                {
                    ++_My_item;
                }
            }
            return *this;
        }

        // Pre decrement
        _Vector_iterator& operator--()
        {
            _CONCRT_ASSERT( _My_index>0 ); // operator--() applied to iterator already at beginning of concurrent_vector.
            size_t _K = _My_index--;
            if( _My_item )
            {
                // Following test uses 2's-complement wizardry.
                if( (_K& (_K-2))==0 )
                {
                    // k is a power of two that is at least k-2.
                    _My_item= nullptr;
                }
                else
                {
                    --_My_item;
                }
            }
            return *this;
        }

        // Post increment
        _Vector_iterator operator++(int)
        {
            _Vector_iterator _Result = *this;
            operator++();
            return _Result;
        }

        // Post decrement
        _Vector_iterator operator--(int)
        {
            _Vector_iterator _Result = *this;
            operator--();
            return _Result;
        }

        // STL support

        typedef ptrdiff_t difference_type;
        typedef _Value value_type;
        typedef _Value* pointer;
        typedef _Value& reference;
        typedef ::std::random_access_iterator_tag iterator_category;
    };

    template<typename _Container, typename _Ty>
    _Vector_iterator<_Container,_Ty> operator+( ptrdiff_t _Offset, const _Vector_iterator<_Container,_Ty>& _Vec )
    {
        return _Vector_iterator<_Container,_Ty>( *_Vec._My_vector, _Vec._My_index+_Offset );
    }

    template<typename _Container, typename _Ty, typename _U>
    bool operator==( const _Vector_iterator<_Container,_Ty>& _I, const _Vector_iterator<_Container,_U>& _J )
    {
        return _I._My_index==_J._My_index && _I._My_vector == _J._My_vector;
    }

    template<typename _Container, typename _Ty, typename _U>
    bool operator!=( const _Vector_iterator<_Container,_Ty>& _I, const _Vector_iterator<_Container,_U>& _J )
    {
        return !(_I==_J);
    }

    template<typename _Container, typename _Ty, typename _U>
    bool operator<( const _Vector_iterator<_Container,_Ty>& _I, const _Vector_iterator<_Container,_U>& _J )
    {
        return _I._My_index<_J._My_index && _I._My_vector == _J._My_vector;
    }

    template<typename _Container, typename _Ty, typename _U>
    bool operator>( const _Vector_iterator<_Container,_Ty>& _I, const _Vector_iterator<_Container,_U>& _J )
    {
        return _J<_I;
    }

    template<typename _Container, typename _Ty, typename _U>
    bool operator>=( const _Vector_iterator<_Container,_Ty>& _I, const _Vector_iterator<_Container,_U>& _J )
    {
        return !(_I<_J);
    }

    template<typename _Container, typename _Ty, typename _U>
    bool operator<=( const _Vector_iterator<_Container,_Ty>& _I, const _Vector_iterator<_Container,_U>& _J )
    {
        return !(_J<_I);
    }

    template<typename _Container, typename _Ty, typename _U>
    ptrdiff_t operator-( const _Vector_iterator<_Container,_Ty>& _I, const _Vector_iterator<_Container,_U>& _J )
    {
        return ptrdiff_t(_I._My_index)-ptrdiff_t(_J._My_index);
    }

    template<typename _Ty, class _Ax>
    class _Allocator_base
    {
    public:
        typedef typename ::std::allocator_traits<_Ax>::template
            rebind_alloc<_Ty> _Allocator_type;
        using _Allocator_traits = ::std::allocator_traits<_Allocator_type>;

        _Allocator_type _My_allocator;

        _Allocator_base()
            : _My_allocator()
        {
        }

        _Allocator_base(const _Allocator_type &_Al)
            : _My_allocator(_Al)
        {
        }
    };

} // namespace details

/// <summary>
///     The <c>concurrent_vector</c> class is a sequence container class that allows random access to any element.
///     It enables concurrency-safe append, element access, iterator access, and iterator traversal operations.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements to be stored in the vector.
/// </typeparam>
/// <typeparam name="_Ax">
///     The type that represents the stored allocator object that encapsulates details about the allocation and
///     deallocation of memory for the concurrent vector. This argument is optional and the default value is
///     <c>allocator&lt;</c><typeparamref name="_Ty"/><c>&gt;</c>.
/// </typeparam>
/// <remarks>
///     For detailed information on the <c>concurrent_vector</c> class, see <see cref="Parallel Containers and Objects"/>.
/// </remarks>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class _Ax>
class concurrent_vector: protected details::_Allocator_base<_Ty, _Ax>,
                         private details::_Concurrent_vector_base_v4
{
private:
    typedef concurrent_vector<_Ty, _Ax> _Myt;
    using typename details::_Allocator_base<_Ty, _Ax>::_Allocator_traits;

    template<typename _C, typename _U>
    friend class details::_Vector_iterator;

public:

    /// <summary>
    ///     A type that counts the number of elements in a concurrent vector.
    /// </summary>
    /**/
    typedef details::_Concurrent_vector_base_v4::_Size_type size_type;

    /// <summary>
    ///     A type that represents the allocator class for the concurrent vector.
    /// </summary>
    /**/
    typedef typename details::_Allocator_base<_Ty, _Ax>::_Allocator_type allocator_type;

    /// <summary>
    ///     A type that represents the data type stored in a concurrent vector.
    /// </summary>
    /**/
    typedef _Ty value_type;

    /// <summary>
    /// A type that provides the signed distance between two elements in a concurrent vector.
    /// </summary>
    /**/
    typedef ptrdiff_t difference_type;

    /// <summary>
    ///     A type that provides a reference to an element stored in a concurrent vector.
    /// </summary>
    /**/
    typedef _Ty& reference;

    /// <summary>
    ///     A type that provides a reference to a <c>const</c> element stored in a concurrent vector for reading and
    ///     performing <c>const</c> operations.
    /// </summary>
    /**/
    typedef const _Ty& const_reference;

    /// <summary>
    ///     A type that provides a pointer to an element in a concurrent vector.
    /// </summary>
    /**/
    typedef _Ty *pointer;

    /// <summary>
    ///     A type that provides a pointer to a <c>const</c> element in a concurrent vector.
    /// </summary>
    /**/
    typedef const _Ty *const_pointer;

    /// <summary>
    ///     A type that provides a random-access iterator that can read any element in a concurrent vector. Modification of an
    ///     element using the iterator is not concurrency-safe.
    /// </summary>
    /**/
    typedef details::_Vector_iterator<concurrent_vector,_Ty> iterator;

    /// <summary>
    ///     A type that provides a random-access iterator that can read a <c>const</c> element in a concurrent vector.
    /// </summary>
    /**/
    typedef details::_Vector_iterator<concurrent_vector,const _Ty> const_iterator;

    /// <summary>
    ///     A type that provides a random-access iterator that can read any element in a reversed concurrent vector. Modification of an
    ///     element using the iterator is not concurrency-safe.
    /// </summary>
    /**/
    typedef ::std::reverse_iterator<iterator> reverse_iterator;

    /// <summary>
    ///     A type that provides a random-access iterator that can read any <c>const</c> element in the concurrent vector.
    /// </summary>
    /**/
    typedef ::std::reverse_iterator<const_iterator> const_reverse_iterator;

    /// <summary>
    ///     Constructs a concurrent vector.
    /// </summary>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the vector.
    ///     <para>The first constructor specify an empty initial vector and explicitly specifies the allocator type.
    ///     to be used.</para>
    ///     <para>The second and third constructors specify a copy of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fourth constructor specifies a move of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fifth constructor specifies a repetition of a specified number (<paramref name="_N"/>) of elements of the default
    ///     value for class <typeparamref name="_Ty"/>.</para>
    ///     <para>The sixth constructor specifies a repetition of (<paramref name="_N"/>) elements of value <paramref name="_Item"/>.</para>
    ///     <para>The last constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    explicit concurrent_vector(const allocator_type &_Al = allocator_type())
        : details::_Allocator_base<_Ty, _Ax>(_Al)
    {
        _My_vector_allocator_ptr = &_Internal_allocator;
    }

    /// <summary>
    ///     Constructs a concurrent vector.
    /// </summary>
    /// <param name="_Vector">
    ///     The source <c>concurrent_vector</c> object to copy or move elements from.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the vector.
    ///     <para>The first constructor specify an empty initial vector and explicitly specifies the allocator type.
    ///     to be used.</para>
    ///     <para>The second and third constructors specify a copy of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fourth constructor specifies a move of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fifth constructor specifies a repetition of a specified number (<paramref name="_N"/>) of elements of the default
    ///     value for class <typeparamref name="_Ty"/>.</para>
    ///     <para>The sixth constructor specifies a repetition of (<paramref name="_N"/>) elements of value <paramref name="_Item"/>.</para>
    ///     <para>The last constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    concurrent_vector( const concurrent_vector& _Vector)
        : details::_Allocator_base<_Ty, _Ax>(_Vector.get_allocator())
    {
        _My_vector_allocator_ptr = &_Internal_allocator;
        _Internal_copy(_Vector, sizeof(_Ty), &_Copy_array);
    }

    /// <summary>
    ///     Constructs a concurrent vector.
    /// </summary>
    /// <typeparam name="M">
    ///     The allocator type of the source vector.
    /// </typeparam>
    /// <param name="_Vector">
    ///     The source <c>concurrent_vector</c> object to copy or move elements from.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the vector.
    ///     <para>The first constructor specify an empty initial vector and explicitly specifies the allocator type.
    ///     to be used.</para>
    ///     <para>The second and third constructors specify a copy of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fourth constructor specifies a move of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fifth constructor specifies a repetition of a specified number (<paramref name="_N"/>) of elements of the default
    ///     value for class <typeparamref name="_Ty"/>.</para>
    ///     <para>The sixth constructor specifies a repetition of (<paramref name="_N"/>) elements of value <paramref name="_Item"/>.</para>
    ///     <para>The last constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    template<class M>
    concurrent_vector( const concurrent_vector<_Ty, M>& _Vector, const allocator_type& _Al = allocator_type() )
        : details::_Allocator_base<_Ty, _Ax>(_Al)
    {
        _My_vector_allocator_ptr = &_Internal_allocator;
        _Internal_copy(_Vector._Internal_vector_base(), sizeof(_Ty), &_Copy_array);
    }


    /// <summary>
    ///     Constructs a concurrent vector.
    /// </summary>
    /// <param name="_Vector">
    ///     The source <c>concurrent_vector</c> object to copy or move elements from.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the vector.
    ///     <para>The first constructor specify an empty initial vector and explicitly specifies the allocator type.
    ///     to be used.</para>
    ///     <para>The second and third constructors specify a copy of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fourth constructor specifies a move of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fifth constructor specifies a repetition of a specified number (<paramref name="_N"/>) of elements of the default
    ///     value for class <typeparamref name="_Ty"/>.</para>
    ///     <para>The sixth constructor specifies a repetition of (<paramref name="_N"/>) elements of value <paramref name="_Item"/>.</para>
    ///     <para>The last constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    concurrent_vector( concurrent_vector && _Vector)
        : details::_Allocator_base<_Ty, _Ax>(_Vector.get_allocator())
    {
        _My_vector_allocator_ptr = &_Internal_allocator;
        _Concurrent_vector_base_v4::_Internal_swap(_Vector._Internal_vector_base());
    }

    /// <summary>
    ///     Constructs a concurrent vector.
    /// </summary>
    /// <param name="_N">
    ///     The initial size of the <c>concurrent_vector</c> object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the vector.
    ///     <para>The first constructor specify an empty initial vector and explicitly specifies the allocator type.
    ///     to be used.</para>
    ///     <para>The second and third constructors specify a copy of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fourth constructor specifies a move of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fifth constructor specifies a repetition of a specified number (<paramref name="_N"/>) of elements of the default
    ///     value for class <typeparamref name="_Ty"/>.</para>
    ///     <para>The sixth constructor specifies a repetition of (<paramref name="_N"/>) elements of value <paramref name="_Item"/>.</para>
    ///     <para>The last constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    explicit concurrent_vector(size_type _N)
    {
        _My_vector_allocator_ptr = &_Internal_allocator;
        if ( !_N ) return;
        _Internal_reserve(_N, sizeof(_Ty), max_size()); _My_early_size = _N;
        _CONCRT_ASSERT( _My_first_block == _Segment_index_of(_N-1)+1 );
        _Initialize_array(static_cast<_Ty*>(_My_segment[0]._My_array), nullptr, _N);
    }

    /// <summary>
    ///     Constructs a concurrent vector.
    /// </summary>
    /// <param name="_N">
    ///     The initial capacity of the <c>concurrent_vector</c> object.
    /// </param>
    /// <param name="_Item">
    ///     The value of elements in the constructed object.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the vector.
    ///     <para>The first constructor specify an empty initial vector and explicitly specifies the allocator type.
    ///     to be used.</para>
    ///     <para>The second and third constructors specify a copy of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fourth constructor specifies a move of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fifth constructor specifies a repetition of a specified number (<paramref name="_N"/>) of elements of the default
    ///     value for class <typeparamref name="_Ty"/>.</para>
    ///     <para>The sixth constructor specifies a repetition of (<paramref name="_N"/>) elements of value <paramref name="_Item"/>.</para>
    ///     <para>The last constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    concurrent_vector(size_type _N, const_reference _Item, const allocator_type& _Al = allocator_type())
        : details::_Allocator_base<_Ty, _Ax>(_Al)
    {
        _My_vector_allocator_ptr = &_Internal_allocator;
        _Internal_assign( _N, _Item );
    }

    /// <summary>
    ///     Constructs a concurrent vector.
    /// </summary>
    /// <typeparam name="_InputIterator">
    ///     The type of the input iterator.
    /// </typeparam>
    /// <param name="_Begin">
    ///     Position of the first element in the range of elements to be copied.
    /// </param>
    /// <param name="_End">
    ///     Position of the first element beyond the range of elements to be copied.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the vector.
    ///     <para>The first constructor specify an empty initial vector and explicitly specifies the allocator type.
    ///     to be used.</para>
    ///     <para>The second and third constructors specify a copy of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fourth constructor specifies a move of the concurrent vector <paramref name="_Vector"/>.</para>
    ///     <para>The fifth constructor specifies a repetition of a specified number (<paramref name="_N"/>) of elements of the default
    ///     value for class <typeparamref name="_Ty"/>.</para>
    ///     <para>The sixth constructor specifies a repetition of (<paramref name="_N"/>) elements of value <paramref name="_Item"/>.</para>
    ///     <para>The last constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    template<class _InputIterator>
    concurrent_vector(_InputIterator _Begin, _InputIterator _End, const allocator_type &_Al = allocator_type())
        : details::_Allocator_base<_Ty, _Ax>(_Al)
    {
        _My_vector_allocator_ptr = &_Internal_allocator;
        _Internal_assign(_Begin, _End, static_cast<_Is_integer_tag<::std::numeric_limits<_InputIterator>::is_integer> *>(0) );
    }

    /// <summary>
    ///     Assigns the contents of another <c>concurrent_vector</c> object to this one. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Vector">
    ///     The source <c>concurrent_vector</c> object.
    /// </param>
    /// <returns>
    ///     A reference to this <c>concurrent_vector</c> object.
    /// </returns>
    /**/
    concurrent_vector& operator=( const concurrent_vector& _Vector )
    {
        if( this != &_Vector )
            _Concurrent_vector_base_v4::_Internal_assign(_Vector, sizeof(_Ty), &_Destroy_array, &_Assign_array, &_Copy_array);
        return *this;
    }

    /// <summary>
    ///     Assigns the contents of another <c>concurrent_vector</c> object to this one. This method is not concurrency-safe.
    /// </summary>
    /// <typeparam name="M">
    ///     The allocator type of the source vector.
    /// </typeparam>
    /// <param name="_Vector">
    ///     The source <c>concurrent_vector</c> object.
    /// </param>
    /// <returns>
    ///     A reference to this <c>concurrent_vector</c> object.
    /// </returns>
    /**/
    template<class M>
    concurrent_vector& operator=( const concurrent_vector<_Ty, M>& _Vector )
    {
        if( static_cast<void*>( this ) != static_cast<const void*>( &_Vector ) )
        {
            _Concurrent_vector_base_v4::_Internal_assign(_Vector._Internal_vector_base(),
                                                         sizeof(_Ty), &_Destroy_array, &_Assign_array, &_Copy_array);
        }
        return *this;
    }

    /// <summary>
    ///     Assigns the contents of another <c>concurrent_vector</c> object to this one. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Vector">
    ///     The source <c>concurrent_vector</c> object.
    /// </param>
    /// <returns>
    ///     A reference to this <c>concurrent_vector</c> object.
    /// </returns>
    /**/
    concurrent_vector& operator=( concurrent_vector && _Vector )
    {
        if( static_cast<void*>( this ) != static_cast<const void*>( &_Vector ) )
        {
            _Concurrent_vector_base_v4::_Internal_swap(_Vector._Internal_vector_base());
            _Vector.clear();
        }
        return *this;
    }

    /// <summary>
    ///     Grows this concurrent vector by <paramref name="_Delta"/> elements. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Delta">
    ///     The number of elements to append to the object.
    /// </param>
    /// <returns>
    ///     An iterator to first item appended.
    /// </returns>
    /// <remarks>
    ///     If <paramref name="_Item"/> is not specified, the new elements are default constructed.
    /// </remarks>
    /**/
    iterator grow_by( size_type _Delta )
    {
        return iterator(*this, _Delta ? _Internal_grow_by( _Delta, sizeof(_Ty), &_Initialize_array, nullptr ) : _My_early_size);
    }

    /// <summary>
    ///     Grows this concurrent vector by <paramref name="_Delta"/> elements. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Delta">
    ///     The number of elements to append to the object.
    /// </param>
    /// <param name="_Item">
    ///     The value to initialize the new elements with.
    /// </param>
    /// <returns>
    ///     An iterator to first item appended.
    /// </returns>
    /// <remarks>
    ///     If <paramref name="_Item"/> is not specified, the new elements are default constructed.
    /// </remarks>
    /**/
    iterator grow_by( size_type _Delta, const_reference _Item )
    {
        return iterator(*this, _Delta ? _Internal_grow_by( _Delta, sizeof(_Ty), &_Initialize_array_by, static_cast<const void*>(&_Item) ) : _My_early_size);
    }

    /// <summary>
    ///     Grows this concurrent vector until it has at least <paramref name="_N"/> elements. This method is concurrency-safe.
    /// </summary>
    /// <param name="_N">
    ///     The new minimum size for the <c>concurrent_vector</c> object.
    /// </param>
    /// <returns>
    ///     An iterator that points to beginning of appended sequence, or to the element at index <paramref name="_N"/> if no
    ///     elements were appended.
    /// </returns>
    /**/
    iterator grow_to_at_least( size_type _N )
    {
        size_type _M = 0;
        if( _N )
        {
            _M = _Internal_grow_to_at_least_with_result( _N, sizeof(_Ty), &_Initialize_array, nullptr );
            if( _M > _N )
                _M = _N;
        }
        return iterator(*this, _M);
    };

    /// <summary>
    ///     Appends the given item to the end of the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Item">
    ///     The value to be appended.
    /// </param>
    /// <returns>
    ///     An iterator to item appended.
    /// </returns>
    /**/
    iterator push_back( const_reference _Item )
    {
        size_type _K;
        void *_Ptr = _Internal_push_back(sizeof(_Ty), _K);
        _Internal_loop_guide _Loop(1, _Ptr);
        _Loop._Init(&_Item);
        return iterator(*this, _K, _Ptr);
    }

    /// <summary>
    ///     Appends the given item to the end of the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Item">
    ///     The value to be appended.
    /// </param>
    /// <returns>
    ///     An iterator to item appended.
    /// </returns>
    /**/
    iterator push_back( _Ty &&_Item )
    {
        size_type _K;
        void *_Ptr = _Internal_push_back(sizeof(_Ty), _K);
        new (_Ptr) _Ty( ::std::move(_Item));
        return iterator(*this, _K, _Ptr);
    }

    /// <summary>
    ///     Provides access to the element at the given index in the concurrent vector. This method is concurrency-safe for read operations,
    ///     and also while growing the vector, as long as the you have ensured that the value <paramref name="_Index"/> is less than
    ///     the size of the concurrent vector.
    /// </summary>
    /// <param name="_Index">
    ///     The index of the element to be retrieved.
    /// </param>
    /// <returns>
    ///     A reference to the item at the given index.
    /// </returns>
    /// <remarks>
    ///     The version of <c>operator []</c> that returns a non-<c>const</c> reference cannot be used to concurrently write to the element
    ///     from different threads. A different synchronization object should be used to synchronize concurrent read and write operations
    ///     to the same data element.
    ///     <para>No bounds checking is performed to ensure that <paramref name="_Index"/> is a valid index into the concurrent vector.</para>
    /// </remarks>
    /**/
    reference operator[]( size_type _Index )
    {
        return _Internal_subscript(_Index);
    }

    /// <summary>
    ///     Provides read access to element at the given index in the concurrent vector. This method is concurrency-safe for read operations,
    ///     and also while growing the vector, as long as the you have ensured that the value <paramref name="_Index"/> is less than
    ///     the size of the concurrent vector.
    /// </summary>
    /// <param name="_Index">
    ///     The index of the element to be retrieved.
    /// </param>
    /// <returns>
    ///     A <c>const</c> reference to the item at the given index.
    /// </returns>
    /// <remarks>
    ///     The version of <c>operator []</c> that returns a non-<c>const</c> reference cannot be used to concurrently write to the element
    ///     from different threads. A different synchronization object should be used to synchronize concurrent read and write operations
    ///     to the same data element.
    ///     <para>No bounds checking is performed to ensure that <paramref name="_Index"/> is a valid index into the concurrent vector.</para>
    /// </remarks>
    /**/
    const_reference operator[]( size_type _Index ) const
    {
        return _Internal_subscript(_Index);
    }

    /// <summary>
    ///     Provides access to the element at the given index in the concurrent vector. This method is concurrency-safe for read operations,
    ///     and also while growing the vector, as long as you have ensured that the value <paramref name="_Index"/> is less than
    ///     the size of the concurrent vector.
    /// </summary>
    /// <param name="_Index">
    ///     The index of the element to be retrieved.
    /// </param>
    /// <returns>
    ///     A reference to the item at the given index.
    /// </returns>
    /// <remarks>
    ///     The version of the function <c>at</c> that returns a non-<c>const</c> reference cannot be used to concurrently write to the element
    ///     from different threads. A different synchronization object should be used to synchronize concurrent read and write operations
    ///     to the same data element.
    ///     <para>The method throws <c>out_of_range</c> if <paramref name="_Index"/> is greater than or equal to the size of the concurrent vector,
    ///     and <c>range_error</c> if the index is for a broken portion of the vector. For details on how a vector can become broken,
    ///     see <see cref="Parallel Containers and Objects"/>.</para>
    /// </remarks>
    /**/
    reference at( size_type _Index )
    {
        return _Internal_subscript_with_exceptions(_Index);
    }

    /// <summary>
    ///     Provides access to the element at the given index in the concurrent vector. This method is concurrency-safe for read operations,
    ///     and also while growing the vector, as long as you have ensured that the value <paramref name="_Index"/> is less than
    ///     the size of the concurrent vector.
    /// </summary>
    /// <param name="_Index">
    ///     The index of the element to be retrieved.
    /// </param>
    /// <returns>
    ///     A <c>const</c> reference to the item at the given index.
    /// </returns>
    /// <remarks>
    ///     The version of the function <c>at</c> that returns a non-<c>const</c> reference cannot be used to concurrently write to the element
    ///     from different threads. A different synchronization object should be used to synchronize concurrent read and write operations
    ///     to the same data element.
    ///     <para>The method throws <c>out_of_range</c> if <paramref name="_Index"/> is greater than or equal to the size of the concurrent vector,
    ///     and <c>range_error</c> if the index is for a broken portion of the vector. For details on how a vector can become broken,
    ///     see <see cref="Parallel Containers and Objects"/>.</para>
    /// </remarks>
    /**/
    const_reference at( size_type _Index ) const
    {
        return _Internal_subscript_with_exceptions(_Index);
    }

    /// <summary>
    ///     Returns the number of elements in the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     The number of elements in this <c>concurrent_vector</c> object.
    /// </returns>
    /// <remarks>
    ///     The returned size is guaranteed to include all elements appended by calls to the function <c>push_back</c>,
    ///     or grow operations that have completed prior to invoking this method. However, it may also include elements
    ///     that are allocated but still under construction by concurrent calls to any of the growth methods.
    /// </remarks>
    /**/
    size_type size() const
    {
        size_type _Sz = _My_early_size;
        size_type _Cp = _Internal_capacity();
        return _Cp < _Sz ? _Cp : _Sz;
    }

    /// <summary>
    ///     Tests if the concurrent vector is empty at the time this method is called. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the vector was empty at the moment the function was called, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool empty() const
    {
        return !_My_early_size;
    }

    /// <summary>
    ///     Returns the maximum size to which the concurrent vector can grow without having to allocate more memory.
    ///     This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     The maximum size to which the concurrent vector can grow without having to allocate more memory.
    /// </returns>
    /// <remarks>
    ///     Unlike an STL <c>vector</c>, a <c>concurrent_vector</c> object does not move existing elements if it allocates more memory.
    /// </remarks>
    /**/
    size_type capacity() const
    {
        return _Internal_capacity();
    }

    /// <summary>
    ///     Allocates enough space to grow the concurrent vector to size <paramref name="_N"/> without having to allocate more memory later.
    ///     This method is not concurrency-safe.
    /// </summary>
    /// <param name="_N">
    ///     The number of elements to reserve space for.
    /// </param>
    /// <remarks>
    ///     <c>reserve</c> is not concurrency-safe. You must ensure that no other threads are invoking methods
    ///     on the concurrent vector when you call this method. The capacity of the concurrent vector after the method returns
    ///     may be bigger than the requested reservation.
    /// </remarks>
    /**/
    void reserve( size_type _N )
    {
        if( _N )
            _Internal_reserve(_N, sizeof(_Ty), max_size());
    }

    /// <summary>
    ///     Compacts the internal representation of the concurrent vector to reduce fragmentation and optimize memory usage.
    ///     This method is not concurrency-safe.
    /// </summary>
    /// <remarks>
    ///     This method will internally re-allocate memory move elements around, invalidating all the iterators.
    ///     <c>shrink_to_fit</c> is not concurrency-safe. You must ensure that no other threads are invoking methods
    ///     on the concurrent vector when you call this function.
    /// </remarks>
    /**/
    void shrink_to_fit();

    /// <summary>
    ///     Changes the size of the concurrent vector to the requested size, deleting or adding elements as
    ///     necessary. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_N">
    ///     The new size of the concurrent vector.
    /// </param>
    /// <remarks>
    ///     If the size of the container is less than the requested size, elements are added to the vector until it reaches the
    ///     requested size. If the size of the container is larger than the requested size, the elements closest to the end of the container
    ///     are deleted until the container reaches the size <paramref name="_N"/>. If the present size of the container is the same as the requested
    ///     size, no action is taken.
    ///     <para><c>resize</c> is not concurrency safe.  You must ensure that no other threads are invoking methods
    ///     on the concurrent vector when you call this method.</para>
    /// </remarks>
    /**/
    void resize(size_type _N)
    {
        _Internal_resize( _N, sizeof(_Ty), max_size(), _Destroy_array, _Initialize_array, nullptr);
    }

    /// <summary>
    ///     Changes the size of the concurrent vector to the requested size, deleting or adding elements as
    ///     necessary. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_N">
    ///     The new size of the concurrent_vector.
    /// </param>
    /// <param name="_Val">
    ///     The value of new elements added to the vector if the new size is larger than the original size. If the value is omitted,
    ///     the new objects are assigned the default value for their type.
    /// </param>
    /// <remarks>
    ///     If the size of the container is less than the requested size, elements are added to the vector until it reaches the
    ///     requested size. If the size of the container is larger than the requested size, the elements closest to the end of the container
    ///     are deleted until the container reaches the size <paramref name="_N"/>. If the present size of the container is the same as the requested
    ///     size, no action is taken.
    ///     <para><c>resize</c> is not concurrency safe.  You must ensure that no other threads are invoking methods
    ///     on the concurrent vector when you call this method.</para>
    /// </remarks>
    /**/
    void resize(size_type _N, const _Ty& _Val)
    {
        _Internal_resize( _N, sizeof(_Ty), max_size(), _Destroy_array, _Initialize_array_by, static_cast<const void*>(&_Val) );
    }

    /// <summary>
    ///     Returns the maximum number of elements the concurrent vector can hold. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     The maximum number of elements the <c>concurrent_vector</c> object can hold.
    /// </returns>
    /**/
    size_type max_size() const
    {
        return (~size_type(0))/sizeof(_Ty);
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the beginning of
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the beginning of
    ///     the concurrent vector.
    /// </returns>
    /**/
    iterator begin()
    {
        return iterator(*this,0);
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the end of
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the end of
    ///     the concurrent vector.
    /// </returns>
    /**/
    iterator end()
    {
        return iterator(*this,size());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the beginning of
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the beginning of
    ///     the concurrent vector.
    /// </returns>
    /**/
    const_iterator begin() const
    {
        return const_iterator(*this,0);
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the end of
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the end of
    ///     the concurrent vector.
    /// </returns>
    /**/
    const_iterator end() const
    {
        return const_iterator(*this,size());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the beginning of
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the beginning of
    ///     the concurrent vector.
    /// </returns>
    /**/
    reverse_iterator rbegin()
    {
        return reverse_iterator(end());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the end of
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the end of
    ///     the concurrent vector.
    /// </returns>
    /**/
    reverse_iterator rend()
    {
        return reverse_iterator(begin());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the beginning
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the beginning of
    ///     the concurrent vector.
    /// </returns>
    /**/
    const_reverse_iterator rbegin() const
    {
        return const_reverse_iterator(end());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the end of
    ///     the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="reverse_iterator"/> or <typeparamref name="const_reverse_iterator"/> to the end of
    ///     the concurrent vector.
    /// </returns>
    /**/
    const_reverse_iterator rend() const
    {
        return const_reverse_iterator(begin());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="const_iterator"/> to the beginning of the concurrent vector.
    ///     This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="const_iterator"/> to the beginning of the concurrent vector.
    /// </returns>
    /**/
    const_iterator cbegin() const
    {
        return (((const _Myt *)this)->begin());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="const_iterator"/> to the end of the concurrent vector.
    ///     This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="const_iterator"/> to the end of the concurrent vector.
    /// </returns>
    /**/
    const_iterator cend() const
    {
        return (((const _Myt *)this)->end());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="const_reverse_iterator"/> to the beginning of the concurrent vector.
    ///     This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="const_reverse_iterator"/> to the beginning of the concurrent vector.
    /// </returns>
    /**/
    const_reverse_iterator crbegin() const
    {
        return (((const _Myt *)this)->rbegin());
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="const_reverse_iterator"/> to the end of the concurrent vector.
    ///     This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="const_reverse_iterator"/> to the end of the concurrent vector.
    /// </returns>
    /**/
    const_reverse_iterator crend() const
    {
        return (((const _Myt *)this)->rend());
    }

    /// <summary>
    ///     Returns a reference or a <c>const</c> reference to the first element in the concurrent vector. If the
    ///     concurrent vector is empty, the return value is undefined.  This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     A reference or a <c>const</c> reference to the first element in the concurrent vector.
    /// </returns>
    /**/
    reference front()
    {
        _CONCRT_ASSERT( size()>0 );
        return static_cast<_Ty*>(_My_segment[0]._My_array)[0];
    }

    /// <summary>
    ///     Returns a reference or a <c>const</c> reference to the first element in the concurrent vector. If the
    ///     concurrent vector is empty, the return value is undefined. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     A reference or a <c>const</c> reference to the first element in the <c>concurrent_vector</c> object.
    /// </returns>
    /**/
    const_reference front() const
    {
        _CONCRT_ASSERT( size()>0 );
        return static_cast<_Ty*>(_My_segment[0]._My_array)[0];
    }

    /// <summary>
    ///     Returns a reference or a <c>const</c> reference to the last element in the concurrent vector. If the
    ///     concurrent vector is empty, the return value is undefined. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     A reference or a <c>const</c> reference to the last element in the concurrent vector.
    /// </returns>
    /**/
    reference back()
    {
        _Size_type sz = size();
        _CONCRT_ASSERT( sz > 0 );
        return _Internal_subscript( sz-1 );
    }

    /// <summary>
    ///     Returns a reference or a <c>const</c> reference to the last element in the concurrent_vector. If the
    ///     concurrent vector is empty, the return value is undefined. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     A reference or a <c>const</c> reference to the last element in the concurrent vector.
    /// </returns>
    /**/
    const_reference back() const
    {
        _Size_type sz = size();
        _CONCRT_ASSERT( sz > 0 );
        return _Internal_subscript( sz-1 );
    }

    /// <summary>
    ///     Returns a copy of the allocator used to construct the concurrent vector. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     A copy of the allocator used to construct the <c>concurrent_vector</c> object.
    /// </returns>
    /**/
    allocator_type get_allocator() const
    {
        return this->_My_allocator;
    }

    /// <summary>
    ///     Erases the elements of the concurrent vector and assigns to it either <paramref name="_N"/> copies of
    ///     <paramref name="_Item"/>, or values specified by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).
    ///     This method is not concurrency-safe.
    /// </summary>
    /// <param name="_N">
    ///     The number of items to copy into the concurrent vector.
    /// </param>
    /// <param name="_Item">
    ///     Reference to a value used to fill the concurrent vector.
    /// </param>
    /// <remarks>
    ///     <c>assign</c> is not concurrency-safe. You must ensure that no other threads are invoking methods
    ///     on the concurrent vector when you call this method.
    /// </remarks>
    /**/
    void assign(size_type _N, const_reference _Item)
    {
        clear();
        _Internal_assign( _N, _Item );
    }

    /// <summary>
    ///     Erases the elements of the concurrent vector and assigns to it either <paramref name="_N"/> copies of <paramref name="_Item"/>,
    ///     or values specified by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>).
    ///     This method is not concurrency-safe.
    /// </summary>
    /// <typeparam name="_InputIterator">
    ///     The type of the specified iterator.
    /// </typeparam>
    /// <param name="_Begin">
    ///     An iterator to the first element of the source range.
    /// </param>
    /// <param name="_End">
    ///     An iterator to one past the last element of the source range.
    /// </param>
    /// <remarks>
    ///     <c>assign</c> is not concurrency-safe. You must ensure that no other threads are invoking methods
    ///     on the concurrent vector when you call this method.
    /// </remarks>
    /**/
    template<class _InputIterator>
    void assign(_InputIterator _Begin, _InputIterator _End)
    {
        clear();
        _Internal_assign( _Begin, _End, static_cast<_Is_integer_tag<::std::numeric_limits<_InputIterator>::is_integer> *>(0) );
    }

    /// <summary>
    ///     Swaps the contents of two concurrent vectors. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Vector">
    ///     The <c>concurrent_vector</c> object to swap contents with.
    /// </param>
    /**/
    void swap(concurrent_vector &_Vector)
    {
        if( this != &_Vector )
        {
            _Concurrent_vector_base_v4::_Internal_swap(static_cast<_Concurrent_vector_base_v4&>(_Vector));
            ::std::swap(this->_My_allocator, _Vector._My_allocator);
        }
    }

    /// <summary>
    ///     Erases all elements in the concurrent vector. This method is not concurrency-safe.
    /// </summary>
    /// <remarks>
    ///     <c>clear</c> is not concurrency-safe. You must ensure that no other threads are invoking methods
    ///     on the concurrent vector when you call this method. <c>clear</c> does not free internal arrays. To free internal arrays,
    ///     call the function <c>shrink_to_fit</c> after <c>clear</c>.
    /// </remarks>
    /**/
    void clear()
    {
        _Internal_clear(&_Destroy_array);
    }

    /// <summary>
    ///     Erases all elements and destroys this concurrent vector.
    /// </summary>
    /**/
    ~concurrent_vector()
    {
        _Segment_t *_Table = _My_segment;
        _Internal_free_segments( reinterpret_cast<void**>(_Table), _Internal_clear(&_Destroy_array), _My_first_block );
        // base class destructor call
    }

    const ::Concurrency::details::_Concurrent_vector_base_v4 &_Internal_vector_base() const { return *this; }
    ::Concurrency::details::_Concurrent_vector_base_v4 &_Internal_vector_base() { return *this; }
private:

    // Allocate _K items
    static void * __cdecl _Internal_allocator(::Concurrency::details::_Concurrent_vector_base_v4 &_Vb, size_t _K)
    {
        return static_cast<concurrent_vector&>(_Vb)._My_allocator.allocate(_K);
    }
    // Free _K segments from table
    void _Internal_free_segments(void *_Table[], _Segment_index_t _K, _Segment_index_t _First_block);

    // Get reference to element at given _Index.
    _Ty& _Internal_subscript( size_type _Index ) const;

    // Get reference to element at given _Index with errors checks
    _Ty& _Internal_subscript_with_exceptions( size_type _Index ) const;

    // assign _N items by copying _Item
    void _Internal_assign(size_type _N, const_reference _Item);

    // helper class
    template<bool B> class _Is_integer_tag;

    // assign integer items by copying when arguments are treated as iterators. See C++ Standard 2003 23.1.1 p9
    template<class _I>
    void _Internal_assign(_I _First, _I _Last, _Is_integer_tag<true> *)
    {
        _Internal_assign(static_cast<size_type>(_First), static_cast<_Ty>(_Last));
    }
    // inline proxy assign by iterators
    template<class _I>
    void _Internal_assign(_I _First, _I _Last, _Is_integer_tag<false> *) {
        internal_assign_iterators(_First, _Last);
    }
    // assign by iterators
    template<class _I>
    void internal_assign_iterators(_I _First, _I _Last);

    // Construct _N instances of _Ty, starting at "begin".
    static void __cdecl _Initialize_array( void* _Begin, const void*, size_type _N );

    // Construct _N instances of _Ty, starting at "begin".
    static void __cdecl _Initialize_array_by( void* _Begin, const void* _Src, size_type _N );

    // Construct _N instances of _Ty, starting at "begin".
    static void __cdecl _Copy_array( void* _Dst, const void* _Src, size_type _N );

    // Assign _N instances of _Ty, starting at "begin".
    static void __cdecl _Assign_array( void* _Dst, const void* _Src, size_type _N );

    // Destroy _N instances of _Ty, starting at "begin".
    static void __cdecl _Destroy_array( void* _Begin, size_type _N );

    // Exception-aware helper class for filling a segment by exception-danger operators of user class
    class _Internal_loop_guide
    {
    public:
        const pointer _My_array;
        const size_type _N;
        size_type _I;
        _Internal_loop_guide(size_type _NTrials, void *_Ptr)
            : _My_array(static_cast<pointer>(_Ptr)), _N(_NTrials), _I(0)
        {
        }

        void _Init()
        {
            for(; _I < _N; ++_I)
                new( &_My_array[_I] ) _Ty();
        }
        void _Init(const void *_Src)
        {
            for(; _I < _N; ++_I)
                new( &_My_array[_I] ) _Ty(*static_cast<const _Ty*>(_Src));
        }
        void _Copy(const void *_Src)
        {
            for(; _I < _N; ++_I)
                new( &_My_array[_I] ) _Ty(static_cast<const _Ty*>(_Src)[_I]);
        }
        void _Assign(const void *_Src)
        {
            for(; _I < _N; ++_I)
                _My_array[_I] = static_cast<const _Ty*>(_Src)[_I];
        }
        template<class _It> void _Iterate(_It &_Src)
        {
            for(; _I < _N; ++_I, ++_Src)
                new( &_My_array[_I] ) _Ty( *_Src );
        }
        ~_Internal_loop_guide()
        {
            if(_I < _N) // if exception raised, do zeroing on the rest of items
                ::std::memset(_My_array+_I, 0, (_N-_I)*sizeof(value_type));
        }
    private:
        void operator=(const _Internal_loop_guide&); // prevent warning:  assign operator can't be generated
    };
};

/// <summary>
///     Compacts the internal representation of the concurrent vector to reduce fragmentation and optimize memory usage.
/// </summary>
/// <remarks>
///     This method will internally re-allocate memory move elements around, invalidating all the iterators.
///     <c>shrink_to_fit</c> is not concurrency-safe. You must ensure that no other threads are invoking methods
///     on the concurrent vector when you call this function.
/// </remarks>
/**/
template<typename _Ty, class _Ax>
void concurrent_vector<_Ty, _Ax>::shrink_to_fit()
{
    _Internal_segments_table _Old = { 0, nullptr };
    try
    {
        if( _Internal_compact( sizeof(_Ty), &_Old, &_Destroy_array, &_Copy_array ) )
            _Internal_free_segments( _Old._Table, _Pointers_per_long_table, _Old._First_block ); // Free joined and unnecessary segments
    }
    catch(...)
    {
        if( _Old._First_block ) // Free segment allocated for compacting. Only for support of exceptions in ctor of user _Ty[pe]
            _Internal_free_segments( _Old._Table, 1, _Old._First_block );
        throw;
    }
}

template<typename _Ty, class _Ax>
void concurrent_vector<_Ty, _Ax>::_Internal_free_segments(void *_Table[], _Segment_index_t _K, _Segment_index_t _First_block)
{
    // Free the arrays
    while( _K > _First_block )
    {
        --_K;
        _Ty* _Array = static_cast<_Ty*>(_Table[_K]);
        _Table[_K] = nullptr;
        if( _Array > _BAD_ALLOC_MARKER ) // check for correct segment pointer
            this->_My_allocator.deallocate( _Array, _Segment_size(_K) );
    }
    _Ty* _Array = static_cast<_Ty*>(_Table[0]);
    if( _Array > _BAD_ALLOC_MARKER )
    {
        _CONCRT_ASSERT( _First_block > 0 );
        while(_K > 0)
            _Table[--_K] = nullptr;
        this->_My_allocator.deallocate( _Array, _Segment_size(_First_block) );
    }
}

template<typename _Ty, class _Ax>
_Ty& concurrent_vector<_Ty, _Ax>::_Internal_subscript( size_type _Index ) const
{
    _CONCRT_ASSERT( _Index<_My_early_size ); // index out of bounds
    size_type _J = _Index;
    _Segment_index_t _K = _Segment_base_index_of( _J );
    _CONCRT_ASSERT( _My_segment != (_Segment_t*)_My_storage || _K < _Pointers_per_short_table ); // index is under construction
    // no need in load_with_acquire because the thread works in its own space or gets
    _Ty* _Array = static_cast<_Ty*>(_My_segment[_K]._My_array);
    _CONCRT_ASSERT( _Array != _BAD_ALLOC_MARKER ); // instance may be broken by bad allocation; use at() instead
    _CONCRT_ASSERT( _Array != nullptr ); // index is being allocated
    return _Array[_J];
}

template<typename _Ty, class _Ax>
_Ty& concurrent_vector<_Ty, _Ax>::_Internal_subscript_with_exceptions( size_type _Index ) const
{
    if( _Index >= _My_early_size )
        _Internal_throw_exception(0); // throw std::out_of_range
    size_type _J = _Index;
    _Segment_index_t _K = _Segment_base_index_of( _J );
    if( _My_segment == (_Segment_t*)_My_storage && _K >= _Pointers_per_short_table )
        _Internal_throw_exception(1); // throw std::out_of_range
    void *_Array = _My_segment[_K]._My_array; // no need in load_with_acquire
    if( _Array <= _BAD_ALLOC_MARKER ) // check for correct segment pointer
        _Internal_throw_exception(2); // throw std::range_error
    return static_cast<_Ty*>(_Array)[_J];
}

template<typename _Ty, class _Ax>
void concurrent_vector<_Ty, _Ax>::_Internal_assign(size_type _N, const_reference _Item)
{
    _CONCRT_ASSERT( _My_early_size == 0 );
    if( !_N )
        return;
    _Internal_reserve(_N, sizeof(_Ty), max_size());
    _My_early_size = _N;
    _Segment_index_t _K = 0;
    _Size_type _Sz = _Segment_size( _My_first_block );
    while (_Sz < _N)
    {
        _Initialize_array_by(static_cast<_Ty*>(_My_segment[_K]._My_array), static_cast<const void*>(&_Item), _Sz);
        _N -= _Sz;
        if (!_K)
        {
            _K = _My_first_block;
        }
        else {
            ++_K;
            _Sz <<= 1;
        }
    }
    _Initialize_array_by(static_cast<_Ty*>(_My_segment[_K]._My_array), static_cast<const void*>(&_Item), _N);
}

template<typename _Ty, class _Ax> template<class _I>
void concurrent_vector<_Ty, _Ax>::internal_assign_iterators(_I _First, _I _Last)
{
    _CONCRT_ASSERT(_My_early_size == 0);
    size_type _N = ::std::distance(_First, _Last);
    if( !_N ) return;
    _Internal_reserve(_N, sizeof(_Ty), max_size());
    _My_early_size = _N;
    _Segment_index_t _K = 0;
    _Size_type _Sz = _Segment_size( _My_first_block );
    while (_Sz < _N)
    {
        _Internal_loop_guide _Loop(_Sz, _My_segment[_K]._My_array);
        _Loop._Iterate(_First);
        _N -= _Sz;
        if (!_K)
        {
            _K = _My_first_block;
        }
        else {
            ++_K;
            _Sz <<= 1;
        }
    }

    _Internal_loop_guide _Loop(_N, _My_segment[_K]._My_array);
    _Loop._Iterate(_First);
}

template<typename _Ty, class _Ax>
void __cdecl concurrent_vector<_Ty, _Ax>::_Initialize_array( void* _Begin, const void *, size_type _N )
{
    _Internal_loop_guide _Loop(_N, _Begin); _Loop._Init();
}

template<typename _Ty, class _Ax>
void __cdecl concurrent_vector<_Ty, _Ax>::_Initialize_array_by( void* _Begin, const void *_Src, size_type _N )
{
    _Internal_loop_guide _Loop(_N, _Begin); _Loop._Init(_Src);
}

template<typename _Ty, class _Ax>
void __cdecl concurrent_vector<_Ty, _Ax>::_Copy_array( void* _Dst, const void* _Src, size_type _N ) {
    _Internal_loop_guide _Loop(_N, _Dst); _Loop._Copy(_Src);
}

template<typename _Ty, class _Ax>
void __cdecl concurrent_vector<_Ty, _Ax>::_Assign_array( void* _Dst, const void* _Src, size_type _N )
{
    _Internal_loop_guide _Loop(_N, _Dst); _Loop._Assign(_Src);
}

#pragma warning(push)
#pragma warning(disable: 4189) /* local variable _Array is initialized but not used - the compiler optimizes away calls to the destructor */

template<typename _Ty, class _Ax>
void __cdecl concurrent_vector<_Ty, _Ax>::_Destroy_array( void* _Begin, size_type _N )
{
    _Ty* _Array = static_cast<_Ty*>(_Begin);
    for( size_type _J=_N; _J>0; --_J )
        _Array[_J-1].~_Ty(); // destructors are supposed to not throw any exceptions
}

#pragma warning(pop)

/// <summary>
///     Tests if the <c>concurrent_vector</c> object on the left side of the operator is equal to the <c>concurrent_vector</c>
///     object on the right side.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements stored in the concurrent vectors.
/// </typeparam>
/// <typeparam name="A1">
///     The allocator type of the first <c>concurrent_vector</c> object.
/// </typeparam>
/// <typeparam name="A2">
///     The allocator type of the second <c>concurrent_vector</c> object.
/// </typeparam>
/// <param name="_A">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <param name="_B">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <returns>
///     <c>true</c> if the concurrent vector on the left side of the operator is equal to the concurrent vector on the right side
///     of the operator; otherwise <c>false</c>.
/// </returns>
/// <remarks>
///     Two concurrent vectors are equal if they have the same number of elements and their respective elements have the same values.
///     Otherwise, they are unequal.
///     <para> This method is not concurrency-safe with respect to other methods that could modify either of the concurrent vectors
///     <paramref name="_A"/> or <paramref name="_B"/>.</para>
/// </remarks>
/// <seealso cref="concurrent_vector Class"/>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class A1, class A2>
inline bool operator==(const concurrent_vector<_Ty, A1> &_A, const concurrent_vector<_Ty, A2> &_B)
{
    // Simply:    return _A.size() == _B.size() && std::equal(_A.begin(), _A.end(), _B.begin());
    if(_A.size() != _B.size())
        return false;
    typename concurrent_vector<_Ty, A1>::const_iterator _I(_A.begin());
    typename concurrent_vector<_Ty, A2>::const_iterator _J(_B.begin());
    for(; _I != _A.end(); ++_I, ++_J)
    {
        if( !(*_I == *_J) )
            return false;
    }
    return true;
}

/// <summary>
///     Tests if the <c>concurrent_vector</c> object on the left side of the operator is not equal to the <c>concurrent_vector</c>
///     object on the right side.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements stored in the concurrent vectors.
/// </typeparam>
/// <typeparam name="A1">
///     The allocator type of the first <c>concurrent_vector</c> object.
/// </typeparam>
/// <typeparam name="A2">
///     The allocator type of the second <c>concurrent_vector</c> object.
/// </typeparam>
/// <param name="_A">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <param name="_B">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <returns>
///     <c>true</c> if the concurrent vectors are not equal; <c>false</c> if the concurrent vectors are equal.
/// </returns>
/// <remarks>
///     Two concurrent vectors are equal if they have the same number of elements and their respective elements have the same
///     values. Otherwise, they are unequal.
///     <para> This method is not concurrency-safe with respect to other methods that could modify either of the concurrent vectors
///     <paramref name="_A"/> or <paramref name="_B"/>.</para>
/// </remarks>
/// <seealso cref="concurrent_vector Class"/>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class A1, class A2>
inline bool operator!=(const concurrent_vector<_Ty, A1> &_A, const concurrent_vector<_Ty, A2> &_B)
{
    return !(_A == _B);
}

/// <summary>
///     Tests if the <c>concurrent_vector</c> object on the left side of the operator is less than the <c>concurrent_vector</c>
///     object on the right side.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements stored in the concurrent vectors.
/// </typeparam>
/// <typeparam name="A1">
///     The allocator type of the first <c>concurrent_vector</c> object.
/// </typeparam>
/// <typeparam name="A2">
///     The allocator type of the second <c>concurrent_vector</c> object.
/// </typeparam>
/// <param name="_A">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <param name="_B">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <returns>
///     <c>true</c> if the concurrent vector on the left side of the operator is less than the concurrent vector
///     on the right side of the operator; otherwise <c>false</c>.
/// </returns>
/// <remarks>
///     The behavior of this operator is identical to the equivalent operator for the <c>vector</c> class in the <c>std</c>
///     namespace.
///     <para> This method is not concurrency-safe with respect to other methods that could modify either of the concurrent vectors
///     <paramref name="_A"/> or <paramref name="_B"/>.</para>
/// </remarks>
/// <seealso cref="concurrent_vector Class"/>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class A1, class A2>
inline bool operator<(const concurrent_vector<_Ty, A1> &_A, const concurrent_vector<_Ty, A2> &_B)
{
    return (::std::lexicographical_compare(_A.begin(), _A.end(), _B.begin(), _B.end()));
}

/// <summary>
///     Tests if the <c>concurrent_vector</c> object on the left side of the operator is greater than the <c>concurrent_vector</c>
///     object on the right side.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements stored in the concurrent vectors.
/// </typeparam>
/// <typeparam name="A1">
///     The allocator type of the first <c>concurrent_vector</c> object.
/// </typeparam>
/// <typeparam name="A2">
///     The allocator type of the second <c>concurrent_vector</c> object.
/// </typeparam>
/// <param name="_A">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <param name="_B">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <returns>
///     <c>true</c> if the concurrent vector on the left side of the operator is greater than the concurrent vector
///     on the right side of the operator; otherwise <c>false</c>.
/// </returns>
/// <remarks>
///     The behavior of this operator is identical to the equivalent operator for the <c>vector</c> class in the <c>std</c>
///     namespace.
///     <para> This method is not concurrency-safe with respect to other methods that could modify either of the concurrent vectors
///     <paramref name="_A"/> or <paramref name="_B"/>.</para>
/// </remarks>
/// <seealso cref="concurrent_vector Class"/>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class A1, class A2>
inline bool operator>(const concurrent_vector<_Ty, A1> &_A, const concurrent_vector<_Ty, A2> &_B)
{
    return _B < _A;
}

/// <summary>
///     Tests if the <c>concurrent_vector</c> object on the left side of the operator is less than or equal to the <c>concurrent_vector</c>
///     object on the right side.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements stored in the concurrent vectors.
/// </typeparam>
/// <typeparam name="A1">
///     The allocator type of the first <c>concurrent_vector</c> object.
/// </typeparam>
/// <typeparam name="A2">
///     The allocator type of the second <c>concurrent_vector</c> object.
/// </typeparam>
/// <param name="_A">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <param name="_B">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <returns>
///     <c>true</c> if the concurrent vector on the left side of the operator is less than or equal to the concurrent vector
///     on the right side of the operator; otherwise <c>false</c>.
/// </returns>
/// <remarks>
///     The behavior of this operator is identical to the equivalent operator for the <c>vector</c> class in the <c>std</c>
///     namespace.
///     <para> This method is not concurrency-safe with respect to other methods that could modify either of the concurrent vectors
///     <paramref name="_A"/> or <paramref name="_B"/>.</para>
/// </remarks>
/// <seealso cref="concurrent_vector Class"/>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class A1, class A2>
inline bool operator<=(const concurrent_vector<_Ty, A1> &_A, const concurrent_vector<_Ty, A2> &_B)
{
    return !(_B < _A);
}

/// <summary>
///     Tests if the <c>concurrent_vector</c> object on the left side of the operator is greater than or equal to the <c>concurrent_vector</c>
///     object on the right side.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements stored in the concurrent vectors.
/// </typeparam>
/// <typeparam name="A1">
///     The allocator type of the first <c>concurrent_vector</c> object.
/// </typeparam>
/// <typeparam name="A2">
///     The allocator type of the second <c>concurrent_vector</c> object.
/// </typeparam>
/// <param name="_A">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <param name="_B">
///     An object of type <c>concurrent_vector</c>.
/// </param>
/// <returns>
///     <c>true</c> if the concurrent vector on the left side of the operator is greater than or equal to the concurrent vector
///     on the right side of the operator; otherwise <c>false</c>.
/// </returns>
/// <remarks>
///     The behavior of this operator is identical to the equivalent operator for the <c>vector</c> class in the <c>std</c>
///     namespace.
///     <para> This method is not concurrency-safe with respect to other methods that could modify either of the concurrent vectors
///     <paramref name="_A"/> or <paramref name="_B"/>.</para>
/// </remarks>
/// <seealso cref="concurrent_vector Class"/>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class A1, class A2>
inline bool operator>=(const concurrent_vector<_Ty, A1> &_A, const concurrent_vector<_Ty, A2> &_B)
{
    return !(_A < _B);
}

/// <summary>
///     Exchanges the elements of two <c>concurrent_vector</c> objects.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements stored in the concurrent vectors.
/// </typeparam>
/// <typeparam name="_Ax">
///     The allocator type of the concurrent vectors.
/// </typeparam>
/// <param name="_B">
///     The concurrent vector providing the elements to be swapped, or the vector whose elements are to be exchanged with those of the
///     concurrent vector <paramref name="_A"/>.
/// </param>
/// <param name="_A">
///     The concurrent vector whose elements are to be exchanged with those of the concurrent vector <paramref name="_B"/>.
/// </param>
/// <remarks>
///     The template function is an algorithm specialized on the container class <c>concurrent_vector</c> to execute the member function
///     <paramref name="_A"/>.<see cref="concurrent_vector::swap Method">concurrent_vector::swap</see>(<paramref name="_B"/>). These are
///     instances of the partial ordering of function templates by the compiler. When template functions are overloaded in such a way that
///     the match of the template with the function call is not unique, then the compiler will select the most specialized version of the
///     template function. The general version of the template function, <c>template &lt;class T&gt; void swap(T&amp;, T&amp;)</c>, in the
///     algorithm class works by assignment and is a slow operation. The specialized version in each container is much faster as it can
///     work with the internal representation of the container class.
///     <para> This method is not concurrency-safe. You must ensure that no other threads are performing operations on either of the concurrent
///     vectors when you call this method.</para>
/// </remarks>
/// <seealso cref="concurrent_vector Class"/>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template<typename _Ty, class _Ax>
inline void swap(concurrent_vector<_Ty, _Ax> &_A, concurrent_vector<_Ty, _Ax> &_B)
{
    _A.swap( _B );
}

} // namespace Concurrency

namespace concurrency = ::Concurrency;

#pragma pop_macro("new")
#pragma warning (pop)
#pragma pack(pop)
