/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* internal_concurrent_hash.h
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/
#pragma once

#include <utility>
#include "internal_split_ordered_list.h"
#include <concrt.h>

#pragma pack(push,_CRT_PACKING)

namespace Concurrency
{
namespace details
{
// Template class for hash compare
template<typename _Key_type, typename _Hasher, typename _Key_equality>
class _Hash_compare
{
public:
    typedef _Hasher hasher;

    _Hash_compare()
    {
    }

    _Hash_compare(hasher _Hasharg) : _M_hash_object(_Hasharg)
    {
    }

    _Hash_compare(hasher _Hasharg, _Key_equality _Keyeqarg) : _M_hash_object(_Hasharg), _M_key_compare_object(_Keyeqarg)
    {
    }

    size_t operator()(const _Key_type& _Keyval) const
    {
        return ((size_t)_M_hash_object(_Keyval));
    }

    bool operator()(const _Key_type& _Keyval1, const _Key_type& _Keyval2) const
    {
        return (!_M_key_compare_object(_Keyval1, _Keyval2));
    }

    hasher        _M_hash_object;        // The hash object
    _Key_equality _M_key_compare_object; // The equality comparator object
};

// An efficient implementation of the _Reverse function utilizes a 2^8 or 2^16 lookup table holding the
// bit-reversed values of [0..2^8 - 1] or [0..2^16 - 1] respectively. Those values can also be computed
// on the fly at a slightly higher cost.
extern _CONCRTIMP const unsigned char _Byte_reverse_table[];

// Given a byte, reverses it
inline unsigned char _Reverse_byte(unsigned char _Original_byte)
{
    // return ((_Original_byte * 0x80200802ULL) & 0x0884422110ULL) * 0x0101010101ULL >> 32;
    return _Byte_reverse_table[_Original_byte];
}

// Finds the most significant bit in a size_t
inline unsigned char _Get_msb(size_t _Mask)
{
    unsigned long _Index = 0;

#if (defined (_M_IX86) || defined (_M_ARM))
    _BitScanReverse(&_Index, _Mask);
#else  /* (defined (_M_IX86) || defined (_M_ARM)) */
    _BitScanReverse64(&_Index, _Mask);
#endif  /* (defined (_M_IX86) || defined (_M_ARM)) */

    return (unsigned char) _Index;
}

#pragma warning(push)
#pragma warning(disable: 4127) // Warning 4127 -- while (true) has a constant expression in it

template <typename _Traits>
class _Concurrent_hash : public _Traits
{
public:
    // Type definitions
    typedef _Concurrent_hash<_Traits> _Mytype;
    typedef typename _Split_ordered_list<typename _Traits::value_type, typename _Traits::allocator_type> _Mylist;
    typedef typename _Traits::_Key_compare _Key_compare;
    typedef typename _Traits::_Value_compare _Value_compare;

    typedef typename _Traits::value_type value_type;
    typedef typename _Traits::key_type key_type;
    typedef typename _Traits::allocator_type allocator_type;
    typedef value_type& reference;
    typedef const value_type& const_reference;
    typedef ::std::allocator_traits<allocator_type> _Al_traits;
    typedef typename _Al_traits::pointer pointer;
    typedef typename _Al_traits::const_pointer const_pointer;

    typedef typename _Al_traits::size_type size_type;
    typedef typename _Al_traits::difference_type difference_type;

    typedef ::std::conditional_t<::std::is_same_v<key_type, value_type>, typename _Mylist::const_iterator, typename _Mylist::iterator> iterator;
    typedef typename _Mylist::const_iterator const_iterator;
    typedef iterator local_iterator;
    typedef const_iterator const_local_iterator;

    typedef typename _Mylist::_Nodeptr _Nodeptr;

    // Iterators that walk the entire split-order list, including dummy nodes
    typedef typename _Mylist::_Full_iterator _Full_iterator;
    typedef typename _Mylist::_Full_const_iterator _Full_const_iterator;

    typedef typename _Al_traits::template rebind_traits<_Full_iterator> _Alfi_traits;

    static constexpr size_type _Initial_bucket_number = 8;                               // Initial number of buckets
    static constexpr size_type _Initial_bucket_load = 4;                                 // Initial maximum number of elements per bucket
    static constexpr size_type _Pointers_per_table = sizeof(size_type) * 8;              // One bucket segment per bit

    // Constructors/Destructors
    _Concurrent_hash(size_type _Number_of_buckets = _Initial_bucket_number, const _Key_compare& _Parg = _Key_compare(), const allocator_type& _Allocator = allocator_type())
        : _Traits(_Parg), _M_split_ordered_list(_Allocator), _M_allocator(_Allocator), _M_number_of_buckets(_Number_of_buckets), _M_maximum_bucket_size((float) _Initial_bucket_load)
    {
        _Init();
    }

    _Concurrent_hash(const _Concurrent_hash& _Right, const allocator_type& _Allocator) : _Traits(_Right._M_comparator), _M_split_ordered_list(_Allocator), _M_allocator(_Allocator)
    {
        _Copy(_Right);
    }

    _Concurrent_hash(const _Concurrent_hash& _Right) : _Traits(_Right._M_comparator), _M_split_ordered_list(_Right.get_allocator()), _M_allocator(_Right.get_allocator())
    {
        _Init();
        _Copy(_Right);
    }

