/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
* Microsoft would like to acknowledge that this concurrency data structure implementation
* is based on Intel's implementation in its Threading Building Blocks ("Intel Material").
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* concurrent_queue.h
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

/*
    Intel Material Copyright 2005-2008 Intel Corporation.  All Rights Reserved.
*/

#pragma once


#include <crtdefs.h>
#include <iterator>
#include <memory>
#include <cstddef>
#include <crtdbg.h>
#include <concrt.h>
#include <type_traits>
#include <utility>

#define _PPL_CONTAINER

#if !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64))
    #error ERROR: Concurrency Runtime is supported only on X64, X86, ARM, and ARM64 architectures.
#endif  /* !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64)) */

#if defined (_M_CEE)
    #error ERROR: Concurrency Runtime is not supported when compiling /clr.
#endif  /* defined (_M_CEE) */

#pragma pack(push,_CRT_PACKING)
#pragma warning(push)
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
class concurrent_queue;

namespace details
{

    class _Concurrent_queue_rep;

    typedef size_t _Ticket;

    class _Concurrent_queue_iterator_rep;
    class _Concurrent_queue_iterator_base_v4;
    template<typename _Container, typename _Value> class _Concurrent_queue_iterator;

    //  Type-independent portion of concurrent_queue.
    class _Concurrent_queue_base_v4
    {
        // Internal representation
        _Concurrent_queue_rep* _My_rep;

        friend class _Concurrent_queue_rep;
        friend struct _Micro_queue;
        friend class _Micro_queue_pop_finalizer;
        friend class _Concurrent_queue_iterator_rep;
        friend class _Concurrent_queue_iterator_base_v4;
    protected:
        // Prefix on a page
        struct _Page
        {
            _Page* _Next;
            size_t _Mask;
        };

        // Always a power of 2
        size_t _Items_per_page;

        // Size of an item
        size_t _Item_size;

    private:
        virtual void _Move_item( _Page& _Dst, size_t _Index, void* _Src ) = 0;
        virtual void _Copy_item( _Page& _Dst, size_t _Index, const void* _Src ) = 0;
        virtual void _Assign_and_destroy_item( void* _Dst, _Page& _Src, size_t _Index ) = 0;
    protected:
        _CONCRTIMP _Concurrent_queue_base_v4( size_t _Item_size );
        _CONCRTIMP virtual ~_Concurrent_queue_base_v4();

        // Enqueue item at tail of queue
        _CONCRTIMP void _Internal_push( const void* _Src );

        // Enqueue item at tail of queue by move
        _CONCRTIMP void _Internal_move_push( void* _Src );

        // swap the internal representation
        _CONCRTIMP void _Internal_swap( _Concurrent_queue_base_v4& other );

        // Attempt to dequeue item from queue.
        /** NULL if there was no item to dequeue. */
        _CONCRTIMP bool _Internal_pop_if_present( void* _Dst );

        // Get size of queue
        _CONCRTIMP size_t _Internal_size() const;

        // Test instantaneous queue empty
        _CONCRTIMP bool _Internal_empty() const;

        // custom allocator
        virtual _Page *_Allocate_page() = 0;

        // custom de-allocator
        virtual void _Deallocate_page( _Page *p ) = 0;

        // free any remaining pages
        _CONCRTIMP void _Internal_finish_clear() ;

        // throw an exception
        _CONCRTIMP void _Internal_throw_exception() const;

    private:
        // Deny copy construction
        _Concurrent_queue_base_v4( const _Concurrent_queue_base_v4& );

        // Deny assignment
        void operator=( const _Concurrent_queue_base_v4& );
    };

    typedef _Concurrent_queue_base_v4 _Concurrent_queue_base ;


    // A queue using simple locking.
    /** For efficiency, this class has no constructor.
        The caller is expected to zero-initialize it. */
    struct _Micro_queue
    {
        class _Pop_finalizer;
        class _Push_finalizer;

        _Subatomic<_Concurrent_queue_base::_Page*> _Head_page;
        _Subatomic<_Ticket> _Head_counter;

        _Subatomic<_Concurrent_queue_base::_Page*> _Tail_page;
        _Subatomic<_Ticket> _Tail_counter;

        volatile long _Page_mutex_flag;

