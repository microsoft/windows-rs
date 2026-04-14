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
* concurrent_priority_queue.h
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

/*
    Intel Material Copyright 2005-2011 Intel Corporation.  All Rights Reserved.
*/

#pragma once

#include <crtdefs.h>
#include <memory>
#include <iterator>
#include <limits>
#include <algorithm>
#include <cstring>
#include <vector>
#include <crtdbg.h>
#include <concrt.h>

#if !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64))
    #error ERROR: Concurrency Runtime is supported only on X64, X86, ARM, and ARM64 architectures.
#endif  /* !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64)) */

#if defined (_M_CEE)
    #error ERROR: Concurrency Runtime is not supported when compiling /clr.
#endif  /* defined (_M_CEE) */

#pragma pack(push,_CRT_PACKING)
#pragma warning (push)
#pragma warning (disable: 4510 4512 4610)  // disable warnings for compiler unable to generate constructor

/// <summary>
///     The <c>concurrency</c> namespace provides classes and functions that give you access to the Concurrency Runtime,
///     a concurrent programming framework for C++. For more information, see <see cref="Concurrency Runtime"/>.
/// </summary>
/**/
namespace Concurrency
{

namespace details
{
    /// <summary>
    ///     _Aggregated_operation base class
    /// </summary>
    template <typename _Derived>
    class _Aggregated_operation
    {
     public:
        volatile int _M_status;
        _Derived * _M_pNext;

        _Aggregated_operation() :
            _M_status(0),
            _M_pNext(nullptr)
        {
        }
    };

    /// <summary>
    ///     An aggregator for collecting operations coming from multiple sources and executing them serially on a single thread.
    ///     _Operation_type must be derived from _Aggregated_operation. The parameter _Handler_type is a functor that will be passed the
    ///     list of operations and is expected to handle each operation appropriately, setting the status of each operation to non-zero.
    /// </summary>
    template < typename _Operation_type, typename _Handler_type >
    class _Aggregator
    {
    public:
        _Aggregator() : _M_handler_busy(0)
        {
            _M_pPending_operations = nullptr;
        }

        ~_Aggregator()
        {
        }

        void _Initialize_handler(_Handler_type _Handler)
        {
            _M_handle_operations = _Handler;
        }

        /// <summary>
        ///     Place operation in list and either handle list or wait for operation to complete.
        /// </summary>
        void _Execute(_Operation_type *_Op)
        {
            _Operation_type *_Res;

            // Insert the operation in the queue
            do
            {
                _Op->_M_pNext = _Res = _M_pPending_operations;
            } while (_InterlockedCompareExchangePointer(reinterpret_cast<void *volatile *>(&_M_pPending_operations), _Op, _Res) != _Res);

            if (_Res == nullptr)
            {
                // This item was the first one in the list; this thread will handle the operations. Note that by the time the
                // thread pops the list of operations to handle, there may be several operations queued up.
                _Start_handle_operations();
                _CONCRT_ASSERT(_Op->_M_status != 0);
            }
            else
            {
                // This item was not the first one on the list; a different thread is going to handle this operation, wait for
                // the result to be ready.
                ::Concurrency::details::_SpinWaitBackoffNone _SpinWait;
                while (_Op->_M_status == 0)
                {
                    _SpinWait._SpinOnce();
                }
            }
        }

     private:
        // An atomically updated list (also known as a mailbox) of pending operations
        _Operation_type * volatile _M_pPending_operations;

        // Controls thread access to _M_handle_operations
        volatile long _M_handler_busy;

        // The method that handles the operations
        _Handler_type _M_handle_operations;