    _Concurrent_hash(_Concurrent_hash&& _Right) : _Traits(_Right._M_comparator), _M_split_ordered_list(_Right.get_allocator()), _M_allocator(_Right.get_allocator()),
        _M_number_of_buckets(_Initial_bucket_number), _M_maximum_bucket_size((float) _Initial_bucket_load)
    {
        _Init();
        swap(_Right);
    }

    _Concurrent_hash& operator=(const _Concurrent_hash& _Right)
    {
        if (this != &_Right)
        {
            _Copy(_Right);
        }

        return (*this);
    }

    _Concurrent_hash& operator=(_Concurrent_hash&& _Right)
    {
        if (this != &_Right)
        {
            clear();
            swap(_Right);
        }

        return (*this);
    }

    ~_Concurrent_hash()
    {
        // Delete all node segments
        for (size_type _Index = 0; _Index < _Pointers_per_table; ++_Index)
        {
            if (_M_buckets[_Index] != nullptr)
            {
                size_type _Seg_size = _Segment_size(_Index);
                for (size_type _Index2 = 0; _Index2 < _Seg_size; ++_Index2)
                {
                    _Alfi_traits::destroy(_M_allocator, &_M_buckets[_Index][_Index2]);
                }
                _M_allocator.deallocate(_M_buckets[_Index], _Seg_size);
            }
        }
    }

    static size_type __cdecl _Segment_index_of( size_type _Index )
    {
        return size_type( _Get_msb( _Index|1 ) );
    }

    static size_type _Segment_base( size_type _K )
    {
        return (size_type(1)<<_K & ~size_type(1));
    }

    static size_type _Segment_size( size_type _K )
    {
        return _K ? size_type(1)<<_K : 2;
    }

    /// <summary>
    ///     Returns the stored allocator object for this concurrent container. This method is concurrency safe.
    /// </summary>
    /// <returns>
    ///     The stored allocator object for this concurrent container.
    /// </returns>
    /**/
    allocator_type get_allocator() const
    {
        return _M_split_ordered_list.get_allocator();
    }

    /// <summary>
    ///     Tests whether no elements are present. This method is concurrency safe.
    /// </summary>
    /// <remarks>
    ///     In the presence of concurrent inserts, whether or not the concurrent container is empty may change immediately
    ///     after calling this function, before the return value is even read.
    /// </remarks>
    /// <returns>
    ///     <c>true</c> if the concurrent container is empty, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool empty() const
    {
        return _M_split_ordered_list.empty();
    }

    /// <summary>
    ///     Returns the number of elements in this concurrent container. This method is concurrency safe.
    /// </summary>
    /// <remarks>
    ///     In the presence of concurrent inserts, the number of elements in the concurrent container may change immediately
    ///     after calling this function, before the return value is even read.
    /// </remarks>
    /// <returns>
    ///     The number of items in the container.
    /// </returns>
    /**/
    size_type size() const
    {
        return _M_split_ordered_list.size();
    }

    /// <summary>
    ///     Returns the maximum size of the concurrent container, determined by the allocator. This method is concurrency safe.
    /// </summary>
    /// <remarks>
    ///     This upper bound value may actually be higher than what the container can actually hold.
    /// </remarks>
    /// <returns>
    ///     The maximum number of elements that can be inserted into this concurrent container.
    /// </returns>
    /**/
    size_type max_size() const
    {
        return _M_split_ordered_list.max_size();
    }

    /// <summary>
    ///     Returns an iterator pointing to the first element in the concurrent container. This method is concurrency safe.
    /// </summary>
    /// <returns>
    ///     An iterator to the first element in the concurrent container.
    /// </returns>
    /**/
    iterator begin()
    {
        return _M_split_ordered_list.begin();
    }

    /// <summary>
    ///     Returns an iterator pointing to the first element in the concurrent container. This method is concurrency safe.
    /// </summary>
    /// <returns>
    ///     An iterator to the first element in the concurrent container.
    /// </returns>
    /**/
    const_iterator begin() const
    {
        return _M_split_ordered_list.begin();
    }

    /// <summary>
    ///     Returns an iterator pointing to the location succeeding the last element in the concurrent container.
    ///     This method is concurrency safe.
    /// </summary>
    /// <returns>
    ///     An iterator to the location succeeding the last element in the concurrent container.
    /// </returns>
    /**/
    iterator end()
    {
        return _M_split_ordered_list.end();
    }

    /// <summary>
    ///     Returns a const_iterator pointing to the location succeeding the last element in the concurrent container.
    ///     This method is concurrency safe.
    /// </summary>
    /// <returns>
    ///     A const_iterator to the location succeeding the last element in the concurrent container.
    /// </returns>
    /**/
    const_iterator end() const
    {
        return _M_split_ordered_list.end();
    }

    /// <summary>
    ///     Returns a const iterator pointing to the first element in the concurrent container. This method is concurrency safe.
    /// </summary>
    /// <returns>
    ///     A const iterator to the first element in the concurrent container.
    /// </returns>
    /**/
    const_iterator cbegin() const
    {
        return _M_split_ordered_list.cbegin();
    }

    /// <summary>
    ///     Returns a const iterator pointing to the location succeeding the last element in the concurrent container. This method is concurrency safe.
    /// </summary>
    /// <returns>
    ///     A const iterator to the location succeeding the last element in the concurrent container.
    /// </returns>
    /**/
    const_iterator cend() const
    {
        return _M_split_ordered_list.cend();
    }