        void _Push( void* _Item, _Ticket _K, _Concurrent_queue_base& _Base, void (_Concurrent_queue_base::*moveOp)(_Concurrent_queue_base_v4::_Page&, size_t, void*));

        bool _Pop( void* _Dest, _Ticket _K, _Concurrent_queue_base& _Base );
    };

    // Disable warning C4324: structure was padded due to __declspec(align())
    // This padding is expected and necessary.
    #pragma warning(push)
    #pragma warning(disable: 4324)


    // Internal representation of a ConcurrentQueue.
    /** For efficiency, this class has no constructor.
        The caller is expected to zero-initialize it. */
    class _Concurrent_queue_rep
    {
    private:
        friend struct _Micro_queue;

        // Approximately n_queue/golden ratio
        static constexpr size_t _Phi = 3;

    public:
        // Must be power of 2
        static constexpr size_t _N_queue = 8;

        // Map ticket to an array index
        static size_t _Index( _Ticket _K )
        {
            return _K*_Phi%_N_queue;
        }

        __declspec(align(64))
        _Subatomic<_Ticket> _Head_counter;

        __declspec(align(64))
        _Subatomic<_Ticket> _Tail_counter;

        __declspec(align(64))
        _Micro_queue _Array[_N_queue];

        _Micro_queue& _Choose( _Ticket _K )
        {
            // The formula here approximates LRU in a cache-oblivious way.
            return _Array[_Index(_K)];
        }
    };

    #pragma warning(pop)


    // Type-independent portion of _Concurrent_queue_iterator.
    class _Concurrent_queue_iterator_base_v4 {
        // concurrent_queue over which we are iterating.
        /** NULL if one past last element in queue. */
        _Concurrent_queue_iterator_rep* _My_rep;

        template<typename _C, typename _Ty, typename _U>
        friend bool operator==( const _Concurrent_queue_iterator<_C,_Ty>&, const _Concurrent_queue_iterator<_C,_U>& );

        template<typename _C, typename _Ty, typename _U>
        friend bool operator!=( const _Concurrent_queue_iterator<_C,_Ty>&, const _Concurrent_queue_iterator<_C,_U>& );
    protected:
        // Pointer to current item
        mutable void* _My_item;

        // Default constructor
        _Concurrent_queue_iterator_base_v4()
            : _My_rep(nullptr), _My_item(nullptr)
        {
        }

        // Copy constructor
        _Concurrent_queue_iterator_base_v4( const _Concurrent_queue_iterator_base_v4& _I )
            : _My_rep(nullptr), _My_item(nullptr)
        {
            _Assign(_I);
        }

        // Construct iterator pointing to head of queue.
        _CONCRTIMP _Concurrent_queue_iterator_base_v4( const _Concurrent_queue_base&  );

        // Assignment
        _CONCRTIMP void _Assign( const _Concurrent_queue_iterator_base_v4& );

        // Advance iterator one step towards tail of queue.
        _CONCRTIMP void _Advance();

        // Destructor
        _CONCRTIMP ~_Concurrent_queue_iterator_base_v4();
    };

    typedef _Concurrent_queue_iterator_base_v4 concurrent_queue_iterator_base;

    // Meets requirements of a forward iterator for STL.
    /** _Value is either the _Ty or const _Ty type of the container. */
    template<typename _Container, typename _Value>
    class _Concurrent_queue_iterator: public _Concurrent_queue_iterator_base_v4
    {
    public:
        using iterator_category = ::std::forward_iterator_tag;
        using value_type = ::std::remove_const_t<_Value>;
        using difference_type = ::std::ptrdiff_t;
        using pointer = _Value *;
        using reference = _Value&;

    private:
        template<typename _Ty, class _Ax> friend class ::Concurrency::concurrent_queue;

        // Construct iterator pointing to head of queue.
        _Concurrent_queue_iterator( const _Concurrent_queue_base& _Queue )
            : _Concurrent_queue_iterator_base_v4(_Queue)
        {
        }
    public:
        _Concurrent_queue_iterator()
        {
        }

        /** If _Value==_Container::value_type, then this routine is the copy constructor.
            If _Value==const _Container::value_type, then this routine is a conversion constructor. */
        _Concurrent_queue_iterator( const _Concurrent_queue_iterator<_Container,typename _Container::value_type>& _Other )
            : _Concurrent_queue_iterator_base_v4(_Other)
        {
        }