        // Trigger the handling of operations when the handler is free
        void _Start_handle_operations()
        {
            _Operation_type * _POp_list;

            // Acquire the _M_handler_busy flag. Only one thread can possibly spin here at a time
            ::Concurrency::details::_SpinWaitBackoffNone _SpinWait;
            while (_M_handler_busy == 1)
            {
                _SpinWait._SpinOnce();
            }

            long _OldVal = _InterlockedExchange(&_M_handler_busy, 1);
            _CONCRT_ASSERT(_OldVal == 0);

            // Get the list of pending operations
            _POp_list = reinterpret_cast<_Operation_type *>(_InterlockedExchangePointer(reinterpret_cast<void * volatile *>(&_M_pPending_operations), nullptr));

            // Handle all the operations
            _M_handle_operations(_POp_list);

            // Release the handler
            _OldVal = _InterlockedExchange(&_M_handler_busy, 0);
            _CONCRT_ASSERT(_OldVal == 1);
        }
    };

} // namespace details

/// <summary>
///     The <c>concurrent_priority_queue</c> class is a container that allows multiple threads to concurrently push and pop items.
///     Items are popped in priority order where priority is determined by a functor supplied as a template argument.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the elements to be stored in the priority queue.
/// </typeparam>
/// <typeparam name="_Compare">
///     The type of the function object that can compare two element values as sort keys to determine their relative order
///     in the priority queue. This argument is optional and the binary predicate <c>less&lt;</c><typeparamref name="_Ty"/><c>&gt;</c>
///     is the default value.
/// </typeparam>
/// <typeparam name="_Ax">
///     The type that represents the stored allocator object that encapsulates details about the allocation and
///     deallocation of memory for the concurrent priority queue. This argument is optional and the default value is
///     <c>allocator&lt;</c><typeparamref name="_Ty"/><c>&gt;</c>.
/// </typeparam>
/// <remarks>
///     For detailed information on the <c>concurrent_priority_queue</c> class, see <see cref="Parallel Containers and Objects"/>.
/// </remarks>
/// <seealso cref="Parallel Containers and Objects"/>
/**/
template <typename _Ty, typename _Compare=::std::less<_Ty>, typename _Ax = ::std::allocator<_Ty>>
class concurrent_priority_queue
{
public:

    /// <summary>
    ///     A type that represents the data type stored in a concurrent priority queue.
    /// </summary>
    /**/
    typedef _Ty value_type;

    /// <summary>
    ///     A type that represents a reference to an element of the type stored in a concurrent priority queue.
    /// </summary>
    /**/
    typedef _Ty& reference;

    /// <summary>
    ///     A type that represents a const reference to an element of the type stored in a concurrent priority queue.
    /// </summary>
    /**/
    typedef const _Ty& const_reference;

    /// <summary>
    ///     A type that counts the number of elements in a concurrent priority queue.
    /// </summary>
    /**/
    typedef size_t size_type;

    /// <summary>
    ///     A type that represents the allocator class for the concurrent priority queue.
    /// </summary>
    /**/
    typedef _Ax allocator_type;

    /// <summary>
    ///     Constructs a concurrent priority queue.
    /// </summary>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the priority queue.
    ///     <para>The first constructor specifies an empty initial priority queue and optionally specifies an allocator.</para>
    ///     <para>The second constructor specifies a priority queue with an initial capacity <paramref name="_Init_capacity"/> and optionally specifies
    ///     an allocator.</para>
    ///     <para>The third constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>) and
    ///     optionally specifies an allocator.</para>
    ///     <para>The fourth and fifth constructors specify a copy of the priority queue <paramref name="_Src"/>.</para>
    ///     <para>The sixth and seventh constructors specify a move of the priority queue <paramref name="_Src"/>.</para>
    /// </remarks>
    /**/
    explicit concurrent_priority_queue(const allocator_type& _Al = allocator_type()) : _M_mark(0), _M_size(0), _M_data(_Al)
    {
        _M_my_aggregator._Initialize_handler(_My_functor_type(this));
    }

    /// <summary>
    ///     Constructs a concurrent priority queue.
    /// </summary>
    /// <param name="_Init_capacity">
    ///     The initial capacity of the <c>concurrent_priority_queue</c> object.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the priority queue.
    ///     <para>The first constructor specifies an empty initial priority queue and optionally specifies an allocator.</para>
    ///     <para>The second constructor specifies a priority queue with an initial capacity <paramref name="_Init_capacity"/> and optionally specifies
    ///     an allocator.</para>
    ///     <para>The third constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>) and
    ///     optionally specifies an allocator.</para>
    ///     <para>The fourth and fifth constructors specify a copy of the priority queue <paramref name="_Src"/>.</para>
    ///     <para>The sixth and seventh constructors specify a move of the priority queue <paramref name="_Src"/>.</para>
    /// </remarks>
    /**/
    explicit concurrent_priority_queue(size_type _Init_capacity, const allocator_type& _Al = allocator_type()) : _M_mark(0), _M_size(0), _M_data(_Al)
    {
        _M_data.reserve(_Init_capacity);
        _M_my_aggregator._Initialize_handler(_My_functor_type(this));
    }