    /// <summary>
    ///     Removes elements from the container at specified positions. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Where">
    ///     The iterator position to erase from.
    /// </param>
    /// <remarks>
    ///     The first member function removes the element of the controlled sequence pointed to by <paramref name="_Where"/>. The second
    ///     member function removes the elements in the range [<paramref name="_First"/>, <paramref name="_Last"/>).
    ///     <para>The third member function removes the elements in the range delimited by equal_range(<paramref name="_Keyval"/>)</see>. </para>
    /// </remarks>
    /// <returns>
    ///     The first two member functions return an iterator that designates the first element remaining beyond any elements removed,
    ///     or end() if no such element exists. The third member function returns the number of elements it removes.
    /// </returns>
    /**/
    iterator unsafe_erase(const_iterator _Where)
    {
        if (_Where == end())
        {
            return end();
        }

        return _Erase(_Where);
    }

    /// <summary>
    ///     Removes elements from the container at specified positions. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_First">
    ///     The position of the first element in the range of elements to be erased.
    /// </param>
    /// <param name="_Last">
    ///     The position of the first element beyond the range of elements to be erased.
    /// </param>
    /// <remarks>
    ///     The first member function removes the element of the controlled sequence pointed to by <paramref name="_Where"/>. The second
    ///     member function removes the elements in the range [<paramref name="_First"/>, <paramref name="_Last"/>).
    ///     <para>The third member function removes the elements in the range delimited by equal_range(<paramref name="_Keyval"/>)</see>. </para>
    /// </remarks>
    /// <returns>
    ///     The first two member functions return an iterator that designates the first element remaining beyond any elements removed,
    ///     or end() if no such element exists. The third member function returns the number of elements it removes.
    /// </returns>
    /**/
    iterator unsafe_erase(const_iterator _First, const_iterator _Last)
    {
        while (_First != _Last)
        {
            unsafe_erase(_First++);
        }

        return _M_split_ordered_list._Get_iterator(_First);
    }

    /// <summary>
    ///     Removes elements from the container at specified positions. This method is not concurrency-safe.
    /// </summary>
    /// <param name="_Keyval">
    ///     The key value to erase.
    /// </param>
    /// <remarks>
    ///     The first member function removes the element of the controlled sequence pointed to by <paramref name="_Where"/>. The second
    ///     member function removes the elements in the range [<paramref name="_First"/>, <paramref name="_Last"/>).
    ///     <para>The third member function removes the elements in the range delimited by equal_range(<paramref name="_Keyval"/>)</see>. </para>
    /// </remarks>
    /// <returns>
    ///     The first two member functions return an iterator that designates the first element remaining beyond any elements removed,
    ///     or end() if no such element exists. The third member function returns the number of elements it removes.
    /// </returns>
    /**/
    size_type unsafe_erase(const key_type& _Keyval)
    {
        ::std::pair<iterator, iterator> _Where = equal_range(_Keyval);
        size_type _Count = _Distance(_Where.first, _Where.second);
        unsafe_erase(_Where.first, _Where.second);
        return _Count;
    }

    /// <summary>
    ///     Swaps the contents of two concurrent containers. This function is not concurrency safe.
    /// </summary>
    /// <param name="_Right">
    ///     The container to swap elements from.
    /// </param>
    /// <remarks>
    ///     The member function throws an <see cref="invalid_argument Class">invalid_argument</see> exception if the swap is being
    ///     performed on unequal allocators.
    /// </remarks>
    /**/
    void swap(_Concurrent_hash& _Right)
    {
        if (this != &_Right)
        {
            using ::std::swap;
            swap(this->_M_comparator, _Right._M_comparator); // intentional ADL
            _M_split_ordered_list.swap(_Right._M_split_ordered_list);
            _Swap_buckets(_Right);
            ::std::swap(_M_number_of_buckets, _Right._M_number_of_buckets);
            ::std::swap(_M_maximum_bucket_size, _Right._M_maximum_bucket_size);
        }
    }

    /// <summary>
    ///     Erases all the elements in the concurrent container. This function is not concurrency safe.
    /// </summary>
    /**/
    void clear()
    {
        // Clear list
        _M_split_ordered_list.clear();

        // Clear buckets
        for (size_type _Index = 0; _Index < _Pointers_per_table; ++_Index)
        {
            if (_M_buckets[_Index] != nullptr)
            {
                size_type _Seg_size = _Segment_size(_Index);
                for (size_type _Index2 = 0; _Index2 < _Seg_size; ++_Index2)
                {
                    _Alfi_traits::destroy(_M_allocator, &_M_buckets[_Index][_Index2]);
                }
                _M_allocator.deallocate(_M_buckets[_Index], _Seg_size);
            }
        }

        // memset all the buckets to zero and initialize the dummy node 0
        _Init();
    }

    /// <summary>
    ///     Finds an element that matches a specified key. This function is concurrency safe.
    /// </summary>
    /// <param name="_Keyval">
    ///     The key value to search for.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the location of the first element that matched the key provided,
    ///     or the iterator <c>end()</c> if no such element exists.
    /// </returns>
    /**/
    iterator find(const key_type& _Keyval)
    {
        return _Find(_Keyval);
    }

    /// <summary>
    ///     Finds an element that matches a specified key. This function is concurrency safe.
    /// </summary>
    /// <param name="_Keyval">
    ///     The key value to search for.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the location of the first element that matched the key provided,
    ///     or the iterator <c>end()</c> if no such element exists.
    /// </returns>
    /**/
    const_iterator find(const key_type& _Keyval) const
    {
        return _Find(_Keyval);
    }