        // Iterator assignment
        _Concurrent_queue_iterator& operator=( const _Concurrent_queue_iterator& _Other )
        {
            _Assign(_Other);
            return *this;
        }

        // Reference to current item
        _Value& operator*() const
        {
            return *static_cast<_Value*>(_My_item);
        }

        _Value* operator->() const
        {
            return &operator*();
        }

        // Advance to next item in queue
        _Concurrent_queue_iterator& operator++()
        {
            _Advance();
            return *this;
        }

        // Post increment
        _Concurrent_queue_iterator operator++(int)
        {
            _Concurrent_queue_iterator _Result = *this;
            _Advance();
            return _Result;
        }
    }; // _Concurrent_queue_iterator

    template<typename _C, typename _Ty, typename _U>
    bool operator==( const _Concurrent_queue_iterator<_C,_Ty>& _I, const _Concurrent_queue_iterator<_C,_U>& _J )
    {
        return _I._My_item==_J._My_item;
    }

    template<typename _C, typename _Ty, typename _U>
    bool operator!=( const _Concurrent_queue_iterator<_C,_Ty>& _I, const _Concurrent_queue_iterator<_C,_U>& _J )
    {
        return _I._My_item!=_J._My_item;
    }

} // namespace details;


/// <summary>
///     The <c>concurrent_queue</c> class is a sequence container class that allows first-in,
///     first-out access to its elements. It enables a limited set of concurrency-safe operations, such as
///     <c>push</c> and <c>try_pop</c>.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements to be stored in the queue.
/// </typeparam>
/// <typeparam name="_Ax">
///     The type that represents the stored allocator object that encapsulates details about the allocation and
///     deallocation of memory for this concurrent queue. This argument is optional and the default value is
///     <c>allocator&lt;</c><typeparamref name="_Ty"/><c>&gt;</c>.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Parallel Containers and Objects"/>.
/// </remarks>
/**/
template<typename _Ty, class _Ax>
class concurrent_queue: public ::Concurrency::details::_Concurrent_queue_base_v4
{
    template<typename _Container, typename _Value> friend class ::Concurrency::details::_Concurrent_queue_iterator;

    // allocator type
    typedef typename ::std::allocator_traits<_Ax>::template rebind_alloc<char> _Page_allocator_type;
    _Page_allocator_type _My_allocator;
    using _Page_allocator_traits = ::std::allocator_traits<_Page_allocator_type>;

    // Class used to ensure exception-safety of method "pop"
    class _Destroyer
    {
    private:
        _Ty& _My_value;

        void operator=(const _Destroyer&); // prevent warning: assign operator can't be generated
    public:
        _Destroyer( _Ty& _Value )
            : _My_value(_Value)
        {
        }

        ~_Destroyer()
        {
            _My_value.~_Ty();
        }
    };

    _Ty& _Get_ref( _Page& _Pg, size_t _Index )
    {
        _CONCRT_ASSERT( _Index<_Items_per_page );
        return static_cast<_Ty*>(static_cast<void*>(&_Pg+1))[_Index];
    }

    /*override*/ virtual void _Copy_item( _Page& _Dst, size_t _Index, const void* _Src )
    {
        new( &_Get_ref(_Dst,_Index) ) _Ty(*static_cast<const _Ty*>(_Src));
    }

    /*override*/ virtual void _Move_item( _Page& _Dst, size_t _Index, void* _Src )
    {
        new( &_Get_ref(_Dst,_Index) ) _Ty(::std::move(*static_cast<_Ty*>(_Src)));
    }

    /*override*/ virtual void _Assign_and_destroy_item( void* _Dst, _Page& _Src, size_t _Index )
    {
        _Ty& _From = _Get_ref(_Src,_Index);
        _Destroyer _D(_From);
        if (_Dst != nullptr)
        {
            *static_cast<_Ty*>(_Dst) = ::std::move(_From);
        }
    }

    /*overide*/ virtual _Page *_Allocate_page()
    {
        size_t _N = sizeof(_Page) + _Items_per_page*_Item_size;
        _Page *_Pg = reinterpret_cast<_Page*>(_My_allocator.allocate( _N ));
        if( !_Pg )
            _Internal_throw_exception();
        return _Pg;
    }