    /// <summary>
    ///     Constructs a concurrent priority queue.
    /// </summary>
    /// <typeparam name="_InputIterator">
    ///     The type of the input iterator.
    /// </typeparam>
    /// <param name="_Begin">
    ///     The position of the first element in the range of elements to be copied.
    /// </param>
    /// <param name="_End">
    ///     The position of the first element beyond the range of elements to be copied.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the priority queue.
    ///     <para>The first constructor specifies an empty initial priority queue and optionally specifies an allocator.</para>
    ///     <para>The second constructor specifies a priority queue with an initial capacity <paramref name="_Init_capacity"/> and optionally specifies
    ///     an allocator.</para>
    ///     <para>The third constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>) and
    ///     optionally specifies an allocator.</para>
    ///     <para>The fourth and fifth constructors specify a copy of the priority queue <paramref name="_Src"/>.</para>
    ///     <para>The sixth and seventh constructors specify a move of the priority queue <paramref name="_Src"/>.</para>
    /// </remarks>
    /**/
    template<typename _InputIterator>
    concurrent_priority_queue(_InputIterator _Begin, _InputIterator _End, const allocator_type& _Al = allocator_type()) : _M_data(_Begin, _End, _Al)
    {
        // Set the mark to 0 to indicate that nothing is sorted.
        _M_mark = 0;
        _M_size = _M_data.size();
        _M_my_aggregator._Initialize_handler(_My_functor_type(this));
        _Heapify();
    }

    /// <summary>
    ///     Constructs a concurrent priority queue.
    /// </summary>
    /// <param name="_Src">
    ///     The source <c>concurrent_priority_queue</c> object to copy or move elements from.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the priority queue.
    ///     <para>The first constructor specifies an empty initial priority queue and optionally specifies an allocator.</para>
    ///     <para>The second constructor specifies a priority queue with an initial capacity <paramref name="_Init_capacity"/> and optionally specifies
    ///     an allocator.</para>
    ///     <para>The third constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>) and
    ///     optionally specifies an allocator.</para>
    ///     <para>The fourth and fifth constructors specify a copy of the priority queue <paramref name="_Src"/>.</para>
    ///     <para>The sixth and seventh constructors specify a move of the priority queue <paramref name="_Src"/>.</para>
    /// </remarks>
    /**/
    concurrent_priority_queue(const concurrent_priority_queue& _Src) : _M_mark(_Src._M_mark), _M_size(_Src._M_size), _M_data(_Src._M_data)
    {
        _M_my_aggregator._Initialize_handler(_My_functor_type(this));
        _Heapify();
    }

    /// <summary>
    ///     Constructs a concurrent priority queue.
    /// </summary>
    /// <param name="_Src">
    ///     The source <c>concurrent_priority_queue</c> object to copy or move elements from.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the priority queue.
    ///     <para>The first constructor specifies an empty initial priority queue and optionally specifies an allocator.</para>
    ///     <para>The second constructor specifies a priority queue with an initial capacity <paramref name="_Init_capacity"/> and optionally specifies
    ///     an allocator.</para>
    ///     <para>The third constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>) and
    ///     optionally specifies an allocator.</para>
    ///     <para>The fourth and fifth constructors specify a copy of the priority queue <paramref name="_Src"/>.</para>
    ///     <para>The sixth and seventh constructors specify a move of the priority queue <paramref name="_Src"/>.</para>
    /// </remarks>
    /**/
    concurrent_priority_queue(const concurrent_priority_queue& _Src, const allocator_type& _Al) : _M_mark(_Src._M_mark), _M_data(_Src._M_data.begin(), _Src._M_data.end(), _Al)
    {
        _M_size = _M_data.size();
        _M_my_aggregator._Initialize_handler(_My_functor_type(this));
        _Heapify();
    }