    /// <summary>
    ///     Counts the number of elements matching a specified key. This function is concurrency safe.
    /// </summary>
    /// <param name="_Keyval">
    ///     The key to search for.
    /// </param>
    /// <returns>
    ///     The number of times number of times the key appears in the container.
    /// </returns>
    /**/
    size_type count(const key_type& _Keyval) const
    {
        size_type _Count = 0;
        const_iterator _It = _Find(_Keyval);
        for (;_It != end() && !this->_M_comparator(this->_Key_function(*_It), _Keyval); ++_It)
        {
            ++_Count;
        }
        return _Count;
    }

    /// <summary>
    ///     Finds a range that matches a specified key. This function is concurrency safe.
    /// </summary>
    /// <param name="_Keyval">
    ///     The key value to search for.
    /// </param>
    /// <returns>
    ///     A <see cref="pair Class">pair</see> where the first element is an iterator to the beginning and the second element
    ///     is an iterator to the end of the range.
    /// </returns>
    /// <remarks>
    ///     <para>It is possible for concurrent inserts to cause additional keys to be inserted after the begin iterator and
    ///     before the end iterator.</para>
    /// </remarks>
    /**/
    ::std::pair<iterator, iterator> equal_range(const key_type& _Keyval)
    {
        return _Equal_range(_Keyval);
    }

    /// <summary>
    ///     Finds a range that matches a specified key. This function is concurrency safe.
    /// </summary>
    /// <param name="_Keyval">
    ///     The key value to search for.
    /// </param>
    /// <returns>
    ///     A <see cref="pair Class">pair</see> where the first element is an iterator to the beginning and the second element
    ///     is an iterator to the end of the range.
    /// </returns>
    /// <remarks>
    ///     <para>It is possible for concurrent inserts to cause additional keys to be inserted after the begin iterator and
    ///     before the end iterator.</para>
    /// </remarks>
    /**/
    ::std::pair<const_iterator, const_iterator> equal_range(const key_type& _Keyval) const
    {
        return _Equal_range(_Keyval);
    }

    /// <summary>
    ///     Returns the current number of buckets in this container.
    /// </summary>
    /// <returns>
    ///     The current number of buckets in this container.
    /// </returns>
    /**/
    size_type unsafe_bucket_count() const
    {
        return _M_number_of_buckets;
    }

    /// <summary>
    ///     Returns the maximum number of buckets in this container.
    /// </summary>
    /// <returns>
    ///     The maximum number of buckets in this container.
    /// </returns>
    /**/
    size_type unsafe_max_bucket_count() const
    {
        return _Segment_size(_Pointers_per_table-1);
    }

    /// <summary>
    ///     Returns the number of items in a specific bucket of this container.
    /// </summary>
    /// <param name="_Bucket">
    ///     The bucket to search for.
    /// </param>
    /// <returns>
    ///     The current number of buckets in this container.
    /// </returns>
    /**/
    size_type unsafe_bucket_size(size_type _Bucket)
    {
        size_type _Count = 0;

        if (!_Is_initialized(_Bucket))
        {
            return _Count;
        }

        _Full_iterator _Iterator = _Get_bucket(_Bucket);
        ++_Iterator;

        for (; _Iterator != _M_split_ordered_list._End() && !_Iterator._Ptr->_Is_dummy(); ++_Iterator)
        {
            ++_Count;
        }

        return _Count;
    }

    /// <summary>
    ///     Returns the bucket index that a specific key maps to in this container.
    /// </summary>
    /// <param name="_Keyval">
    ///     The element key being searched for.
    /// </param>
    /// <returns>
    ///     The bucket index for the key in this container.
    /// </returns>
    /**/
    size_type unsafe_bucket(const key_type& _Keyval) const
    {
        _Split_order_key _Order_key = (_Split_order_key) this->_M_comparator(_Keyval);
        size_type _Bucket = _Order_key % _M_number_of_buckets;
        return _Bucket;
    }

    /// <summary>
    ///     Returns an iterator to the first element in this container for a specific bucket.
    /// </summary>
    /// <param name="_Bucket">
    ///     The bucket index.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the beginning of the bucket.
    /// </returns>
    /**/
    local_iterator unsafe_begin(size_type _Bucket)
    {
        // It is possible the bucket being searched for has not yet been initialized
        if (!_Is_initialized(_Bucket))
        {
            _Initialize_bucket(_Bucket);
        }

        _Full_iterator _Iterator = _Get_bucket(_Bucket);
        return _M_split_ordered_list._Get_first_real_iterator(_Iterator);
    }

    /// <summary>
    ///     Returns an iterator to the first element in this container for a specific bucket.
    /// </summary>
    /// <param name="_Bucket">
    ///     The bucket index.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the beginning of the bucket.
    /// </returns>
    /**/
    const_local_iterator unsafe_begin(size_type _Bucket) const
    {
        // It is possible the bucket being searched for has not yet been initialized
        if (!_Is_initialized(_Bucket))
        {
            const_cast<_Concurrent_hash*>(this)->_Initialize_bucket(_Bucket);
        }

        _Full_const_iterator _Iterator = _Get_bucket(_Bucket);
        return _M_split_ordered_list._Get_first_real_iterator(_Iterator);
    }