    /*override*/ virtual void _Deallocate_page( _Page *_Pg )
    {
        size_t _N = sizeof(_Page) + _Items_per_page*_Item_size;
        _My_allocator.deallocate( reinterpret_cast<char*>(_Pg), _N );
    }

public:
    /// <summary>
    ///     A type that represents the data type stored in a concurrent queue.
    /// </summary>
    /**/
    typedef _Ty value_type;

    /// <summary>
    ///     A type that represents the allocator class for the concurrent queue.
    /// </summary>
    /**/
    typedef _Ax allocator_type;

    /// <summary>
    ///     A type that provides a reference to an element stored in a concurrent queue.
    /// </summary>
    /**/
    typedef _Ty& reference;

    /// <summary>
    ///     A type that provides a reference to a <c>const</c> element stored in a concurrent queue for reading and
    ///     performing <c>const</c> operations.
    /// </summary>
    /**/
    typedef const _Ty& const_reference;

    /// <summary>
    ///     A type that counts the number of elements in a concurrent queue.
    /// </summary>
    /**/
    typedef ::std::size_t size_type;

    /// <summary>
    ///     A type that provides the signed distance between two elements in a concurrent queue.
    /// </summary>
    /**/
    typedef ::std::ptrdiff_t difference_type;

    /// <summary>
    ///     Constructs a concurrent queue.
    /// </summary>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the queue.
    ///     <para>The first constructor specifies an empty initial queue and explicitly specifies the allocator
    ///     type to be used.</para>
    ///     <para>The second constructor specifies a copy of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The third constructor specifies a move of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The fourth constructor specifies values supplied by the iterator range
    ///     [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    explicit concurrent_queue(const allocator_type  &_Al = allocator_type())
        : _Concurrent_queue_base_v4( sizeof(_Ty) ), _My_allocator( _Al )
    {
    }

    /// <summary>
    ///     Constructs a concurrent queue.
    /// </summary>
    /// <param name="_OtherQ">
    ///     The source <c>concurrent_queue</c> object to copy or move elements from.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the queue.
    ///     <para>The first constructor specifies an empty initial queue and explicitly specifies the allocator
    ///     type to be used.</para>
    ///     <para>The second constructor specifies a copy of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The third constructor specifies a move of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The fourth constructor specifies values supplied by the iterator range
    ///     [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    concurrent_queue(const concurrent_queue& _OtherQ, const allocator_type &_Al = allocator_type());

    /// <summary>
    ///     Constructs a concurrent queue.
    /// </summary>
    /// <param name="_OtherQ">
    ///     The source <c>concurrent_queue</c> object to copy or move elements from.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the queue.
    ///     <para>The first constructor specifies an empty initial queue and explicitly specifies the allocator
    ///     type to be used.</para>
    ///     <para>The second constructor specifies a copy of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The third constructor specifies a move of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The fourth constructor specifies values supplied by the iterator range
    ///     [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    concurrent_queue(concurrent_queue&& _OtherQ, const allocator_type &_Al = allocator_type());

    /// <summary>
    ///     Constructs a concurrent queue.
    /// </summary>
    /// <typeparam name="_InputIterator">
    ///     The type of the input iterator that specifies a range of values.
    /// </typeparam>
    /// <param name="_Begin">
    ///     Position of the first element in the range of elements to be copied.
    /// </param>
    /// <param name="_End">
    ///     Position of the first element beyond the range of elements to be copied.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the queue.
    ///     <para>The first constructor specifies an empty initial queue and explicitly specifies the allocator
    ///     type to be used.</para>
    ///     <para>The second constructor specifies a copy of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The third constructor specifies a move of the concurrent queue <paramref name="_OtherQ"/>.</para>
    ///     <para>The fourth constructor specifies values supplied by the iterator range
    ///     [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
    /// </remarks>
    /**/
    template<typename _InputIterator>
    concurrent_queue(_InputIterator _Begin, _InputIterator _End)
        : _Concurrent_queue_base_v4( sizeof(_Ty) ), _My_allocator( allocator_type() )
    {
        while (_Begin != _End)
        {
            this->push(*_Begin);
            ++_Begin;
        }
    }

    /// <summary>
    ///     Destroys the concurrent queue.
    /// </summary>
    /**/
    ~concurrent_queue();