    /// <summary>
    ///     Constructs a concurrent priority queue.
    /// </summary>
    /// <param name="_Src">
    ///     The source <c>concurrent_priority_queue</c> object to copy or move elements from.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the priority queue.
    ///     <para>The first constructor specifies an empty initial priority queue and optionally specifies an allocator.</para>
    ///     <para>The second constructor specifies a priority queue with an initial capacity <paramref name="_Init_capacity"/> and optionally specifies
    ///     an allocator.</para>
    ///     <para>The third constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>) and
    ///     optionally specifies an allocator.</para>
    ///     <para>The fourth and fifth constructors specify a copy of the priority queue <paramref name="_Src"/>.</para>
    ///     <para>The sixth and seventh constructors specify a move of the priority queue <paramref name="_Src"/>.</para>
    /// </remarks>
    /**/
    concurrent_priority_queue(concurrent_priority_queue&& _Src) : _M_mark(_Src._M_mark), _M_size(_Src._M_size), _M_data(::std::move(_Src._M_data))
    {
        // Set _M_mark and _M_size for the argument to 0 because its data has been moved over to this priority queue.
        _Src._M_mark = 0;
        _Src._M_size = 0;
        _M_my_aggregator._Initialize_handler(_My_functor_type(this));
        _Heapify();
    }

    /// <summary>
    ///     Constructs a concurrent priority queue.
    /// </summary>
    /// <param name="_Src">
    ///     The source <c>concurrent_priority_queue</c> object to copy or move elements from.
    /// </param>
    /// <param name="_Al">
    ///     The allocator class to use with this object.
    /// </param>
    /// <remarks>
    ///     All constructors store an allocator object <paramref name="_Al"/> and initialize the priority queue.
    ///     <para>The first constructor specifies an empty initial priority queue and optionally specifies an allocator.</para>
    ///     <para>The second constructor specifies a priority queue with an initial capacity <paramref name="_Init_capacity"/> and optionally specifies
    ///     an allocator.</para>
    ///     <para>The third constructor specifies values supplied by the iterator range [<paramref name="_Begin"/>, <paramref name="_End"/>) and
    ///     optionally specifies an allocator.</para>
    ///     <para>The fourth and fifth constructors specify a copy of the priority queue <paramref name="_Src"/>.</para>
    ///     <para>The sixth and seventh constructors specify a move of the priority queue <paramref name="_Src"/>.</para>
    /// </remarks>
    /**/
    concurrent_priority_queue(concurrent_priority_queue&& _Src, const allocator_type& _Al) : _M_mark(_Src._M_mark), _M_size(_Src._M_size), _M_data(::std::move(_Src._M_data), _Al)
    {
        // Set _M_mark and _M_size for the argument to 0 because its data has been moved over to this priority queue.
        _Src._M_mark = 0;
        _Src._M_size = 0;
        _M_my_aggregator._Initialize_handler(_My_functor_type(this));
        _Heapify();
    }

    /// <summary>
    ///     Assigns the contents of another <c>concurrent_priority_queue</c> object to this one. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Src">
    ///     The source <c>concurrent_priority_queue</c> object.
    /// </param>
    /// <returns>
    ///     A reference to this <c>concurrent_priority_queue</c> object.
    /// </returns>
    /**/
    concurrent_priority_queue& operator=(const concurrent_priority_queue& _Src)
    {
        if (this != &_Src)
        {
            ::std::vector<value_type, allocator_type>(_Src._M_data.begin(), _Src._M_data.end(), _Src._M_data.get_allocator()).swap(_M_data);
            _M_mark = _Src._M_mark;
            _M_size = _Src._M_size;
        }
        return *this;
    }

    /// <summary>
    ///     Assigns the contents of another <c>concurrent_priority_queue</c> object to this one. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Src">
    ///     The source <c>concurrent_priority_queue</c> object.
    /// </param>
    /// <returns>
    ///     A reference to this <c>concurrent_priority_queue</c> object.
    /// </returns>
    /**/
    concurrent_priority_queue& operator=(concurrent_priority_queue&& _Src)
    {
        if (this != &_Src)
        {
            _M_data = ::std::move(_Src._M_data);
            _M_mark = _Src._M_mark;
            _M_size = _Src._M_size;
            // Set _M_mark and _M_size for the arguement to 0 because its data has been moved over to this priority queue.
            _Src._M_mark = 0;
            _Src._M_size = 0;
        }
        return *this;
    }