    /// <summary>
    ///     Returns an iterator to the last element in this container for a specific bucket.
    /// </summary>
    /// <param name="_Bucket">
    ///     The bucket index.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the end of the bucket.
    /// </returns>
    /**/
    local_iterator unsafe_end(size_type _Bucket)
    {
        // If we've increased the number of buckets, there's a chance the intermediate dummy
        // node marking the end of this bucket has not yet been lazily initialized.
        // Inserting from _M_number_of_buckets/2 to _M_number_of_buckets will recursively
        // initialize all the dummy nodes in the map.
        for(size_type _Bucket_index = _M_number_of_buckets >> 1; _Bucket_index < _M_number_of_buckets; ++_Bucket_index)
        {
            if (!_Is_initialized(_Bucket_index))
            {
                _Initialize_bucket(_Bucket_index);
            }
        }

        _Full_iterator _Iterator = _Get_bucket(_Bucket);

        // Find the end of the bucket, denoted by the dummy element
        do
        {
            ++_Iterator;
        }
        while(_Iterator != _M_split_ordered_list._End() && !_Iterator._Ptr->_Is_dummy());

        // Return the first real element past the end of the bucket
        return _M_split_ordered_list._Get_first_real_iterator(_Iterator);
    }

    /// <summary>
    ///     Returns an iterator to the last element in this container for a specific bucket.
    /// </summary>
    /// <param name="_Bucket">
    ///     The bucket index.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the end of the bucket.
    /// </returns>
    /**/
    const_local_iterator unsafe_end(size_type _Bucket) const
    {
        // If we've increased the number of buckets, there's a chance the intermediate dummy
        // node marking the end of this bucket has not yet been lazily initialized.
        // Inserting from _M_number_of_buckets/2 to _M_number_of_buckets will recursively
        // initialize all the dummy nodes in the map.
        for(size_type _Bucket_index = _M_number_of_buckets >> 1; _Bucket_index < _M_number_of_buckets; ++_Bucket_index)
        {
            if (!_Is_initialized(_Bucket_index))
            {
                const_cast<_Concurrent_hash*>(this)->_Initialize_bucket(_Bucket_index);
            }
        }

        _Full_const_iterator _Iterator = _Get_bucket(_Bucket);

        // Find the end of the bucket, denoted by the dummy element
        do
        {
            ++_Iterator;
        }
        while(_Iterator != _M_split_ordered_list._End() && !_Iterator._Ptr->_Is_dummy());

        // Return the first real element past the end of the bucket
        return _M_split_ordered_list._Get_first_real_iterator(_Iterator);
    }

    /// <summary>
    ///     Returns an iterator to the first element in this container for a specific bucket.
    /// </summary>
    /// <param name="_Bucket">
    ///     The bucket index.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the beginning of the bucket.
    /// </returns>
    /**/
    const_local_iterator unsafe_cbegin(size_type) const
    {
        return ((const _Mytype *) this)->begin();
    }

    /// <summary>
    ///     Returns an iterator to the first element in this container for a specific bucket.
    /// </summary>
    /// <param name="_Bucket">
    ///     The bucket index.
    /// </param>
    /// <returns>
    ///     An iterator pointing to the beginning of the bucket.
    /// </returns>
    /**/
    const_local_iterator unsafe_cend(size_type) const
    {
        return ((const _Mytype *) this)->end();
    }

    /// <summary>
    ///     Computes and returns the current load factor of the container. The load factor is the number of
    ///     elements in the container divided by the number of buckets.
    /// </summary>
    /// <returns>
    ///     The load factor for the container.
    /// </returns>
    /**/
    float load_factor() const
    {
        return (float) size() / (float) unsafe_bucket_count();
    }

    /// <summary>
    ///     Gets or sets the maximum load factor of the container. The maximum load factor is the
    ///     largest number of elements than can be in any bucket before the container grows its
    ///     internal table.
    /// </summary>
    /// <returns>
    ///     The first member function returns the stored maximum load factor. The second member function does not return a value
    ///     but throws an <see cref="out_of_range Class">out_of_range</see> exception if the supplied load factor is invalid..
    /// </returns>
    /**/
    float max_load_factor() const
    {
        return _M_maximum_bucket_size;
    }

    /// <summary>
    ///     Gets or sets the maximum load factor of the container. The maximum load factor is the
    ///     largest number of elements than can be in any bucket before the container grows its
    ///     internal table.
    /// </summary>
    /// <returns>
    ///     The first member function returns the stored maximum load factor. The second member function does not return a value
    ///     but throws an <see cref="out_of_range Class">out_of_range</see> exception if the supplied load factor is invalid..
    /// </returns>
    /**/
    void max_load_factor(float _Newmax)
    {
        // The _Newmax != _Newmax is a check for NaN, because NaN is != to itself
        if (_Newmax != _Newmax || _Newmax < 0)
        {
            _STD _Xout_of_range("invalid hash load factor");
        }

        _M_maximum_bucket_size = _Newmax;
    }

    // This function is a no op, because the underlying split-ordered list
    // is already sorted, so an increase in the bucket number will be
    // reflected next time this bucket is touched.