    /// <summary>
    ///     Enqueues an item at tail end of the concurrent queue. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Src">
    ///     The item to be added to the queue.
    /// </param>
    /// <remarks>
    ///     <c>push</c> is concurrency-safe with respect to calls to the methods <c>push</c>, <c>try_pop</c>, and <c>empty</c>.
    /// </remarks>
    /**/
    void push( const _Ty& _Src )
    {
        _Internal_push( &_Src );
    }

    /// <summary>
    ///     Enqueues an item at tail end of the concurrent queue. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Src">
    ///     The item to be added to the queue.
    /// </param>
    /// <remarks>
    ///     <c>push</c> is concurrency-safe with respect to calls to the methods <c>push</c>, <c>try_pop</c>, and <c>empty</c>.
    /// </remarks>
    /**/
    void push( _Ty&& _Src )
    {
        _Internal_move_push( &_Src );
    }

    /// <summary>
    ///     Dequeues an item from the queue if one is available. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Dest">
    ///     A reference to a location to store the dequeued item.
    /// </param>
    /// <returns>
    ///     <c>true</c> if an item was successfully dequeued,<c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     If an item was successfully dequeued, the parameter <paramref name="_Dest"/> receives the
    ///     dequeued value, the original value held in the queue is destroyed, and this function returns
    ///     <c>true</c>. If there was no item to dequeue, this function returns <c>false</c> without blocking,
    ///     and the contents of the <paramref name="_Dest"/> parameter are undefined.
    ///     <para><c>try_pop</c> is concurrency-safe with respect to calls to the methods <c>push</c>, <c>try_pop</c>,
    ///     and <c>empty</c>.</para>
    /// </remarks>
    /**/
    bool try_pop( _Ty& _Dest )
    {
        return _Internal_pop_if_present( &_Dest );
    }

    /// <summary>
    ///     Returns the number of items in the queue. This method is not concurrency-safe.
    /// </summary>
    /// <returns>
    ///     The size of the concurrent queue.
    /// </returns>
    /// <remarks>
    ///     <c>unsafe_size</c> is not concurrency-safe and can produce incorrect results if called concurrently
    ///     with calls to the methods <c>push</c>, <c>try_pop</c>, and <c>empty</c>.
    /// </remarks>
    /**/
    size_type unsafe_size() const
    {
        return _Internal_size();
    }

    /// <summary>
    ///     Tests if the concurrent queue is empty at the moment this method is called. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the concurrent queue was empty at the moment we looked, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     While this method is concurrency-safe with respect to calls to the methods <c>push</c>, <c>try_pop</c>, and
    ///     <c>empty</c>, the value returned might be incorrect by the time it is inspected by the calling thread.
    /// </remarks>
    /**/
    bool empty() const
    {
        return _Internal_empty();
    }

    /// <summary>
    ///     Returns a copy of the allocator used to construct the concurrent queue. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     A copy of the allocator used to construct the concurrent queue.
    /// </returns>
    /**/
    allocator_type get_allocator() const
    {
        return this->_My_allocator;
    }

    /// <summary>
    ///     Clears the concurrent queue, destroying any currently enqueued elements. This method is not concurrency-safe.
    /// </summary>
    /**/
    void clear();

    /// <summary>
    ///     A type that represents a non-thread-safe iterator over the elements in a concurrent queue.
    /// </summary>
    /**/
    typedef details::_Concurrent_queue_iterator<concurrent_queue,_Ty> iterator;