    /// <summary>
    ///     Tests if the concurrent priority queue is empty at the time this method is called. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the priority queue was empty at the moment the function was called, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool empty() const
    {
        return (_M_size == 0);
    }

    /// <summary>
    ///     Returns the number of elements in the concurrent priority queue. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     The number of elements in this <c>concurrent_priority_queue</c> object.
    /// </returns>
    /// <remarks>
    ///     The returned size is guaranteed to include all elements added by calls to the function <c>push</c>.
    ///     However, it may not reflect results of pending concurrent operations.
    /// </remarks>
    /**/
    size_type size() const
    {
        return _M_size;
    }

    /// <summary>
    ///     Adds an element to the concurrent priority queue. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Elem">
    ///     The element to be added to the concurrent priority queue.
    /// </param>
    void push(const value_type& _Elem)
    {
        _Cpq_operation _Op_data(_Elem, _PUSH_OP_COPY);
        _M_my_aggregator._Execute(&_Op_data);
        if (_Op_data._M_status == _FAILED)
        {
            // Rethrow the saved exception.
            ::std::rethrow_exception(_Op_data._M_exception_ptr);
        }
    }

    /// <summary>
    ///     Adds an element to the concurrent priority queue. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Elem">
    ///     The element to be added to the concurrent priority queue.
    /// </param>
    void push(value_type&& _Elem)
    {
        _Cpq_operation _Op_data(_Elem, _PUSH_OP_MOVE);
        _M_my_aggregator._Execute(&_Op_data);
        if (_Op_data._M_status == _FAILED)
        {
            // Rethrow the saved exception
            ::std::rethrow_exception(_Op_data._M_exception_ptr);
        }
    }

    /// <summary>
    ///     Removes and returns the highest priority element from the queue if the queue is non-empty. This method is concurrency-safe.
    /// </summary>
    /// <param name="_Elem">
    ///     A reference to a variable that will be populated with the highest priority element, if the queue is non-empty.
    /// </param>
    /// <returns>
    ///     <c>true</c> if a value was popped, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool try_pop(reference _Elem)
    {
        _Cpq_operation _Op_data(_POP_OP);
        _Op_data._M_elem = &_Elem;
        _M_my_aggregator._Execute(&_Op_data);
        return (_Op_data._M_status == _SUCCEEDED);
    }

    /// <summary>
    ///     Erases all elements in the concurrent priority. This method is not concurrency-safe.
    /// </summary>
    /// <remarks>
    ///     <c>clear</c> is not concurrency-safe. You must ensure that no other threads are invoking methods
    ///     on the concurrent priority queue when you call this method. <c>clear</c> does not free memory.
    /// </remarks>
    /**/
    void clear()
    {
        _M_data.clear();
        _M_mark = 0;
        _M_size = 0;
    }

    /// <summary>
    ///     Swaps the contents of two concurrent priority queues. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Queue">
    ///     The <c>concurrent_priority_queue</c> object to swap contents with.
    /// </param>
    /**/
    void swap(concurrent_priority_queue& _Queue)
    {
        _M_data.swap(_Queue._M_data);
        ::std::swap(_M_mark, _Queue._M_mark);
        ::std::swap(_M_size, _Queue._M_size);
    }

    /// <summary>
    ///     Returns a copy of the allocator used to construct the concurrent priority queue. This method is concurrency-safe.
    /// </summary>
    /// <returns>
    ///     A copy of the allocator used to construct the <c>concurrent_priority_queue</c> object.
    /// </returns>
    /**/
    allocator_type get_allocator() const { return _M_data.get_allocator(); }

 private:
    enum _Operation_type {_INVALID_OP, _PUSH_OP_COPY, _PUSH_OP_MOVE, _POP_OP};
    enum _Operation_status { _WAIT=0, _SUCCEEDED, _FAILED };

    class _Cpq_operation : public ::Concurrency::details::_Aggregated_operation<_Cpq_operation>
    {
     public:
        _Operation_type _M_type;
        union
        {
            value_type * _M_elem;
            size_type    _M_size;
        };
        ::std::exception_ptr _M_exception_ptr;