    /// <summary>
    ///     Rebuilds the hash table.
    /// </summary>
    /// <param name="_Buckets">
    ///     The desired number of buckets.
    /// </param>
    /// <remarks>
    ///     The member function alters the number of buckets to be at least <paramref name="_Buckets"/> and rebuilds the hash
    ///     table as needed. The number of buckets must be a power of 2. If not a power of 2, it will be rounded up to
    ///     the next largest power of 2.
    ///     <para>It throws an <see cref="out_of_range Class">out_of_range</see> exception if the number of buckets
    ///     is invalid (either 0 or greater than the maximum number of buckets).</para>
    /// </remarks>
    /**/
    void rehash(size_type _Buckets)
    {
        size_type _Current_buckets = _M_number_of_buckets;

        if (_Current_buckets > _Buckets)
        {
            return;
        }
        else if (_Buckets <= 0 || _Buckets > unsafe_max_bucket_count())
        {
            _STD _Xout_of_range("invalid number of buckets");
        }
        // Round up the number of buckets to the next largest power of 2
        _M_number_of_buckets = ((size_type) 1) << _Get_msb(_Buckets*2-1);
    }

protected:
    // Insert an element in the hash given its value
    template<typename _ValTy>
    ::std::pair<iterator, bool> _Insert(_ValTy&& _Value)
    {
        _Split_order_key _Order_key = (_Split_order_key) this->_M_comparator(this->_Key_function(_Value));
        size_type _Bucket = _Order_key % _M_number_of_buckets;

        // If bucket is empty, initialize it first
        if (!_Is_initialized(_Bucket))
        {
            _Initialize_bucket(_Bucket);
        }

        long _New_count;
        _Order_key = _Split_order_regular_key(_Order_key);
        _Full_iterator _Iterator = _Get_bucket(_Bucket);
        _Full_iterator _Last = _M_split_ordered_list._End();
        _Full_iterator _Where = _Iterator;
        _Nodeptr _New_node = _M_split_ordered_list._Buynode(_Order_key, ::std::forward<_ValTy>(_Value));

        _ASSERT_EXPR(_Where != _Last, L"Invalid head node");

        // First node is a dummy node
        ++_Where;

        for (;;)
        {
            if (_Where == _Last || _Mylist::_Get_key(_Where) > _Order_key)
            {
                // Try to insert it in the right place
                ::std::pair<iterator, bool> _Result = _M_split_ordered_list._Insert(_Iterator, _Where, _New_node, &_New_count);

                if (_Result.second)
                {
                    // Insertion succeeded, adjust the table size, if needed
                    _Adjust_table_size(_New_count, _M_number_of_buckets);
                    return _Result;
                }
                else
                {
                    // Insertion failed: either the same node was inserted by another thread, or
                    // another element was inserted at exactly the same place as this node.
                    // Proceed with the search from the previous location where order key was
                    // known to be larger (note: this is legal only because there is no safe
                    // concurrent erase operation supported).
                    _Where = _Iterator;
                    ++_Where;
                    continue;
                }
            }
            else if (!this->_M_allow_multimapping && _Mylist::_Get_key(_Where) == _Order_key &&
                this->_M_comparator(this->_Key_function(*_Where), this->_Key_function(_New_node->_Myval)) == 0)
            {
                // If the insert failed (element already there), then delete the new one
                _M_split_ordered_list._Erase(_New_node);

                // Element already in the list, return it
                return ::std::pair<iterator, bool>(_M_split_ordered_list._Get_iterator(_Where), false);
            }

            // Move the iterator forward
            _Iterator = _Where;
            ++_Where;
        }
    }

    template<class _Iterator>
    void _Insert(_Iterator _First, _Iterator _Last)
    {
        for (_Iterator _I = _First; _I != _Last; ++_I)
        {
            _Insert(*_I);
        }
    }

private:

    // Initialize the hash and keep the first bucket open
    void _Init()
    {
        // Allocate an array of segment pointers
        memset(_M_buckets, 0, _Pointers_per_table * sizeof(void *));

        // Insert the first element in the split-ordered list
        _Full_iterator _Dummy_node = _M_split_ordered_list._Begin();
        _Set_bucket(0, _Dummy_node);
    }

    void _Copy(const _Mytype& _Right)
    {
        clear();

        _M_maximum_bucket_size = _Right._M_maximum_bucket_size;
        _M_number_of_buckets = _Right._M_number_of_buckets;

        try
        {
            _Insert(_Right.begin(), _Right.end());
            this->_M_comparator = _Right._M_comparator;
        }
        catch(...)
        {
            _M_split_ordered_list.clear();
            throw;
        }
    }

    void _Swap_buckets(_Concurrent_hash& _Right)
    {
        if (_M_allocator == _Right._M_allocator)
        {
            // Swap all node segments
            for (size_type _Index = 0; _Index < _Pointers_per_table; ++_Index)
            {
                _Full_iterator * _Iterator_pointer = _M_buckets[_Index];
                _M_buckets[_Index] = _Right._M_buckets[_Index];
                _Right._M_buckets[_Index] = _Iterator_pointer;
            }
        }
        else
        {
            _STD _Xinvalid_argument("swap is invalid on non-equal allocators");
        }
    }

    // Hash APIs
    size_type _Distance(const_iterator _First, const_iterator _Last) const
    {
        size_type _Num = 0;

        for (const_iterator _Iterator = _First; _Iterator != _Last; ++_Iterator)
        {
            ++_Num;
        }

        return _Num;
    }

    // Find the element in the split-ordered list
    iterator _Find(const key_type& _Keyval)
    {
        _Split_order_key _Order_key = (_Split_order_key) this->_M_comparator(_Keyval);
        size_type _Bucket = _Order_key % _M_number_of_buckets;

        // If _Bucket is empty, initialize it first
        if (!_Is_initialized(_Bucket))
        {
            _Initialize_bucket(_Bucket);
        }

        _Order_key = _Split_order_regular_key(_Order_key);
        _Full_iterator _Last = _M_split_ordered_list._End();

        for (_Full_iterator _Iterator = _Get_bucket(_Bucket); _Iterator != _Last; ++_Iterator)
        {
            if (_Mylist::_Get_key(_Iterator) > _Order_key)
            {
                // If the order key is smaller than the current order key, the element
                // is not in the hash.
                return end();
            }
            else if (_Mylist::_Get_key(_Iterator) == _Order_key)
            {
                // The fact that order keys match does not mean that the element is found.
                // Key function comparison has to be performed to check whether this is the
                // right element. If not, keep searching while order key is the same.
                if (!this->_M_comparator(this->_Key_function(*_Iterator), _Keyval))
                {
                    return _M_split_ordered_list._Get_iterator(_Iterator);
                }
            }
        }

        return end();
    }