    /// <summary>
    ///     A type that represents a non-thread-safe <c>const</c> iterator over elements in a concurrent queue.
    /// </summary>
    /**/
    typedef details::_Concurrent_queue_iterator<concurrent_queue,const _Ty> const_iterator;

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     beginning of the concurrent queue. This method is not concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     beginning of the  concurrent queue object.
    /// </returns>
    /// <remarks>
    ///     The iterators for the <c>concurrent_queue</c> class are primarily intended for debugging, as they are slow, and iteration
    ///     is not concurrency-safe with respect to other queue operations.
    /// </remarks>
    /**/
    iterator unsafe_begin()
    {
        return iterator(*this);
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     end of the concurrent queue. This method is not concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     end of the concurrent queue.
    /// </returns>
    /// <remarks>
    ///     The iterators for the <c>concurrent_queue</c> class are primarily intended for debugging, as they are slow, and iteration
    ///     is not concurrency-safe with respect to other queue operations.
    /// </remarks>
    /**/
    iterator unsafe_end()
    {
        return iterator();
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     beginning of the concurrent queue. This method is not concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     beginning of the concurrent queue.
    /// </returns>
    /// <remarks>
    ///     The iterators for the <c>concurrent_queue</c> class are primarily intended for debugging, as they are slow, and iteration
    ///     is not concurrency-safe with respect to other queue operations.
    /// </remarks>
    /**/
    const_iterator unsafe_begin() const
    {
        return const_iterator(*this);
    }

    /// <summary>
    ///     Returns an iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     end of the concurrent queue. This method is not concurrency-safe.
    /// </summary>
    /// <returns>
    ///     An iterator of type <typeparamref name="iterator"/> or <typeparamref name="const_iterator"/> to the
    ///     end of the concurrent queue.
    /// </returns>
    /// <remarks>
    ///     The iterators for the <c>concurrent_queue</c> class are primarily intended for debugging, as they are slow, and iteration
    ///     is not concurrency-safe with respect to other queue operations.
    /// </remarks>
    /**/
    const_iterator unsafe_end() const
    {
        return const_iterator();
    }
};


/// <summary>
///     Constructs a concurrent queue.
/// </summary>
/// <param name="_OtherQ">
///     The source <c>concurrent_queue</c> object to copy or move elements from.
/// </param>
/// <param name="_Al">
///     The allocator class to use with this object.
/// </param>
/// <remarks>
///     All constructors store an allocator object <paramref name="_Al"/> and initialize the queue.
///     <para>The first constructor specifies an empty initial queue and explicitly specifies the allocator
///     type to be used.</para>
///     <para>The second constructor specifies a copy of the concurrent queue <paramref name="_OtherQ"/>.</para>
///     <para>The third constructor specifies a move of the concurrent queue <paramref name="_OtherQ"/>.</para>
///     <para>The fourth constructor specifies values supplied by the iterator range
///     [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
/// </remarks>
/**/
template<typename _Ty, class _Ax>
concurrent_queue<_Ty,_Ax>::concurrent_queue(const concurrent_queue& _Queue, const allocator_type& _Al)
    : _Concurrent_queue_base_v4( sizeof(_Ty) ), _My_allocator(_Al)
{
    concurrent_queue::const_iterator _QEnd = _Queue.unsafe_end();
    for (concurrent_queue::const_iterator _It = _Queue.unsafe_begin(); _It != _QEnd; ++_It)
        this->push(*_It);
}

/// <summary>
///     Constructs a concurrent queue.
/// </summary>
/// <param name="_OtherQ">
///     The source <c>concurrent_queue</c> object to copy or move elements from.
/// </param>
/// <remarks>
///     All constructors store an allocator object <paramref name="_Al"/> and initialize the queue.
///     <para>The first constructor specifies an empty initial queue and explicitly specifies the allocator
///     type to be used.</para>
///     <para>The second constructor specifies a copy of the concurrent queue <paramref name="_OtherQ"/>.</para>
///     <para>The third constructor specifies a move of the concurrent queue <paramref name="_OtherQ"/>.</para>
///     <para>The fourth constructor specifies values supplied by the iterator range
///     [<paramref name="_Begin"/>, <paramref name="_End"/>).</para>
/// </remarks>
/**/
template<typename _Ty, class _Ax>
concurrent_queue<_Ty,_Ax>::concurrent_queue(concurrent_queue&& _Queue, const allocator_type& _Al)
    : _Concurrent_queue_base_v4( sizeof(_Ty) ), _My_allocator(_Al)
{
    _Internal_swap(_Queue);
}

/// <summary>
///     Destroys the concurrent queue.
/// </summary>
/**/
template<typename _Ty, class _Ax>
concurrent_queue<_Ty,_Ax>::~concurrent_queue()
{
    clear();
    _Internal_finish_clear();
}

/// <summary>
///     Clears the concurrent queue, destroying any currently enqueued elements. This method is not concurrency-safe.
/// </summary>
/**/
template<typename _Ty, class _Ax>
void concurrent_queue<_Ty,_Ax>::clear()
{
    while( !empty() )
    {
        if (!_Internal_pop_if_present(nullptr))
        {
            _CONCRT_ASSERT(empty());
            break;
        }
    }
}

} // namespace Concurrency

namespace concurrency = ::Concurrency;

#pragma pop_macro("new")
#pragma warning(pop)
#pragma pack(pop)