        _Cpq_operation(const_reference _E, _Operation_type _T) :
            _M_type(_T), _M_elem(const_cast<value_type*>(&_E)) {}
        _Cpq_operation(value_type&& _E, _Operation_type _T) :
            _M_type(_T), _M_elem(const_cast<value_type*>(&_E)) {}
        _Cpq_operation(_Operation_type _T) : _M_type(_T) {}
    };

    class _My_functor_type
    {
        concurrent_priority_queue<_Ty, _Compare, _Ax> *_M_PCpq;
     public:
        _My_functor_type() {}
        _My_functor_type(concurrent_priority_queue<_Ty, _Compare, _Ax> * _PCpq) : _M_PCpq(_PCpq) {}
        void operator()(_Cpq_operation* _POp_list)
        {
            _M_PCpq->_M_handle_operations(_POp_list);
        }
    };

    ::Concurrency::details::_Aggregator< _Cpq_operation, _My_functor_type > _M_my_aggregator;

    // Padding added to avoid false sharing
    char _M_padding1[64 /* CACHE_LINE_SIZE */ - sizeof(::Concurrency::details::_Aggregator< _Cpq_operation, _My_functor_type >)];

    // The point at which unsorted elements begin
    size_type _M_mark;

    // Size of the concurrent priority queue. This is cached instead of using vector::size(), because that method is unsafe in the presence
    // of concurrent pushes/pops.
    volatile size_type _M_size;

    // The comparator function to determine relative priority between elements stored in the priority queue.
    _Compare _M_compare;

    // Padding added to avoid false sharing
    char _M_padding2[64 /* CACHE_LINE_SIZE */ - sizeof(size_type) - sizeof(_Compare)];

    // Storage for the heap of elements in queue, plus unheapified elements. _M_data has the following structure:
    //
    //     binary unheapified
    //      heap   elements
    //    ____|_______|____
    //    |       |       |
    //    v       v       v
    //    [_|...|_|_|...|_| |...| ]
    //     0       ^       ^       ^
    //             |       |       |__capacity
    //             |       |__size
    //             |__mark
    //
    //    Thus, _M_data stores the binary heap starting at position 0 through _M_mark-1 (it may be empty).  Then there are 0 or more elements
    //    that have not yet been inserted into the heap, in positions _M_mark through size-1.
    ::std::vector<value_type, allocator_type> _M_data;