    // Find the element in the split-ordered list
    const_iterator _Find(const key_type& _Keyval) const
    {
        _Split_order_key _Order_key = (_Split_order_key) this->_M_comparator(_Keyval);
        size_type _Bucket = _Order_key % _M_number_of_buckets;

        // If _Bucket has not been initialized, keep searching up for a parent bucket
        // that has been initialized.  Worst case is the entire map will be read.
        while (!_Is_initialized(_Bucket))
        {
            _Bucket = _Get_parent(_Bucket);
        }

        _Order_key = _Split_order_regular_key(_Order_key);
        _Full_const_iterator _Last = _M_split_ordered_list._End();

        for (_Full_const_iterator _Iterator = _Get_bucket(_Bucket); _Iterator != _Last; ++_Iterator)
        {
            if (_Mylist::_Get_key(_Iterator) > _Order_key)
            {
                // If the order key is smaller than the current order key, the element
                // is not in the hash.
                return end();
            }
            else if (_Mylist::_Get_key(_Iterator) == _Order_key)
            {
                // The fact that order keys match does not mean that the element is found.
                // Key function comparison has to be performed to check whether this is the
                // right element. If not, keep searching while order key is the same.
                if (!this->_M_comparator(this->_Key_function(*_Iterator), _Keyval))
                {
                    return _M_split_ordered_list._Get_iterator(_Iterator);
                }
            }
        }

        return end();
    }

    // Erase an element from the list. This is not a concurrency safe function.
    iterator _Erase(const_iterator _Iterator)
    {
        _Split_order_key _Order_key = (_Split_order_key) this->_M_comparator(this->_Key_function(*_Iterator));
        size_type _Bucket = _Order_key % _M_number_of_buckets;

        // If bucket is empty, initialize it first
        if (!_Is_initialized(_Bucket))
        {
            _Initialize_bucket(_Bucket);
        }

        _Order_key = _Split_order_regular_key(_Order_key);

        _Full_iterator _Previous = _Get_bucket(_Bucket);
        _Full_iterator _Last = _M_split_ordered_list._End();
        _Full_iterator _Where = _Previous;

        _ASSERT_EXPR(_Where != _Last, L"Invalid head node");

        // First node is a dummy node
        ++_Where;

        for (;;)
        {
            if (_Where == _Last)
            {
                return end();
            }
            else if (_M_split_ordered_list._Get_iterator(_Where) == _Iterator)
            {
                return _M_split_ordered_list._Erase(_Previous, _Iterator);
            }

            // Move the iterator forward
            _Previous = _Where;
            ++_Where;
        }
    }

    // Return the [begin, end] pair of iterators with the same key values.
    // This operation makes sense only if mapping is many-to-one.
    ::std::pair<iterator, iterator> _Equal_range(const key_type& _Keyval)
    {
        _Split_order_key _Order_key = (_Split_order_key) this->_M_comparator(_Keyval);
        size_type _Bucket = _Order_key % _M_number_of_buckets;

        // If _Bucket is empty, initialize it first
        if (!_Is_initialized(_Bucket))
        {
            _Initialize_bucket(_Bucket);
        }

        _Order_key = _Split_order_regular_key(_Order_key);
        _Full_iterator _Last = _M_split_ordered_list._End();

        for (_Full_iterator _Iterator = _Get_bucket(_Bucket); _Iterator != _Last; ++_Iterator)
        {
            if (_Mylist::_Get_key(_Iterator) > _Order_key)
            {
                // There is no element with the given key
                return ::std::pair<iterator, iterator>(end(), end());
            }
            else if (_Mylist::_Get_key(_Iterator) == _Order_key && !this->_M_comparator(this->_Key_function(*_Iterator), _Keyval))
            {
                iterator _Begin = _M_split_ordered_list._Get_iterator(_Iterator);
                iterator _End= _Begin;

                for (;_End != end() && !this->_M_comparator(this->_Key_function(*_End), _Keyval); ++_End)
                {
                }

                return ::std::pair<iterator, iterator>(_Begin, _End);
            }
        }

        return ::std::pair<iterator, iterator>(end(), end());
    }

    // Return the [begin, end] pair of const iterators with the same key values.
    // This operation makes sense only if mapping is many-to-one.
    ::std::pair<const_iterator, const_iterator> _Equal_range(const key_type& _Keyval) const
    {
        _Split_order_key _Order_key = (_Split_order_key) this->_M_comparator(_Keyval);
        size_type _Bucket = _Order_key % _M_number_of_buckets;

        // If _Bucket has not been initialized, keep searching up for a parent bucket
        // that has been initialized.  Worst case is the entire map will be read.
        while (!_Is_initialized(_Bucket))
        {
            _Bucket = _Get_parent(_Bucket);
        }

        _Order_key = _Split_order_regular_key(_Order_key);
        _Full_const_iterator _Last = _M_split_ordered_list._End();

        for (_Full_const_iterator _Iterator = _Get_bucket(_Bucket); _Iterator != _Last; ++_Iterator)
        {
            if (_Mylist::_Get_key(_Iterator) > _Order_key)
            {
                // There is no element with the given key
                return ::std::pair<const_iterator, const_iterator>(end(), end());
            }
            else if (_Mylist::_Get_key(_Iterator) == _Order_key && !this->_M_comparator(this->_Key_function(*_Iterator), _Keyval))
            {
                const_iterator _Begin = _M_split_ordered_list._Get_iterator(_Iterator);
                const_iterator _End = _Begin;

                for (; _End != end() && !this->_M_comparator(this->_Key_function(*_End), _Keyval); ++_End)
                {
                }

                return ::std::pair<const_iterator, const_iterator>(_Begin, _End);
            }
        }

        return ::std::pair<const_iterator, const_iterator>(end(), end());
    }

    // Bucket APIs
    void _Initialize_bucket(size_type _Bucket)
    {
        // Bucket 0 has no parent. Initialize it and return.
        if (_Bucket == 0)
        {
            _Init();
            return;
        }

        size_type _Parent_bucket = _Get_parent(_Bucket);

        // All _Parent_bucket buckets have to be initialized before this bucket is
        if (!_Is_initialized(_Parent_bucket))
        {
            _Initialize_bucket(_Parent_bucket);
        }

        _Full_iterator _Parent = _Get_bucket(_Parent_bucket);

        // Create a dummy first node in this bucket
        _Full_iterator _Dummy_node = _M_split_ordered_list._Insert_dummy(_Parent, _Split_order_dummy_key(_Bucket));
        _Set_bucket(_Bucket, _Dummy_node);
    }

    void _Adjust_table_size(size_type _Total_elements, size_type _Current_size)
    {
        // Grow the table by a factor of 2 if possible and needed
        if (((float) _Total_elements / (float) _Current_size) > _M_maximum_bucket_size)
        {
             // Double the size of the hash only if size has not changed inbetween loads
            _InterlockedCompareExchangeSizeT(&_M_number_of_buckets, 2 * _Current_size, _Current_size);
        }
    }

    size_type _Get_parent(size_type _Bucket) const
    {
        // Unsets bucket's most significant turned-on bit
        unsigned char _Msb = _Get_msb(_Bucket);
        return _Bucket & ~(1 << _Msb);
    }

    // Dynamic sized array (segments)
    _Full_iterator _Get_bucket(size_type _Bucket) const
    {
        size_type _Segment = _Segment_index_of(_Bucket);
        _Bucket -= _Segment_base(_Segment);
        return _M_buckets[_Segment][_Bucket];
    }

    void _Set_bucket(size_type _Bucket, _Full_iterator _Dummy_head)
    {
        size_type _Segment = _Segment_index_of(_Bucket);
        _Bucket -= _Segment_base(_Segment);

        if (_M_buckets[_Segment] == nullptr)
        {
            size_type _Seg_size = _Segment_size(_Segment);
            _Full_iterator * _New_segment = _M_allocator.allocate(_Seg_size);
            ::std::_Uninitialized_value_construct_n(_New_segment, _Seg_size, _M_allocator);
            if (_InterlockedCompareExchangePointer((void * volatile *) &_M_buckets[_Segment], _New_segment, nullptr) != nullptr)
            {
                _M_allocator.deallocate(_New_segment, _Seg_size);
            }
        }
        _M_buckets[_Segment][_Bucket] = _Dummy_head;
    }

    bool _Is_initialized(size_type _Bucket) const
    {
        size_type _Segment = _Segment_index_of(_Bucket);
        _Bucket -= _Segment_base(_Segment);

        if (_M_buckets[_Segment] == nullptr)
        {
            return false;
        }

        _Full_iterator _Iterator = _M_buckets[_Segment][_Bucket];
        return (_Iterator._Ptr != nullptr);
    }

    // Utilities for keys
    _Split_order_key _Reverse(_Map_key _Order_key) const
    {
        _Split_order_key _Reversed_order_key;

        unsigned char * _Original = (unsigned char *) &_Order_key;
        unsigned char * _Reversed = (unsigned char *) &_Reversed_order_key;

        int _Size = sizeof(_Map_key);
        for (int _Index = 0; _Index < _Size; ++_Index)
        {
            _Reversed[_Size - _Index - 1] = _Reverse_byte(_Original[_Index]);
        }

        return _Reversed_order_key;
    }

    // A regular order key has its original hash value reversed and the last bit set
    _Split_order_key _Split_order_regular_key(_Map_key _Order_key) const
    {
        return _Reverse(_Order_key) | 0x1;
    }

    // A dummy order key has its original hash value reversed and the last bit unset
    _Split_order_key _Split_order_dummy_key(_Map_key _Order_key) const
    {
        return _Reverse(_Order_key) & ~(0x1);
    }

    // Shared variables
    _Full_iterator *                                                _M_buckets[_Pointers_per_table]; // The segment table
    _Mylist                                                         _M_split_ordered_list;           // List where all the elements are kept
    typename _Alfi_traits::allocator_type                           _M_allocator;                    // Allocator object for segments
    size_type                                                       _M_number_of_buckets;            // Current table size
    float                                                           _M_maximum_bucket_size;          // Maximum size of the bucket
};

#pragma warning(pop) // Warning 4127 -- while (true) has a constant expression in it

} // namespace details;
} // namespace Concurrency

namespace concurrency = ::Concurrency;

#pragma pack(pop)