    /// <summary>
    ///     Handle a batch of operations pending on the concurrent priority queue. Only one thread can execute this routine at a time.
    /// </summary>
    void _M_handle_operations(_Cpq_operation *_POp_list)
    {
        _Cpq_operation *_PTmp, *_PPop_list=nullptr;

        _CONCRT_ASSERT(_M_mark == _M_data.size());

        // First pass processes all constant time operations: pushes, some pops.
        while (_POp_list != nullptr)
        {
            _CONCRT_ASSERT(_POp_list->_M_type != _INVALID_OP);
            _PTmp = _POp_list;
            _POp_list = _POp_list->_M_pNext;
            if (_PTmp->_M_type == _PUSH_OP_COPY)
            {
                try
                {
                    _M_data.push_back(*(_PTmp->_M_elem));
                    ++_M_size;
                    _InterlockedExchange((volatile long *) &_PTmp->_M_status, _SUCCEEDED);
                }
                catch (...)
                {
                    _PTmp->_M_exception_ptr = ::std::current_exception();
                    _InterlockedExchange((volatile long *) &_PTmp->_M_status, _FAILED);
                }
            }
            else if (_PTmp->_M_type == _PUSH_OP_MOVE)
            {
                try
                {
                    _M_data.push_back(::std::move(*(_PTmp->_M_elem)));
                    ++_M_size;
                    _InterlockedExchange((volatile long *) &_PTmp->_M_status, _SUCCEEDED);
                }
                catch (...)
                {
                    _PTmp->_M_exception_ptr = ::std::current_exception();
                    _InterlockedExchange((volatile long *) &_PTmp->_M_status, _FAILED);
                }
            }
            else
            {
                _CONCRT_ASSERT(_PTmp->_M_type == _POP_OP);
                if (_M_mark < _M_size && _M_compare(_M_data[0], _M_data[_M_size - 1]))
                {
                    // There are newly pushed elems and the last one is higher than top
                    // Because we're going to pop the last element, we can move the _M_data instead of copying it.
                    *(_PTmp->_M_elem) = ::std::move(_M_data[_M_size - 1]);
                    --_M_size;
                    _InterlockedExchange((volatile long *) &_PTmp->_M_status, _SUCCEEDED);
                    _M_data.pop_back();

                    _CONCRT_ASSERT(_M_mark <= _M_size);
                }
                else
                {
                    // No convenient item to pop; postpone
                    _PTmp->_M_pNext = _PPop_list;
                    _PPop_list = _PTmp;
                }
            }
        }

        // Second pass processes pop operations.
        while (_PPop_list != nullptr)
        {
            _PTmp = _PPop_list;
            _PPop_list = _PPop_list->_M_pNext;
            _CONCRT_ASSERT(_PTmp->_M_type == _POP_OP);

            if (_M_size == 0)
            {
                _InterlockedExchange((volatile long *) &_PTmp->_M_status, _FAILED);
            }
            else
            {
                _CONCRT_ASSERT(_M_mark <= _M_size);

                if (_M_mark < _M_size && _M_compare(_M_data[0], _M_data[_M_size - 1]))
                {
                    // There are newly pushed elems and the last one is higher than top.
                    // Because we're going to pop the last element, we can move the _M_data instead of copying it.
                    *(_PTmp->_M_elem) = ::std::move(_M_data[_M_size - 1]); // copy the _M_data
                    --_M_size;
                    _InterlockedExchange((volatile long *) &_PTmp->_M_status, _SUCCEEDED);
                    _M_data.pop_back();
                }
                else
                {
                    // Extract top and push last element down heap.
                    *(_PTmp->_M_elem) = ::std::move(_M_data[0]); // copy the _M_data
                    --_M_size;
                    _InterlockedExchange((volatile long *) &_PTmp->_M_status, _SUCCEEDED);
                    _Reheap();
                }
            }
        }
        _CONCRT_ASSERT(_M_size == _M_data.size());
        // _Heapify any leftover pushed elements before doing the next batch of operations.
        if (_M_mark < _M_size)
        {
            _Heapify();
        }
        _CONCRT_ASSERT(_M_mark == _M_size);
    }

    /// <summary>
    ///     Merge unsorted elements into heap.
    /// </summary>
    void _Heapify()
    {
        if (_M_mark == 0 && _M_size > 0)
        {
            _M_mark = 1;
        }

        for (; _M_mark < _M_size; ++_M_mark)
        {
            // for each unheapified element under size
            size_type _Cur_pos = _M_mark;
            value_type _To_place = ::std::move(_M_data[_M_mark]);
            do
            { // push _To_place up the heap
                size_type _Parent = (_Cur_pos - 1) >> 1;
                if (!_M_compare(_M_data[_Parent], _To_place))
                {
                    break;
                }
                _M_data[_Cur_pos] = ::std::move(_M_data[_Parent]);
                _Cur_pos = _Parent;

            } while( _Cur_pos != 0 );

            _M_data[_Cur_pos] = ::std::move(_To_place);
        }
    }

    /// <summary>
    ///     Re-_Heapify by pushing last element down the heap from the root.
    /// </summary>
    void _Reheap()
    {
        size_type _Cur_pos=0, _Child=1;
        // Use _M_data.size() instead of _M_size throughout this function, because _M_size has already
        // been decremented to account for the pop right before Reheap was invoked.
        while (_Child < _M_mark)
        {
            size_type _Target = _Child;
            if (_Child + 1 < _M_mark && _M_compare(_M_data[_Child], _M_data[_Child + 1]))
            {
                ++_Target;
            }
            // _Target now has the higher priority _Child.
            if (_M_compare(_M_data[_Target], _M_data[_M_data.size() - 1]))
            {
                break;
            }

            _M_data[_Cur_pos] = ::std::move(_M_data[_Target]);
            _Cur_pos = _Target;
            _Child = (_Cur_pos << 1) + 1;
        }

        _M_data[_Cur_pos] = ::std::move(_M_data[_M_data.size() - 1]);
        _M_data.pop_back();

        if (_M_mark > _M_data.size())
        {
            _M_mark = _M_data.size();
        }
    }
};

} // namespace Concurrency

namespace concurrency = ::Concurrency;

#pragma warning (pop)
#pragma pack(pop)
