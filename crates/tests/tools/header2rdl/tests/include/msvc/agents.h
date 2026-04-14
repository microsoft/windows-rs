/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* agents.h
*
* Main public header file for ConcRT's asynchronous agents layer. This is the only header file a
* C++ program must include to use asynchronous agents.
*
* The core runtime, Parallel Patterns Library (PPL), and resource manager are defined in separate header files.
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#include <crtdefs.h>
#include <concrt.h>
#include <stdexcept>
#include <functional>
#include <tuple>
#include <type_traits>
#include <vector>
#include <concurrent_queue.h>

#define _AGENTS_H

#pragma pack(push,_CRT_PACKING)
#pragma warning(push)
#pragma warning(disable: 4100) // Unreferenced formal parameter - needed for document generation
#pragma warning(disable: 4702) // Unreachable code - needed for retail version code path
#pragma warning(disable: 4297) // Function expected not to throw but does
// Forward declarations

/// <summary>
///     The <c>Concurrency</c> namespace provides classes and functions that provide access to the Concurrency Runtime,
///     a concurrent programming framework for C++. For more information, see <see cref="Concurrency Runtime"/>.
/// </summary>
/**/
namespace Concurrency
{
/// <summary>
///     Each message instance has an identity that follows it as it is
///     cloned and passed between messaging components. This cannot be the
///     address of the message object.
/// </summary>
/**/
typedef __int32 runtime_object_identity;

/// <summary>
///     A lock holder that acquires a non-reentrant lock on instantiation and releases
///     it on destruction.
/// </summary>
/**/
typedef ::Concurrency::details::_NonReentrantPPLLock::_Scoped_lock _NR_lock;

/// <summary>
///     A lock holder that acquires a reentrant lock on instantiation and releases
///     it on destruction
/// </summary>
/**/
typedef ::Concurrency::details::_ReentrantPPLLock::_Scoped_lock _R_lock;


//***************************************************************************
// Internal namespace:
//
// ::Concurrency::details contains definitions to support routines in the public namespaces and macros.
// Clients should not directly interact with this namespace.
//***************************************************************************

namespace details
{
    //**************************************************************************
    // Core Messaging Support:
    //**************************************************************************

    //
    // A base class to derive from that keeps unique IDs on its derived classes
    //
    class _Runtime_object : public _AllocBase
    {
    public:
        // Creates a new runtime object.
        _CONCRTIMP _Runtime_object();

        // Creates a runtime object from an identity.
        _CONCRTIMP _Runtime_object(::Concurrency::runtime_object_identity _Id);

        // Gets the runtime object identity.
        virtual ::Concurrency::runtime_object_identity _GetId() const
        {
            return _M_id;
        }

    protected:
        // The runtime object identity.
        ::Concurrency::runtime_object_identity _M_id;
    };

    // A queue used to hold the messages for the messaging blocks
    template<class _Message>
    class _Queue : public _AllocBase
    {
    protected:
        // A pointer to the head of the queue.
        _Message * _M_pHead;

        // A pointer to a pointer to the tail of the queue.
        _Message ** _M_ppTail;

        // The number of elements presently stored in the queue.
        size_t _M_count;

    public:
        typedef _Message type;

        // Create a Queue
        _Queue() : _M_pHead(nullptr), _M_ppTail(&_M_pHead), _M_count(0)
        {
        }

        // Destroy the queue
        ~_Queue()
        {
        }

        // Returns the count of items in the queue
        size_t _Count() const
        {
            return _M_count;
        }

        // Add an item to the tail of the queue
        //
        // Returns a Boolean indicating whether the operation succeeded.
        bool _Enqueue(_Message *_Element)
        {
            _CONCRT_ASSERT(_Element->_M_pNext == nullptr);
            _CONCRT_ASSERT(*_M_ppTail == nullptr);

            *_M_ppTail = _Element;
            _Element->_M_pNext = nullptr;
            _M_ppTail = &(_Element->_M_pNext);
            _M_count++;

            return true;
        }

        // Remove the specified element from the queue
        //
        // Returns a Boolean indicating whether the operation succeeded, that is, the message was found in the queue.
        bool _Remove(_Message * _OldElement)
        {
            bool _Result = false;

            _CONCRT_ASSERT(_OldElement != nullptr);

            if (_M_pHead == _OldElement)
            {
                _M_pHead = _OldElement->_M_pNext;
                if (_M_pHead == nullptr)
                {
                    _M_ppTail = &_M_pHead;
                }

                _OldElement->_M_pNext = nullptr;
                _M_count--;
                _Result = true;
            }
            else
            {
                _Message * _Next = nullptr;
                for (_Message * _Node = _M_pHead; _Node != nullptr; _Node = _Next)
                {
                    _Next = _Node->_M_pNext;

                    if (_Node->_M_pNext == _OldElement)
                    {
                        _Node->_M_pNext = _OldElement->_M_pNext;
                        // if this is the last element of the _Queue
                        if (_Node->_M_pNext == nullptr && _M_count == 1)
                        {
                            _M_ppTail = &_M_pHead;
                        }

                        _OldElement->_M_pNext = nullptr;
                        _M_count--;
                        _Result = true;
                        break;
                    }
                }
            }

            return _Result;
        }

        // Dequeue an item from the head of queue
        //
        // Returns a pointer to the message found at the head of the queue.
        _Message * _Dequeue()
        {
            if (_M_pHead == nullptr)
            {
                return nullptr;
            }

            _Message * _Result = _M_pHead;

            _M_pHead = _Result->_M_pNext;
            if (_M_pHead == nullptr)
            {
                _M_ppTail = &_M_pHead;
            }

            _Result->_M_pNext = nullptr;
            _M_count--;
            return _Result;
        }

        // Return the item at the head of the queue, without dequeuing
        //
        // Returns a pointer to the message found at the head of the queue.
        _Message * _Peek()
        {
            return _M_pHead;
        }

        // Return true if the ID matches the message at the head of the queue
        bool _Is_head(runtime_object_identity _MsgId)
        {
            // Peek at the next message in the message buffer.  Use it to
            // check if the IDs match
            _Message * _Msg = _M_pHead;

            if (_Msg == nullptr || _Msg->msg_id() != _MsgId)
            {
                return false;
            }

            return true;
        }
    };

    //
    // _Dynamic_array implements a container very similar to std::vector.
    // However, it exposes a reduced subset of functionality that is
    // geared towards use in network_link_registry. The array access is not
    // thread-safe.
    //
    template<class _Type>
    class _Dynamic_array
    {
    public:

        typedef _Dynamic_array<_Type> _Myt;

        typedef _Type& reference;
        typedef _Type const& const_reference;

        //
        // Construct a dynamic array
        //
        _Dynamic_array()
        {
            _Init();
        }

        //
        // Release any resources used by dynamic array
        //
        ~_Dynamic_array()
        {
            _Clear();
        }

        //
        // Assignment operator. Copy the contents of _Right
        //
        _Myt& operator=(const _Myt& _Right)
        {
            if (this != &_Right)
            {
                // Remove all the elements
                _Clear();

                // Allocate space for the new elements
                size_t _Size = _Right._Size();
                _Grow(_Size);

                // Copy over the new elements
                for (size_t _I=0; _I < _Size; _I++)
                {
                    _Push_back(_Right[_I]);
                }
            }

            return *this;
        }

        //
        // Clear all the elements in the array
        //
        void _Clear()
        {
            if (_M_array != nullptr)
            {
                delete [] _M_array;
                _Init();
            }
        }

        //
        // Add an element to the end of the array
        //
        void _Push_back(_Type const& _Element)
        {
            if (_M_index >= _M_size)
            {
                // Not enough space. Grow the array
                size_t _NewSize = (_M_index + 1) * _S_growthFactor;
                _Grow(_NewSize);
            }

            _CONCRT_ASSERT(_M_index < _M_size);
            _M_array[_M_index] = _Element;
            _M_index++;
        }

        //
        // Index operation. Retrieve an element at the specified index. No bounds check is done.
        //
        reference operator[](size_t _Pos)
        {
            _CONCRT_ASSERT(_Pos < _M_size);
            return _M_array[_Pos];
        }

        //
        // Index operation. Retrieve an element at the specified index. No bounds check is done.
        //
        const_reference operator[](size_t _Pos) const
        {
            _CONCRT_ASSERT(_Pos < _M_size);
            return _M_array[_Pos];
        }

        //
        // Returns the count of elements in the array
        //
        size_t _Size() const
        {
            return _M_index;
        }

        //
        // Swap the contents of this array with _Right
        //
        void _Swap(_Myt& _Right)
        {
            if (this != &_Right)
            {
                // Swap the details.
                _Type * _Array = _M_array;
                size_t _Index = _M_index;
                size_t _Size = _M_size;

                _M_array = _Right._M_array;
                _M_index = _Right._M_index;
                _M_size = _Right._M_size;

                _Right._M_array = _Array;
                _Right._M_index = _Index;
                _Right._M_size = _Size;
            }
        }

    private:
        //
        // Initialize the array
        //
        void _Init()
        {
            _M_array = nullptr;
            _M_index = 0;
            _M_size = 0;
        }

        //
        // Grow the array to the given size. The old elements are copied over.
        //
        void _Grow(size_t _NewSize)
        {
            _CONCRT_ASSERT( _NewSize > _M_size );

            _Type * _Array = new _Type[_NewSize];

            if (_M_array != nullptr)
            {
                // Copy over the elements
                for (size_t _I = 0; _I < _M_size; _I++)
                {
                    _Array[_I] = _M_array[_I];
                }

                delete [] _M_array;
            }

            _M_array = _Array;
            _M_size = _NewSize;
        }

        // Private data members

        // Array of elements
        _Type * _M_array;

        // Index where the next element should be inserted
        size_t  _M_index;

        // Capacity of the array.
        size_t  _M_size;

        static constexpr int _S_growthFactor = 2;
    };

    //
    // Returns an identifier for the given object that could be used
    // in an ETW trace (call to _Trace_agents)
    //
    template <class _Type>
    __int64 _Trace_agents_get_id(_Type * _PObject)
    {
        return reinterpret_cast<__int64>(_PObject);
    }

} // namespace details

//**************************************************************************
// Public Namespace:
//
// Anything in the Concurrency namespace is intended for direct client consumption.
//
//**************************************************************************

//
// Forward declarations:
//
template<class _Type> class ISource;
template<class _Type> class ITarget;

//**************************************************************************
// Network link registry
//**************************************************************************

// Forward declaration for use in the iterator
template<class _Block> class network_link_registry;

/// <summary>
///     Const iterator for network link registry. Message blocks should use
///     the link_registry::iterator type for iteration.
/// </summary>
/// <typeparam name="_Block">
///     The network block type
/// </typeparam>
/**/
template<class _Block>
class _Network_link_iterator
{
public:

    typedef _Network_link_iterator<_Block> _Myt;
    typedef network_link_registry<_Block> _MyContainer;

    // Element type
    typedef _Block* _EType;

    // Const iterator - iterator shall not be used to modify the links
    typedef _EType const& const_reference;
    typedef _EType const* const_pointer;

    /// <summary>
    ///     Construct iterator
    /// </summary>
    /**/
    _Network_link_iterator(_MyContainer * _PNetwork_link, size_t _Index) : _M_pNetwork_link(_PNetwork_link), _M_index(_Index), _M_value(nullptr)
    {
        _M_pNetwork_link->_Next_index(_M_index);
    }

    /// <summary>
    ///     Copy construct an iterator
    /// </summary>
    /**/
    _Network_link_iterator(_Myt const& _Right)
    {
        _M_pNetwork_link = _Right._M_pNetwork_link;
        _M_index = _Right._M_index;
    }

    /// <summary>
    ///     Copy assign an iterator
    /// </summary>
    /**/
    _Myt const& operator=(_Myt const& _Right)
    {
        _M_pNetwork_link = _Right._M_pNetwork_link;
        _M_index = _Right._M_index;
        return *this;
    }

    /// <summary>
    ///     Returns the object pointed to by the iterator
    /// </summary>
    /// <returns>
    ///     Reference to the object pointed to by the iterator
    /// </returns>
    /**/
    const_reference operator*()
    {
        _M_value = _M_pNetwork_link->_Get_element(_M_index);
        return _M_value;
    }

    /// <summary>
    ///     Returns a pointer to the class object
    /// </summary>
    /// <returns>
    ///     Returns a pointer to the class object
    /// </returns>
    /**/
    const_pointer operator->() const
    {
        return (&**this);
    }

    /// <summary>
    ///     Pre-increment the iterator to point to the next element
    /// </summary>
    /// <returns>
    ///     Reference to the object pointer to by the iterator after
    ///     incrementing it
    /// </returns>
    /**/
    _Myt& operator++()
    {
        ++_M_index;
        _M_pNetwork_link->_Next_index(_M_index);
        return (*this);
    }

    /// <summary>
    ///     Post-increment the iterator to point to the next element
    /// </summary>
    /// <returns>
    ///     Reference to the object pointer to by the iterator before
    ///     incrementing it
    /// </returns>
    /**/
    _Myt operator++(int)
    {
        _Myt _Tmp = *this;
        ++*this;
        return (_Tmp);
    }

private:

    // Pointer to the underlying container (network link registry)
    _MyContainer * _M_pNetwork_link;

    // Current index
    size_t _M_index;

    // Current value
    _EType _M_value;
};

/// <summary>
///     The <c>network_link_registry</c> abstract base class manages the links between source
///     and target blocks.
/// </summary>
/// <typeparam name="_Block">
///     The block data type being stored in the <c>network_link_registry</c>.
/// </typeparam>
/// <remarks>
///     The <c>network link registry</c> is not safe for concurrent access.
/// </remarks>
/// <seealso cref="single_link_registry Class"/>
/// <seealso cref="multi_link_registry Class"/>
/**/
template<class _Block>
class network_link_registry
{
public:

    /// <summary>
    ///     A type that represents the block type stored in the <c>network_link_registry</c> object.
    /// </summary>
    /**/
    typedef _Block type;

    /// <summary>
    ///     A type that represents an element pointer stored in the <c>network_link_registry</c> object.
    /// </summary>
    /**/
    typedef _Block * _EType;

    /// <summary>
    ///     A type that provides a reference to a <c>const</c> element stored in a
    ///     <c>network_link_registry</c> object for reading and performing const operations.
    /// </summary>
    /**/
    typedef _EType const& const_reference;

    /// <summary>
    ///     A type that provides a pointer to a <c>const</c> element in a
    ///     <c>network_link_registry</c> object.
    /// </summary>
    /**/
    typedef _EType const* const_pointer;

    // Make the iterators friends so that they can access some of the
    // private routines such as _Get_element.
    /**/
    friend class _Network_link_iterator<_Block>;

    /// <summary>
    ///     A type that provides an iterator that can read or modify any element in a
    ///     <c>network_link_registry</c> object.
    /// </summary>
    /**/
    typedef _Network_link_iterator<_Block> iterator;

    /// <summary>
    ///     When overridden in a derived class, adds a link to the <c>network_link_registry</c>
    ///     object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be added.
    /// </param>
    /**/
    virtual void add(_EType _Link) = 0;

    /// <summary>
    ///     When overridden in a derived class, removes a specified block from the
    ///     <c>network_link_registry</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be removed, if found.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the link was found and removed, <c>false</c> otherwise.
    /// </returns>
    /**/
    virtual bool remove(_EType _Link) = 0;

    /// <summary>
    ///     When overridden in a derived class, searches the <c>network_link_registry</c> object
    ///     for a specified block.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block that is being searched for in the <c>network_link_registry</c>
    ///     object.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the block was found, <c>false</c> otherwise.
    /// </returns>
    /**/
    virtual bool contains(_EType _Link) = 0;

    /// <summary>
    ///     When overridden in a derived class, returns the number of items in the
    ///     <c>network_link_registry</c> object.
    /// </summary>
    /// <returns>
    ///     The number of items in the <c>network_link_registry</c> object.
    /// </returns>
    /**/
    virtual size_t count() = 0;

    /// <summary>
    ///     When overridden in a derived class, returns an iterator to the first element in the
    ///     <c>network_link_registry</c> object.
    /// </summary>
    /// <remarks>
    ///     The end state of the iterator is indicated by a <c>NULL</c> link.
    /// </remarks>
    /// <returns>
    ///     An iterator addressing the first element in the <c>network_link_registry</c> object.
    /// </returns>
    /**/
    virtual iterator begin() = 0;

protected:

    /// <summary>
    ///     Skips empty slots and updates the index to the next
    ///     non-empty slot. This is called by the iterator.
    /// </summary>
    /// <param name="_Index">
    ///     A reference to the index that is to be updated.
    /// </param>
    /**/
    virtual void _Next_index(size_t& _Index) = 0;

    /// <summary>
    ///     Retrieves the element at the given index. If the index is out of bounds,
    ///     <c>NULL</c> is returned. Users need to use the iterator to access the links.
    /// </summary>
    /// <param name="_Index">
    ///     Index of the link to be retrieved.
    /// </param>
    /// <returns>
    ///     The element in the registry at  the index specified by the <paramref name="_Index"/> parameter.
    /// </returns>
    /**/
    virtual _EType _Get_element(size_t _Index) const = 0;
};

/// <summary>
///     The <c>single_link_registry</c> object is a <c>network_link_registry</c> that manages
///     only a single source or target block.
/// </summary>
/// <typeparam name="_Block">
///     The block data type being stored in the <c>single_link_registry</c> object.
/// </typeparam>
/// <seealso cref="multi_link_registry Class"/>
/**/
template<class _Block>
class single_link_registry : public network_link_registry<_Block>
{
public:

    /// <summary>
    ///     Constructs a <c>single_link_registry</c> object.
    /// </summary>
    /**/
    single_link_registry() : _M_connectedLink(nullptr)
    {
    }

    /// <summary>
    ///     Destroys the <c>single_link_registry</c> object.
    /// </summary>
    /// <remarks>
    ///     The method throws an <see cref="invalid_operation Class">invalid_operation</see> exception if
    ///     it is called before the link is removed.
    /// </remarks>
    /**/
    virtual ~single_link_registry()
    {
        // It is an error to delete link registry with links
        // still present
        if (count() != 0)
        {
            throw invalid_operation("Deleting link registry before removing all the links");
        }
    }

    /// <summary>
    ///     Adds a link to the <c>single_link_registry</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be added.
    /// </param>
    /// <remarks>
    ///     The method throws an <see cref="invalid_link_target Class">invalid_link_target</see> exception
    ///     if there is already a link in this registry.
    /// </remarks>
    /**/
    virtual void add(typename network_link_registry<_Block>::_EType _Link)
    {
        if (_Link == nullptr)
        {
            return;
        }

        // Only one link can be added.
        if (_M_connectedLink != nullptr)
        {
            throw invalid_link_target("_Link");
        }

        _M_connectedLink = _Link;
    }

    /// <summary>
    ///     Removes a link from the <c>single_link_registry</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be removed, if found.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the link was found and removed, <c>false</c> otherwise.
    /// </returns>
    /**/
    virtual bool remove(typename network_link_registry<_Block>::_EType _Link)
    {
        if ((_Link != nullptr) && (_M_connectedLink == _Link))
        {
            _M_connectedLink = nullptr;
            return true;
        }

        return false;
    }

    /// <summary>
    ///     Searches the <c>single_link_registry</c> object for a specified block.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block that is to be searched for in the <c>single_link_registry</c> object.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the link was found, <c>false</c> otherwise.
    /// </returns>
    /**/
    virtual bool contains(typename network_link_registry<_Block>::_EType _Link)
    {
        return ((_Link != nullptr) && (_M_connectedLink == _Link));
    }

    /// <summary>
    ///     Counts the number of items in the <c>single_link_registry</c> object.
    /// </summary>
    /// <returns>
    ///     The number of items in the <c>single_link_registry</c> object.
    /// </returns>
    /**/
    virtual size_t count()
    {
        return (_M_connectedLink == nullptr) ? 0 : 1;
    }

    /// <summary>
    ///     Returns an iterator to the first element in the <c>single_link_registry</c> object.
    /// </summary>
    /// <remarks>
    ///     The end state is indicated by a <c>NULL</c> link.
    /// </remarks>
    /// <returns>
    ///     An iterator addressing the first element in the <c>single_link_registry</c> object.
    /// </returns>
    /**/
    virtual typename network_link_registry<_Block>::iterator begin()
    {
        return (typename network_link_registry<_Block>::iterator(this, 0));
    }

protected:

    /// <summary>
    ///     Skips empty slots and updates the index to the next
    ///     non-empty slot. This is called by the iterator.
    /// </summary>
    /// <param name="_Index">
    ///     A reference to the index that is to be updated.
    /// </param>
    /**/
    virtual void _Next_index(size_t& _Index)
    {
        if (_M_connectedLink == nullptr)
        {
            _Index++;
        }
    }

    /// <summary>
    ///     Retrieves the element at the given index. If the index is out of bounds,
    ///     <c>NULL</c> is returned. Users need to use the iterator to access the links.
    /// </summary>
    /// <param name="_Index">
    ///     The index of the link to be retrieved.
    /// </param>
    /// <returns>
    ///     The element in the registry at  the index specified by the <paramref name="_Index"/> parameter.
    /// </returns>
    /**/
    virtual typename network_link_registry<_Block>::_EType _Get_element(size_t _Index) const
    {
        if (_Index == 0)
        {
            return _M_connectedLink;
        }

        return nullptr;
    }

private:

    // A single pointer is used to hold the link
    typename network_link_registry<_Block>::_EType _M_connectedLink;
};

/// <summary>
///     The <c>multi_link_registry</c> object is a <c>network_link_registry</c> that manages multiple
///     source blocks or multiple target blocks.
/// </summary>
/// <typeparam name="_Block">
///     The block data type being stored in the <c>multi_link_registry</c> object.
/// </typeparam>
/// <seealso cref="single_link_registry Class"/>
/**/
template<class _Block>
class multi_link_registry : public network_link_registry<_Block>
{
public:

    /// <summary>
    ///     Constructs a <c>multi_link_registry</c> object.
    /// </summary>
    /**/
    multi_link_registry() : _M_maxLinks(_NOT_SET)
    {
    }

    /// <summary>
    ///     Destroys the <c>multi_link_registry</c> object.
    /// </summary>
    /// <remarks>
    ///     The method throws an <see cref="invalid_operation Class">invalid_operation</see> exception if
    ///     called before all links are removed.
    /// </remarks>
    /**/
    virtual ~multi_link_registry()
    {
        // It is an error to delete link registry with links
        // still present
        if (count() != 0)
        {
            throw invalid_operation("Deleting link registry before removing all the links");
        }
    }

    /// <summary>
    ///     Sets an upper bound on the number of links that the <c>multi_link_registry</c> object
    ///     can hold.
    /// </summary>
    /// <param name="_MaxLinks">
    ///     The maximum number of links that the <c>multi_link_registry</c> object can hold.
    /// </param>
    /// <remarks>
    ///     After a bound is set, unlinking an entry will cause the <c>multi_link_registry</c>
    ///     object to enter an immutable state where further calls to <c>add</c> will throw an
    ///     <c>invalid_link_target</c> exception.
    /// </remarks>
    /**/
    void set_bound(size_t _MaxLinks)
    {
        _CONCRT_ASSERT(count() == 0);
        _M_maxLinks = _MaxLinks;
    }

    /// <summary>
    ///     Adds a link to the <c>multi_link_registry</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be added.
    /// </param>
    /// <remarks>
    ///     The method throws an <see cref="invalid_link_target Class">invalid_link_target</see> exception if
    ///     the link is already present in the registry, or if a bound has already been set with the <c>set_bound</c>
    ///     function and a link has since been removed.
    /// </remarks>
    /**/
    virtual void add(typename network_link_registry<_Block>::_EType _Link)
    {
        if (_Link == nullptr)
        {
            return;
        }

        _Add(_Link);
    }

    /// <summary>
    ///     Removes a link from the <c>multi_link_registry</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be removed, if found.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the link was found and removed, <c>false</c> otherwise.
    /// </returns>
    /**/
    virtual bool remove(typename network_link_registry<_Block>::_EType _Link)
    {
        if (_Link == nullptr)
        {
            return false;
        }

        return (_Remove(_Link));
    }

    /// <summary>
    ///     Searches the <c>multi_link_registry</c> object for a specified block.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block that is to be searched for in the <c>multi_link_registry</c> object.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the specified block was found, <c>false</c> otherwise.
    /// </returns>
    /**/
    virtual bool contains(typename network_link_registry<_Block>::_EType _Link)
    {
        if (_Link == nullptr)
        {
            return false;
        }

        return (_Find(_Link) < _M_vector._Size());
    }

    /// <summary>
    ///     Counts the number of items in the <c>multi_link_registry</c> object.
    /// </summary>
    /// <returns>
    ///     The number of items in the <c>multi_link_registry</c> object.
    /// </returns>
    /**/
    virtual size_t count()
    {
        return _Count();
    }

    /// <summary>
    ///     Returns an iterator to the first element in the <c>multi_link_registry</c> object.
    /// </summary>
    /// <remarks>
    ///     The end state is indicated by a <c>NULL</c> link.
    /// </remarks>
    /// <returns>
    ///     An iterator addressing the first element in the <c>multi_link_registry</c> object.
    /// </returns>
    /**/
    virtual typename network_link_registry<_Block>::iterator begin()
    {
        return (typename network_link_registry<_Block>::iterator(this, 0));
    }

protected:

    /// <summary>
    ///     Skips empty slots and updates the index to the next
    ///     non-empty slot. This is called by the iterator.
    /// </summary>
    /// <param name="_Index">
    ///     A reference to the index that is to be updated.
    /// </param>
    /**/
    virtual void _Next_index(size_t& _Index)
    {
        size_t _Size = _M_vector._Size();
        while (_Index < _Size)
        {
            if (_M_vector[_Index] != nullptr)
            {
                break;
            }

            ++_Index;
        }
    }

    /// <summary>
    ///     Retrieves the element at the given index. If the index is out of bounds,
    ///     <c>NULL</c> is returned. Users need to use the iterator to access the links
    /// </summary>
    /// <param name="_Index">
    ///     Index of the link to be retrieved.
    /// </param>
    /// <returns>
    ///     The element in the registry at the index specified by the <paramref name="_Index"/> parameter.
    /// </returns>
    /**/
    virtual typename network_link_registry<_Block>::_EType _Get_element(size_t _Index) const
    {
        if (_Index < _M_vector._Size())
        {
            return _M_vector[_Index];
        }

        return nullptr;
    }

private:

    /// <summary>
    ///     Adds a link to the <c>multi_link_registry</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be added.
    /// </param>
    /**/
    void _Add(typename network_link_registry<_Block>::_EType _Link)
    {
        size_t _Size = _M_vector._Size();
        size_t _Insert_pos = 0;

        _CONCRT_ASSERT(_Link != nullptr);

        // If max links is set, ensure that inserting the new
        // link will not exceed the bound.
        if ((_M_maxLinks != _NOT_SET) && ((_Size+1) > (size_t) _M_maxLinks))
        {
            throw invalid_link_target("_Link");
        }

        for (size_t _Index = 0; _Index < _Size; _Index++)
        {
            if (_M_vector[_Index] != nullptr)
            {
                // We want to find the first NULL entry after all the
                // non-NULL entries.
                _Insert_pos = _Index + 1;

                // Throw if duplicate entry is found
                if (_M_vector[_Index] == _Link)
                {
                    throw invalid_link_target("_Link");
                }
            }
        }

        if (_Insert_pos < _Size)
        {
            _M_vector[_Insert_pos] = _Link;
        }
        else
        {
            _M_vector._Push_back(_Link);
        }
    }

    /// <summary>
    ///     Removes a link from the <c>multi_link_registry</c>
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be removed, if found.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the specified link was found and removed, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool _Remove(typename network_link_registry<_Block>::_EType _Link)
    {
        _CONCRT_ASSERT(_Link != nullptr);

        for (size_t _Index = 0; _Index < _M_vector._Size(); _Index++)
        {
            if (_M_vector[_Index] == _Link)
            {
                _M_vector[_Index] = nullptr;

                // If max links is set, prevent new additions to the registry
                if (_M_maxLinks != _NOT_SET && _M_maxLinks > 0)
                {
                    // Setting the bound to 0. This causes add to always throw.
                    _M_maxLinks = 0;
                }

                return true;
            }
        }

        return false;
    }


    /// <summary>
    ///     Searches the registry for the given link
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block that is to be searched.
    /// </param>
    /// <returns>
    ///     Index of the entry if found.
    /// </returns>
    /**/
    virtual size_t _Find(typename network_link_registry<_Block>::_EType _Link)
    {
        size_t _Index = 0;
        for (_Index = 0; _Index < _M_vector._Size(); _Index++)
        {
            if (_M_vector[_Index] == _Link)
            {
                break;
            }
        }

        return _Index;
    }

    /// <summary>
    ///     Returns the count of items in the registry.
    /// </summary>
    /// <returns>
    ///     The count of items in the registry.
    /// </returns>
    /**/
    size_t _Count() const
    {
        size_t _Count = 0;

        for (size_t _Index = 0; _Index < _M_vector._Size(); _Index++)
        {
            if (_M_vector[_Index] != nullptr)
            {
                _Count++;
            }
        }

        return _Count;
    }

    static constexpr size_t _NOT_SET = SIZE_MAX;

    // Maximum number of links allowed.
    size_t _M_maxLinks;

    // ::Concurrency::details::_Dynamic_array is used to hold the links
    ::Concurrency::details::_Dynamic_array<typename network_link_registry<_Block>::_EType> _M_vector;
};

// Forward declaration for the iterator
template<class _LinkRegistry> class source_link_manager;

/// <summary>
///     Const Iterator for referenced link manager.
/// </summary>
/// <typeparam name="_LinkRegistry">
///     The underlying network link registry
/// </typeparam>
/**/
template<class _LinkRegistry>
class _Source_link_iterator
{
public:

    typedef typename _LinkRegistry::type  _Block;

    typedef _Source_link_iterator<_LinkRegistry> _Myt;
    typedef source_link_manager<_LinkRegistry> _MyContainer;

    // Element type
    typedef _Block* _EType;

    // Const iterator - iterator shall not be used to modify the links
    typedef _EType const& const_reference;
    typedef _EType const* const_pointer;

    /// <summary>
    ///     Construct iterator
    /// </summary>
    /**/
    _Source_link_iterator(_MyContainer * _PNetwork_link, size_t _Index) : _M_pNetwork_link(_PNetwork_link), _M_index(_Index), _M_sentinel(nullptr)
    {
        // Take a snapshot of the link registry. This will reference the registry.
        _M_pNetwork_link->_To_array(_M_array);
    }

    /// <summary>
    ///     Destruct iterator
    /// </summary>
    /**/
    virtual ~_Source_link_iterator()
    {
        if (_M_pNetwork_link != nullptr)
        {
            _M_pNetwork_link->release();
        }
    }
    /// <summary>
    ///     Copy construct an iterator
    /// </summary>
    /**/
    _Source_link_iterator(_Myt const& _Right)
    {
        _M_pNetwork_link = _Right._M_pNetwork_link;
        _M_index = _Right._M_index;
        _M_array = _Right._M_array;

        _M_pNetwork_link->reference();
    }

    /// <summary>
    ///     Copy assign an iterator
    /// </summary>
    /**/
    _Myt const& operator=(_Myt const& _Right)
    {
        _MyContainer * _OldContainer = _M_pNetwork_link;
        _CONCRT_ASSERT(_OldContainer != nullptr);

        _M_pNetwork_link = _Right._M_pNetwork_link;
        _M_index = _Right._M_index;
        _M_array = _Right._M_array;

        if (_OldContainer != _M_pNetwork_link)
        {
            _OldContainer->release();
            _M_pNetwork_link->reference();
        }

        return *this;
    }

    /// <summary>
    ///     Returns the object pointed to by the iterator
    /// </summary>
    /// <returns>
    ///     Reference to the object pointed to by the iterator
    /// </returns>
    /**/
    const_reference operator*()
    {
        return _Get(0);
    }

    /// <summary>
    ///     Returns a pointer to the class object
    /// </summary>
    /// <returns>
    ///     Returns a pointer to the class object
    /// </returns>
    /**/
    const_pointer operator->() const
    {
        return (&**this);
    }

    /// <summary>
    ///     Index operation. Retrieve an element at the specified index.
    /// </summary>
    /**/
    const_reference operator[](size_t _Pos) const
    {
        return _Get(_Pos);
    }

    /// <summary>
    ///     Pre-increment the iterator to point to the next element
    /// </summary>
    /// <returns>
    ///     Reference to the object pointer to by the iterator after incrementing it
    /// </returns>
    /**/
    _Myt& operator++()
    {
        ++_M_index;
        return (*this);
    }

    /// <summary>
    ///     Post-increment the iterator to point to the next element
    /// </summary>
    /// <returns>
    ///     Reference to the object pointer to by the iterator before incrementing it
    /// </returns>
    /**/
    _Myt operator++(int)
    {
        _Myt _Tmp = *this;
        ++*this;
        return (_Tmp);
    }

private:

    // Get the element at the given offset.
    const_reference _Get(size_t _Pos) const
    {
        size_t _Index = _M_index + _Pos;
        if (_Index >= _M_array._Size())
        {
            return _M_sentinel;
        }

        return _M_array[_Index];
    }

    // Array to hold the snapshot of the link registry
    ::Concurrency::details::_Dynamic_array<_EType> _M_array;

    // Pointer to the underlying container (network link registry)
    _MyContainer * _M_pNetwork_link;

    // Current index
    size_t _M_index;

    // Sentinel value to return on bounds overflow
    _EType _M_sentinel;
};

/// <summary>
///     The <c>source_link_manager</c> object manages messaging block network links
///     to <c>ISource</c> blocks.
/// </summary>
/// <typeparam name="_LinkRegistry">
///     The network link registry.
/// </typeparam>
/// <remarks>
///     Currently, the source blocks are reference counted. This is a wrapper on a
///     <c>network_link_registry</c> object that allows concurrent access to the links and
///     provides the ability to reference the links through callbacks. Message
///     blocks (<c>target_block</c>s or <c>propagator_block</c>s) should use this class
///     for their source links.
/// </remarks>
/// <seealso cref="single_link_registry Class"/>
/// <seealso cref="multi_link_registry Class"/>
/**/
template<class _LinkRegistry>
class source_link_manager
{
public:

    /// <summary>
    ///     The type of link registry being managed by the <c>source_link_manager</c> object.
    /// </summary>
    /**/
    typedef _LinkRegistry type;

    /// <summary>
    ///     The type of the blocks being managed by the <c>source_link_manager</c> object.
    /// </summary>
    /**/
    typedef typename _LinkRegistry::type _Block;

    /// <summary>
    ///     The method signature for a callback method for this <c>source_link_manager</c> object.
    /// </summary>
    /**/
    typedef ::std::function<void(_Block *, bool)>  _Callback_method;

    /// <summary>
    ///     A type that represents a pointer to an element stored in the <c>source_link_manager</c> object.
    /// </summary>
    /**/
    typedef _Block * _EType;

    /// <summary>
    ///     A type that provides a reference to a <c>const</c> element stored in a <c>source_link_manager</c> object
    ///     for reading and performing const operations.
    /// </summary>
    /**/
    typedef _EType const& const_reference;

    /// <summary>
    ///     A type that provides a pointer to a <c>const</c> element in a <c>source_link_manager</c> object.
    /// </summary>
    /**/
    typedef _EType const* const_pointer;

    // Iterator
    friend class _Source_link_iterator<_LinkRegistry>;

    /// <summary>
    ///     A type that provides an iterator that can read or modify any element in the
    ///     <c>source_link_manager</c> object.
    /// </summary>
    /**/
    typedef _Source_link_iterator<_LinkRegistry> iterator;

    /// <summary>
    ///     A type that provides a reentrant lock for the <c>source_link_manager</c> object.
    /// </summary>
    /**/
    typedef ::Concurrency::details::_ReentrantPPLLock _LockType;

    /// <summary>
    ///     A type that provides a RAII scoped lock holder for a lock.
    /// </summary>
    /**/
    typedef _LockType::_Scoped_lock _LockHolder;

    /// <summary>
    ///     Constructs a <c>source_link_manager</c> object.
    /// </summary>
    /**/
    source_link_manager() : _M_iteratorCount(0), _M_pLinkedTarget(nullptr)
    {
    }

    /// <summary>
    ///     Destroys the <c>source_link_manager</c> object.
    /// </summary>
    /**/
    ~source_link_manager()
    {
        _CONCRT_ASSERT(_M_pendingRemove._Size() == 0);
    }

    /// <summary>
    ///     Registers the target block that holds this <c>source_link_manager</c> object.
    /// </summary>
    /// <param name="_PTarget">
    ///     The target block holding this <c>source_link_manager</c> object.
    /// </param>
    /**/
    void register_target_block(_Inout_ ITarget<typename _Block::source_type> * _PTarget)
    {
        _M_pLinkedTarget = _PTarget;
    }

    /// <summary>
    ///     Sets the maximum number of source links that can be added to this
    ///     <c>source_link_manager</c> object.
    /// </summary>
    /// <param name="_MaxLinks">
    ///     The maximum number of links.
    /// </param>
    /**/
    void set_bound(size_t _MaxLinks)
    {
        _M_links.set_bound(_MaxLinks);
    }

    /// <summary>
    ///     Adds a source link to the <c>source_link_manager</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be added.
    /// </param>
    /**/
    void add(_EType _Link)
    {
        if (_Link == nullptr)
        {
            return;
        }

        {
            _LockHolder _Lock(_M_lock);
            _M_links.add(_Link);

            // We need to add the _Link first and then invoke the
            // callback because _Add could throw.

            // As soon as the above lock is released, remove would
            // find the link that was added and could unlink it before
            // we are able to invoke the notification below. Keeping an
            // active iterator would prevent that from happening.
            _M_iteratorCount = _M_iteratorCount + 1;
        }

        // Acquire a reference on this link by the target
        _Link->acquire_ref(_M_pLinkedTarget);

        // Release the active iterator
        release();
    }

    /// <summary>
    ///     Removes a link from the <c>source_link_manager</c> object.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block to be removed, if found.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the link was found and removed, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool remove(_EType _Link)
    {
        bool _Removed = false;
        _EType _RemovedLink = nullptr;
        ITarget<typename _Block::source_type> * _LinkedTarget = _M_pLinkedTarget;

        if (_Link == nullptr)
        {
            return false;
        }

        {
             _LockHolder _Lock(_M_lock);
             _Removed = _M_links.remove(_Link);

             if (!_Removed)
             {
                 // No change was made
                 return _Removed;
             }

             if (_M_iteratorCount == 0)
             {
                 // Set the removed link to indicate that
                 // notification callback needs to be invoked.
                 _RemovedLink = _Link;
             }
             else
             {
                 // The iterator will complete the pending operation
                 _M_pendingRemove._Push_back(_Link);
             }
        }

        // NOTE: touching "this" pointer is dangerous as soon as the above lock is released

        // Release the reference for this link
        if (_RemovedLink != nullptr)
        {
            _RemovedLink->release_ref(_LinkedTarget);
        }

         return _Removed;
    }

    /// <summary>
    ///     Acquires a reference on the <c>source_link_manager</c> object.
    /// </summary>
    /**/
    void reference()
    {
        _LockHolder _Lock(_M_lock);
        _M_iteratorCount = _M_iteratorCount + 1;
    }

    /// <summary>
    ///     Releases the reference on the <c>source_link_manager</c> object.
    /// </summary>
    /**/
    void release()
    {
        ITarget<typename _Block::source_type> * _LinkedTarget = _M_pLinkedTarget;
        ::Concurrency::details::_Dynamic_array<_EType> _LinksToRemove;

        {
            _LockHolder _Lock(_M_lock);
            _CONCRT_ASSERT(_M_iteratorCount > 0);
            _M_iteratorCount = _M_iteratorCount - 1;

            if (_M_iteratorCount == 0)
            {
                if (_M_pendingRemove._Size() > 0)
                {
                    // Snap the pending remove list with the lock held
                    _M_pendingRemove._Swap(_LinksToRemove);
                }
            }
        }

        // NOTE: touching "this" pointer is dangerous as soon as the above lock is released

        // Release the references
        size_t _Size = _LinksToRemove._Size();

        for (size_t _I=0; _I < _Size; _I++)
        {
            _LinksToRemove[_I]->release_ref(_LinkedTarget);
        }
    }

    /// <summary>
    ///     Searches the <c>network_link_registry</c> within this <c>source_link_manager</c>
    ///     object for a specified block.
    /// </summary>
    /// <param name="_Link">
    ///     A pointer to a block that is to be searched for in the <c>source_link_manager</c> object.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the specified block was found, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool contains(_EType _Link)
    {
        _LockHolder _Lock(_M_lock);
        return _M_links.contains(_Link);
    }

    /// <summary>
    ///     Counts the number of linked blocks in the <c>source_link_manager</c> object.
    /// </summary>
    /// <returns>
    ///     The number of linked blocks in the <c>source_link_manager</c> object.
    /// </returns>
    /**/
    size_t count()
    {
        _LockHolder _Lock(_M_lock);
        return _M_links.count();
    }


    /// <summary>
    ///     Returns an iterator to the first element in the <c>source_link_manager</c> object.
    /// </summary>
    /// <remarks>
    ///     The end state of the iterator is indicated by a <c>NULL</c> link.
    /// </remarks>
    /// <returns>
    ///     An iterator addressing the first element in the <c>source_link_manager</c> object.
    /// </returns>
    /**/
    iterator begin()
    {
        return (iterator(this, 0));
    }

private:

    // Called by the iterator. This routine takes a snapshot of the links
    // in the registry and copies it to the array provided.
    void _To_array(::Concurrency::details::_Dynamic_array<_EType>& _Array)
    {
        _LockHolder _Lock(_M_lock);
        _M_iteratorCount = _M_iteratorCount + 1;

        for(auto _Link = _M_links.begin(); *_Link != nullptr; ++_Link)
        {
            _Array._Push_back(*_Link);
        }
    }

    // Internal lock used for synchronization
    _LockType _M_lock;

    // Count to indicate that an iterator is active
    volatile long _M_iteratorCount;

    // A vector of all pending link remove operations
    ::Concurrency::details::_Dynamic_array<_EType> _M_pendingRemove;

    // Underlying link registry
    _LinkRegistry _M_links;

    // Target block holding this source link manager
    ITarget<typename _Block::source_type> * volatile _M_pLinkedTarget;
};

/// <summary>
///     The valid responses for an offer of a <c>message</c> object to a block.
/// </summary>
/**/
enum message_status
{
    /// <summary>
    ///     The target accepted the message.
    /// </summary>
    /**/
    accepted,
    /// <summary>
    ///     The target did not accept the message.
    /// </summary>
    /**/
    declined,
    /// <summary>
    ///     The target postponed the message.
    /// </summary>
    /**/
    postponed,
    /// <summary>
    ///     The target tried to accept the message, but it was no longer available.
    /// </summary>
    /**/
    missed
};

/// <summary>
///     The basic message envelope containing the data payload being passed between
///     messaging blocks.
/// </summary>
/// <typeparam name="_Type">
///     The data type of the payload within the message.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/**/
template<class _Type>
class message : public ::Concurrency::details::_Runtime_object
{
    friend class ::Concurrency::details::_Queue<message<_Type>>;

public:
    /// <summary>
    ///     Constructs a <c>message</c> object.
    /// </summary>
    /// <param name="_P">
    ///     The payload of this message.
    /// </param>
    /// <remarks>
    ///     The constructor that takes a pointer to a <c>message</c> object as an argument
    ///     throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if the parameter <paramref name="_Msg"/> is <c>NULL</c>.
    /// </remarks>
    /**/
    message(_Type const &_P) : payload(_P), _M_pNext(nullptr), _M_refCount(0) { }

    /// <summary>
    ///     Constructs a <c>message</c> object.
    /// </summary>
    /// <param name="_P">
    ///     The payload of this message.
    /// </param>
    /// <param name="_Id">
    ///     The unique ID of this message.
    /// </param>
    /// <remarks>
    ///     The constructor that takes a pointer to a <c>message</c> object as an argument
    ///     throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if the parameter <paramref name="_Msg"/> is <c>NULL</c>.
    /// </remarks>
    /**/
    message(_Type const &_P, runtime_object_identity _Id)
        : ::Concurrency::details::_Runtime_object(_Id), payload(_P), _M_pNext(nullptr), _M_refCount(0)
    {
    }

    /// <summary>
    ///     Constructs a <c>message</c> object.
    /// </summary>
    /// <param name="_Msg">
    ///     A reference or pointer to a <c>message</c> object.
    /// </param>
    /// <remarks>
    ///     The constructor that takes a pointer to a <c>message</c> object as an argument
    ///     throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if the parameter <paramref name="_Msg"/> is <c>NULL</c>.
    /// </remarks>
    /**/
    message(message const & _Msg) : payload(_Msg.payload), _M_pNext(nullptr), _M_refCount(0) { }

    /// <summary>
    ///     Constructs a <c>message</c> object.
    /// </summary>
    /// <param name="_Msg">
    ///     A reference or pointer to a <c>message</c> object.
    /// </param>
    /// <remarks>
    ///     This method throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if the parameter <paramref name="_Msg"/> is <c>NULL</c>.
    /// </remarks>
    /**/
    message(_In_ message const * _Msg) : payload((_Msg == nullptr) ? nullptr : _Msg->payload), _M_pNext(nullptr), _M_refCount(0)
    {
        if (_Msg == nullptr)
        {
            throw ::std::invalid_argument("_Msg");
        }
    }

    /// <summary>
    ///     Destroys the <c>message</c> object.
    /// </summary>
    /**/
    virtual ~message() { }

    /// <summary>
    ///     Returns the ID of the <c>message</c> object.
    /// </summary>
    /// <returns>
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object.
    /// </returns>
    /**/
    runtime_object_identity msg_id() const
    {
        return _M_id;
    }

    /// <summary>
    ///     The payload of the <c>message</c> object.
    /// </summary>
    /**/
    _Type const payload;

    /// <summary>
    ///     Adds to the reference count for the <c>message</c> object.  Used for message blocks that
    ///     need reference counting to determine message lifetimes.
    /// </summary>
    /// <returns>
    ///     The new value of the reference count.
    /// </returns>
    /**/
    long add_ref()
    {
        return _InterlockedIncrement(&_M_refCount);
    }

    /// <summary>
    ///     Subtracts from the reference count for the <c>message</c> object.  Used for message blocks that
    ///     need reference counting to determine message lifetimes.
    /// </summary>
    /// <returns>
    ///     The new value of the reference count.
    /// </returns>
    /**/
    long remove_ref()
    {
        return _InterlockedDecrement(&_M_refCount);
    }

    /// <summary>
    ///     A type alias for <typeparamref name="_Type"/>.
    /// </summary>
    /**/
    typedef _Type type;

private:
    // The intrusive next pointer used by blocks that need
    // to chain messages it's holding together
    message * _M_pNext;

    // Avoid warnings about not generating assignment operators.
    message<_Type> const &operator =(message<_Type> const &);

    // A reference count for the message
    volatile long _M_refCount;
};

//**************************************************************************
// Message processor:
//**************************************************************************

/// <summary>
///     The <c>message_processor</c> class is the abstract base class for processing of
///     <c>message</c> objects.  There is no guarantee on the ordering of the messages.
/// </summary>
/// <typeparam name="_Type">
///     The data type of the payload within messages handled by this <c>message_processor</c> object.
/// </typeparam>
/// <seealso cref="ordered_message_processor Class"/>
/**/
template<class _Type>
class message_processor
{
public:
    /// <summary>
    ///     A type alias for <typeparamref name="_Type"/>.
    /// </summary>
    /**/
    typedef _Type type;

    /// <summary>
    ///     When overridden in a derived class, places messages into the block asynchronously.
    /// </summary>
    /// <param name="_Msg">
    ///     A <c>message</c> object to send asynchronously.
    /// </param>
    /// <remarks>
    ///     Processor implementations should override this method.
    /// </remarks>
    /**/
    virtual void async_send(_Inout_opt_ message<_Type> * _Msg) = 0;

    /// <summary>
    ///     When overridden in a derived class, places messages into the block synchronously.
    /// </summary>
    /// <param name="_Msg">
    ///     A <c>message</c> object to send synchronously.
    /// </param>
    /// <remarks>
    ///     Processor implementations should override this method.
    /// </remarks>
    /**/
    virtual void sync_send(_Inout_opt_ message<_Type> * _Msg) = 0;

    /// <summary>
    ///     When overridden in a derived class, waits for all asynchronous operations to complete.
    /// </summary>
    /// <remarks>
    ///     Processor implementations should override this method.
    /// </remarks>
    /**/
    virtual void wait() = 0;

protected:

    /// <summary>
    ///     When overridden in a derived class, performs the forward processing of
    ///     messages into the block. Called once every time a new message is added and
    ///     the queue is found to be empty.
    /// </summary>
    /// <remarks>
    ///     Message block implementations should override this method.
    /// </remarks>
    /**/
    virtual void process_incoming_message() = 0;

    /// <summary>
    ///     Wrapper for <c>process_incoming_message</c> suitable for use as a argument to
    ///     <c>CreateThread</c> and other similar methods.
    /// </summary>
    /// <param name="_Data">
    ///     A pointer to a message processor passed as a void pointer.
    /// </param>
    /**/
    static void __cdecl _Process_incoming_message_wrapper(void * _Data)
    {
       message_processor<_Type> * _PMessageProcessor = (message_processor<_Type> *) _Data;
       _PMessageProcessor->process_incoming_message();
    }
};

/// <summary>
///     An <c>ordered_message_processor</c> is a <c>message_processor</c> that allows message blocks
///     to process messages in the order they were received.
/// </summary>
/// <typeparam name="_Type">
///     The payload type of messages handled by the processor.
/// </typeparam>
/**/
template<class _Type>
class ordered_message_processor : public message_processor<_Type>
{
public:
    /// <summary>
    ///     The signature of the callback method invoked while processing messages.
    /// </summary>
    /**/
    typedef ::std::function<void(message<_Type> *)>  _Handler_method;

    /// <summary>
    ///     The signature of the callback method invoked while propagating messages.
    /// </summary>
    /**/
    typedef ::std::function<void()>  _Propagator_method;

    /// <summary>
    ///     A type alias for <typeparamref name="_Type"/>.
    /// </summary>
    /**/
    typedef _Type type;

    /// <summary>
    ///     Constructs an <c>ordered_message_processor</c> object.
    /// </summary>
    /// <remarks>
    ///     This <c>ordered_message_processor</c> will not schedule asynchronous or synchronous
    ///     handlers until the <c>initialize</c> function is called.
    /// </remarks>
    /**/
    ordered_message_processor() :
      _M_queuedDataCount(0),
      _M_pScheduler(nullptr),
      _M_pScheduleGroup(nullptr),
      _M_stopProcessing(1),
      _M_lwtCount(0),
      _M_handler(nullptr),
      _M_processor(nullptr),
      _M_propagator(nullptr)
    {
    }

    /// <summary>
    ///     Destroys the <c>ordered_message_processor</c> object.
    /// </summary>
    /// <remarks>
    ///     Waits for all outstanding asynchronous operations before destroying the processor.
    /// </remarks>
    /**/
    virtual ~ordered_message_processor()
    {
        wait();
    }

    /// <summary>
    ///     Initializes the <c>ordered_message_processor</c> object with the appropriate
    ///     callback function, scheduler and schedule group.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A pointer to the scheduler to be used for scheduling light-weight tasks.
    /// </param>
    /// <param name="_PScheduleGroup">
    ///     A pointer to the schedule group to be used for scheduling light-weight tasks.
    /// </param>
    /// <param name="_Handler">
    ///     The handler functor invoked during callback.
    /// </param>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    void initialize(_Inout_opt_ Scheduler * _PScheduler, _Inout_opt_ ScheduleGroup * _PScheduleGroup, _Handler_method const& _Handler)
    {
        _M_pScheduler = _PScheduler;
        _M_pScheduleGroup = _PScheduleGroup;
        _M_handler = _Handler;
        _M_stopProcessing = 0;
    }

    /// <summary>
    ///     Initialize batched message processing
    /// </summary>
    /// <param name="_Processor">
    ///     The processor functor invoked during callback.
    /// </param>
    /// <param name="_Propagator">
    ///     The propagator functor invoked during callback.
    /// </param>
    virtual void initialize_batched_processing(_Handler_method const& _Processor, _Propagator_method const& _Propagator)
    {
        _M_processor = _Processor;
        _M_propagator = _Propagator;
    }

    /// <summary>
    ///     Synchronously queues up messages and starts a processing task, if this has not been done
    ///     already.
    /// </summary>
    /// <param name="_Msg">
    ///     A pointer to a message.
    /// </param>
    /**/
    virtual void sync_send(_Inout_opt_ message<_Type> * _Msg)
    {
        if (_M_handler == nullptr)
        {
            throw invalid_operation("sync_send called without registering a callback");
        }

        _Sync_send_helper(_Msg);
    }

    /// <summary>
    ///     Asynchronously queues up messages and starts a processing task, if this has not been done
    ///     already.
    /// </summary>
    /// <param name="_Msg">
    ///     A pointer to a message.
    /// </param>
    /**/
    virtual void async_send(_Inout_opt_ message<_Type> * _Msg)
    {
        if (_M_handler == nullptr)
        {
            throw invalid_operation("async_send called without registering a callback");
        }

        //
        // If there is a message to send, enqueue it in the processing queue.
        // async_send can be sent a NULL message if the block wishes to reprocess
        // the messages that are in its queue.  For example, an unbounded_buffer
        // that has its head node released after reservation.
        //
        if (_Msg != nullptr)
        {
            _M_queuedMessages.push(_Msg);
        }

        if (_InterlockedIncrement(&_M_queuedDataCount) == 1)
        {
            // Indicate that an LWT is in progress. This will cause the
            // destructor to block.
            _InterlockedIncrement(&_M_lwtCount);

            if (_M_stopProcessing == 0)
            {
                _CONCRT_ASSERT(_M_lwtCount > 0);

                _Trace_agents(AGENTS_EVENT_SCHEDULE, ::Concurrency::details::_Trace_agents_get_id(this));

                TaskProc _Proc = &::Concurrency::ordered_message_processor<_Type>::_Process_incoming_message_wrapper;
#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
                if (_M_pScheduleGroup != nullptr)
                {
                    _M_pScheduleGroup->ScheduleTask(_Proc, this);
                }
                else if (_M_pScheduler != nullptr)
                {
                    _M_pScheduler->ScheduleTask(_Proc, this);
                }
                else
                {
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */
                    ::Concurrency::details::_CurrentScheduler::_ScheduleTask(_Proc, this);
#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
                }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

                // The LWT will decrement _M_lwtCount.
                return;
            }

            // If we get here then no task was scheduled. Decrement LWT count to reflect this fact
            _InterlockedDecrement(&_M_lwtCount);
        }
    }

    /// <summary>
    ///     A processor-specific spin wait used in destructors of message blocks to make sure
    ///     that all asynchronous processing tasks have time to finish before destroying the block.
    /// </summary>
    /**/
    virtual void wait()
    {
        // Cease processing of any new messages
        _InterlockedIncrement(&_M_stopProcessing);

        // This spin makes sure all previously initiated message processings
        // will still process correctly.  As soon as this count reaches zero, we can
        // proceed with the message block destructor.
        ::Concurrency::details::_SpinWaitBackoffNone spinWait(::Concurrency::details::_Context::_Yield);
        while(_M_lwtCount != 0)
        {
            spinWait._SpinOnce();
        }

        // Synchronize with sync_send
        {
            _NR_lock _Lock(_M_asyncSendLock);
            _Clear_queued_messages();
        }

    }

protected:

    /// <summary>
    ///     The processing function that is called asynchronously.  It dequeues messages and begins
    ///     processing them.
    /// </summary>
    /**/
    virtual void process_incoming_message()
    {
        _Trace_agents(AGENTS_EVENT_START, ::Concurrency::details::_Trace_agents_get_id(this));
        long _Count = _Process_message_helper();
        _Trace_agents(AGENTS_EVENT_END, ::Concurrency::details::_Trace_agents_get_id(this), _Count);

        // Indicate that an LWT completed
        _InterlockedDecrement(&_M_lwtCount);

        // Do not access any members here. If the count goes to
        // 0 as a result of the above decrement, the object
        // could be immediately deleted.
    }

 private:

    void _Clear_queued_messages()
    {
        message<_Type> * _Msg = nullptr;
        while (_M_queuedMessages.try_pop(_Msg))
        {
            delete _Msg;
        }
    }

    void _Sync_send_helper(message<_Type> * _Msg)
    {
        _NR_lock _Lock(_M_asyncSendLock);

        // Message block destructors sets the _M_stopProcessing flag to stop
        // processing any more messages. This is required to guarantee
        // that the destructor's wait_for_async_sends will complete
        if (_M_stopProcessing == 0)
        {
            if (_M_queuedDataCount > 0)
            {
                long _Count = _InterlockedExchange((volatile long *) &_M_queuedDataCount, 0);
                _Invoke_handler(_Count);
            }

            _Invoke_handler(_Msg);
        }
        else
        {
            // Destructor is running. Do not process the message
            // Delete the msg, if any.
            if (_Msg != nullptr)
            {
                delete _Msg;
            }
        }

    }

    // Helper function to dequeue and process messages to any targets
    long _Process_message_helper()
    {
        _NR_lock _Lock(_M_asyncSendLock);

        long _Messages_processed = 0;

        // Do batched processing of messages
        // Read off the number of messages to process in this iteration by snapping a count
        volatile long _Count = _M_queuedDataCount;
        bool _StopProcessing = false;

        // This count could be 0 if there was both a synchronous and asynchronous
        // send occurring.  One of them could have sent all of the messages for the other
        while (_Count > 0)
        {
            // Process _Count number of messages
            _Invoke_handler(_Count);
            _Messages_processed += _Count;

            // Subtract the count and see if there are new things to process
            volatile long _Orig = _InterlockedExchangeAdd((volatile long *) &_M_queuedDataCount, -_Count);
            _CONCRT_ASSERT(_Orig >= _Count);
            if (_Orig == _Count)
            {
                // Because _Count did not change, we processed everything there is to process
                break;
            }

            if (_StopProcessing)
            {
                break;
            }

            // After reading the flag process the currently queued messages
            // Any messages received after we observe this flag (to be set) will not
            // be processed.
            _StopProcessing = (_M_stopProcessing == 0) ? false : true;

            // Snap the count and try to process more
            _Count = _M_queuedDataCount;
        }

        return _Messages_processed;
    }

    // Invoke the handler in the message block for the given
    // count
    void _Invoke_handler(long _Count)
    {
        // Process _Count number of messages
        for(int _I = 0; _I < _Count; _I++)
        {
            message<_Type> * _Msg = nullptr;
            _M_queuedMessages.try_pop(_Msg);
            if (_M_processor == nullptr)
            {
                // If a processor function does not exist, the message processor is using single
                // message processing rather than batched processing.  There should also be no
                // propagator function defined in this case.
                _CONCRT_ASSERT(_M_propagator == nullptr);
                _M_handler(_Msg);
            }
            else
            {
                // Use the batched message processing function
                _M_processor(_Msg);
            }
        }

        // Call the handler which propagates the message(s)
        if (_M_propagator != nullptr)
        {
            _M_propagator();
        }
    }

    // Invoke the message block handler for the given message
    void _Invoke_handler(message<_Type> * _Msg)
    {
        if (_M_processor == nullptr)
        {
            // If a processor function does not exist, the message processor is using single
            // message processing rather than batched processing.  There should also be no
            // propagator function defined in this case.
            _CONCRT_ASSERT(_M_propagator == nullptr);
            _M_handler(_Msg);
        }
        else
        {
            // Use the batched message processing function
            _M_processor(_Msg);

            // Call the handler which propagates the message(s)
            if (_M_propagator != nullptr)
            {
                _M_propagator();
            }
        }
    }

 private:
    /// <summary>
    ///     A queue of the messages
    /// </summary>
    /**/
    concurrent_queue<message<_Type> *> _M_queuedMessages;

    /// <summary>
    ///     A lock to use for queueing incoming messages.
    /// </summary>
    /**/
    ::Concurrency::details::_NonReentrantPPLLock _M_asyncSendLock;

    /// <summary>
    ///     A count of the current number of messages to process.  Used as a flag
    ///     to see if a new process message task needs to be created.
    /// </summary>
    /**/
    volatile long _M_queuedDataCount;

    /// <summary>
    ///     The scheduler to process messages on
    /// </summary>
    /**/
    Scheduler * _M_pScheduler;

    /// <summary>
    ///     The schedule group to process messages on
    /// </summary>
    /**/
    ScheduleGroup * _M_pScheduleGroup;

    /// <summary>
    ///     A flag set in the destructor of a block to cease processing of new messages.
    ///     This is required to guarantee that _M_queuedDataCount will get to 0 eventually.
    /// </summary>
    /**/
    volatile long _M_stopProcessing;

    /// <summary>
    ///     A counter to indicate the number of outstanding LWTs
    /// </summary>
    /**/
    volatile long _M_lwtCount;

    /// <summary>
    ///     A message handler object which exposes the callback to be invoked
    /// </summary>
    /**/
    _Handler_method _M_handler;

    /// <summary>
    ///     A message processing object which exposes the callback to be invoked
    /// </summary>
    /**/
    _Handler_method _M_processor;

    /// <summary>
    ///     A message propagating object which exposes the callback to be invoked
    /// </summary>
    /**/
    _Propagator_method _M_propagator;
};

/// <summary>
///     The <c>ITarget</c> class is the interface for all target blocks.  Target blocks
///     consume messages offered to them by <c>ISource</c> blocks.
/// </summary>
/// <typeparam name="_Type">
///     The data type of the payload within the messages accepted by the target block.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/// <seealso cref="ISource Class"/>
/**/
template<class _Type>
class ITarget
{
    //
    // ISource<T> is a friend class because calls to Source->link_target()
    // and Source->unlink_target() need to call their respective
    // Target->link_source() and Target->unlink_source() on the block they are
    // linking/unlinking.  Those functions are private here because we don't
    // want users calling link_source() or unlink_source() directly.  link_source/
    // unlink_source don't call respective link_target/unlink_target because an
    // infinite loop would occur.
    //
    friend class ISource<_Type>;

public:
    /// <summary>
    ///     Destroys the <c>ITarget</c> object.
    /// </summary>
    /**/
    virtual ~ITarget() {}

    // It is important that calls to propagate do *not* take the same lock on an
    // internal message structure that is used by Consume and the LWT.  Doing so could
    // result in a deadlock with the Consume call.

    /// <summary>
    ///     When overridden in a derived class, asynchronously passes a message from a source block to
    ///     this target block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if either the <paramref name="_PMessage"/> or <paramref name="_PSource"/> parameter is <c>NULL</c>.
    /// </remarks>
    /**/
    virtual message_status propagate(_Inout_opt_ message<_Type> * _PMessage, _Inout_opt_ ISource<_Type> * _PSource) = 0;

    /// <summary>
    ///     When overridden in a derived class, synchronously passes a message to the target block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if either the <paramref name="_PMessage"/> or <paramref name="_PSource"/> parameter is <c>NULL</c>.
    ///     <para>Using the <c>send</c> method outside of message initiation and to propagate messages
    ///     within a network is dangerous and can lead to deadlock.</para>
    ///     <para>When <c>send</c> returns, the message has either already been accepted, and transferred into
    ///     the target block, or it has been declined by the target.</para>
    /// </remarks>
    /**/
    virtual message_status send(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource) = 0;

    /// <summary>
    ///     When overridden in a derived class, returns true or false depending on whether the
    ///     message block accepts messages offered by a source that is not linked to it. If the overridden method returns
    ///     <c>true</c>, the target cannot postpone an offered message, as consumption of a postponed message
    ///     at a later time requires the source to be identified in its source link registry.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the block can accept message from a source that is not linked to it
    ///     <c>false</c> otherwise.
    /// </returns>
    /**/
    virtual bool supports_anonymous_source()
    {
        return false;
    }

    /// <summary>
    ///     A type alias for <typeparamref name="_Type"/>.
    /// </summary>
    /**/
    typedef _Type type;

    /// <summary>
    ///     The signature of any method used by the block that returns a <c>bool</c> value to determine
    ///     whether an offered message should be accepted.
    /// </summary>
    /**/
    typedef ::std::function<bool(_Type const&)> filter_method;

protected:

    /// <summary>
    ///     When overridden in a derived class, links a specified source block to this <c>ITarget</c> block.
    /// </summary>
    /// <param name="_PSource">
    ///     The <c>ISource</c> block being linked to this <c>ITarget</c> block.
    /// </param>
    /// <remarks>
    ///     This function should not be called directly on an <c>ITarget</c> block. Blocks should be connected together
    ///     using the <c>link_target</c> method on <c>ISource</c> blocks, which will invoke the <c>link_source</c> method
    ///     on the corresponding target.
    /// </remarks>
    /**/
    virtual void link_source(_Inout_ ISource<_Type> * _PSource) = 0;

    /// <summary>
    ///     When overridden in a derived class, unlinks a specified source block from this <c>ITarget</c> block.
    /// </summary>
    /// <param name="_PSource">
    ///     The <c>ISource</c> block being unlinked from this <c>ITarget</c> block.
    /// </param>
    /// <remarks>
    ///     This function should not be called directly on an <c>ITarget</c> block. Blocks should be disconnected
    ///     using the <c>unlink_target</c> or <c>unlink_targets</c> methods on <c>ISource</c> blocks, which will invoke
    ///     the <c>unlink_source</c> method on the corresponding target.
    /// </remarks>
    /**/
    virtual void unlink_source(_Inout_ ISource<_Type> * _PSource) = 0;

    /// <summary>
    ///     When overridden in a derived class, unlinks all source blocks from this <c>ITarget</c> block.
    /// </summary>
    /**/
    virtual void unlink_sources() = 0;
};

/// <summary>
///     The <c>ISource</c> class is the interface for all source blocks.  Source blocks
///     propagate messages to <c>ITarget</c> blocks.
/// </summary>
/// <typeparam name="_Type">
///     The data type of the payload within the messages produced by the source block.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/// <seealso cref="ITarget Class"/>
/**/
template<class _Type>
class ISource
{
public:
    /// <summary>
    ///     Destroys the <c>ISource</c> object.
    /// </summary>
    /**/
    virtual ~ISource() {}

    /// <summary>
    ///     When overridden in a derived class, links a target block to this <c>ISource</c> block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block being linked to this <c>ISource</c> block.
    /// </param>
    /**/
    virtual void link_target(_Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     When overridden in a derived class, unlinks a target block from this <c>ISource</c> block,
    ///     if found to be previously linked.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block being unlinked from this <c>ISource</c> block.
    /// </param>
    /**/
    virtual void unlink_target(_Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     When overridden in a derived class, unlinks all target blocks from this
    ///     <c>ISource</c> block.
    /// </summary>
    /**/
    virtual void unlink_targets() = 0;

    /// <summary>
    ///     When overridden in a derived class, accepts a message that was offered by this <c>ISource</c> block,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>accept</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     The <c>accept</c> method is called by a target while a message is being offered by this <c>ISource</c> block.
    ///     The message pointer returned may be different from the one passed into the <c>propagate</c> method
    ///     of the <c>ITarget</c> block, if this source decides to make a copy of the message.
    /// </remarks>
    /**/
    virtual message<_Type> * accept(runtime_object_identity _MsgId, _Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     When overridden in a derived class, reserves a message previously offered by this <c>ISource</c> block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>reserve</c> method.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise. Reservations can fail
    ///     for many reasons, including: the message was already reserved or accepted by another target, the source could
    ///     deny reservations, and so forth.
    /// </returns>
    /// <remarks>
    ///     After you call <c>reserve</c>, if it succeeds, you must call either <c>consume</c> or <c>release</c>
    ///     in order to take or give up possession of the message, respectively.
    /// </remarks>
    /**/
    virtual bool reserve(runtime_object_identity _MsgId, _Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     When overridden in a derived class, consumes a message previously offered by this <c>ISource</c> block
    ///     and successfully reserved by the target, transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the reserved <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>consume</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     The <c>consume</c> method is similar to <c>accept</c>, but must always be preceded by a call to <c>reserve</c> that
    ///     returned <c>true</c>.
    /// </remarks>
    /**/
    virtual message<_Type> * consume(runtime_object_identity _MsgId, _Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     When overridden in a derived class, releases a previous successful message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the reserved <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>release</c> method.
    /// </param>
    /**/
    virtual void release(runtime_object_identity _MsgId, _Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     When overridden in a derived class, acquires a reference count on this <c>ISource</c> block, to prevent deletion.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being linked to this source
    ///     during the <c>link_target</c> method.
    /// </remarks>
    /**/
    virtual void acquire_ref(_Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     When overridden in a derived class, releases a reference count on this <c>ISource</c> block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being unlinked from this source.
    ///     The source block is allowed to release any resources reserved for the target block.
    /// </remarks>
    /**/
    virtual void release_ref(_Inout_ ITarget<_Type> * _PTarget) = 0;

    /// <summary>
    ///     A type alias for <typeparamref name="_Type"/>.
    /// </summary>
    /**/
    typedef _Type source_type;

protected:
    /// <summary>
    ///     Links this source to a target.
    /// </summary>
    /// <param name="_PLinkFrom">
    ///     A pointer to the target.
    /// </param>
    /// <remarks>
    ///     This function definition is required because ISource blocks need to call
    ///     Target->link_source(), which is a private member of ITarget.  ISource is
    ///     declared as a friend class, so this is a way for derived classes of ISource
    ///     to properly link/unlink their targets during link_target(), unlink_target() and
    ///     unlink_targets()
    /// </remarks>
    /**/
    void _Invoke_link_source(ITarget<_Type> * _PLinkFrom)
    {
        _PLinkFrom->link_source(this);
    }

    /// <summary>
    ///     Unlinks this source from a target.
    /// </summary>
    /// <param name="_PUnlinkFrom">
    ///     A pointer to the target.
    /// </param>
    /// <remarks>
    ///     This function definition is required because ISource blocks need to call
    ///     Target->unlink_source(), which is a private member of ITarget.  ISource is
    ///     declared as a friend class, so this is a way for derived classes of ISource
    ///     to properly link/unlink their targets during link_target(), unlink_target() and
    ///     unlink_targets()
    /// </remarks>
    /**/
    void _Invoke_unlink_source(ITarget<_Type> * _PUnlinkFrom)
    {
        _PUnlinkFrom->unlink_source(this);
    }
};

//**************************************************************************
// Direct Messaging APIs:
//**************************************************************************

/// <summary>
///     A general receive implementation, allowing a context to wait for data from
///     exactly one source and filter the values that are accepted.  If the specified timeout is not
///     COOPERATIVE_TIMEOUT_INFINITE, an exception (operation_timed_out) will be thrown if the specified amount
///     of time expires before a message is received.  Note that zero length timeouts should likely use
///     try_receive as opposed to receive with a timeout of zero as it is more efficient and does not
///     throw exceptions on timeouts.
/// </summary>
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/// <param name="_Src">
///     A pointer to the source from which data is expected.
/// </param>
/// <param name="_Timeout">
///     The maximum time for which the method should for the data, in milliseconds.
/// </param>
/// <param name="_Filter_proc">
///     A pointer to a filter which will indicate whether to accept the data or not.
/// </param>
/// <returns>
///     A value from the source, of the payload type.
/// </returns>
/**/
template <class _Type>
_Type _Receive_impl(ISource<_Type> * _Src, unsigned int _Timeout, typename ITarget<_Type>::filter_method const* _Filter_proc)
{
    // The Blocking Recipient messaging block class is internal to the receive function
    class _Blocking_recipient : public ITarget<_Type>
    {
    public:
        // Create a Blocking Recipient
        _Blocking_recipient(ISource<_Type> * _PSource,
            unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE) :
            _M_fState(_NotInitialized), _M_pConnectedTo(nullptr), _M_pMessage(nullptr), _M_timeout(_Timeout), _M_pFilter(nullptr)
        {
            _Connect(_PSource);
        }

        // Create a Blocking Recipient
        _Blocking_recipient(ISource<_Type> * _PSource,
            typename ITarget<_Type>::filter_method const& _Filter,
            unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE) :
            _M_fState(_NotInitialized), _M_pConnectedTo(nullptr), _M_pMessage(nullptr), _M_timeout(_Timeout), _M_pFilter(nullptr)
        {
            if (_Filter != nullptr)
            {
                _M_pFilter = new typename ITarget<_Type>::filter_method(_Filter);
            }

            _Connect(_PSource);
        }

        // Cleans up any resources that may have been created by the BlockingRecipient.
        ~_Blocking_recipient()
        {
            _Disconnect();

            delete _M_pFilter;
            delete _M_pMessage;
        }

        // Gets the value of the message sent to this BlockingRecipient.  Blocks by
        // spinning until a message has arrived.
        _Type _Value()
        {
            _Wait_for_message();

            return _M_pMessage->payload;
        }

        // The main propagation function for ITarget blocks.  Called by a source
        // block, generally within an asynchronous task to send messages to its targets.
        virtual message_status propagate(message<_Type> * _PMessage, ISource<_Type> * _PSource)
        {
            // Throw exception if the message being propagated to this block is NULL
            if (_PMessage == nullptr)
            {
                throw ::std::invalid_argument("_PMessage");
            }

            if (_PSource == nullptr)
            {
                throw ::std::invalid_argument("_PSource");
            }

            // Reject if the recipient has already received a message
            if (_M_fState == _Initialized)
            {
                return declined;
            }

            // Reject if the message does not meet the filter requirements
            if (_M_pFilter != nullptr && !(*_M_pFilter)(_PMessage->payload))
            {
                return declined;
            }

            // Accept the message
            _CONCRT_ASSERT(_PSource != nullptr);
            _M_pMessage = _PSource->accept(_PMessage->msg_id(), this);

            if (_M_pMessage != nullptr)
            {
                // Set the initialized flag on this block
                if (_InterlockedExchange(&_M_fState, _Initialized) == _Blocked)
                {
                    _M_ev.set();
                }

                return accepted;
            }

            return missed;
        }

        // Synchronously sends a message to this block.  When this function completes the message will
        // already have propagated into the block.
        virtual message_status send(message<_Type> * _PMessage, ISource<_Type> * _PSource)
        {
            if (_PMessage == nullptr)
            {
                throw ::std::invalid_argument("_PMessage");
            }

            if (_PSource == nullptr)
            {
                throw ::std::invalid_argument("_PSource");
            }

            // Only the connected source is allowed to send messages
            // to the blocking recipient. Decline messages without
            // a source.

            return declined;
        }

    private:

        // Link a source block
        virtual void link_source(ISource<_Type> * _PSrc)
        {
            _M_pConnectedTo = _PSrc;
            _PSrc->acquire_ref(this);
        }

        // Remove a source messaging block for this BlockingRecipient
        virtual void unlink_source(ISource<_Type> * _PSource)
        {
            if (_InterlockedCompareExchangePointer(reinterpret_cast<void *volatile *>(&_M_pConnectedTo), (void *)nullptr, _PSource) == _PSource)
            {
                _PSource->release_ref(this);
            }
        }

        // Remove the source messaging block for this BlockingRecipient
        virtual void unlink_sources()
        {
            ISource<_Type> * _PSource = reinterpret_cast<ISource<_Type> *>(_InterlockedExchangePointer(reinterpret_cast<void *volatile *>(&_M_pConnectedTo), (void *)nullptr));
            if (_PSource != nullptr)
            {
                _PSource->unlink_target(this);
                _PSource->release_ref(this);
            }
        }


        // Connect the blocking recipient to the source
        void _Connect(ISource<_Type> * _PSource)
        {
            if (_PSource == nullptr)
            {
                throw ::std::invalid_argument("_PSource");
            }

            _PSource->link_target(this);
        }

        // Cleanup the connection to the blocking recipient's source. There is no need
        // to do anything about the associated context.
        void _Disconnect()
        {
            unlink_sources();
        }

        // Internal function used to block while waiting for a message to arrive
        // at this BlockingRecipient
        void _Wait_for_message()
        {
            bool _Timeout = false;

            // If we haven't received a message yet, cooperatively block.
            if (_InterlockedCompareExchange(&_M_fState, _Blocked, _NotInitialized) == _NotInitialized)
            {
                if (_M_ev.wait(_M_timeout) == COOPERATIVE_WAIT_TIMEOUT)
                {
                    _Timeout = true;
                }
            }

            // Unlinking from our source guarantees that there are no threads in propagate
            _Disconnect();

            if (_M_fState != _Initialized)
            {
                // We had to have timed out if we came out of the wait
                // without being initialized.
                _CONCRT_ASSERT(_Timeout);

                throw operation_timed_out();
            }
        }

        // States for this block
        enum
        {
            _NotInitialized,
            _Blocked,
            _Initialized
        };

        volatile long _M_fState;

        // The source messaging block connected to this Recipient
        ISource<_Type> * _M_pConnectedTo;

        // The message that was received
        message<_Type> * volatile _M_pMessage;

        // The timeout.
        unsigned int _M_timeout;

        // The event we wait upon
        event _M_ev;

        // The filter that is called on this block before accepting a message
        typename ITarget<_Type>::filter_method * _M_pFilter;
    };

    if (_Filter_proc != nullptr)
    {
        _Blocking_recipient _Recipient(_Src, *_Filter_proc, _Timeout);
        return _Recipient._Value();
    }
    else
    {
        _Blocking_recipient _Recipient(_Src, _Timeout);
        return _Recipient._Value();
    }
}

/// <summary>
///     A general receive implementation, allowing a context to wait for data from
///     exactly one source and filter the values that are accepted.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_Timeout">
///     The maximum time for which the method should for the data, in milliseconds.
/// </param>
/// <returns>
///     A value from the source, of the payload type.
/// </returns>
/// <remarks>
///     If the parameter <paramref name="_Timeout"/> has a value other than the constant <c>COOPERATIVE_TIMEOUT_INFINITE</c>,
///     the exception <see cref="operation_timed_out Class">operation_timed_out</see> is thrown if the specified amount
///     of time expires before a message is received.  If you want a zero length timeout, you should use the
///     <see cref="try_receive Function">try_receive</see> function, as opposed to calling <c>receive</c> with a timeout
///     of <c>0</c> (zero), as it is more efficient and does not throw exceptions on timeouts.
///     <para>For more information, see <see cref="Message Passing Functions"/>.</para>
/// </remarks>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
_Type receive(_Inout_ ISource<_Type> * _Src, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE)
{
    return _Receive_impl(_Src, _Timeout, nullptr);
}

/// <summary>
///     A general receive implementation, allowing a context to wait for data from
///     exactly one source and filter the values that are accepted.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_Filter_proc">
///     A filter function which determines whether messages should be accepted.
/// </param>
/// <param name="_Timeout">
///     The maximum time for which the method should for the data, in milliseconds.
/// </param>
/// <returns>
///     A value from the source, of the payload type.
/// </returns>
/// <remarks>
///     If the parameter <paramref name="_Timeout"/> has a value other than the constant <c>COOPERATIVE_TIMEOUT_INFINITE</c>,
///     the exception <see cref="operation_timed_out Class">operation_timed_out</see> is thrown if the specified amount
///     of time expires before a message is received.  If you want a zero length timeout, you should use the
///     <see cref="try_receive Function">try_receive</see> function, as opposed to calling <c>receive</c> with a timeout
///     of <c>0</c> (zero), as it is more efficient and does not throw exceptions on timeouts.
///     <para>For more information, see <see cref="Message Passing Functions"/>.</para>
/// </remarks>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
_Type receive(_Inout_ ISource<_Type> * _Src, typename ITarget<_Type>::filter_method const& _Filter_proc, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE)
{
    return _Receive_impl(_Src, _Timeout, &_Filter_proc);
}

/// <summary>
///     A general receive implementation, allowing a context to wait for data from
///     exactly one source and filter the values that are accepted.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_Timeout">
///     The maximum time for which the method should for the data, in milliseconds.
/// </param>
/// <returns>
///     A value from the source, of the payload type.
/// </returns>
/// <remarks>
///     If the parameter <paramref name="_Timeout"/> has a value other than the constant <c>COOPERATIVE_TIMEOUT_INFINITE</c>,
///     the exception <see cref="operation_timed_out Class">operation_timed_out</see> is thrown if the specified amount
///     of time expires before a message is received.  If you want a zero length timeout, you should use the
///     <see cref="try_receive Function">try_receive</see> function, as opposed to calling <c>receive</c> with a timeout
///     of <c>0</c> (zero), as it is more efficient and does not throw exceptions on timeouts.
///     <para>For more information, see <see cref="Message Passing Functions"/>.</para>
/// </remarks>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
_Type receive(ISource<_Type> &_Src, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE)
{
    return _Receive_impl(&_Src, _Timeout, nullptr);
}

/// <summary>
///     A general receive implementation, allowing a context to wait for data from
///     exactly one source and filter the values that are accepted.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_Filter_proc">
///     A filter function which determines whether messages should be accepted.
/// </param>
/// <param name="_Timeout">
///     The maximum time for which the method should for the data, in milliseconds.
/// </param>
/// <returns>
///     A value from the source, of the payload type.
/// </returns>
/// <remarks>
///     If the parameter <paramref name="_Timeout"/> has a value other than the constant <c>COOPERATIVE_TIMEOUT_INFINITE</c>,
///     the exception <see cref="operation_timed_out Class">operation_timed_out</see> is thrown if the specified amount
///     of time expires before a message is received.  If you want a zero length timeout, you should use the
///     <see cref="try_receive Function">try_receive</see> function, as opposed to calling <c>receive</c> with a timeout
///     of <c>0</c> (zero), as it is more efficient and does not throw exceptions on timeouts.
///     <para>For more information, see <see cref="Message Passing Functions"/>.</para>
/// </remarks>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
_Type receive(ISource<_Type> &_Src, typename ITarget<_Type>::filter_method const& _Filter_proc, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE)
{
    return _Receive_impl(&_Src, _Timeout, &_Filter_proc);
}

/// <summary>
///     Helper function that implements try_receive
///     A general try-receive implementation, allowing a context to look for data from
///     exactly one source and filter the values that are accepted. If the data is not
///     ready, try_receive will return false.
/// </summary>
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/// <param name="_Src">
///     A pointer to the source from which data is expected.
/// </param>
/// <param name="_value">
///     A reference to a location where the result will be placed.
/// </param>
/// <param name="_Filter_proc">
///     A pointer to a filter which will indicate whether to accept the data or not.
/// </param>
/// <returns>
///     A bool indicating whether a payload was placed in <paramref name="_value"/> or not.
/// </returns>
/**/
template <class _Type>
bool _Try_receive_impl(ISource<_Type> * _Src, _Type & _value, typename ITarget<_Type>::filter_method const * _Filter_proc)
{
    // The Immediate Recipient messaging block class is internal to the receive function
    class _Immediate_recipient : public ITarget<_Type>
    {
    public:
        // Create an Immediate Recipient
        _Immediate_recipient(ISource<_Type> * _PSource) :
            _M_pConnectedTo(nullptr), _M_pMessage(nullptr), _M_isInitialized(0), _M_pFilter(nullptr)
        {
            _Connect(_PSource);
        }

        // Create an Immediate Recipient
        _Immediate_recipient(ISource<_Type> * _PSource,
            typename ITarget<_Type>::filter_method const& _Filter) :
            _M_pConnectedTo(nullptr), _M_pMessage(nullptr), _M_isInitialized(0), _M_pFilter(nullptr)
        {
            if (_Filter != nullptr)
            {
                _M_pFilter = new typename ITarget<_Type>::filter_method(_Filter);
            }

            _Connect(_PSource);
        }

        // Cleans up any resources that may have been created by the ImmediateRecipient.
        ~_Immediate_recipient()
        {
            _Disconnect();

            delete _M_pFilter;
            delete _M_pMessage;
        }

        // Gets the value of the message sent to this ImmediateRecipient.
        bool _Value(_Type & _value)
        {
            // Unlinking from our source guarantees that there are no threads in propagate
            _Disconnect();

            if (_M_pMessage != nullptr)
            {
                _value = _M_pMessage->payload;
                return true;
            }

            return false;
        }

        // The main propagation function for ITarget blocks.  Called by a source
        // block, generally within an asynchronous task to send messages to its targets.
        virtual message_status propagate(message<_Type> * _PMessage, ISource<_Type> * _PSource)
        {
            message_status _Result = accepted;

            // Throw exception if the message being propagated to this block is NULL
            if (_PMessage == nullptr)
            {
                throw ::std::invalid_argument("_PMessage");
            }

            if (_PSource == nullptr)
            {
                throw ::std::invalid_argument("_PSource");
            }

            // Reject if the recipient has already received a message
            if (_M_isInitialized == 1)
            {
                return declined;
            }

            // Reject if the message does not meet the filter requirements
            if (_M_pFilter != nullptr && !(*_M_pFilter)(_PMessage->payload))
            {
                return declined;
            }

            // Accept the message
            _CONCRT_ASSERT(_PSource != nullptr);
            _M_pMessage = _PSource->accept(_PMessage->msg_id(), this);

            // Set the initialized flag on this block

            if (_M_pMessage != nullptr)
            {
                // Fence to ensure that the above update to _M_pMessage is visible
                _InterlockedExchange(&_M_isInitialized, 1);
                _Result = accepted;
            }
            else
            {
                _Result = missed;
            }

            return _Result;
        }


        // Synchronously sends a message to this block.  When this function completes the message will
        // already have propagated into the block.
        virtual message_status send(message<_Type> * _PMessage, ISource<_Type> * _PSource)
        {
            if (_PMessage == nullptr)
            {
                throw ::std::invalid_argument("_PMessage");
            }

            if (_PSource == nullptr)
            {
                throw ::std::invalid_argument("_PSource");
            }

            // Only the connected source is allowed to send messages
            // to the blocking recipient. Decline messages without
            // a source.

            return declined;
        }

    private:

        // Add a source messaging block
        virtual void link_source(ISource<_Type> * _PSrc)
        {
            _M_pConnectedTo = _PSrc;
            _PSrc->acquire_ref(this);
        }

        // Remove a source messaging block for this BlockingRecipient
        virtual void unlink_source(ISource<_Type> * _PSource)
        {
            if (_InterlockedCompareExchangePointer(reinterpret_cast<void *volatile *>(&_M_pConnectedTo), (void *)nullptr, _PSource) == _PSource)
            {
                _PSource->release_ref(this);
            }
        }

        // Remove the source messaging block for this BlockingRecipient
        virtual void unlink_sources()
        {
            ISource<_Type> * _PSource = reinterpret_cast<ISource<_Type> *>(_InterlockedExchangePointer(reinterpret_cast<void *volatile *>(&_M_pConnectedTo), (void *)nullptr));
            if (_PSource != nullptr)
            {
                _PSource->unlink_target(this);
                _PSource->release_ref(this);
            }
        }

        // Connect to a source block
        void _Connect(ISource<_Type> * _PSource)
        {
            if (_PSource == nullptr)
            {
                throw ::std::invalid_argument("_PSource");
            }

            _CONCRT_ASSERT(_M_isInitialized == 0);

            _PSource->link_target(this);
        }

        //
        // Cleanup the connection to the trigger's source. There is no need
        // to do anything about the associated context.
        //
        void _Disconnect()
        {
            unlink_sources();
        }

        // The source messaging block connected to this Recipient
        ISource<_Type> * _M_pConnectedTo;

        // The message that was received
        message<_Type> * volatile _M_pMessage;

        // A flag for whether or not this block has been initialized with a value
        volatile long _M_isInitialized;

        // The filter that is called on this block before accepting a message
        typename ITarget<_Type>::filter_method * _M_pFilter;
    };

    if (_Filter_proc != nullptr)
    {
        _Immediate_recipient _Recipient(_Src, *_Filter_proc);
        return _Recipient._Value(_value);
    }
    else
    {
        _Immediate_recipient _Recipient(_Src);
        return _Recipient._Value(_value);
    }
}

/// <summary>
///     A general try-receive implementation, allowing a context to look for data from
///     exactly one source and filter the values that are accepted. If the data is not
///     ready, the method will return false.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_value">
///     A reference to a location where the result will be placed.
/// </param>
/// <returns>
///     A <c>bool</c> value indicating whether or not a payload was placed in <paramref name="_value"/>.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
bool try_receive(_Inout_ ISource<_Type> * _Src, _Type & _value)
{
    return _Try_receive_impl(_Src, _value, nullptr);
}

/// <summary>
///     A general try-receive implementation, allowing a context to look for data from
///     exactly one source and filter the values that are accepted. If the data is not
///     ready, the method will return false.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_value">
///     A reference to a location where the result will be placed.
/// </param>
/// <param name="_Filter_proc">
///     A filter function which determines whether messages should be accepted.
/// </param>
/// <returns>
///     A <c>bool</c> value indicating whether or not a payload was placed in <paramref name="_value"/>.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
bool try_receive(_Inout_ ISource<_Type> * _Src, _Type & _value, typename ITarget<_Type>::filter_method const& _Filter_proc)
{
    return _Try_receive_impl(_Src, _value, &_Filter_proc);
}

/// <summary>
///     A general try-receive implementation, allowing a context to look for data from
///     exactly one source and filter the values that are accepted. If the data is not
///     ready, the method will return false.
/// </summary>
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_value">
///     A reference to a location where the result will be placed.
/// </param>
/// <returns>
///     A <c>bool</c> value indicating whether or not a payload was placed in <paramref name="_value"/>.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
bool try_receive(ISource<_Type> & _Src, _Type & _value)
{
    return _Try_receive_impl(&_Src, _value, nullptr);
}

/// <summary>
///     A general try-receive implementation, allowing a context to look for data from
///     exactly one source and filter the values that are accepted. If the data is not
///     ready, the method will return false.
/// </summary>
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/// <param name="_Src">
///     A pointer or reference to the source from which data is expected.
/// </param>
/// <param name="_value">
///     A reference to a location where the result will be placed.
/// </param>
/// <param name="_Filter_proc">
///     A filter function which determines whether messages should be accepted.
/// </param>
/// <returns>
///     A <c>bool</c> value indicating whether or not a payload was placed in <paramref name="_value"/>.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="send Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
bool try_receive(ISource<_Type> & _Src, _Type & _value, typename ITarget<_Type>::filter_method const& _Filter_proc)
{
    return _Try_receive_impl(&_Src, _value, &_Filter_proc);
}

namespace details
{
    //**************************************************************************
    // Supporting blocks for send and asend
    //**************************************************************************

    // Originator block that pushes messages to a target
    template <class _Type>
    class _AnonymousOriginator : public ISource<_Type>
    {
    public:

        typedef single_link_registry<ITarget<_Type>> _Target_registry;

        // Create an Originator
        _AnonymousOriginator() : _M_pMessage(nullptr),  _M_pTarget(nullptr)
        {
        }

        // Cleans up any resources that may have been created by the Originator.
        virtual ~_AnonymousOriginator()
        {
            delete _M_pMessage;
        }

        // Removes a target messaging block for this Originator
        virtual void unlink_target(ITarget<_Type> *)
        {
            throw invalid_operation("unlink_target is not supported on _AnonymousOriginator");
        }

        // Removes the target messaging block from this Originator
        virtual void unlink_targets()
        {
            throw invalid_operation("unlink_targets is not supported on _AnonymousOriginator");
        }

        // Accept on this Originator is called by a target to take ownership of a
        // propagated message
        virtual message<_Type> * accept(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget != _M_pTarget)
            {
                return nullptr;
            }

            if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
            {
                return nullptr;
            }

            // The IDs match. Actually transfer ownership of the message and
            // unlink away from the target
            message<_Type> * _Result = _M_pMessage;

            // The ownership of this message has changed.  Set the internal pointer to NULL
            // so it won't be deleted in the destructor
            _M_pMessage = nullptr;

            return _Result;
        }

        // Reserve shall not be called by blocks that supports push
        virtual bool reserve(runtime_object_identity, ITarget<_Type> *)
        {
            throw invalid_operation("reserve is not supported on _AnonymousOriginator");
        }

        // Consume shall not be called by blocks that supports push
        virtual message<_Type> * consume(runtime_object_identity, ITarget<_Type> *)
        {
            throw invalid_operation("consume is not supported on _AnonymousOriginator");
        }

        // Release needs to be defined for ISource blocks, but Originator doesn't need to
        // do anything for reservation release because there can only be one target block to read
        // the data at a later time.
        virtual void release(runtime_object_identity, ITarget<_Type> *)
        {
            throw invalid_operation("release is not supported on _AnonymousOriginator");
        }

        virtual void acquire_ref(_Inout_ ITarget<_Type> *)
        {
            throw invalid_operation("acquire_ref is not supported on _AnonymousOriginator");
        }

        virtual void release_ref(_Inout_ ITarget<_Type> *)
        {
            throw invalid_operation("release_ref is not supported on _AnonymousOriginator");
        }

    private:
        friend class _Originator;

        // Send the given value to the target
        bool _internal_send(ITarget<_Type> * _PTarget, _Type const & _Value)
        {
            _M_pTarget = _PTarget;

            _CONCRT_ASSERT(_M_pTarget != nullptr);
            _CONCRT_ASSERT(_M_pTarget->supports_anonymous_source());

            // Create the message
            message_status _Status = declined;
            message<_Type> * _Msg = new message<_Type>(_Value);

            _CONCRT_ASSERT(_M_pMessage == nullptr);
            _M_pMessage = _Msg;

            // Send the message
            _Status = _M_pTarget->send(_M_pMessage, this);

            // If the message is declined, the destructor will
            // delete the message

            // status should not be postponed.
            _CONCRT_ASSERT(_Status != postponed);
            return (_Status == accepted);
        }

        bool _internal_asend(ITarget<_Type> * _PTarget, _Type const & _Value)
        {
            _M_pTarget = _PTarget;

            _CONCRT_ASSERT(_M_pTarget != nullptr);
            _CONCRT_ASSERT(_M_pTarget->supports_anonymous_source());

            // Create the message
            message_status _Status = declined;
            message<_Type> * _Msg = new message<_Type>(_Value);

            _CONCRT_ASSERT(_M_pMessage == nullptr);
            _M_pMessage = _Msg;

            // Send the message
            _Status = _M_pTarget->propagate(_M_pMessage, this);

            // If the message is declined, the destructor will
            // delete the message

            // status should not be postponed.
            if (_Status == postponed)
            {
                throw invalid_operation("Messages offered by _AnonymousOriginator shall not be postponed");
            }

            return (_Status == accepted);
        }

        // Add a target messaging block for this Originator
        virtual void link_target(ITarget<_Type> *)
        {
            throw invalid_operation("link_target is not supported on _AnonymousOriginator");
        }

        // The message that will be propagated by the Originator
        message<_Type> * _M_pMessage;

        // The single target for this block
        ITarget<_Type> * _M_pTarget;
    };

    // The Originator messaging block class is internal to the send function.
    template <class _Type>
    class _SyncOriginator : public ISource<_Type>
    {
    public:

        typedef single_link_registry<ITarget<_Type>> _Target_registry;

        // Create an Originator
        _SyncOriginator() :
          _M_pMessage(nullptr),
          _M_fStatus(postponed),
          _M_referenceCount(0)
        {
        }

        // Cleans up any resources that may have been created by the Originator.
        virtual ~_SyncOriginator()
        {
            unlink_targets();

            _Wait_on_ref();

            delete _M_pMessage;
        }

        // Removes a target messaging block for this Originator
        virtual void unlink_target(ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }
            {
                // Hold the lock to ensure that the target doesn't unlink while
                // propagation is in progress.
                _R_lock _Lock(_M_internalLock);
                if (_M_connectedTargets.remove(_PTarget))
                {
                    this->_Invoke_unlink_source(_PTarget);

                    // Indicate that the send is complete
                    _Done(declined);
                }
            }
        }

        // Removes the target messaging block from this Originator
        virtual void unlink_targets()
        {
            // Hold the lock to ensure that the target doesn't unlink while
            // propagation is in progress.
            _R_lock _Lock(_M_internalLock);

            for (typename _Target_registry::iterator _Iter = _M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
            {
                ITarget<_Type> * _PTarget = *_Iter;
                if (_M_connectedTargets.remove(_PTarget))
                {
                    this->_Invoke_unlink_source(_PTarget);
                }
            }

            // All targets should be unlinked
            _CONCRT_ASSERT(_M_connectedTargets.count() == 0);

            // Indicate that the send is complete
            _Done(declined);
        }

        // Accept on this Originator is called by a target to take ownership of a
        // propagated message
        virtual message<_Type> * accept(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                return nullptr;
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                return nullptr;
            }

            if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
            {
                return nullptr;
            }

            // The IDs match. Actually transfer ownership of the message and
            // unlink away from the target
            message<_Type> * _Result = _M_pMessage;

            // The ownership of this message has changed.  Set the internal pointer to NULL
            // so it won't be deleted in the destructor
            _M_pMessage = nullptr;

            // The message has been accepted/consumed, propagate indication that it has succeeded
            _Done(accepted);

            return _Result;
        }

        // Reserve needs to be defined for ISource blocks, but Originator doesn't need to
        // do anything for reservation because there can only be one target block to read
        // the data at a later time.
        virtual bool reserve(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                return false;
            }

            if (_M_pMessage->msg_id() != _MsgId)
            {
                return false;
            }

            return true;
        }

        // Consume is called by a target messaging block to take ownership of a
        // previously reserved message.
        virtual message<_Type> * consume(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                throw bad_target();
            }

            return accept(_MsgId, _PTarget);
        }

        // Release needs to be defined for ISource blocks, but Originator doesn't need to
        // do anything for reservation release because there can only be one target block to read
        // the data at a later time.
        virtual void release(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                throw bad_target();
            }

            if ((_M_pMessage == nullptr) || (_M_pMessage->msg_id() != _MsgId))
            {
                throw message_not_found();
            }

            // If the previously reserved message is released, then propagate
            // declined  to indicate that the message was not accepted.
            _Done(declined);
        }

        virtual void acquire_ref(_Inout_ ITarget<_Type> *)
        {
            _InterlockedIncrement(&_M_referenceCount);
        }

        virtual void release_ref(_Inout_ ITarget<_Type> *)
        {
            _InterlockedDecrement(&_M_referenceCount);
        }

    private:

        friend class _Originator;

        // Send the given value to the target
        bool _internal_send(ITarget<_Type> * _PTarget, _Type const & _Value)
        {
            // _send should only be called once.
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            message_status _Status = declined;
            message<_Type> * _Msg = new message<_Type>(_Value);

            {
                // Hold the lock to ensure that the target doesn't unlink while
                // propagation is in progress.
                _R_lock _Lock(_M_internalLock);

                // link to the target, create a message and send it
                link_target(_PTarget);

                _CONCRT_ASSERT(_M_pMessage == nullptr);
                _M_pMessage = _Msg;

                // Send the message synchronously to the target
                _Status = _PTarget->send(_M_pMessage, this);
            }

            if (_Status == postponed)
            {
                // If the target postponed the message, wait for it to
                // be accepted/declined.
                _Wait_for_completion();

                // Procure the final status
                _Status = _M_fStatus;
            }

            // status should not be postponed.
            _CONCRT_ASSERT(_Status != postponed);

            return (_Status == accepted);
        }

        // Add a target messaging block for this Originator
        virtual void link_target(ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            _M_connectedTargets.add(_PTarget);
            this->_Invoke_link_source(_PTarget);

            // There should be no pending messages to propagate at this time.
            _CONCRT_ASSERT(_M_pMessage == nullptr);
        }

        // Wait for the status to reach one of the terminal
        // states (!= postponed)
        void _Wait_for_completion()
        {
            // Wait for the event to be signalled
            _M_ev.wait(COOPERATIVE_TIMEOUT_INFINITE);
            _CONCRT_ASSERT(_M_fStatus != postponed);

        }

        void _Wait_on_ref()
        {
            ::Concurrency::details::_SpinWaitBackoffNone spinWait;
            while(_M_referenceCount != 0)
            {
                spinWait._SpinOnce();
            }
        }

        // Indicate that the send operation has completed
        void _Done(message_status _Status)
        {
            // postponed is not a done state
            _CONCRT_ASSERT(_Status != postponed);

            _M_fStatus = _Status;
            _M_ev.set();
        }

        // The message that will be propagated by the Originator
        message<_Type> * _M_pMessage;

        // Event to indicate completion
        event _M_ev;

        // Final status of the send
        volatile message_status _M_fStatus;

        // A lock for modifying the buffer or the connected blocks
        ::Concurrency::details::_ReentrantPPLLock _M_internalLock;

        // Connected targets
        _Target_registry _M_connectedTargets;

        volatile long _M_referenceCount;
    };

    // The Originator messaging block class is internal to the send function.
    template <class _Type>
    class _AsyncOriginator : public ISource<_Type>
    {
    public:

        typedef single_link_registry<ITarget<_Type>> _Target_registry;

        // Cleans up any resources that may have been created by the AsyncOriginator.
        virtual ~_AsyncOriginator()
        {
            unlink_targets();

            delete _M_pMessage;
        }

        // Removes a target messaging block for this AsyncOriginator
        virtual void unlink_target(ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            bool _Unlinked = false;
            {
                // Hold the lock to ensure that the target doesn't unlink while
                // propagation is in progress.
                _R_lock _Lock(_M_internalLock);

                if (_M_connectedTargets.remove(_PTarget))
                {
                    this->_Invoke_unlink_source(_PTarget);
                    _Unlinked = true;
                }
            }

            // Release the lock before decrementing the refcount. Otherwise, the
            // lock release could corrupt memory.
            if (_Unlinked)
            {
                _Release_ref();
            }
        }

        // Removes the target messaging block from this AsyncOriginator
        virtual void unlink_targets()
        {
            bool _Unlinked = false;
            {
                // Hold the lock to ensure that the target doesn't unlink while
                // propagation is in progress.
                _R_lock _Lock(_M_internalLock);

                for (typename _Target_registry::iterator _Iter = _M_connectedTargets.begin();
                    *_Iter != nullptr;
                    ++_Iter)
                {
                    ITarget<_Type> * _PTarget = *_Iter;
                    if (_M_connectedTargets.remove(_PTarget))
                    {
                        this->_Invoke_unlink_source(_PTarget);
                        _Unlinked = true;
                    }

                }

                // All targets should be unlinked
                _CONCRT_ASSERT(_M_connectedTargets.count() == 0);
            }

            // Release the lock before decrementing the refcount. Otherwise, the
            // lock release could corrupt memory.
            if (_Unlinked)
            {
                _Release_ref();
            }
        }

        // Accept on this AsyncOriginator is called by a target to take ownership of a
        // propagated message. This can only be called from propagate.
        virtual message<_Type> * accept(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                return nullptr;
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                return nullptr;
            }

            if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
            {
                return nullptr;
            }

            //
            // If the IDs match, actually transfer ownership of the message.
            //
            message<_Type> * _Result = _M_pMessage;
            _M_pMessage = nullptr;

            return _Result;
        }

        // Reserve needs to be defined for ISource blocks, but AsyncOriginator doesn't need to
        // do anything for reservation because there can only be one target block to read
        // the data at a later time.
        virtual bool reserve(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                return false;
            }

            if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
            {
                return false;
            }

            return true;
        }

        // Consume is called by a target messaging block to take ownership of a
        // previously reserved message.
        virtual message<_Type> * consume(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                throw bad_target();
            }

            if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
            {
                return nullptr;
            }

            // The ownership of this message has changed.  Set the internal pointer to NULL
            // so it won't be deleted in the destructor

            message<_Type> * _Result = _M_pMessage;
            _M_pMessage = nullptr;

            // We are done. Unlink from the target. DO NOT TOUCH "this" pointer after unlink
            unlink_target(_PTarget);

            return _Result;
        }

        // Release needs to be defined for ISource blocks, but AsyncOriginator doesn't need to
        // do anything for reservation release because there can only be one target block to read
        // the data at a later time.
        virtual void release(runtime_object_identity _MsgId, ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            if (!_M_connectedTargets.contains(_PTarget))
            {
                throw bad_target();
            }

            if ((_M_pMessage == nullptr) || (_M_pMessage->msg_id() != _MsgId))
            {
                throw message_not_found();
            }

            // We can be connected to only 1 target. Unlink from the target.
            // DO NOT TOUCH "this" pointer after unlink
            unlink_target(_PTarget);
        }

        virtual void acquire_ref(_Inout_ ITarget<_Type> *)
        {
            _Acquire_ref();
        }

        virtual void release_ref(_Inout_ ITarget<_Type> *)
        {
            _Release_ref();
        }

    private:

        friend class _Originator;

        // Create an AsyncOriginator (constructor is private to ensure that
        // it is allocated on the heap).
        _AsyncOriginator() :
          _M_pMessage(nullptr),
          _M_refcount(0)
        {
        }

        // Send the given value to the target
        bool _internal_send(ITarget<_Type> * _PTarget, _Type const & _Value)
        {
            // Keep a refcount so that this object doesn't get deleted if
            // the target decides to unlink before we release our lock
            _Acquire_ref();

            message_status _Status = declined;
            message<_Type> * _Msg = new message<_Type>(_Value);

            {
                // Hold the lock to ensure that the target doesn't unlink while
                // propagation is in progress.
                _R_lock _Lock(_M_internalLock);

                // link to the target, create a message and send it
                link_target(_PTarget);

                _CONCRT_ASSERT(_M_pMessage == nullptr);
                _M_pMessage = _Msg;

                _Status = _PTarget->propagate(_M_pMessage, this);
            }

            // If the status is anything other than postponed, unlink away
            // from the target and delete the AsyncOriginator.
            if (_Status != postponed)
            {
                unlink_target(_PTarget);
            }

            // Release the reference acquired above
            _Release_ref();

            return (_Status == accepted);
        }

        // Add a target messaging block for this AsyncOriginator
        virtual void link_target(ITarget<_Type> * _PTarget)
        {
            if (_PTarget == nullptr)
            {
                throw ::std::invalid_argument("_PTarget");
            }

            // Acquire a reference that will be released by unlink_target
            _Acquire_ref();
            _M_connectedTargets.add(_PTarget);
            this->_Invoke_link_source(_PTarget);

            // There should be no pending messages to propagate at this time.
            _CONCRT_ASSERT(_M_pMessage == nullptr);

        }

        // Acquire a reference on the async originator object
        void _Acquire_ref()
        {
            _InterlockedIncrement(&_M_refcount);
        }

        // Release the reference on the async originator object. The object
        // will be deleted when the reference count goes to 0.
        void _Release_ref()
        {
            _CONCRT_ASSERT(_M_refcount > 0);
            if (_InterlockedDecrement(&_M_refcount) == 0)
            {
                delete this;
            }
        }

        // The message that will be propagated by the AsyncOriginator
        message<_Type> * _M_pMessage;

        // Reference count to manage object lifetime
        volatile long _M_refcount;

        // The internal lock for this block for its message
        ::Concurrency::details::_ReentrantPPLLock _M_internalLock;

        // connected targets
        _Target_registry _M_connectedTargets;
    };

    // static class that exposes methods to initiate messages into
    // a dataflow network
    class _Originator
    {
    public:

        // Synchronous initiation of messages
        template <class _Type>
        static bool _send(ITarget<_Type> * _Trg, const _Type& _Data)
        {
            if (_Trg != nullptr && _Trg->supports_anonymous_source())
            {
                // _send will block until the message is accepted/rejected.
                // Note that this invokes the send method on the target which
                // would synchronously process the message.
                _AnonymousOriginator<_Type> _Send_block;
                return _Send_block._internal_send(_Trg, _Data);
            }
            else
            {
                // Create a blocking originator on the stack. _send will block until the
                // message is accepted/rejected.
                _SyncOriginator<_Type> _Orig;
                return _Orig._internal_send(_Trg, _Data);
            }
        }

        // Asynchronous initiation of messages
        template <class _Type>
        static bool _asend(ITarget<_Type> * _Trg, const _Type& _Data)
        {
            // If the block can participate in posting messages without requiring a call back, use that
            // method of initiating the message rather for efficiency purposes.
            if (_Trg != nullptr && _Trg->supports_anonymous_source())
            {
                _AnonymousOriginator<_Type> _Asend_block;
                return _Asend_block._internal_asend(_Trg, _Data);
            }
            else
            {
                // Needs to be allocated on the heap
                _AsyncOriginator<_Type> * _AsyncOrig = new _AsyncOriginator<_Type>;
                return _AsyncOrig->_internal_send(_Trg, _Data);
            }
        }
    };

} // namespace details

/// <summary>
///     A synchronous send operation, which waits until the target either accepts or declines the message.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Trg">
///     A pointer or reference to the target to which data is sent.
/// </param>
/// <param name="_Data">
///     A reference to the data to be sent.
/// </param>
/// <returns>
///     <c>true</c> if the message was accepted, <c>false</c> otherwise.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
bool send(_Inout_ ITarget<_Type> * _Trg, const _Type& _Data)
{
    return details::_Originator::_send(_Trg, _Data);
}


/// <summary>
///     A synchronous send operation, which waits until the target either accepts or declines the message.
/// </summary>
/// <typeparam name="_Type">
///     The payload type.
/// </typeparam>
/// <param name="_Trg">
///     A pointer or reference to the target to which data is sent.
/// </param>
/// <param name="_Data">
///     A reference to the data to be sent.
/// </param>
/// <returns>
///     <c>true</c> if the message was accepted, <c>false</c> otherwise.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="asend Function"/>
/**/
template <class _Type>
bool send(ITarget<_Type> &_Trg, const _Type &_Data)
{
    return ::Concurrency::send(&_Trg, _Data);
}

/// <summary>
///     An asynchronous send operation, which schedules a task to propagate the data to the target block.
/// </summary>
/// <typeparam name="_Type">
///     The type of the data to be sent.
/// </typeparam>
/// <param name="_Trg">
///     A pointer or reference to the target to which data is sent.
/// </param>
/// <param name="_Data">
///     A reference to the data to be sent.
/// </param>
/// <returns>
///     <c>true</c> if the message was accepted before the method returned, <c>false</c> otherwise.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="send Function"/>
/**/
template <class _Type>
bool asend(_Inout_ ITarget<_Type> * _Trg, const _Type& _Data)
{
    return details::_Originator::_asend(_Trg, _Data);
}


/// <summary>
///     An asynchronous send operation, which schedules a task to propagate the value to the target block.
/// </summary>
/// <typeparam name="_Type">
///     The type of the data to be sent.
/// </typeparam>
/// <param name="_Trg">
///     A pointer or reference to the target to which data is sent.
/// </param>
/// <param name="_Data">
///     A reference to the data to be sent.
/// </param>
/// <returns>
///     <c>true</c> if the message was accepted, <c>false</c> otherwise.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Message Passing Functions"/>.
/// </remarks>
/// <seealso cref="receive Function"/>
/// <seealso cref="try_receive Function"/>
/// <seealso cref="send Function"/>
/**/
template <class _Type>
bool asend(ITarget<_Type> &_Trg, const _Type &_Data)
{
    return ::Concurrency::asend(&_Trg, _Data);
}

//**************************************************************************
// Target Block:
//**************************************************************************

/// <summary>
///     The <c>target_block</c> class is an abstract base class that provides basic link management
///     functionality and error checking for target only blocks.
/// </summary>
/// <typeparam name="_SourceLinkRegistry">
///     The link registry to be used for holding the source links.
/// </typeparam>
/// <typeparam name="_MessageProcessorType">
///     The processor type for message processing.
/// </typeparam>
/// <seealso cref="ITarget Class"/>
/**/
template<class _SourceLinkRegistry,
    class _MessageProcessorType = ordered_message_processor<typename _SourceLinkRegistry::type::source_type>>
class target_block : public ITarget<typename _SourceLinkRegistry::type::source_type>
{
public:
    /// <summary>
    ///     The type of the payload for the incoming messages to this <c>target_block</c> object.
    /// </summary>
    /**/
    typedef typename _SourceLinkRegistry::type::source_type _Source_type;

    /// <summary>
    ///     The type of the <c>source_link_manager</c> this <c>target_block</c> object.
    /// </summary>
    /**/
    typedef source_link_manager<_SourceLinkRegistry> _SourceLinkManager;

    /// <summary>
    ///     The type of the iterator for the <c>source_link_manager</c> for this <c>target_block</c> object.
    /// </summary>
    /**/
    typedef typename _SourceLinkManager::iterator source_iterator;

    using typename ITarget<_Source_type>::filter_method;

    /// <summary>
    ///     Constructs a <c>target_block</c> object.
    /// </summary>
    /**/
    target_block() : _M_pFilter(nullptr), _M_fDeclineMessages(false)
    {
        _Trace_agents(AGENTS_EVENT_CREATE,
            ::Concurrency::details::_Trace_agents_get_id(this),
            ::Concurrency::details::_Trace_agents_get_id(&_M_messageProcessor));
    }

    /// <summary>
    ///     Destroys the <c>target_block</c> object.
    /// </summary>
    /**/
    virtual ~target_block()
    {
        // All sources should have been unlinked
        _CONCRT_ASSERT(_M_connectedSources.count() == 0);
        delete _M_pFilter;

        _Trace_agents(AGENTS_EVENT_DESTROY, ::Concurrency::details::_Trace_agents_get_id(this));
    }

    /// <summary>
    ///     Asynchronously passes a message from a source block to this target block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     <para> The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if either the <paramref name="_PMessage"/> or <paramref name="_PSource"/> parameter is <c>NULL</c>.</para>
    /// </remarks>
    /**/
    virtual message_status propagate(_Inout_opt_ message<_Source_type> * _PMessage, _Inout_opt_ ISource<_Source_type> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by <c>consume</c> and the LWT.  Doing so could
        // result in a deadlock.

        if (_PMessage == nullptr)
        {
            throw ::std::invalid_argument("_PMessage");
        }

        if (_PSource == nullptr)
        {
            throw ::std::invalid_argument("_PSource");
        }

        if (_M_fDeclineMessages)
        {
            return declined;
        }

        if (_M_pFilter != nullptr && !(*_M_pFilter)(_PMessage->payload))
        {
            return declined;
        }

        return propagate_message(_PMessage, _PSource);
    }

    /// <summary>
    ///     Synchronously passes a message from a source block to this target block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if either the <paramref name="_PMessage"/> or <paramref name="_PSource"/> parameter is <c>NULL</c>.
    ///     <para>Using the <c>send</c> method outside of message initiation and to propagate messages
    ///     within a network is dangerous and can lead to deadlock.</para>
    ///     <para>When <c>send</c> returns, the message has either already been accepted, and transferred into
    ///     the target block, or it has been declined by the target.</para>
    /// </remarks>
    /**/
    virtual message_status send(_Inout_ message<_Source_type> * _PMessage, _Inout_ ISource<_Source_type> * _PSource)
    {
        if (_PMessage == nullptr)
        {
            throw ::std::invalid_argument("_PMessage");
        }

        if (_PSource == nullptr)
        {
            throw ::std::invalid_argument("_PSource");
        }

        if (_M_fDeclineMessages)
        {
            return declined;
        }

        if (_M_pFilter != nullptr && !(*_M_pFilter)(_PMessage->payload))
        {
            return declined;
        }

        return send_message(_PMessage, _PSource);
    }

protected:

    /// <summary>
    ///     When overridden in a derived class, this method asynchronously passes a message from an <c>ISource</c>
    ///     block to this <c>target_block</c> object. It is invoked by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(_Inout_ message<_Source_type> * _PMessage, _Inout_ ISource<_Source_type> * _PSource) = 0;

    /// <summary>
    ///     When overridden in a derived class, this method synchronously passes a message from an <c>ISource</c>
    ///     block to this <c>target_block</c> object. It is invoked by the <c>send</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     By default, this block returns <c>declined</c> unless overridden by a derived class.
    /// </remarks>
    /**/
    virtual message_status send_message(_Inout_ message<_Source_type> *, _Inout_ ISource<_Source_type> *)
    {
        // By default we do not allow send()
        return declined;
    }

    /// <summary>
    ///     Links a specified source block to this <c>target_block</c> object.
    /// </summary>
    /// <param name="_PSource">
    ///     A pointer to the <c>ISource</c> block that is to be linked.
    /// </param>
    /// <remarks>
    ///     This function should not be called directly on a <c>target_block</c> object. Blocks should be connected together
    ///     using the <c>link_target</c> method on <c>ISource</c> blocks, which will invoke the <c>link_source</c> method
    ///     on the corresponding target.
    /// </remarks>
    /**/
    virtual void link_source(_Inout_ ISource<_Source_type> * _PSource)
    {
        _M_connectedSources.add(_PSource);
        _Trace_agents(AGENTS_EVENT_LINK,
            ::Concurrency::details::_Trace_agents_get_id(_PSource),
            ::Concurrency::details::_Trace_agents_get_id(this));
    }

    /// <summary>
    ///     Unlinks a specified source block from this <c>target_block</c> object.
    /// </summary>
    /// <param name="_PSource">
    ///     A pointer to the <c>ISource</c> block that is to be unlinked.
    /// </param>
    ///     This function should not be called directly on n <c>target_block</c> object. Blocks should be disconnected
    ///     using the <c>unlink_target</c> or <c>unlink_targets</c> methods on <c>ISource</c> blocks, which will invoke
    ///     the <c>unlink_source</c> method on the corresponding target.
    /**/
    virtual void unlink_source(_Inout_ ISource<_Source_type> * _PSource)
    {
        _Trace_agents(AGENTS_EVENT_UNLINK,
            ::Concurrency::details::_Trace_agents_get_id(_PSource),
            ::Concurrency::details::_Trace_agents_get_id(this));

        _M_connectedSources.remove(_PSource);
    }

    /// <summary>
    ///     Unlinks all source blocks from this <c>target_block</c> object.
    /// </summary>
    /**/
    virtual void unlink_sources()
    {
        for (source_iterator _Iter = _M_connectedSources.begin(); *_Iter != nullptr; ++_Iter)
        {
             ISource<_Source_type> * _PSource = *_Iter;
             _PSource->unlink_target(this);
        }
    }

    /// <summary>
    ///     When overridden in a derived class, processes a message that was accepted by this <c>target_block</c> object.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the message that is to be handled.
    /// </param>
    /**/
    virtual void process_message(message<_Source_type> *)
    {
    }

    //
    // Utility routines
    //

    /// <summary>
    ///     Registers a filter method that will be invoked on
    ///     every message received.
    /// </summary>
    /// <param name="_Filter">
    ///     The filter method.
    /// </param>
    /**/
    void register_filter(filter_method const& _Filter)
    {
        if (_Filter != nullptr)
        {
            _M_pFilter = new filter_method(_Filter);
        }
    }

    /// <summary>
    ///     Indicates to the block that new messages should be declined.
    /// </summary>
    /// <remarks>
    ///     This method is called by the destructor to ensure that new messages are declined while destruction is in progress.
    /// </remarks>
    /**/
    void decline_incoming_messages()
    {
        _M_fDeclineMessages = true;
    }

    /// <summary>
    ///     Initializes the base object. Specifically, the <c>message_processor</c> object needs
    ///     to be initialized.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The scheduler to be used for scheduling tasks.
    /// </param>
    /// <param name="_PScheduleGroup">
    ///     The schedule group to be used for scheduling tasks.
    /// </param>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    void initialize_target(_Inout_opt_ Scheduler * _PScheduler = nullptr, _Inout_opt_ ScheduleGroup * _PScheduleGroup = nullptr)
    {
        // Register a callback with the processor
        _M_messageProcessor.initialize(_PScheduler, _PScheduleGroup,
            // Processing and Propagating function used by ordered_message_processors
            [this](message<_Source_type> * _PMessage)
            {
                // Handle message by calling process_message to maintain CRT100 compatibility
                this->process_message(_PMessage);
            });

        // Register this target block as the owner of the connected sources
        _M_connectedSources.register_target_block(this);
    }

    /// <summary>
    ///     Enables batched processing for this block.
    /// </summary>
    /**/
    void enable_batched_processing()
    {
        _M_messageProcessor.initialize_batched_processing(
            // Processing function used by CRT110
            [this](message<_Source_type> * _PMessage)
            {
                // Handle message through new process_input_message to use CRT110 batch processing
                this->process_input_messages(_PMessage);
            }, nullptr);
    }

    /// <summary>
    ///     Asynchronously sends a message for processing.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the message being sent.
    /// </param>
    /**/
    void async_send(_Inout_opt_ message<_Source_type> * _PMessage)
    {
        _M_messageProcessor.async_send(_PMessage);
    }

    /// <summary>
    ///     Synchronously send a message for processing.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the message being sent.
    /// </param>
    /**/
    void sync_send(_Inout_opt_ message<_Source_type> * _PMessage)
    {
        _M_messageProcessor.sync_send(_PMessage);
    }

    /// <summary>
    ///     Waits for all asynchronous propagations to complete.
    /// </summary>
    /// <remarks>
    ///     This method is used by message block destructors to ensure all asynchronous operations
    ///     have had time to finish before destroying the block.
    /// </remarks>
    /**/
    void wait_for_async_sends()
    {
        // Decline new messages to ensure that messages are not dropped during the wait
        decline_incoming_messages();

        _M_messageProcessor.wait();
    }

    /// <summary>
    ///     Unlinks all sources after waiting for outstanding asynchronous send operations to complete.
    /// </summary>
    /// <remarks>
    ///     All target blocks should call this routine to remove the sources in their destructor.
    /// </remarks>
    /**/
    void remove_sources()
    {
        wait_for_async_sends();

        unlink_sources();
    }

    /// <summary>
    ///     Processes messages that are received as inputs.
    /// </summary>
    /**/
    virtual void process_input_messages(_Inout_ message<_Source_type> *)
    {
        throw invalid_operation("To use batched processing, you must override process_input_messages in the message block.");
    }

    /// <summary>
    ///     The container for all the sources connected to this block.
    /// </summary>
    /**/
    _SourceLinkManager _M_connectedSources;

    /// <summary>
    ///     The filter function which determines whether offered messages should be accepted.
    /// </summary>
    /**/
    filter_method * _M_pFilter;

    /// <summary>
    ///     A <c>bool</c> that is set to indicate that all messages should be declined
    ///     in preparation for deleting the block
    /// <summary>
    /**/
    bool _M_fDeclineMessages;

    /// <summary>
    ///     The <c>message_processor</c> for this <c>target_block</c>.
    /// <summary>
    /**/
    _MessageProcessorType _M_messageProcessor;
};

//**************************************************************************
// Source Block:
//**************************************************************************

/// <summary>
///     The <c>source_block</c> class is an abstract base class for source-only blocks. The class
///     provides basic link management functionality as well as common error checks.
/// </summary>
/// <typeparam name="_TargetLinkRegistry">
///     Link registry to be used for holding the target links.
/// </typeparam>
/// <typeparam name="_MessageProcessorType">
///     Processor type for message processing.
/// </typeparam>
/// <remarks>
///     Message blocks should derive from this block to take advantage of link management and
///     synchronization provided by this class.
/// </remarks>
/// <seealso cref="ISource Class"/>
/**/
template<class _TargetLinkRegistry,
    class _MessageProcessorType = ordered_message_processor<typename _TargetLinkRegistry::type::type>>
class source_block : public ISource<typename _TargetLinkRegistry::type::type>
{
public:

    /// <summary>
    ///     The payload type of messages handled by this <c>source_block</c>.
    /// </summary>
    /**/
    typedef typename _TargetLinkRegistry::type::type _Target_type;

    /// <summary>
    ///     The iterator to walk the connected targets.
    /// </summary>
    /**/
    typedef typename _TargetLinkRegistry::iterator target_iterator;

    /// <summary>
    ///     Constructs a <c>source_block</c> object.
    /// </summary>
    /**/
    source_block() :
      _M_pReservedFor(nullptr),
      _M_reservedId(-1),
      _M_referenceCount(0)
    {
        _Trace_agents(AGENTS_EVENT_CREATE,
            ::Concurrency::details::_Trace_agents_get_id(this),
            ::Concurrency::details::_Trace_agents_get_id(&_M_messageProcessor));
    }

    /// <summary>
    ///     Destroys the <c>source_block</c> object.
    /// </summary>
    /**/
    virtual ~source_block()
    {
        // All targets should have been unlinked
        _CONCRT_ASSERT(_M_connectedTargets.count() == 0);

        _Trace_agents(AGENTS_EVENT_DESTROY, ::Concurrency::details::_Trace_agents_get_id(this));
    }

    /// <summary>
    ///     Links a target block to this <c>source_block</c> object.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to an <c>ITarget</c> block to link to this <c>source_block</c> object.
    /// </param>
    /// <remarks>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception if the
    ///     parameter <paramref name="_PTarget"/> is <c>NULL</c>.
    /// </remarks>
    /**/
    virtual void link_target(_Inout_ ITarget<_Target_type> * _PTarget)
    {
        _R_lock _Lock(_M_internalLock);

        if (_PTarget == nullptr)
        {
            throw ::std::invalid_argument("_PTarget");
        }

        _M_connectedTargets.add(_PTarget);
        this->_Invoke_link_source(_PTarget);
        link_target_notification(_PTarget);
    }

    /// <summary>
    ///     Unlinks a target block from this <c>source_block</c> object.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to an <c>ITarget</c> block to unlink from this <c>source_block</c> object.
    /// </param>
    /// <remarks>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception if the
    ///     parameter <paramref name="_PTarget"/> is <c>NULL</c>.
    /// </remarks>
    /**/
    virtual void unlink_target(_Inout_ ITarget<_Target_type> * _PTarget)
    {
        _R_lock _Lock(_M_internalLock);

        if (_PTarget == nullptr)
        {
            throw ::std::invalid_argument("_PTarget");
        }

        if (_M_connectedTargets.remove(_PTarget))
        {
            // We were able to remove the target from our list.
            // Inform the target to unlink from us
            this->_Invoke_unlink_source(_PTarget);
        }
    }

    /// <summary>
    ///     Unlinks all target blocks from this <c>source_block</c> object.
    /// </summary>
    /**/
    virtual void unlink_targets()
    {
        _R_lock _Lock(_M_internalLock);

        for (target_iterator _Iter = _M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
        {
            ITarget<_Target_type> * _PTarget = *_Iter;
            _CONCRT_ASSERT(_PTarget != nullptr);

            unlink_target(_PTarget);
        }

        // All the targets should be unlinked.
        _CONCRT_ASSERT(_M_connectedTargets.count() == 0);
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>source_block</c> object, transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>accept</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception if the
    ///     parameter <paramref name="_PTarget"/> is <c>NULL</c>.
    ///     <para>
    ///     The <c>accept</c> method is called by a target while a message is being offered by this <c>ISource</c> block.
    ///     The message pointer returned may be different from the one passed into the <c>propagate</c> method
    ///     of the <c>ITarget</c> block, if this source decides to make a copy of the message.
    ///     </para>
    /// </remarks>
    /**/
    virtual message<_Target_type> * accept(runtime_object_identity _MsgId, _Inout_ ITarget<_Target_type> * _PTarget)
    {
        if (_PTarget == nullptr)
        {
            throw ::std::invalid_argument("_PTarget");
        }

        // Assert if the target is not connected
        _CONCRT_ASSERT(_M_connectedTargets.contains(_PTarget));

        return accept_message(_MsgId);
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>source_block</c> object.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>reserve</c> method.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise. Reservations can fail
    ///     for many reasons, including: the message was already reserved or accepted by another target, the source could
    ///     deny reservations, and so forth.
    /// </returns>
    /// <remarks>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception if the
    ///     parameter <paramref name="_PTarget"/> is <c>NULL</c>.
    ///     <para>
    ///     After you call <c>reserve</c>, if it succeeds, you must call either <c>consume</c> or <c>release</c>
    ///     in order to take or give up possession of the message, respectively.
    ///     </para>
    /// </remarks>
    /**/
    virtual bool reserve(runtime_object_identity _MsgId, _Inout_ ITarget<_Target_type> * _PTarget)
    {
        _R_lock _Lock(_M_internalLock);

        if (_PTarget == nullptr)
        {
            throw ::std::invalid_argument("_PTarget");
        }

        if ( _M_pReservedFor != nullptr)
        {
            // Someone else is holding the reservation
            return false;
        }

        if (!reserve_message(_MsgId))
        {
            // Failed to reserve the msg ID
            return false;
        }

        // Save the reserving target and the msg ID
        _M_pReservedFor = _PTarget;
        _M_reservedId = _MsgId;

        return true;
    }

    /// <summary>
    ///     Consumes a message previously offered by this <c>source_block</c> object and successfully reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the reserved <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>consume</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     <para>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception if the
    ///     parameter <paramref name="_PTarget"/> is <c>NULL</c>.
    ///     </para>
    ///     <para>
    ///     The method throws a <see cref="bad_target Class">bad_target</see> exception if the parameter <paramref name="_PTarget"/>
    ///     does not represent the target that called <c>reserve</c>.
    ///     </para>
    ///     <para>
    ///     The <c>consume</c> method is similar to <c>accept</c>, but must always be preceded by a call to <c>reserve</c> that
    ///     returned <c>true</c>.
    ///     </para>
    /// </remarks>
    /**/
    virtual message<_Target_type> * consume(runtime_object_identity _MsgId, _Inout_ ITarget<_Target_type> * _PTarget)
    {
        _R_lock _Lock(_M_internalLock);

        if (_PTarget == nullptr)
        {
            throw ::std::invalid_argument("_PTarget");
        }

        if (_M_pReservedFor == nullptr || _PTarget != _M_pReservedFor)
        {
            throw bad_target();
        }

        message<_Target_type> * _Msg = consume_message(_MsgId);

        if (_Msg != nullptr)
        {
            // Clear the reservation
            // _M_pReservedId is intentionally not reset so that it can assist in debugging
            _M_pReservedFor = nullptr;

            // Reservation is assumed to block propagation. Notify that propagation can now be resumed
            resume_propagation();
        }

        return _Msg;
    }

    /// <summary>
    ///     Releases a previous successful message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the reserved <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>release</c> method.
    /// </param>
    /// <remarks>
    ///     <para>
    ///     The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception if the
    ///     parameter <paramref name="_PTarget"/> is <c>NULL</c>.
    ///     </para>
    ///     <para>
    ///     The method throws a <see cref="bad_target Class">bad_target</see> exception if the parameter <paramref name="_PTarget"/>
    ///     does not represent the target that called <c>reserve</c>.
    ///     </para>
    /// </remarks>
    /**/
    virtual void release(runtime_object_identity _MsgId, _Inout_ ITarget<_Target_type> * _PTarget)
    {
        _R_lock _Lock(_M_internalLock);

        if (_PTarget == nullptr)
        {
            throw ::std::invalid_argument("_PTarget");
        }

        if (_PTarget != _M_pReservedFor)
        {
            throw bad_target();
        }

        release_message(_MsgId);

        // Clear the reservation
        // _M_pReservedId is intentionally not reset so that it can assist in debugging
        _M_pReservedFor = nullptr;

        // Reservation is assumed to block propagation. Notify that propagation can now be resumed
        resume_propagation();
    }

    /// <summary>
    ///     Acquires a reference count on this <c>source_block</c> object, to prevent deletion.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being linked to this source
    ///     during the <c>link_target</c> method.
    /// </remarks>
    /**/
    virtual void acquire_ref(_Inout_ ITarget<_Target_type> *)
    {
        _InterlockedIncrement(&_M_referenceCount);
    }

    /// <summary>
    ///     Releases a reference count on this <c>source_block</c> object.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being unlinked from this source.
    ///     The source block is allowed to release any resources reserved for the target block.
    /// </remarks>
    /**/
    virtual void release_ref(_Inout_ ITarget<_Target_type> * _PTarget)
    {
        if (_PTarget != nullptr)
        {
            _R_lock _Lock(_M_internalLock);

            // We assume that each target would keep a single reference on its source, so
            // we call unlink target notification on every release. Otherwise, we would be
            // required to keep a reference count per target.
            // Note: unlink_target_notification can check the value of this _PTarget pointer, but
            // must not dereference it, as it may have already been deleted.
            unlink_target_notification(_PTarget);
        }

        _InterlockedDecrement(&_M_referenceCount);

        // It is *unsafe* to touch the "this" pointer after decrementing the reference count
    }

protected:

    //
    // Protected methods that a derived class can override to customize
    // the functionality
    //

    /// <summary>
    ///     A callback that notifies that a new target has been linked to this <c>source_block</c> object.
    /// </summary>
    /// <param name="_PTarget">
    ///     The <c>ITarget</c> block that was linked.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<_Target_type> *)
    {
        // By default, we restart propagation if there is no pending reservation
        if (_M_pReservedFor == nullptr)
        {
            propagate_to_any_targets(nullptr);
        }
    }

    /// <summary>
    ///     A callback that notifies that a target has been unlinked from this <c>source_block</c> object.
    /// </summary>
    /// <param name="_PTarget">
    ///     The <c>ITarget</c> block that was unlinked.
    /// </param>
    /**/
    virtual void unlink_target_notification(_Inout_ ITarget<_Target_type> * _PTarget)
    {
        // At this point, the target has already been disconnected from the
        // source.  It is safe to check the value of this pointer, but not
        // safe to dereference it, as it may have already been deleted.

        // If the target being unlinked is the one holding the reservation,
        // release the reservation
        if (_M_pReservedFor == _PTarget)
        {
            release(_M_reservedId, _PTarget);
        }
    }

    /// <summary>
    ///     When overridden in a derived class, accepts an offered message by the source.
    ///     Message blocks should override this method to validate the <paramref name="_MsgId"/> and
    ///     return a message.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the <c>message</c> object.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     To transfer ownership, the original message pointer should be returned. To maintain
    ///     ownership, a copy of message payload needs to be made and returned.
    /// </remarks>
    /**/
    virtual message<_Target_type> * accept_message(runtime_object_identity _MsgId) = 0;

    /// <summary>
    ///     When overridden in a derived class, reserves a message previously offered by this
    ///     <c>source_block</c> object.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId) = 0;

    /// <summary>
    ///     When overridden in a derived class, consumes a message that was previously reserved.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being consumed.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_Target_type> * consume_message(runtime_object_identity _MsgId) = 0;

    /// <summary>
    ///     When overridden in a derived class, releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId) = 0;

    /// <summary>
    ///     When overridden in a derived class, resumes propagation after a reservation has been released.
    /// </summary>
    /**/
    virtual void resume_propagation() = 0;

    /// <summary>
    ///     Process input messages.  This is only useful for propagator blocks, which derive from source_block
    /// </summary>
    /**/
    virtual void process_input_messages(_Inout_ message<_Target_type> *)
    {
        // source_blocks do not need to process anything
    }

    /// <summary>
    ///     Propagate messages to targets.
    /// </summary>
    /**/
    virtual void propagate_output_messages()
    {
        throw invalid_operation("To use batched processing, you must override propagate_output_messages in the message block.");
    }

    /// <summary>
    ///     When overridden in a derived class, propagates the given message to any or all of the linked targets.
    ///     This is the main propagation routine for message blocks.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the message that is to be propagated.
    /// </param>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<_Target_type> * _PMessage)
    {
        (void) _PMessage;
        throw invalid_operation("To use ordered message processing, you must override propagate_to_any_targets in the message block.");
    }

    //
    // Utility routines
    //
    /// <summary>
    ///     Initializes the <c>message_propagator</c> within this <c>source_block</c>.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The scheduler to be used for scheduling tasks.
    /// </param>
    /// <param name="_PScheduleGroup">
    ///     The schedule group to be used for scheduling tasks.
    /// </param>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    void initialize_source(_Inout_opt_ Scheduler * _PScheduler = nullptr, _Inout_opt_ ScheduleGroup * _PScheduleGroup = nullptr)
    {
        // Register a callback
        _M_messageProcessor.initialize(_PScheduler, _PScheduleGroup,
            [this](message<_Target_type> * _PMessage)
            {
                this->_Handle_message(_PMessage);
            });
    }

    /// <summary>
    ///     Enables batched processing for this block.
    /// </summary>
    /**/
    void enable_batched_processing()
    {
        // Register callbacks for CRT110 batched processing
        _M_messageProcessor.initialize_batched_processing(
            // Processing function used by CRT110
            [this](message<_Target_type> * _PMessage)
            {
                // Handle message through new process_input_message to use CRT110 batch processing
                this->process_input_messages(_PMessage);
            },
            [this]()
            {
                this->_Propagate_message();
            });
    }

    /// <summary>
    ///     Synchronously queues up messages and starts a propagation task, if this has not been done
    ///     already.
    /// </summary>
    /// <param name="_Msg">
    ///     A pointer to a <c>message</c> object to synchronously send.
    /// </param>
    /**/
    virtual void sync_send(_Inout_opt_ message<_Target_type> * _Msg)
    {
        // Caller shall not be holding any locks when calling this routine
        _M_messageProcessor.sync_send(_Msg);
    }

    /// <summary>
    ///     Asynchronously queues up messages and starts a propagation task, if this has not been done
    ///     already
    /// </summary>
    /// <param name="_Msg">
    ///     A pointer to a <c>message</c> object to asynchronously send.
    /// </param>
    /**/
    virtual void async_send(_Inout_opt_ message<_Target_type> * _Msg)
    {
        _M_messageProcessor.async_send(_Msg);
    }

    /// <summary>
    ///     Waits for all asynchronous propagations to complete. This propagator-specific spin wait is used
    ///     in destructors of message blocks to make sure that all asynchronous propagations have time to finish
    ///     before destroying the block.
    /// </summary>
    /**/
    void wait_for_outstanding_async_sends()
    {
        _M_messageProcessor.wait();
    }

    /// <summary>
    ///     Removes all target links for this source block. This should be called from the destructor.
    /// </summary>
    /**/
    void remove_targets()
    {
        // Wait for outstanding propagation to complete.
        wait_for_outstanding_async_sends();

        unlink_targets();

        _Wait_on_ref();
    }

    //
    // Protected members
    //

    /// <summary>
    ///     Connected target that is holding a reservation
    /// </summary>
    /**/
    ITarget<_Target_type> * _M_pReservedFor;

    /// <summary>
    ///     Reserved message ID
    /// </summary>
    /**/
    runtime_object_identity _M_reservedId;

    /// <summary>
    ///     Connected targets
    /// </summary>
    /**/
    _TargetLinkRegistry _M_connectedTargets;

    /// <summary>
    ///     Processor used for asynchronous message handling
    /// </summary>
    /**/
    _MessageProcessorType _M_messageProcessor;

private:

    /// Private methods


    // Message handler callback for the propagator. Invokes propagate_to_any_targets
    // which derived classes should implement.
    /**/
    void _Handle_message(message<_Target_type> * _PMessage)
    {
        // Hold a lock to synchronize with unlink targets
        _R_lock _Lock(_M_internalLock);
        propagate_to_any_targets(_PMessage);
    }

    // Message handler callback for the processor. Invokes process_input_messages
    // which derived classes should implement.
    /**/
    void _Process_message(message<_Target_type> * _PMessage)
    {
        // Don't need a lock to process the message
        process_input_messages(_PMessage);
    }

    // Message handler callback for the propagator. Invokes propagate_output_messages
    // which derived classes should implement.
    /**/
    void _Propagate_message()
    {
        // Hold a lock to synchronize with unlink targets
        _R_lock _Lock(_M_internalLock);
        propagate_output_messages();
    }

    // Wait for the reference on this block to drop to zero
    /**/
    void _Wait_on_ref(long _RefCount = 0)
    {
        ::Concurrency::details::_SpinWaitBackoffNone spinWait;
        while(_M_referenceCount != _RefCount)
        {
            spinWait._SpinOnce();
        }
    }

    // Private Data members

    /// <summary>
    ///     Internal lock used for the following synchronization:
    ///     1. Synchronize between link and unlink target
    ///     2. Synchronize between propagate_to_any_targets and unlink_target
    ///     3. Synchronize between reserve and consume/release
    /// </summary>
    /**/
    ::Concurrency::details::_ReentrantPPLLock _M_internalLock;

    volatile long _M_referenceCount;

};

//**************************************************************************
// Propagator (source and target) Block:
//**************************************************************************
/// <summary>
///     The <c>propagator_block</c> class is an abstract base class for message blocks that are both a source and target.
///     It combines the functionality of both the <c>source_block</c> and <c>target_block</c> classes.
/// </summary>
/// <typeparam name="_TargetLinkRegistry">
///     The link registry to be used for holding the target links.
/// </typeparam>
/// <typeparam name="_SourceLinkRegistry">
///     The link registry to be used for holding the source links.
/// </typeparam>
/// <typeparam name="_MessageProcessorType">
///     The processor type for message processing.
/// </typeparam>
/// <remarks>
///     To avoid multiple inheritance, the <c>propagator_block</c> class inherits from the <c>source_block</c> class and <c>ITarget</c>
///     abstract class.  Most of the functionality in the <c>target_block</c> class is replicated here.
/// </remarks>
/// <seealso cref="source_block Class"/>
/// <seealso cref="ITarget Class"/>
/**/
template<class _TargetLinkRegistry, class _SourceLinkRegistry,
    class _MessageProcessorType = ordered_message_processor<typename _TargetLinkRegistry::type::type>>
class propagator_block : public source_block<_TargetLinkRegistry, _MessageProcessorType>, public ITarget<typename _SourceLinkRegistry::type::source_type>
{
public:
    /// <summary>
    ///     The type of the payload for the incoming message to this <c>propagator_block</c>.
    /// </summary>
    /**/
    typedef typename _SourceLinkRegistry::type::source_type _Source_type;

    /// <summary>
    ///     The type of the <c>source_link_manager</c> this <c>propagator_block</c>.
    /// </summary>
    /**/
    typedef source_link_manager<_SourceLinkRegistry> _SourceLinkManager;

    /// <summary>
    ///     The type of the iterator for the <c>source_link_manager</c> for this <c>propagator_block</c>.
    /// </summary>
    /**/
    typedef typename _SourceLinkManager::iterator source_iterator;

    using typename source_block<_TargetLinkRegistry, _MessageProcessorType>::_Target_type;
    using typename ITarget<_Source_type>::filter_method;

    /// <summary>
    ///     Constructs a <c>propagator_block</c> object.
    /// </summary>
    /**/
    propagator_block() : _M_pFilter(nullptr), _M_fDeclineMessages(false)
    {
    }

    /// <summary>
    ///     Destroys a <c>propagator_block</c> object.
    /// </summary>
    /**/
    virtual ~propagator_block()
    {
        remove_network_links();

        delete _M_pFilter;
    }

    /// <summary>
    ///     Asynchronously passes a message from a source block to this target block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     The <c>propagate</c> method is invoked on a target block by a linked source block. It queues up an
    ///     asynchronous task to handle the message, if one is not already queued or executing.
    ///     <para> The method throws an <see cref="invalid_argument Class">invalid_argument</see> exception
    ///     if either the <paramref name="_PMessage"/> or <paramref name="_PSource"/> parameter is <c>NULL</c>.</para>
    /// </remarks>
    /**/
    virtual message_status propagate(_Inout_opt_ message<_Source_type> * _PMessage, _Inout_opt_ ISource<_Source_type> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by <c>consume</c> and the LWT.  Doing so could
        // result in a deadlock.

        if (_PMessage == nullptr)
        {
            throw ::std::invalid_argument("_PMessage");
        }

        if (_PSource == nullptr)
        {
            throw ::std::invalid_argument("_PSource");
        }

        if (_M_fDeclineMessages)
        {
            return declined;
        }

        if (_M_pFilter != nullptr && !(*_M_pFilter)(_PMessage->payload))
        {
            return declined;
        }

        return propagate_message(_PMessage, _PSource);
    }

    /// <summary>
    ///     Synchronously initiates a message to this block.  Called by an <c>ISource</c> block.
    ///     When this function completes, the message will already have propagated into the block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     This method throws an <see cref="invalid_argument Class">invalid_argument</see> exception if either
    ///     the <paramref name="_PMessage"/> or <paramref name="_PSource"/> parameter is <c>NULL</c>.
    /// </remarks>
    /**/
    virtual message_status send(_Inout_ message<_Source_type> * _PMessage, _Inout_ ISource<_Source_type> * _PSource)
    {
        if (_PMessage == nullptr)
        {
            throw ::std::invalid_argument("_PMessage");
        }

        if (_PSource == nullptr)
        {
            throw ::std::invalid_argument("_PSource");
        }

        if (_M_fDeclineMessages)
        {
            return declined;
        }

        if (_M_pFilter != nullptr && !(*_M_pFilter)(_PMessage->payload))
        {
            return declined;
        }

        return send_message(_PMessage, _PSource);
    }

protected:

    /// <summary>
    ///     When overridden in a derived class, this method asynchronously passes a message from an <c>ISource</c>
    ///     block to this <c>propagator_block</c> object. It is invoked by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(_Inout_ message<_Source_type> * _PMessage, _Inout_ ISource<_Source_type> * _PSource) = 0;

    /// <summary>
    ///     When overridden in a derived class, this method synchronously passes a message from an <c>ISource</c>
    ///     block to this <c>propagator_block</c> object. It is invoked by the <c>send</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     By default, this block returns <c>declined</c> unless overridden by a derived class.
    /// </remarks>
    /**/
    virtual message_status send_message(_Inout_ message<_Source_type> *, _Inout_ ISource<_Source_type> *)
    {
        // By default we do not allow send()
        return declined;
    }

    /// <summary>
    ///     Links a specified source block to this <c>propagator_block</c> object.
    /// </summary>
    /// <param name="_PSource">
    ///     A pointer to the <c>ISource</c> block that is to be linked.
    /// </param>
    /**/
    virtual void link_source(_Inout_ ISource<_Source_type> * _PSource)
    {
        _M_connectedSources.add(_PSource);
        _Trace_agents(AGENTS_EVENT_LINK,
            ::Concurrency::details::_Trace_agents_get_id(_PSource),
            ::Concurrency::details::_Trace_agents_get_id(this));
    }

    /// <summary>
    ///     Unlinks a specified source block from this <c>propagator_block</c> object.
    /// </summary>
    /// <param name="_PSource">
    ///     A pointer to the <c>ISource</c> block that is to be unlinked.
    /// </param>
    /**/
    virtual void unlink_source(_Inout_ ISource<_Source_type> * _PSource)
    {
        _Trace_agents(AGENTS_EVENT_UNLINK,
            ::Concurrency::details::_Trace_agents_get_id(_PSource),
            ::Concurrency::details::_Trace_agents_get_id(this));

        _M_connectedSources.remove(_PSource);
    }

    /// <summary>
    ///     Unlinks all source blocks from this <c>propagator_block</c> object.
    /// </summary>
    /**/
    virtual void unlink_sources()
    {
        for (source_iterator _Iter = _M_connectedSources.begin(); *_Iter != nullptr; ++_Iter)
        {
             ISource<_Source_type> * _PSource = *_Iter;
             _PSource->unlink_target(this);
        }
    }

    //
    // Utility routines
    //

    /// <summary>
    ///     Process input messages.  This is only useful for propagator blocks, which derive from source_block
    /// </summary>
    /**/
    virtual void process_input_messages(_Inout_ message<_Target_type> *)
    {
        throw invalid_operation("To use batched processing, you must override process_input_messages in the message block.");
    }

    /// <summary>
    ///     Initializes the base object. Specifically, the <c>message_processor</c> object needs
    ///     to be initialized.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The scheduler to be used for scheduling tasks.
    /// </param>
    /// <param name="_PScheduleGroup">
    ///     The schedule group to be used for scheduling tasks.
    /// </param>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    void initialize_source_and_target(_Inout_opt_ Scheduler * _PScheduler = nullptr, _Inout_opt_ ScheduleGroup * _PScheduleGroup = nullptr)
    {
        this->initialize_source(_PScheduler, _PScheduleGroup);

        // Register this propagator block as the owner of the connected sources
        _M_connectedSources.register_target_block(this);
    }

    /// <summary>
    ///     Registers a filter method that will be invoked on every received message.
    /// </summary>
    /// <param name="_Filter">
    ///     The filter method.
    /// </param>
    /**/
    void register_filter(filter_method const& _Filter)
    {
        if (_Filter != nullptr)
        {
            _M_pFilter = new filter_method(_Filter);
        }
    }

    /// <summary>
    ///     Indicates to the block that new messages should be declined.
    /// </summary>
    /// <remarks>
    ///     This method is called by the destructor to ensure that new messages are declined while destruction is in progress.
    /// </remarks>
    /**/
    void decline_incoming_messages()
    {
        _M_fDeclineMessages = true;
    }

    /// <summary>
    ///     Removes all the source and target network links from this <c>propagator_block</c> object.
    /// </summary>
    /**/
    void remove_network_links()
    {
        // Decline messages while the links are being removed
        decline_incoming_messages();

        // Remove all the target links. This waits for
        // all outstanding async propagation operations.
        this->remove_targets();

        // unlink all sources. The above steps guarantee that
        // they can be removed safely.
        unlink_sources();
    }

    /// <summary>
    ///     The container for all the sources connected to this block.
    /// </summary>
    /**/
    _SourceLinkManager _M_connectedSources;

    /// <summary>
    ///     The filter function which determines whether offered messages should be accepted.
    /// </summary>
    /**/
    filter_method * _M_pFilter;

    /// <summary>
    ///     A <c>bool</c> that is set to indicate that all messages should be declined
    ///     in preparation for deleting the block
    /// <summary>
    /**/
    volatile bool _M_fDeclineMessages;
};

//**************************************************************************
// Unbounded Buffers:
//**************************************************************************

/// <summary>
///     An <c>unbounded_buffer</c> messaging block is a multi-target, multi-source, ordered
///     <c>propagator_block</c> capable of storing an unbounded number of messages.
/// </summary>
/// <typeparam name="_Type">
///     The payload type of the messages stored and propagated by the buffer.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/// <seealso cref="overwrite_buffer Class"/>
/// <seealso cref="single_assignment Class"/>
/**/
template<class _Type>
class unbounded_buffer : public propagator_block<multi_link_registry<ITarget<_Type>>, multi_link_registry<ISource<_Type>>>
{
private:
    typedef multi_link_registry<ITarget<_Type>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;

    /// <summary>
    ///     Constructs an <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>unbounded_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    unbounded_buffer() :
      _M_fForceRepropagation(false)
    {
        this->initialize_source_and_target();
        this->enable_batched_processing();
    }

    /// <summary>
    ///     Constructs an <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>unbounded_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    unbounded_buffer(filter_method const& _Filter) :
      _M_fForceRepropagation(false)
    {
        this->initialize_source_and_target();
        this->enable_batched_processing();
        this->register_filter(_Filter);
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs an <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>unbounded_buffer</c> object is scheduled.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>unbounded_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    unbounded_buffer(Scheduler& _PScheduler) :
      _M_fForceRepropagation(false)
    {
        this->initialize_source_and_target(&_PScheduler);
        this->enable_batched_processing();
    }

    /// <summary>
    ///     Constructs an <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>unbounded_buffer</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>unbounded_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    unbounded_buffer(Scheduler& _PScheduler, filter_method const& _Filter) :
      _M_fForceRepropagation(false)
    {
        this->initialize_source_and_target(&_PScheduler);
        this->enable_batched_processing();
        this->register_filter(_Filter);
    }

    /// <summary>
    ///     Constructs an <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>unbounded_buffer</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>unbounded_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    unbounded_buffer(ScheduleGroup& _PScheduleGroup) :
      _M_fForceRepropagation(false)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
        this->enable_batched_processing();
    }

    /// <summary>
    ///     Constructs an <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>unbounded_buffer</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>unbounded_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    unbounded_buffer(ScheduleGroup& _PScheduleGroup, filter_method const& _Filter) :
      _M_fForceRepropagation(false)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
        this->enable_batched_processing();
        this->register_filter(_Filter);
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Destroys the <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /**/
    ~unbounded_buffer()
    {
        // Remove all links
        this->remove_network_links();

        // Clean up any messages left in this message block
        _Delete_stored_messages();
    }

    /// <summary>
    ///     Adds an item to the <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_Item">
    ///     The item to add.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the item was accepted, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool enqueue(_Type const& _Item)
    {
        return ::Concurrency::send<_Type>(this, _Item);
    }

    /// <summary>
    ///     Removes an item from the <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <returns>
    ///     The payload of the message removed from the <c>unbounded_buffer</c>.
    /// </returns>
    /**/
    _Type dequeue()
    {
        return ::Concurrency::receive<_Type>(this);
    }


protected:

    //
    // propagator_block protected function implementations
    //

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>unbounded_buffer</c> messaging block.
    ///     It is invoked by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by <c>consume</c> and the LWT.  Doing so could
        // result in a deadlock.

        message_status _Result = accepted;

        // Accept the message being propagated
        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        if (_PMessage != nullptr)
        {
            this->async_send(_PMessage);
        }
        else
        {
            _Result = missed;
        }

        return _Result;
    }

    /// <summary>
    ///     Synchronously passes a message from an <c>ISource</c> block to this <c>unbounded_buffer</c> messaging block.
    ///     It is invoked by the <c>send</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status send_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        if (_PMessage != nullptr)
        {
            this->sync_send(_PMessage);
        }
        else
        {
            return missed;
        }

        return accepted;
    }

    /// <summary>
    ///     Overrides the <c>supports_anonymous_source</c> method to indicate that this block can
    ///     accept messages offered to it by a source that is not linked.
    /// </summary>
    /// <returns>
    ///     <c>true</c> because the block does not postpone offered messages.
    /// </returns>
    /**/
    virtual bool supports_anonymous_source()
    {
        return true;
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>unbounded_buffer</c> messaging block,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<_Type> * accept_message(runtime_object_identity _MsgId)
    {
        //
        // Peek at the head message in the message buffer.  If the IDs match
        // dequeue and transfer ownership
        //
        message<_Type> * _Msg = nullptr;

        if (_M_messageBuffer._Is_head(_MsgId))
        {
            _Msg = _M_messageBuffer._Dequeue();
        }

        return _Msg;
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId)
    {
        // Allow reservation if this is the head message
        return _M_messageBuffer._Is_head(_MsgId);
    }

    /// <summary>
    ///     Consumes a message previously offered by the <c>unbounded_buffer</c> messaging block and reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being consumed.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_Type> * consume_message(runtime_object_identity _MsgId)
    {
        // By default, accept the message
        return accept_message(_MsgId);
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId)
    {
        // The head message is the one reserved.
        if (!_M_messageBuffer._Is_head(_MsgId))
        {
            throw message_not_found();
        }
    }

    /// <summary>
    ///     Resumes propagation after a reservation has been released.
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // If there are any messages in the buffer, propagate them out
        if (_M_messageBuffer._Count() > 0)
        {
            // Set the flag to force a repropagation. This flag is cleared when a propagation happens
            // The only functions that call this are release, consume, and link_target, all of which
            // hold the internal lock, so the flag is guaranteed to be read by propagation, which also
            // holds the same lock.
            _M_fForceRepropagation = true;

            // async send a NULL value to initiate the repropagation
            this->async_send(nullptr);
        }
    }

    /// <summary>
    ///     A callback that notifies that a new target has been linked to this <c>unbounded_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<_Type> * _PTarget)
    {
        // If the message queue is blocked due to reservation
        // there is no need to do any message propagation
        if (this->_M_pReservedFor != nullptr)
        {
            return;
        }

        message<_Type> * _Msg = _M_messageBuffer._Peek();

        if (_Msg != nullptr)
        {
            // Propagate the head message to the new target
            message_status _Status = _PTarget->propagate(_Msg, this);

            if (_Status == accepted)
            {
                // The target accepted the message, restart propagation.
                _Propagate_priority_order(_M_messageBuffer);
            }

            // If the status is anything other than accepted, then leave
            // the message queue blocked.
        }
    }

    /// <summary>
    ///     Places the <c>message</c> <paramref name="_PMessage"/> in this <c>unbounded_buffer</c> messaging block and
    ///     tries to offer it to all of the linked targets.
    /// </summary>
    virtual void process_input_messages(_Inout_ message<_Type> * _PMessage)
    {
        if (_PMessage != nullptr)
        {
            _M_processedMessages._Enqueue(_PMessage);
        }
    }

    /// <summary>
    ///     Places the <c>message</c> <paramref name="_PMessage"/> in this <c>unbounded_buffer</c> messaging block and
    ///     tries to offer it to all of the linked targets.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to a <c>message</c> object that this <c>unbounded_buffer</c> has taken ownership of.
    /// </param>
    /// <remarks>
    ///     If another message is already ahead of this one in the <c>unbounded_buffer</c>,
    ///     propagation to linked targets will not occur until any earlier messages have been accepted
    ///     or consumed. The first linked target to successfully <c>accept</c> or <c>consume</c> the
    ///     message takes ownership, and no other target can then get the message.
    /// </remarks>
    /**/
    virtual void propagate_output_messages()
    {
        // Move the messages from the processedMessages queue to the internal storage
        // to make them ready for propagating out

        // If there are messages in the message queue, the queue is blocked and a
        // propagation should not happen unless it has been forced using resume_propagation
        bool _FIsBlocked = (_M_messageBuffer._Count() > 0);

        for(;;)
        {
            message<_Type> * _PInputMessage = _M_processedMessages._Dequeue();
            if(_PInputMessage == nullptr)
            {
                break;
            }
            _M_messageBuffer._Enqueue(_PInputMessage);
        }

        if (_M_fForceRepropagation == false && _FIsBlocked == true)
        {
            return;
        }

        // Reset the repropagation flag because a propagation has started.
        _M_fForceRepropagation = false;

        // Attempt to propagate messages to all the targets
        _Propagate_priority_order(_M_messageBuffer);
    }

private:

    /// <summary>
    ///     Propagates messages in priority order.
    /// </summary>
    /// <param name="_MessageBuffer">
    ///     Reference to a message queue with messages to be propagated
    /// </param>
    /**/
    void _Propagate_priority_order(::Concurrency::details::_Queue<message<_Target_type>> & _MessageBuffer)
    {
        message<_Target_type> * _Msg = _MessageBuffer._Peek();

        // If someone has reserved the _Head message, don't propagate anymore
        if (this->_M_pReservedFor != nullptr)
        {
            return;
        }

        while (_Msg != nullptr)
        {
            message_status _Status = declined;

            // Always start from the first target that linked
            for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
            {
                ITarget<_Target_type> * _PTarget = *_Iter;
                _Status = _PTarget->propagate(_Msg, this);

                // Ownership of message changed. Do not propagate this
                // message to any other target.
                if (_Status == accepted)
                {
                    break;
                }

                // If the target just propagated to reserved this message, stop
                // propagating it to others
                if (this->_M_pReservedFor != nullptr)
                {
                    break;
                }
            }

            // If status is anything other than accepted, then the head message
            // was not propagated out.  Thus, nothing after it in the queue can
            // be propagated out.  Cease propagation.
            if (_Status != accepted)
            {
                break;
            }

            // Get the next message
            _Msg = _MessageBuffer._Peek();
        }
    }

    /// <summary>
    ///     Deletes all messages currently stored in this message block.  Should be called
    ///     by the destructor to ensure any messages propagated in are cleaned up.
    /// </summary>
    /**/
    void _Delete_stored_messages()
    {
        // Input messages for this message block are in the base-class input buffer
        // All messages in that buffer are guaranteed to have moved to the output
        // buffer because the destructor first waits for all async sends to finish
        // before reaching this point

        // Delete any messages remaining in the output queue
        for (;;)
        {
            message<_Type> * _Msg = _M_messageBuffer._Dequeue();
            if (_Msg == nullptr)
            {
                break;
            }
            delete _Msg;
        }
    }

    /// <summary>
    ///     Message queue used to store processed messages
    /// </summary>
    /**/
    ::Concurrency::details::_Queue<message<_Type>> _M_processedMessages;

    /// <summary>
    ///     Message queue used to store messages
    /// </summary>
    /**/
    ::Concurrency::details::_Queue<message<_Type>> _M_messageBuffer;

    /// <summary>
    ///     A bool to signal to the processor to force a repropagation to occur
    /// </summary>
    /**/
    bool _M_fForceRepropagation;

private:
    //
    // Hide assignment operator and copy constructor
    //
    unbounded_buffer const &operator =(unbounded_buffer const&);  // no assignment operator
    unbounded_buffer(unbounded_buffer const &);                   // no copy constructor
};

//**************************************************************************
// Overwrite Buffers:
//**************************************************************************

/// <summary>
///     An <c>overwrite_buffer</c> messaging block is a multi-target, multi-source, ordered
///     <c>propagator_block</c> capable of storing a single message at
///     a time. New messages overwrite previously held ones.
/// </summary>
/// <typeparam name="_Type">
///     The payload type of the messages stored and propagated by the buffer.
/// </typeparam>
/// <remarks>
///     An <c>overwrite_buffer</c> messaging block propagates out copies of its stored message to each of its targets.
///     <para>For more information, see <see cref="Asynchronous Message Blocks"/>.</para>
/// </remarks>
/// <seealso cref="unbounded_buffer Class"/>
/// <seealso cref="single_assignment Class"/>
/**/
template<class _Type>
class overwrite_buffer : public propagator_block<multi_link_registry<ITarget<_Type>>, multi_link_registry<ISource<_Type>>>
{
private:
    typedef multi_link_registry<ITarget<_Type>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;
    using typename source_block<_TargetLinkRegistry>::target_iterator;

    /// <summary>
    ///     Constructs an <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>overwrite_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    overwrite_buffer()
        : _M_pMessage(nullptr), _M_pReservedMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target();
    }

    /// <summary>
    ///     Constructs an <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>overwrite_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    overwrite_buffer(filter_method const& _Filter)
        : _M_pMessage(nullptr), _M_pReservedMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target();
        this->register_filter(_Filter);
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs an <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>overwrite_buffer</c> messaging block is scheduled.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>overwrite_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    overwrite_buffer(Scheduler& _PScheduler)
        : _M_pMessage(nullptr), _M_pReservedMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(&_PScheduler);
    }

    /// <summary>
    ///     Constructs an <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>overwrite_buffer</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>overwrite_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    overwrite_buffer(Scheduler& _PScheduler, filter_method const& _Filter)
        : _M_pMessage(nullptr), _M_pReservedMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(&_PScheduler);
        this->register_filter(_Filter);
    }

    /// <summary>
    ///     Constructs an <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>overwrite_buffer</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>overwrite_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    overwrite_buffer(ScheduleGroup& _PScheduleGroup)
        : _M_pMessage(nullptr), _M_pReservedMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Constructs an <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>overwrite_buffer</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>overwrite_buffer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    overwrite_buffer(ScheduleGroup& _PScheduleGroup, filter_method const& _Filter)
        : _M_pMessage(nullptr), _M_pReservedMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
        this->register_filter(_Filter);
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Destroys the <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /**/
    ~overwrite_buffer()
    {
        // Remove all links that are targets of this overwrite_buffer
        this->remove_network_links();

        // Clean up any messages left in this message block
        _Delete_stored_messages();
    }

    /// <summary>
    ///     Checks whether this <c>overwrite_buffer</c> messaging block has a value yet.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the block has received a value, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool has_value() const
    {
        return _M_fIsInitialized != 0;
    }

    /// <summary>
    ///     Gets a reference to the current payload of the message being stored in the <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <returns>
    ///     The payload of the currently stored message.
    /// </returns>
    /// <remarks>
    ///     The value stored in the <c>overwrite_buffer</c> could change immediately after this method returns. This method will
    ///     wait until a message arrives if no message is currently stored in the <c>overwrite_buffer</c>.
    /// </remarks>
    /**/
    _Type value()
    {
        return ::Concurrency::receive<_Type>(this);
    }

protected:

    //
    // propagator_block protected function implementation
    //

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>overwrite_buffer</c> messaging block.
    ///     It is invoked by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by Consume and the LWT.  Doing so could
        // result in a deadlock with the Consume call.

        message_status _Result = accepted;

        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        //
        // If message was accepted, set the member variables for
        // this block and start the asynchronous propagation task
        //
        if (_PMessage != nullptr)
        {
            // Add a reference for the async_send holding the message
            _PMessage->add_ref();

            this->async_send(_PMessage);
        }
        else
        {
            _Result = missed;
        }

        return _Result;
    }

    /// <summary>
    ///     Synchronously passes a message from an <c>ISource</c> block to this <c>overwrite_buffer</c> messaging block.
    ///     It is invoked by the <c>send</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status send_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        //
        // If message was accepted, set the member variables for
        // this block and start the asynchronous propagation task
        //
        if (_PMessage != nullptr)
        {
            // Add a reference for the sync_send holding the message
            _PMessage->add_ref();

            this->sync_send(_PMessage);
        }
        else
        {
            return missed;
        }

        return accepted;
    }

    /// <summary>
    ///     Overrides the <c>supports_anonymous_source</c> method to indicate that this block can
    ///     accept messages offered to it by a source that is not linked.
    /// </summary>
    /// <returns>
    ///     <c>true</c> because the block does not postpone offered messages.
    /// </returns>
    /**/
    virtual bool supports_anonymous_source()
    {
        return true;
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>overwrite_buffer</c> messaging block,
    ///     returning a copy of the message to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     The <c>overwrite_buffer</c> messaging block returns copies of the message
    ///     to its targets, rather than transferring ownership of the currently
    ///     held message.
    /// </remarks>
    /**/
    virtual message<_Type> * accept_message(runtime_object_identity _MsgId)
    {
        //
        // If the internal message has not yet been initialized yet, return NULL
        //
        if (_M_pMessage == nullptr)
        {
            return nullptr;
        }

        //
        // Instead of returning the internal message, we return a copy of the
        // message stored.
        //
        // Because we are returning a copy, the accept routine for an overwritebuffer
        // does not need to grab the internalLock
        //
        message<_Type> * _Msg = nullptr;
        if (_M_pMessage->msg_id() == _MsgId)
        {
            _Msg = new message<_Type>(_M_pMessage->payload);
        }

        return _Msg;
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId)
    {
        // Ensure that this message currently exists in the overwrite buffer
        if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
        {
            return false;
        }

        // Can only reserve one message, any other blocks trying to reserve
        // will return false
        _CONCRT_ASSERT(_M_pReservedMessage == nullptr);

        // Save this message away
        _M_pReservedMessage = _M_pMessage;

        // Add a reference for this message to prevent deletion
        _M_pReservedMessage->add_ref();

        return true;
    }

    /// <summary>
    ///     Consumes a message previously offered by the <c>overwrite_buffer</c> messaging block and reserved by the target,
    ///     returning a copy of the message to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being consumed.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_Type> * consume_message(runtime_object_identity _MsgId)
    {
        // Leave and return NULL if this msgId doesn't match the reserved message
        // Otherwise this is a pull of a later overwritten message, and messages
        // could them appear out of order.
        if (_M_pReservedMessage != nullptr && _M_pReservedMessage->msg_id() != _MsgId)
        {
            return nullptr;
        }
        // This redundant assert is specifically to make the /analyze switch happy, which cannot recognize the same assertion above in if stmnt.
        _CONCRT_ASSERT( _M_pReservedMessage != nullptr );

        _Type _Payload = _M_pReservedMessage->payload;

        // Take the reserved message
        message<_Type> * _Result =  new message<_Type>(_Payload);

        if (_M_pReservedMessage->remove_ref() == 0)
        {
            delete _M_pReservedMessage;
        }
        _M_pReservedMessage = nullptr;

        return _Result;
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId)
    {
        _CONCRT_ASSERT(_M_fIsInitialized);
        _CONCRT_ASSERT(_M_pReservedMessage != nullptr);

        if (_MsgId != _M_pReservedMessage->msg_id())
        {
            throw message_not_found();
        }

        if (_M_pReservedMessage->remove_ref() == 0)
        {
            delete _M_pReservedMessage;
        }
        _M_pReservedMessage = nullptr;
    }

    /// <summary>
    ///     Resumes propagation after a reservation has been released.
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // On reservation we do not stop propagation. So there
        // is nothing to be done to resume propagation.
    }

    /// <summary>
    ///     A callback that notifies that a new target has been linked to this <c>overwrite_buffer</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<_Type> * _PTarget)
    {
        // If there is a message available already, propagate it
        if (_M_pMessage != nullptr)
        {
            _PTarget->propagate(_M_pMessage, this);
        }
    }

    /// <summary>
    ///     Places the <c>message</c> <paramref name="_PMessage"/> in this <c>overwrite_buffer</c> messaging block and
    ///     offers it to all of the linked targets.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to a <c>message</c> object that this <c>overwrite_buffer</c> has taken ownership of.
    /// </param>
    /// <remarks>
    ///     This method overwrites the current message in the <c>overwrite_buffer</c> with the newly
    ///     accepted message <paramref name="_PMessage"/>.
    /// </remarks>
    /**/
    virtual void propagate_to_any_targets(_Inout_ message<_Type> * _PMessage)
    {
        // Move the message from the queuedMessages Buffer to the internal storage

        // Add a reference for the overwrite_buffer holding the message
        _PMessage->add_ref();

        if (_M_pMessage != nullptr)
        {
            if (_M_pMessage->remove_ref() == 0)
            {
                delete _M_pMessage;
            }
        }

        _M_pMessage = _PMessage;

        // Now that message has been received, set this block as initialized
        _M_fIsInitialized = true;

        for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
        {
            // Overwrite buffers can propagate its message out
            // to any number of Targets

            ITarget<_Type> * _PTarget = *_Iter;
            _PTarget->propagate(_PMessage, this);
        }

        if (_PMessage->remove_ref() == 0)
        {
            delete _PMessage;
        }
    }

private:

    /// <summary>
    ///     Deletes all messages currently stored in this message block.  Should be called
    ///     by the destructor to ensure any messages propagated in are cleaned up.
    /// </summary>
    /**/
    void _Delete_stored_messages()
    {
        // Input messages for this message block are in the base-class input buffer
        // All messages in that buffer are guaranteed to have moved to the output
        // buffer because the destructor first waits for all async sends to finish
        // before reaching this point

        // The messages for an overwrite buffer are deleted when overwritten
        // through reference counting.  This final check is put in place in
        // case any message still exists in the buffer when the overwrite_buffer
        // is deleted.  The reference count of this message has not yet reached
        // zero because it hasn't been overwritten yet.  It is safe because of
        // we have finished all propagation.
        if (_M_pMessage != nullptr)
        {
            // A block can only have a reserved message after receiving a message
            // at some point, so it must be within the above if-clause.
            // Now delete the reserved message if it is non-NULL and different from
            // the saved internal message
            if (_M_pReservedMessage != nullptr && _M_pReservedMessage != _M_pMessage)
            {
                delete _M_pReservedMessage;
            }
            delete _M_pMessage;
        }
    }

    //
    //  Private Data Members
    //

    // The message being stored
    message<_Type> * _M_pMessage;

    // The message being reserved
    message<_Type> * _M_pReservedMessage;

    // The marker for whether the overwrite buffer has already been initialized
    volatile bool _M_fIsInitialized;

private:
    //
    // Hide assignment operator and copy constructor
    //
    overwrite_buffer const &operator =(overwrite_buffer const&);  // no assignment operator
    overwrite_buffer(overwrite_buffer const &);                   // no copy constructor
};

//**************************************************************************
// Call:
//**************************************************************************

/// <summary>
///     A <c>call</c> messaging block is a multi-source, ordered <c>target_block</c> that
///     invokes a specified function when receiving a message.
/// </summary>
/// <typeparam name="_Type">
///     The payload type of the messages propagated to this block.
/// </typeparam>
/// <typeparam name="_FunctorType">
///     The signature of functions that this block can accept.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/// <seealso cref="transformer Class"/>
/**/
template<class _Type, class _FunctorType = ::std::function<void(_Type const&)>>
class call : public target_block<multi_link_registry<ISource<_Type>>>
{
private:
    /// <summary>
    ///     The function type that this block executes upon receiving a <c>message</c>.
    /// </summary>
    /**/
    typedef _FunctorType _Call_method;

    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename ITarget<typename target_block<_SourceLinkRegistry>::_Source_type>::filter_method;

    /// <summary>
    ///     Constructs a <c>call</c> messaging block.
    /// </summary>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Call_method"/> is a functor with signature <c>void (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    call(_Call_method const& _Func) :
        _M_pFunc(_Func)
    {
        this->initialize_target();
        this->enable_batched_processing();
    }

    /// <summary>
    ///     Constructs a <c>call</c> messaging block.
    /// </summary>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Call_method"/> is a functor with signature <c>void (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    call(_Call_method const& _Func,
        filter_method const& _Filter) :
        _M_pFunc(_Func)
    {
        this->initialize_target();
        this->enable_batched_processing();
        this->register_filter(_Filter);
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs a <c>call</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>call</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Call_method"/> is a functor with signature <c>void (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    call(Scheduler& _PScheduler,
        _Call_method const& _Func) :
        _M_pFunc(_Func)
    {
        this->initialize_target(&_PScheduler);
        this->enable_batched_processing();
    }

    /// <summary>
    ///     Constructs a <c>call</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>call</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Call_method"/> is a functor with signature <c>void (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    call(Scheduler& _PScheduler,
        _Call_method const& _Func,
        filter_method const& _Filter) :
        _M_pFunc(_Func)
    {
        this->initialize_target(&_PScheduler);
        this->enable_batched_processing();
        this->register_filter(_Filter);
    }

    /// <summary>
    ///     Constructs a <c>call</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>call</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Call_method"/> is a functor with signature <c>void (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    call(ScheduleGroup& _PScheduleGroup,
        _Call_method const& _Func) :
        _M_pFunc(_Func)
    {
        this->initialize_target(nullptr, &_PScheduleGroup);
        this->enable_batched_processing();
    }

    /// <summary>
    ///     Constructs a <c>call</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>call</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Call_method"/> is a functor with signature <c>void (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>call</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    call(ScheduleGroup& _PScheduleGroup,
        _Call_method const& _Func,
        filter_method const& _Filter) :
        _M_pFunc(_Func)
    {
        this->initialize_target(nullptr, &_PScheduleGroup);
        this->enable_batched_processing();
        this->register_filter(_Filter);
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///    Destroys the <c>call</c> messaging block.
    /// </summary>
    /**/
    ~call()
    {
        this->remove_sources();
    }

protected:

    //
    // target_block protected function implementations
    //

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>call</c> messaging block. It is invoked
    ///     by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by Consume and the LWT.  Doing so could
        // result in a deadlock with the Consume call.

        message_status _Result = accepted;

        //
        // Accept the message being propagated
        // Note: depending on the source block propagating the message
        // this may not necessarily be the same message (pMessage) first
        // passed into the function.
        //
        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        if (_PMessage != nullptr)
        {
            this->async_send(_PMessage);
        }
        else
        {
            _Result = missed;
        }

        return _Result;
    }

    /// <summary>
    ///     Synchronously passes a message from an <c>ISource</c> block to this <c>call</c> messaging block. It is invoked
    ///     by the <c>send</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status send_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        message_status _Result = accepted;

        //
        // Accept the message being propagated
        // Note: depending on the source block propagating the message
        // this may not necessarily be the same message (pMessage) first
        // passed into the function.
        //
        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        if (_PMessage != nullptr)
        {
            this->sync_send(_PMessage);
        }
        else
        {
            _Result = missed;
        }

        return _Result;
    }

    /// <summary>
    ///     Overrides the <c>supports_anonymous_source</c> method to indicate that this block can
    ///     accept messages offered to it by a source that is not linked.
    /// </summary>
    /// <returns>
    ///     <c>true</c> because the block does not postpone offered messages.
    /// </returns>
    /**/
    virtual bool supports_anonymous_source()
    {
        return true;
    }

    /// <summary>
    ///     Processes a message that was accepted by this <c>call</c> messaging block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the message that is to be handled.
    /// </param>
    /**/
    virtual void process_message(_Inout_ message<_Type> * _PMessage)
    {
        (void) _PMessage;
        // No longer necessary with CRT110 change
    }

    /// <summary>
    ///     Executes the call function on the input messages.
    /// </summary>
    /**/
    virtual void process_input_messages(_Inout_ message<_Type> * _PMessage)
    {
        // Invoke the function provided by the user
        _CONCRT_ASSERT(_PMessage != nullptr);
        _M_pFunc(_PMessage->payload);
        delete _PMessage;
    }

private:

    //
    //  Private Data Members
    //

    // The call method called by this block
    _Call_method _M_pFunc;

private:
    //
    // Hide assignment operator and copy constructor
    //
    call const &operator =(call const&);  // no assignment operator
    call(call const &);                   // no copy constructor
};

//**************************************************************************
// Transformer:
//**************************************************************************

/// <summary>
///     A <c>transformer</c> messaging block is a single-target, multi-source, ordered
///     <c>propagator_block</c> which can accept messages of one type and is
///     capable of storing an unbounded number of messages of a different type.
/// </summary>
/// <typeparam name="_Input">
///     The payload type of the messages accepted by the buffer.
/// </typeparam>
/// <typeparam name="_Output">
///     The payload type of the messages stored and propagated out by the buffer.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/// <seealso cref="call Class"/>
/**/
template<class _Input, class _Output>
class transformer : public propagator_block<single_link_registry<ITarget<_Output>>, multi_link_registry<ISource<_Input>>>
{
private:
    typedef ::std::function<_Output(_Input const&)> _Transform_method;
    typedef single_link_registry<ITarget<_Output>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Input>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;

    /// <summary>
    ///     Constructs a <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to a target block to link with the transformer.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Transform_method"/> is a functor with signature <c>_Output (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    transformer(_Transform_method const& _Func,
        _Inout_opt_ ITarget<_Output> * _PTarget = nullptr) :
        _M_pFunc(_Func)
    {
        this->initialize_source_and_target();

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }
    }

    /// <summary>
    ///     Constructs a <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to a target block to link with the transform.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Transform_method"/> is a functor with signature <c>_Output (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    transformer(_Transform_method const& _Func,
        _Inout_opt_ ITarget<_Output> * _PTarget,
        filter_method const& _Filter) :
        _M_pFunc(_Func)
    {
        this->initialize_source_and_target();
        this->register_filter(_Filter);

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs a <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>transformer</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to a target block to link with the transformer.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Transform_method"/> is a functor with signature <c>_Output (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    transformer(Scheduler& _PScheduler,
        _Transform_method const& _Func,
        _Inout_opt_ ITarget<_Output> * _PTarget = nullptr) :
        _M_pFunc(_Func)
    {
        this->initialize_source_and_target(&_PScheduler);

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }
    }

    /// <summary>
    ///     Constructs a <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>transformer</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to a target block to link with the transformer.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Transform_method"/> is a functor with signature <c>_Output (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    transformer(Scheduler& _PScheduler,
        _Transform_method const& _Func,
        _Inout_opt_ ITarget<_Output> * _PTarget,
        filter_method const& _Filter) :
        _M_pFunc(_Func)
    {
        this->initialize_source_and_target(&_PScheduler);
        this->register_filter(_Filter);

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }
    }

    /// <summary>
    ///     Constructs a <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>transformer</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to a target block to link with the transformer.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Transform_method"/> is a functor with signature <c>_Output (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    transformer(ScheduleGroup& _PScheduleGroup,
        _Transform_method const& _Func,
        _Inout_opt_ ITarget<_Output> * _PTarget = nullptr) :
        _M_pFunc(_Func)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }
    }

    /// <summary>
    ///     Constructs a <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>transformer</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Func">
    ///     A function that will be invoked for each accepted message.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to a target block to link with the transformer.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="_Transform_method"/> is a functor with signature <c>_Output (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to process a message.</para>
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Input const &amp;)</c>
    ///     which is invoked by this <c>transformer</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    transformer(ScheduleGroup& _PScheduleGroup,
        _Transform_method const& _Func,
        _Inout_opt_ ITarget<_Output> * _PTarget,
        filter_method const& _Filter) :
        _M_pFunc(_Func)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
        this->register_filter(_Filter);

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Destroys the <c>transformer</c> messaging block.
    /// </summary>
    /**/
    ~transformer()
    {
        // Remove all links
        this->remove_network_links();

        // Clean up any messages left in this message block
        _Delete_stored_messages();
    }

protected:

    // Propagator_block protected function implementations

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>transformer</c> messaging block.
    ///     It is invoked by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(_Inout_ message<_Input> * _PMessage, _Inout_ ISource<_Input> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by Consume and the LWT.  Doing so could
        // result in a deadlock with the Consume call.

        message_status _Result = accepted;

        //
        // Accept the message being propagated
        // Note: depending on the source block propagating the message
        // this may not necessarily be the same message (pMessage) first
        // passed into the function.
        //
        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        if (_PMessage != nullptr)
        {
            // Enqueue the input message
            _M_inputMessages.push(_PMessage);
            this->async_send(nullptr);
        }
        else
        {
            _Result = missed;
        }

        return _Result;
    }

    /// <summary>
    ///     Synchronously passes a message from an <c>ISource</c> block to this <c>transformer</c> messaging block.
    ///     It is invoked by the <c>send</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status send_message(_Inout_ message<_Input> * _PMessage, _Inout_ ISource<_Input> * _PSource)
    {
        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        if (_PMessage != nullptr)
        {
            // Enqueue the input message
            _M_inputMessages.push(_PMessage);
            this->sync_send(nullptr);
        }
        else
        {
            return missed;
        }

        return accepted;
    }

    /// <summary>
    ///     Overrides the <c>supports_anonymous_source</c> method to indicate that this block can
    ///     accept messages offered to it by a source that is not linked.
    /// </summary>
    /// <returns>
    ///     <c>true</c> because the block does not postpone offered messages.
    /// </returns>
    /**/
    virtual bool supports_anonymous_source()
    {
        return true;
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>transformer</c> messaging block,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<_Output> * accept_message(runtime_object_identity _MsgId)
    {
        //
        // Peek at the head message in the message buffer.  If the IDs match
        // dequeue and transfer ownership
        //
        message<_Output> * _Msg = nullptr;

        if (_M_messageBuffer._Is_head(_MsgId))
        {
            _Msg = _M_messageBuffer._Dequeue();
        }

        return _Msg;
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId)
    {
        // Allow reservation if this is the head message
        return _M_messageBuffer._Is_head(_MsgId);
    }

    /// <summary>
    ///     Consumes a message previously offered by the <c>transformer</c> and reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being consumed.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_Output> * consume_message(runtime_object_identity _MsgId)
    {
        // By default, accept the message
        return accept_message(_MsgId);
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId)
    {
        // The head message is the one reserved.
        if (!_M_messageBuffer._Is_head(_MsgId))
        {
            throw message_not_found();
        }
    }

    /// <summary>
    ///     Resumes propagation after a reservation has been released.
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // If there are any messages in the buffer, propagate them out
        if (_M_messageBuffer._Count() > 0)
        {
            // async send a NULL value to initiate the repropagation
            this->async_send(nullptr);
        }
    }

    /// <summary>
    ///     A callback that notifies that a new target has been linked to this <c>transformer</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<_Output> *)
    {
        // If the message queue is blocked due to reservation
        // there is no need to do any message propagation
        if (this->_M_pReservedFor != nullptr)
        {
            return;
        }

        _Propagate_priority_order(_M_messageBuffer);
    }

    /// <summary>
    ///     Executes the transformer function on the input messages.
    /// </summary>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<_Output> *)
    {
        message<_Output> * _Msg = nullptr;

        // Process input message.
        message<_Input> * _PInputMessage = nullptr;
        _M_inputMessages.try_pop(_PInputMessage);

        if (_PInputMessage != nullptr)
        {
            // Invoke the TransformMethod on the data
            // Let exceptions flow
            _Output _Out = _M_pFunc(_PInputMessage->payload);

            // Reuse the input message ID
            _Msg = new message<_Output>(_Out, _PInputMessage->msg_id());
            _M_messageBuffer._Enqueue(_Msg);

            // Message cleanup
            delete _PInputMessage;

            if (!_M_messageBuffer._Is_head(_Msg->msg_id()))
            {
                return;
            }
        }

        _Propagate_priority_order(_M_messageBuffer);
    }

private:

    /// <summary>
    ///     Propagates messages in priority order.
    /// </summary>
    /// <param name="_MessageBuffer">
    ///     Reference to a message queue with messages to be propagated
    /// </param>
    /**/
    void _Propagate_priority_order(::Concurrency::details::_Queue<message<_Target_type>> & _MessageBuffer)
    {
        message<_Target_type> * _Msg = _MessageBuffer._Peek();

        // If someone has reserved the _Head message, don't propagate anymore
        if (this->_M_pReservedFor != nullptr)
        {
            return;
        }

        while (_Msg != nullptr)
        {
            message_status _Status = declined;

            // Always start from the first target that linked
            for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
            {
                ITarget<_Target_type> * _PTarget = *_Iter;
                _Status = _PTarget->propagate(_Msg, this);

                // Ownership of message changed. Do not propagate this
                // message to any other target.
                if (_Status == accepted)
                {
                    break;
                }

                // If the target just propagated to reserved this message, stop
                // propagating it to others
                if (this->_M_pReservedFor != nullptr)
                {
                    break;
                }
            }

            // If status is anything other than accepted, then the head message
            // was not propagated out.  Thus, nothing after it in the queue can
            // be propagated out.  Cease propagation.
            if (_Status != accepted)
            {
                break;
            }

            // Get the next message
            _Msg = _MessageBuffer._Peek();
        }
    }

    /// <summary>
    ///     Deletes all messages currently stored in this message block.  Should be called
    ///     by the destructor to ensure any messages propagated in are cleaned up.
    /// </summary>
    /**/
    void _Delete_stored_messages()
    {
        // Delete input messages
        // Because the transformer uses its own input queue, it's possible there are messages
        // in this queue and no LWT will be executed to handle them.
        message<_Input> * _PInputQueueMessage = nullptr;

        while (_M_inputMessages.try_pop(_PInputQueueMessage))
        {
            // Message cleanup
            delete _PInputQueueMessage;
        }

        // Delete any messages remaining in the output queue
        for (;;)
        {
            message<_Output> * _Msg = _M_messageBuffer._Dequeue();
            if (_Msg == nullptr)
            {
                break;
            }
            delete _Msg;
        }
    }

    //
    //  Private Data Members
    //

    // The transformer method called by this block
    _Transform_method _M_pFunc;

    // The queue of input messages for this Transformer block
    concurrent_queue<message<_Input> *> _M_inputMessages;

    /// <summary>
    ///    Message queue used to store outbound messages
    /// </summary>
    /**/
    ::Concurrency::details::_Queue<message<_Output>> _M_messageBuffer;

private:
    //
    // Hide assignment operator and copy constructor
    //
    transformer const &operator =(transformer const &);  // no assignment operator
    transformer(transformer const &);                    // no copy constructor
};

//**************************************************************************
// Timer:
//**************************************************************************
/// <summary>
///     A <c>timer</c> messaging block is a single-target <c>source_block</c> capable of sending
///     a message to its target after a specified time period has elapsed
///     or at specific intervals.
/// </summary>
/// <typeparam name="_Type">
///     The payload type of the output messages of this block.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/**/
template<class _Type>
class timer : public ::Concurrency::details::_Timer, public source_block<single_link_registry<ITarget<_Type>>>
{
private:
    typedef single_link_registry<ITarget<_Type>> _TargetLinkRegistry;
public:
    using typename source_block<_TargetLinkRegistry>::target_iterator;

private:
    /// <summary>
    ///     Tracks the state machine of the timer.
    /// </summary>
    /**/
    enum State
    {
        /// <summary>
        ///     The timer has been initialized, but not yet started.
        /// </summary>
        /**/
        Initialized,
        /// <summary>
        ///     The timer has been started.
        /// </summary>
        /**/
        Started,
        /// <summary>
        ///     The timer has started and been paused.
        /// </summary>
        /**/
        Paused,
        /// <summary>
        ///     The timer has been stopped.
        /// </summary>
        /**/
        Stopped
    };

public:

    /// <summary>
    ///     Constructs a <c>timer</c> messaging block that will fire a given message after a specified interval.
    /// </summary>
    /// <param name="_Ms">
    ///     The number of milliseconds that must elapse after the call to start for the specified message
    ///     to be propagated downstream.
    /// </param>
    /// <param name="_Value">
    ///     The value which will be propagated downstream when the timer elapses.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the timer will propagate its message.
    /// </param>
    /// <param name="_Repeating">
    ///     If true, indicates that the timer will fire periodically every <paramref name="_Ms"/> milliseconds.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_Scheduler"/>
    ///     or <paramref name="_ScheduleGroup"/> parameters.
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    timer(unsigned int _Ms, _Type const& _Value, ITarget<_Type> *_PTarget = nullptr, bool _Repeating = false) :
        _Timer(_Ms, _Repeating)
    {
        _Initialize(_Value, _PTarget, _Repeating);
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs a <c>timer</c> messaging block that will fire a given message after a specified interval.
    /// </summary>
    /// <param name="_Scheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>timer</c> messaging block is scheduled is scheduled.
    /// </param>
    /// <param name="_Ms">
    ///     The number of milliseconds that must elapse after the call to start for the specified message
    ///     to be propagated downstream.
    /// </param>
    /// <param name="_Value">
    ///     The value which will be propagated downstream when the timer elapses.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the timer will propagate its message.
    /// </param>
    /// <param name="_Repeating">
    ///     If true, indicates that the timer will fire periodically every <paramref name="_Ms"/> milliseconds.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_Scheduler"/>
    ///     or <paramref name="_ScheduleGroup"/> parameters.
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    timer(Scheduler& _Scheduler, unsigned int _Ms, _Type const& _Value, _Inout_opt_ ITarget<_Type> *_PTarget = nullptr, bool _Repeating = false) :
        _Timer(_Ms, _Repeating)
    {
        _Initialize(_Value, _PTarget, _Repeating, &_Scheduler);
    }

    /// <summary>
    ///     Constructs a <c>timer</c> messaging block that will fire a given message after a specified interval.
    /// </summary>
    /// <param name="_ScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>timer</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Ms">
    ///     The number of milliseconds that must elapse after the call to start for the specified message
    ///     to be propagated downstream.
    /// </param>
    /// <param name="_Value">
    ///     The value which will be propagated downstream when the timer elapses.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the timer will propagate its message.
    /// </param>
    /// <param name="_Repeating">
    ///     If true, indicates that the timer will fire periodically every <paramref name="_Ms"/> milliseconds.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_Scheduler"/>
    ///     or <paramref name="_ScheduleGroup"/> parameters.
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    timer(ScheduleGroup& _ScheduleGroup, unsigned int _Ms, _Type const& _Value, _Inout_opt_ ITarget<_Type> *_PTarget = nullptr, bool _Repeating = false) :
        _Timer(_Ms, _Repeating)
    {
        _Initialize(_Value, _PTarget, _Repeating, nullptr, &_ScheduleGroup);
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Destroys a <c>timer</c> messaging block.
    /// </summary>
    /**/
    ~timer()
    {
        //
        // Make sure there are no more outstanding timer fires.  Note that this does not mean that the LWT that was queued is finished, it only
        // means that no more timers will fire after the return from _Stop.  We still *MUST* wait on any outstanding LWTs.
        //
        if (_M_state == Started)
            _Stop();

        // Remove all the targets. This will wait for any outstanding LWTs
        this->remove_targets();

        //
        // No more asynchronous operations can happen as of this point.
        //

        // Clean up any messages left in this message block
        _Delete_stored_messages();

        if (_M_fReferencedScheduler)
        {
            ::Concurrency::details::_Scheduler(_M_pScheduler)._Release();
        }
    }

    /// <summary>
    ///     Starts the <c>timer</c> messaging block.  The specified number of milliseconds after this is called, the specified value will be propagated
    ///     downstream as a <c>message</c>.
    /// </summary>
    /**/
    void start()
    {
        if (_M_state == Initialized || _M_state == Paused)
        {
            _M_state = Started;
            _Start();
        }
    }

    /// <summary>
    ///     Stops the <c>timer</c> messaging block.
    /// </summary>
    /**/
    void stop()
    {
        if (_M_state == Started)
            _Stop();

        _M_state = Stopped;
    }

    /// <summary>
    ///     Stops the <c>timer</c> messaging block.  If it is a repeating <c>timer</c> messaging block, it can be restarted with a subsequent
    ///     <c>start()</c> call.  For non-repeating timers, this has the same effect as a <c>stop</c> call.
    /// </summary>
    /**/
    void pause()
    {
        //
        // Non repeating timers cannot pause.  They go to a final stopped state on pause.
        //
        if (!_M_fRepeating)
        {
            stop();
        }
        else
        {
            // Pause only a started timer.

            if (_M_state == Started)
            {
                _Stop();
                _M_state = Paused;
            }
        }
    }

protected:

    /// <summary>
    ///     Accepts a message that was offered by this <c>timer</c> messaging block,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<_Type> * accept_message(runtime_object_identity _MsgId)
    {
        if (_M_pMessage == nullptr || _MsgId != _M_pMessage->msg_id())
        {
            return nullptr;
        }

        message<_Type> *_PMessage = _M_pMessage;
        _M_pMessage = nullptr;

        return _PMessage;
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>timer</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId)
    {
        //
        // Semantically, every timer tick is the same value -- it doesn't matter the message ID.  Because we can only
        // have one target as well, we do not need to track anything here.
        //
        if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
        {
            return false;
        }

        return true;
    }

    /// <summary>
    ///     Consumes a message previously offered by the <c>timer</c> and reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being consumed.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_Type> * consume_message(runtime_object_identity _MsgId)
    {
        return accept_message(_MsgId);
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId)
    {
        if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
        {
            throw message_not_found();
        }

        delete _M_pMessage;
        _M_pMessage = nullptr;
    }

    /// <summary>
    ///     Resumes propagation after a reservation has been released.
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // Because reservation doesn't prevent propagation there is
        // no need to resume on consume/release.
    }

    /// <summary>
    ///     A callback that notifies that a new target has been linked to this <c>timer</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<_Type> * _PTarget)
    {
        // If there is a timer message sitting around, it must be propagated to the target now.

        if (_M_pMessage != nullptr)
        {
            _PTarget->propagate(_M_pMessage, this);
        }
    }

    /// <summary>
    ///     Tries to offer the message produced by the <c>timer</c> block to all of the linked targets.
    /// </summary>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<_Type> *)
    {
        if (_M_pMessage == nullptr)
        {
            _M_pMessage = _NewMessage();
            for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
            {
                ITarget<_Type> * _PTarget = *_Iter;
                _PTarget->propagate(_M_pMessage, this);
            }
        }
    }

private:

    // The timer message we contain
    message<_Type> *_M_pMessage;

    // Current state of the timer.
    State _M_state;

    // The value to send on elapse of the timer.
    _Type _M_value;

    // An indication of whether the timer is repeating.
    bool _M_fRepeating;

    // A flag for whether we need to release a reference on the scheduler.
    bool _M_fReferencedScheduler;

    // Scheduler used for the timer
    Scheduler * _M_pScheduler;

    /// <summary>
    ///     Allocates a new message.
    /// </summary>
    /**/
    message<_Type>* _NewMessage() const
    {
        return new message<_Type>(_M_value);
    }

    /// <summary>
    ///     Called when the timer fires.
    /// </summary>
    /**/
    virtual void _Fire()
    {
        this->async_send(nullptr);
    }

    /// <summary>
    ///     Common initialization.
    /// </summary>
    /// <param name="_Value">
    ///     The value which will be propagated downstream when the timer elapses.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the timer will propagate its message.
    /// </param>
    /// <param name="_Repeating">
    ///     If true, indicates that the timer will fire periodically every _Ms milliseconds.
    /// </param>
    /**/
    void _Initialize(const _Type& _Value, _Inout_ ITarget<_Type> *_PTarget, bool _Repeating, _Inout_opt_ Scheduler * _PScheduler = nullptr, _Inout_opt_ ScheduleGroup * _PScheduleGroup = nullptr)
    {
        _M_pMessage = nullptr;
        _M_value = _Value;
        _M_fRepeating = _Repeating;
        _M_state = Initialized;
        _M_fReferencedScheduler = false;

        //
        // If we are going to utilize the current scheduler for timer firing, we need to capture it now.  Otherwise,
        // the timer threads fired from Windows (what _Fire executes within) will wind up with a default scheduler
        // attached -- probably not the semantic we want.
        //
        if (_PScheduleGroup == nullptr && _PScheduler == nullptr)
        {
            ::Concurrency::details::_Scheduler _sched = ::Concurrency::details::_CurrentScheduler::_Get();
            _PScheduler = _sched._GetScheduler();
            _sched._Reference();
            _M_fReferencedScheduler = true;
        }

        _M_pScheduler = _PScheduler;
        this->initialize_source(_PScheduler, _PScheduleGroup);

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }
    }

    /// <summary>
    ///     Deletes all messages currently stored in this message block.  Should be called
    ///     by the destructor to ensure any messages propagated in are cleaned up.
    /// </summary>
    /**/
    void _Delete_stored_messages()
    {
        // Input messages for this message block are in the base-class input buffer
        // All messages in that buffer are guaranteed to have moved to the output
        // buffer because the destructor first waits for all async sends to finish
        // before reaching this point

        // Delete the message remaining in the output queue
        if (_M_pMessage != nullptr)
        {
            delete _M_pMessage;
        }
    }

private:
    //
    // Hide assignment operator and copy constructor
    //
    timer const &operator =(timer const &);  // no assignment operator
    timer(timer const &);                    // no copy constructor
};

//**************************************************************************
// Single assignment:
//**************************************************************************

/// <summary>
///     A <c>single_assignment</c> messaging block is a multi-target, multi-source, ordered
///     <c>propagator_block</c> capable of storing a single, write-once
///     <c>message</c>.
/// </summary>
/// <typeparam name="_Type">
///     The payload type of the message stored and propagated by the buffer.
/// </typeparam>
/// <remarks>
///     A <c>single_assignment</c> messaging block propagates out copies of its message to each target.
///     <para>For more information, see <see cref="Asynchronous Message Blocks"/>.</para>
/// </remarks>
/// <seealso cref="overwrite_buffer Class"/>
/// <seealso cref="unbounded_buffer Class"/>
/**/
template<class _Type>
class single_assignment : public propagator_block<multi_link_registry<ITarget<_Type>>, multi_link_registry<ISource<_Type>>>
{
private:
    typedef multi_link_registry<ITarget<_Type>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;

    /// <summary>
    ///     Constructs a <c>single_assignment</c> messaging block.
    /// </summary>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>single_assignment</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    single_assignment() :
        _M_pMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target();
    }

    /// <summary>
    ///     Constructs a <c>single_assignment</c> messaging block.
    /// </summary>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>single_assignment</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    single_assignment(filter_method const& _Filter) :
        _M_pMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target();
        this->register_filter(_Filter);
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs a <c>single_assignment</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>single_assignment</c> messaging block is scheduled.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>single_assignment</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    single_assignment(Scheduler& _PScheduler) :
        _M_pMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(&_PScheduler);
    }

    /// <summary>
    ///     Constructs a <c>single_assignment</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>single_assignment</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>single_assignment</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    single_assignment(Scheduler& _PScheduler, filter_method const& _Filter) :
        _M_pMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(&_PScheduler);
        this->register_filter(_Filter);
    }

    /// <summary>
    ///     Constructs a <c>single_assignment</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>single_assignment</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>single_assignment</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    single_assignment(ScheduleGroup& _PScheduleGroup) :
        _M_pMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Constructs a <c>single_assignment</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>single_assignment</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>single_assignment</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    single_assignment(ScheduleGroup& _PScheduleGroup, filter_method const& _Filter) :
        _M_pMessage(nullptr), _M_fIsInitialized(false)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
        this->register_filter(_Filter);
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Destroys the <c>single_assignment</c> messaging block.
    /// </summary>
    /**/
    ~single_assignment()
    {
        // Remove all links
        this->remove_network_links();

        // Clean up any messages left in this message block
        _Delete_stored_messages();
    }

    /// <summary>
    ///     Checks whether this <c>single_assignment</c> messaging block has been initialized with a value yet.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the block has received a value, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool has_value() const
    {
        return (_M_pMessage != nullptr);
    }


    /// <summary>
    ///     Gets a reference to the current payload of the message being stored in the <c>single_assignment</c> messaging block.
    /// </summary>
    /// <returns>
    ///     The payload of the stored message.
    /// </returns>
    /// <remarks>
    ///     This method will wait until a message arrives if no message is currently stored in the <c>single_assignment</c> messaging block.
    /// </remarks>
    /**/
    _Type const & value()
    {
        if (_M_pMessage == nullptr)
        {
            ::Concurrency::receive<_Type>(this);
        }
        _CONCRT_ASSERT(_M_pMessage != nullptr);

        return _M_pMessage->payload;
    }


protected:

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>single_assignment</c> messaging block.
    ///     It is invoked by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by Consume and the LWT.  Doing so could
        // result in a deadlock with the Consume call.

        message_status _Result = accepted;

        // single_assignment messaging block can be initialized only once
        if (_M_fIsInitialized)
        {
            return declined;
        }

        {
            _NR_lock _Lock(_M_propagationLock);

            if (_M_fIsInitialized)
            {
                _Result = declined;
            }
            else
            {
                _PMessage = _PSource->accept(_PMessage->msg_id(), this);

                // Set initialized flag only if we have a message
                if (_PMessage != nullptr)
                {
                    _M_fIsInitialized = true;
                }
                else
                {
                    _Result = missed;
                }
            }
        }

        //
        // If message was accepted, set the member variables for
        // this block and start the asynchronous propagation task
        //
        if (_Result == accepted)
        {
            this->async_send(_PMessage);
        }

        return _Result;
    }

    /// <summary>
    ///     Synchronously passes a message from an <c>ISource</c> block to this <c>single_assignment</c> messaging block.
    ///     It is invoked by the <c>send</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status send_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        message_status _Result = accepted;

        // single_assignment messaging block can be initialized only once
        if (_M_fIsInitialized)
        {
            return declined;
        }

        {
            _NR_lock _Lock(_M_propagationLock);

            if (_M_fIsInitialized)
            {
                _Result = declined;
            }
            else
            {
                _PMessage = _PSource->accept(_PMessage->msg_id(), this);

                // Set initialized flag only if we have a message
                if (_PMessage != nullptr)
                {
                    _M_fIsInitialized = true;
                }
                else
                {
                    _Result = missed;
                }
            }
        }

        //
        // If message was accepted, set the member variables for
        // this block and start the asynchronous propagation task
        //
        if (_Result == accepted)
        {
            this->sync_send(_PMessage);
        }

        return _Result;
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>single_assignment</c> messaging block,
    ///     returning a copy of the message to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     The <c>single_assignment</c> messaging block returns copies of the message
    ///     to its targets, rather than transferring ownership of the currently
    ///     held message.
    /// </remarks>
    /**/
    virtual message<_Type> * accept_message(runtime_object_identity _MsgId)
    {
        // This check is to prevent spoofing and verify that the propagated message is
        // the one that is accepted at the end.
        if (_M_pMessage == nullptr || _MsgId != _M_pMessage->msg_id())
        {
            return nullptr;
        }

        //
        // Instead of returning the internal message, we return a copy of the
        // message passed in.
        //
        // Because we are returning a copy, the accept routine for a single_assignment
        // does not need to grab the internal lock.
        //
        return (new message<_Type>(_M_pMessage->payload));
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>single_assignment</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId)
    {
        if (_M_pMessage == nullptr)
        {
            return false;
        }

        if (_M_pMessage->msg_id() != _MsgId)
        {
            throw message_not_found();
        }

        return true;
    }

    /// <summary>
    ///     Consumes a message previously offered by the <c>single_assignment</c> and reserved by the target,
    ///     returning a copy of the message to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being consumed.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_Type> * consume_message(runtime_object_identity _MsgId)
    {
        _CONCRT_ASSERT(_M_fIsInitialized);

        return accept_message(_MsgId);
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId)
    {
        _CONCRT_ASSERT(_M_fIsInitialized);

        if (_M_pMessage == nullptr || _M_pMessage->msg_id() != _MsgId)
        {
            throw message_not_found();
        }
    }

    /// <summary>
    ///     Resumes propagation after a reservation has been released.
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // Because reservation doesn't stop propagation, we don't
        // need to do anything on resume after consume/release.
    }

    /// <summary>
    ///     A callback that notifies that a new target has been linked to this <c>single_assignment</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<_Type> * _PTarget)
    {
        // If there is a message available already, propagate it.

        if (_M_pMessage != nullptr)
        {
            _PTarget->propagate(_M_pMessage, this);
        }
    }
    /// <summary>
    ///     Places the <c>message</c> <paramref name="_PMessage"/> in this <c>single_assignment</c> messaging block and
    ///     offers it to all of the linked targets.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to a <c>message</c> that this <c>single_assignment</c> messaging block has taken ownership of.
    /// </param>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<_Type> * _PMessage)
    {
        // Initialized flag should have been set by the propagate function using interlocked operation.
        _CONCRT_ASSERT(_M_fIsInitialized);

        // Move the message to the internal storage

        _CONCRT_ASSERT(_M_pMessage == nullptr);
        _M_pMessage = _PMessage;

        for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
        {
            // Single assignment can propagate its message out
            // to any number of Targets

            ITarget<_Type> * _PTarget = *_Iter;
            _PTarget->propagate(_PMessage, this);
        }
    }

private:

    /// <summary>
    ///     Deletes all messages currently stored in this message block.  Should be called
    ///     by the destructor to ensure any messages propagated in are cleaned up.
    /// </summary>
    /**/
    void _Delete_stored_messages()
    {
        // Input messages for this message block are in the base-class input buffer
        // All messages in that buffer are guaranteed to have moved to the output
        // buffer because the destructor first waits for all async sends to finish
        // before reaching this point

        // The messages for a single_assignment are deleted at the end when
        // single_assignment is deleted.
        delete _M_pMessage;
    }

    //
    //  Private Data Members
    //

    // The message being stored
    message<_Type> * _M_pMessage;

    // The lock used to protect propagation
    ::Concurrency::details::_NonReentrantPPLLock _M_propagationLock;

    // The marker for whether the single_assignment has already been initialized
    volatile bool _M_fIsInitialized;

private:
    //
    // Hide assignment operator and copy constructor
    //
    single_assignment const & operator=(single_assignment const &);  // no assignment operator
    single_assignment(single_assignment const &);                    // no copy constructor
};

//**************************************************************************
// Join (single-type)
//**************************************************************************

/// <summary>
///     The type of a <c>join</c> messaging block.
/// </summary>
/**/
enum join_type {
    /// <summary>
    ///     Greedy <c>join</c> messaging blocks immediately accept a message upon propagation.  This is more efficient,
    ///     but has the possibility for live-lock, depending on the network configuration.
    /// </summary>
    /**/
    greedy = 0,
    /// <summary>
    ///     Non-greedy <c>join</c> messaging blocks postpone messages and try and consume them after all have arrived.
    ///     These are guaranteed to work, but slower.
    /// </summary>
    /**/
    non_greedy = 1
};

/// <summary>
///     A <c>join</c> messaging block  is a single-target, multi-source, ordered
///     <c>propagator_block</c> which combines together messages of type <typeparamref name="_Type"/> from each
///     of its sources.
/// </summary>
/// <typeparam name="_Type">
///     The payload type of the messages joined and propagated by the block.
/// </typeparam>
/// <typeparam name="_Jtype">
///     The kind of <c>join</c> block this is, either <c>greedy</c> or <c>non_greedy</c>
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/// <seealso cref="choice Class"/>
/// <seealso cref="multitype_join Class"/>
/// <seealso cref="join_type Enumeration"/>
/**/
template<class _Type, join_type _Jtype = non_greedy>
class join : public propagator_block<single_link_registry<ITarget<::std::vector<_Type>>>, multi_link_registry<ISource<_Type>>>
{
private:
    typedef single_link_registry<ITarget<::std::vector<_Type>>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;
    using typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::source_iterator;

    typedef ::std::vector<_Type> _OutputType;

    /// <summary>
    ///     Constructs a <c>join</c> messaging block.
    /// </summary>
    /// <param name="_NumInputs">
    ///     The number of inputs this <c>join</c> block will be allowed.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>join</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    join(size_t _NumInputs)
        : _M_messageArray(_NumInputs),
          _M_savedMessageIdArray(_NumInputs)
    {
        _Initialize(_NumInputs);
    }

    /// <summary>
    ///     Constructs a <c>join</c> messaging block.
    /// </summary>
    /// <param name="_NumInputs">
    ///     The number of inputs this <c>join</c> block will be allowed.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>join</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    join(size_t _NumInputs, filter_method const& _Filter)
        : _M_messageArray(_NumInputs),
          _M_savedMessageIdArray(_NumInputs)
    {
        _Initialize(_NumInputs);
        this->register_filter(_Filter);
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs a <c>join</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>join</c> messaging block is scheduled.
    /// </param>
    /// <param name="_NumInputs">
    ///     The number of inputs this <c>join</c> block will be allowed.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>join</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    join(Scheduler& _PScheduler, size_t _NumInputs)
        : _M_messageArray(_NumInputs),
          _M_savedMessageIdArray(_NumInputs)
    {
        _Initialize(_NumInputs, &_PScheduler);
    }

    /// <summary>
    ///     Constructs a <c>join</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>join</c> messaging block is scheduled.
    /// </param>
    /// <param name="_NumInputs">
    ///     The number of inputs this <c>join</c> block will be allowed.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>join</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    join(Scheduler& _PScheduler, size_t _NumInputs, filter_method const& _Filter)
        : _M_messageArray(_NumInputs),
          _M_savedMessageIdArray(_NumInputs)
    {
        _Initialize(_NumInputs, &_PScheduler);
        this->register_filter(_Filter);
    }

    /// <summary>
    ///     Constructs a <c>join</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>join</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_NumInputs">
    ///     The number of inputs this <c>join</c> block will be allowed.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>join</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    join(ScheduleGroup& _PScheduleGroup, size_t _NumInputs)
        : _M_messageArray(_NumInputs),
          _M_savedMessageIdArray(_NumInputs)
    {
        _Initialize(_NumInputs, nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Constructs a <c>join</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>join</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_NumInputs">
    ///     The number of inputs this <c>join</c> block will be allowed.
    /// </param>
    /// <param name="_Filter">
    ///     A filter function which determines whether offered messages should be accepted.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PScheduleGroup"/> parameters.
    ///     <para>The type <typeparamref name="filter_method"/> is a functor with signature <c>bool (_Type const &amp;)</c>
    ///     which is invoked by this <c>join</c> messaging block to determine whether or not it should accept
    ///     an offered message.</para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    join(ScheduleGroup& _PScheduleGroup, size_t _NumInputs, filter_method const& _Filter)
        : _M_messageArray(_NumInputs),
          _M_savedMessageIdArray(_NumInputs)
    {
        _Initialize(_NumInputs, nullptr, &_PScheduleGroup);
        this->register_filter(_Filter);
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Destroys the <c>join</c> block.
    /// </summary>
    /**/
    ~join()
    {
        // Remove all links that are targets of this join
        this->remove_network_links();

        // Clean up any messages left in this message block
        _Delete_stored_messages();

        delete [] _M_savedIdBuffer;
    }

protected:
    //
    // propagator_block protected function implementations
    //

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>join</c> messaging block.
    ///     It is invoked by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    message_status propagate_message(_Inout_ message<_Type> * _PMessage, _Inout_ ISource<_Type> * _PSource)
    {
        // It is important that calls to propagate do *not* take the same lock on the
        // internal structure that is used by Consume and the LWT.  Doing so could
        // result in a deadlock with the Consume call.

        message_status _Ret_val = accepted;

        //
        // Find the slot index of this source
        //
        size_t _Slot = 0;
        bool _Found = false;
        for (source_iterator _Iter = this->_M_connectedSources.begin(); *_Iter != nullptr; ++_Iter)
        {
            if (*_Iter == _PSource)
            {
                _Found = true;
                break;
            }

            _Slot++;
        }

        if (!_Found)
        {
            // If this source was not found in the array, this is not a connected source
            // decline the message
            return declined;
        }

        _CONCRT_ASSERT(_Slot < _M_messageArray._M_count);

        bool fIsGreedy = (_Jtype == greedy);

        if (fIsGreedy)
        {
            //
            // Greedy type joins immediately accept the message.
            //
            {
                _NR_lock lockHolder(_M_propagationLock);
                if (_M_messageArray._M_messages[_Slot] != nullptr)
                {
                    _M_savedMessageIdArray._M_savedIds[_Slot] = _PMessage->msg_id();
                    _Ret_val = postponed;
                }
            }

            if (_Ret_val != postponed)
            {
                _M_messageArray._M_messages[_Slot] = _PSource->accept(_PMessage->msg_id(), this);

                if (_M_messageArray._M_messages[_Slot] != nullptr)
                {
                    if (_InterlockedDecrementSizeT(&_M_messagesRemaining) == 0)
                    {
                        // If messages have arrived on all links, start a propagation
                        // of the current message
                        this->async_send(nullptr);
                    }
                }
                else
                {
                    _Ret_val = missed;
                }
            }
        }
        else
        {
            //
            // Non-greedy type joins save the message IDs until they have all arrived
            //

            if (_InterlockedExchange((volatile long *) &_M_savedMessageIdArray._M_savedIds[_Slot], _PMessage->msg_id()) == -1)
            {
                // Decrement the message remaining count if this thread is switching
                // the saved ID from -1 to a valid value.
                if (_InterlockedDecrementSizeT(&_M_messagesRemaining) == 0)
                {
                    this->async_send(nullptr);
                }
            }

            // Always return postponed.  This message will be consumed
            // in the LWT
            _Ret_val = postponed;
        }

        return _Ret_val;
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>join</c> messaging block,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<_OutputType> * accept_message(runtime_object_identity _MsgId)
    {
        //
        // Peek at the head message in the message buffer.  If the IDs match
        // dequeue and transfer ownership
        //
        message<_OutputType> * _Msg = nullptr;

        if (_M_messageBuffer._Is_head(_MsgId))
        {
            _Msg = _M_messageBuffer._Dequeue();
        }

        return _Msg;
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>join</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId)
    {
        // Allow reservation if this is the head message
        return _M_messageBuffer._Is_head(_MsgId);
    }

    /// <summary>
    ///     Consumes a message previously offered by the <c>join</c> messaging block and reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being consumed.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_OutputType> * consume_message(runtime_object_identity _MsgId)
    {
        // By default, accept the message
        return accept_message(_MsgId);
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId)
    {
        // The head message is the one reserved.
        if (!_M_messageBuffer._Is_head(_MsgId))
        {
            throw message_not_found();
        }
    }

    /// <summary>
    ///     Resumes propagation after a reservation has been released.
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // If there are any messages in the buffer, propagate them out
        if (_M_messageBuffer._Count() > 0)
        {
            this->async_send(nullptr);
        }
    }

    /// <summary>
    ///     A callback that notifies that a new target has been linked to this <c>join</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<::std::vector<_Type>> *)
    {
        // If the message queue is blocked due to reservation
        // there is no need to do any message propagation
        if (this->_M_pReservedFor != nullptr)
        {
            return;
        }

        _Propagate_priority_order(_M_messageBuffer);
    }

    /// <summary>
    ///     Constructs an output message containing an input message from each source when
    ///     they have all propagated a message.  Sends this output message out to each of
    ///     its targets.
    /// </summary>
    /**/
    void propagate_to_any_targets(_Inout_opt_ message<_OutputType> *)
    {
        message<_OutputType> * _Msg = nullptr;
        // Create a new message from the input sources
        // If messagesRemaining == 0, we have a new message to create.  Otherwise, this is coming from
        // a consume or release from the target.  In that case we don't want to create a new message.
        if (_M_messagesRemaining == 0)
        {
            // A greedy join can immediately create the message, a non-greedy
            // join must try and consume all the messages it has postponed
            _Msg = _Create_new_message();
        }

        if (_Msg == nullptr)
        {
            // Create message failed.  This happens in non_greedy joins when the
            // reserve/consumption of a postponed message failed.
            _Propagate_priority_order(_M_messageBuffer);
            return;
        }

        bool fIsGreedy = (_Jtype == greedy);

        // For a greedy join, reset the number of messages remaining
        // Check to see if multiple messages have been passed in on any of the links,
        // and postponed. If so, try and reserve/consume them now
        if (fIsGreedy)
        {
            // Look at the saved IDs and reserve/consume any that have passed in while
            // this join was waiting to complete
            _CONCRT_ASSERT(_M_messageArray._M_count == _M_savedMessageIdArray._M_count);

            for (size_t i = 0; i < _M_messageArray._M_count; i++)
            {
                for(;;)
                {
                    runtime_object_identity _Saved_id;
                    // Grab the current saved ID value.  This value could be changing from based on any
                    // calls of source->propagate(this).  If the message ID is different than what is snapped
                    // here, that means, the reserve below must fail.  This is because reserve is trying
                    // to get the same source lock the propagate(this) call must be holding.
                    {
                        _NR_lock lockHolder(_M_propagationLock);

                        _CONCRT_ASSERT(_M_messageArray._M_messages[i] != nullptr);

                        _Saved_id = _M_savedMessageIdArray._M_savedIds[i];

                        if (_Saved_id == -1)
                        {
                            _M_messageArray._M_messages[i] = nullptr;
                            break;
                        }
                        else
                        {
                            _M_savedMessageIdArray._M_savedIds[i] = -1;
                        }
                    }

                    if (_Saved_id != -1)
                    {
                        source_iterator _Iter = this->_M_connectedSources.begin();

                        ISource<_Type> * _PSource = _Iter[i];
                        if ((_PSource != nullptr) && _PSource->reserve(_Saved_id, this))
                        {
                            _M_messageArray._M_messages[i] = _PSource->consume(_Saved_id, this);
                            _InterlockedDecrementSizeT(&_M_messagesRemaining);
                            break;
                        }
                    }
                }
            }

            // If messages have all been received, async_send again, this will start the
            // LWT up to create a new message
            if (_M_messagesRemaining == 0)
            {
                this->async_send(nullptr);
            }
        }

        // Add the new message to the outbound queue
        _M_messageBuffer._Enqueue(_Msg);

        if (!_M_messageBuffer._Is_head(_Msg->msg_id()))
        {
            // another message is at the head of the outbound message queue and blocked
            // simply return
            return;
        }

        _Propagate_priority_order(_M_messageBuffer);
    }

private:

    //
    //  Private Methods
    //

    /// <summary>
    ///     Propagate messages in priority order.
    /// </summary>
    /// <param name="_MessageBuffer">
    ///     Reference to a message queue with messages to be propagated
    /// </param>
    /**/
    void _Propagate_priority_order(::Concurrency::details::_Queue<message<_Target_type>> & _MessageBuffer)
    {
        message<_Target_type> * _Msg = _MessageBuffer._Peek();

        // If someone has reserved the _Head message, don't propagate anymore
        if (this->_M_pReservedFor != nullptr)
        {
            return;
        }

        while (_Msg != nullptr)
        {
            message_status _Status = declined;

            // Always start from the first target that linked
            for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
            {
                ITarget<_Target_type> * _PTarget = *_Iter;
                _Status = _PTarget->propagate(_Msg, this);

                // Ownership of message changed. Do not propagate this
                // message to any other target.
                if (_Status == accepted)
                {
                    break;
                }

                // If the target just propagated to reserved this message, stop
                // propagating it to others
                if (this->_M_pReservedFor != nullptr)
                {
                    break;
                }
            }

            // If status is anything other than accepted, then the head message
            // was not propagated out.  Thus, nothing after it in the queue can
            // be propagated out.  Cease propagation.
            if (_Status != accepted)
            {
                break;
            }

            // Get the next message
            _Msg = _MessageBuffer._Peek();
        }
    }

    /// <summary>
    ///     Constructs a new message from the data output.
    /// </summary>
    /// <returns>
    ///     The created message (NULL if creation failed)
    /// </returns>
    /**/
    message<::std::vector<_Type>> * __cdecl _Create_new_message()
    {
        bool fIsNonGreedy = (_Jtype == non_greedy);

        // If this is a non-greedy join, check each source and try to consume their message
        if (fIsNonGreedy)
        {

            // The iterator _Iter below will ensure that it is safe to touch
            // non-NULL source pointers. Take a snapshot.
            ::std::vector<ISource<_Type> *> _Sources;
            source_iterator _Iter = this->_M_connectedSources.begin();

            while (*_Iter != nullptr)
            {
                ISource<_Type> * _PSource = *_Iter;

                if (_PSource == nullptr)
                {
                    break;
                }

                _Sources.push_back(_PSource);
                ++_Iter;
            }

            if (_Sources.size() != _M_messageArray._M_count)
            {
                // Some of the sources were unlinked. The join is broken
                return nullptr;
            }

            // First, try and reserve all the messages.  If a reservation fails,
            // then release any reservations that had been made.
            for (size_t i = 0; i < _M_savedMessageIdArray._M_count; i++)
            {
                // Snap the current saved ID into a buffer.  This value can be changing behind the scenes from
                // other source->propagate(msg, this) calls, but if so, that just means the reserve below will
                // fail.
                _InterlockedIncrementSizeT(&_M_messagesRemaining);
                _M_savedIdBuffer[i] = _InterlockedExchange((volatile long *) &_M_savedMessageIdArray._M_savedIds[i], -1);

                _CONCRT_ASSERT(_M_savedIdBuffer[i] != -1);

                if (!_Sources[i]->reserve(_M_savedIdBuffer[i], this))
                {
                    // A reservation failed, release all reservations made up until
                    // this block, and wait for another message to arrive on this link
                    for (size_t j = 0; j < i; j++)
                    {
                        _Sources[j]->release(_M_savedIdBuffer[j], this);
                        if (_InterlockedCompareExchange((volatile long *) &_M_savedMessageIdArray._M_savedIds[j], _M_savedIdBuffer[j], -1) == -1)
                        {
                            if (_InterlockedDecrementSizeT(&_M_messagesRemaining) == 0)
                            {
                                this->async_send(nullptr);
                            }
                        }
                    }

                    // Return NULL to indicate that the create failed
                    return nullptr;
                }
            }

            // Because everything has been reserved, consume all the messages.
            // This is guaranteed to return true.
            for (size_t i = 0; i < _M_messageArray._M_count; i++)
            {
                _M_messageArray._M_messages[i] = _Sources[i]->consume(_M_savedIdBuffer[i], this);
                _M_savedIdBuffer[i] = -1;
            }
        }

        if (!fIsNonGreedy)
        {
            // Reinitialize how many messages are being waited for.
            // This is safe because all messages have been received, thus no new async_sends for
            // greedy joins can be called.
            _M_messagesRemaining = _M_messageArray._M_count;
        }

        ::std::vector<_Type> _OutputVector;
        for (size_t i = 0; i < _M_messageArray._M_count; i++)
        {
            _CONCRT_ASSERT(_M_messageArray._M_messages[i] != nullptr);
            _OutputVector.push_back(_M_messageArray._M_messages[i]->payload);

            delete _M_messageArray._M_messages[i];
            if (fIsNonGreedy)
            {
                _M_messageArray._M_messages[i] = nullptr;
            }
        }
        return (new message<::std::vector<_Type>>(_OutputVector));
    }

    /// <summary>
    ///     Initializes the <c>join</c> messaging block.
    /// </summary>
    /// <param name="_NumInputs">
    ///     The number of inputs.
    /// </param>
    /// <param name="_PScheduler">
    ///     The scheduler onto which the task to propagate the <c>join</c> block's message will be scheduled.
    ///     If unspecified, the <c>join</c> messaging block uses the default scheduler.
    /// </param>
    /// <param name="_PScheduleGroup">
    ///     The schedule group into which the task to propagate the <c>join</c> block's message will be scheduled.
    ///     The scheduler used is implied by the schedule group.  If unspecified, the <c>join</c> uses a schedule
    ///     group of the scheduler's choosing.
    /// </param>
    /**/
    void _Initialize(size_t _NumInputs, Scheduler * _PScheduler = nullptr, ScheduleGroup * _PScheduleGroup = nullptr)
    {
        this->initialize_source_and_target(_PScheduler, _PScheduleGroup);

        this->_M_connectedSources.set_bound(_NumInputs);
        _M_messagesRemaining = _NumInputs;

        bool fIsNonGreedy = (_Jtype == non_greedy);

        if (fIsNonGreedy)
        {
            // Non greedy joins need a buffer to snap off saved message IDs to.
            _M_savedIdBuffer = new runtime_object_identity[_NumInputs];
            memset(_M_savedIdBuffer, -1, sizeof(runtime_object_identity) * _NumInputs);
        }
        else
        {
            _M_savedIdBuffer = nullptr;
        }
    }

    /// <summary>
    ///     Deletes all messages currently stored in this message block.  Should be called
    ///     by the destructor to ensure any messages propagated in are cleaned up.
    /// </summary>
    /**/
    void _Delete_stored_messages()
    {
        // Input messages for this message block are in the base-class input buffer
        // All messages in that buffer are guaranteed to have moved to the output
        // buffer because the destructor first waits for all async sends to finish
        // before reaching this point

        // Delete any messages remaining in the output queue
        for (;;)
        {
            message<::std::vector<_Type>> * _Msg = _M_messageBuffer._Dequeue();
            if (_Msg == nullptr)
            {
                break;
            }
            delete _Msg;
        }
    }

    // The current number of messages remaining
    volatile size_t _M_messagesRemaining;

    // An array containing the accepted messages of this join.
    // Wrapped in a struct to enable debugger visualization.
    struct _MessageArray
    {
        size_t _M_count;
        message<_Type>** _M_messages;

        _MessageArray(size_t _NumInputs)
            : _M_count(_NumInputs),
              _M_messages(new message<_Type>*[_NumInputs])
        {
            memset(_M_messages,  0, sizeof(message<_Type> *) * _NumInputs);
        }

        ~_MessageArray()
        {
            for (size_t i = 0; i < _M_count; i++)
                delete _M_messages[i];
            delete [] _M_messages;
        }
    };
    _MessageArray _M_messageArray;

    // An array containing the msg IDs of messages propagated to the array
    // For greedy joins, this contains a log of other messages passed to this
    // join after the first has been accepted
    // For non-greedy joins, this contains the message ID of any message
    // passed to it.
    // Wrapped in a struct to enable debugger visualization.
    struct _SavedMessageIdArray
    {
        size_t _M_count;
        runtime_object_identity * _M_savedIds;

        _SavedMessageIdArray(size_t _NumInputs)
            : _M_count(_NumInputs),
              _M_savedIds(new runtime_object_identity[_NumInputs])
        {
            memset(_M_savedIds, -1, sizeof(runtime_object_identity) * _NumInputs);
        }

        ~_SavedMessageIdArray()
        {
            delete [] _M_savedIds;
        }
    };
    _SavedMessageIdArray _M_savedMessageIdArray;

    // Buffer for snapping saved IDs in non-greedy joins
    runtime_object_identity * _M_savedIdBuffer;

    // A lock for modifying the buffer or the connected blocks
    ::Concurrency::details::_NonReentrantPPLLock _M_propagationLock;

    // Queue to hold output messages
    ::Concurrency::details::_Queue<message<::std::vector<_Type>>> _M_messageBuffer;
};


//**************************************************************************
// Multi-Type Choice and Join helper node:
//**************************************************************************

/// <summary>
///     Base class for Helper node used in multi-type join and choice blocks
///     Order node is a single-target, single-source ordered propagator block
///     The main property of an order node is that it accepts a message of _Type
///     and outputs a message of int, with some unique assigned index number.
/// </summary>
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/**/
template<class _Type>
class _Order_node_base: public propagator_block<single_link_registry<ITarget<size_t>>, multi_link_registry<ISource<_Type>>>
{
public:
    /// <summary>
    ///     Constructs a _Order_node_base within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /**/
    _Order_node_base() :
        _M_pReceiveMessage(nullptr),
        _M_pSendMessage(nullptr),
        _M_index(0)
    {
    }

    /// <summary>
    ///     Cleans up any resources that may have been created by the _Order_node.
    /// </summary>
    /**/
    ~_Order_node_base()
    {
        // The messages for an _Order_node_base are deleted at the end when
        // _Order_node_base is deleted.
        delete _M_pReceiveMessage;
        delete _M_pSendMessage;
    }

    /// <summary>
    ///     Checks whether this block has been initialized yet.
    /// </summary>
    /// <returns>
    ///     true, if the block has received a value, false otherwise.
    /// </returns>
    /**/
    bool has_value() const
    {
        return (_M_pReceiveMessage != nullptr);
    }

    /// <summary>
    ///     Gets a reference to the current payload of the message being stored.
    /// </summary>
    /// <returns>
    ///     The incoming payload.
    /// </returns>
    /**/
    _Type const & value()
    {
        _CONCRT_ASSERT(_M_pReceiveMessage != nullptr);

        return _M_pReceiveMessage->payload;
    }

    /// <summary>
    ///     Resets the _Order_node_base and prepares it for the next propagation
    /// </summary>
    /// <remarks>
    ///     _Reset is called from Populate_destination_tuple through propagate_to_any_targets()
    ///     thus, it always has the internal lock held.  This is only used for _Greedy_node and
    ///     _Non_greedy_node.
    /// </remarks>
    /**/
    virtual void _Reset() = 0;

    /// <summary>
    ///     Reserves a message previously offered by the source.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A bool indicating whether the reservation worked or not
    /// </returns>
    /// <remarks>
    ///     After 'reserve' is called, either 'consume' or 'release' must be called.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity)
    {
        // reserve should never be called for this block.
        _CONCRT_ASSERT(false);

        return false;
    }

    /// <summary>
    ///     Consumes a message previously offered by the source and reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     Similar to 'accept', but is always preceded by a call to 'reserve'
    /// </remarks>
    /**/
    virtual message<size_t> * consume_message(runtime_object_identity)
    {
        // consume should never be called for this block.
        _CONCRT_ASSERT(false);

        return nullptr;
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity)
    {
        // release should never be called for this block.
        _CONCRT_ASSERT(false);
    }

protected:


    /// <summary>
    ///     Resumes propagation after a reservation has been released
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // Because there is only a single target, nothing needs
        // to be done on resume
    }

    /// <summary>
    /// Notification that a target was linked to this source.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<size_t> *)
    {
        if (_M_pSendMessage != nullptr)
        {
            this->propagate_to_any_targets(nullptr);
        }
    }

    /// <summary>
    ///     Create a message that contains an index used to determine the source message
    /// </summary>
    /**/
    void _Create_send_message()
    {
        _M_pSendMessage = new message<size_t>(_M_index);
    }

    /// <summary>
    ///     Validate constructor arguments and fully connect this _Order_node_base.
    /// </summary>
    /**/
    void _Initialize_order_node(ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, Scheduler * _PScheduler = nullptr, ScheduleGroup * _PScheduleGroup = nullptr)
    {
        if (_PSource == nullptr)
        {
            throw ::std::invalid_argument("_PSource");
        }

        _M_index = _Index;

        this->initialize_source_and_target(_PScheduler, _PScheduleGroup);

        // Allow only a single source and ensure that they
        // cannot be unlinked and relinked.
        this->_M_connectedSources.set_bound(1);

        if (_PTarget != nullptr)
        {
            this->link_target(_PTarget);
        }

        _PSource->link_target(this);
    }

    //
    //  Private Data Members
    //

    // The message to be received from the source
    message<_Type> * _M_pReceiveMessage;

    // The message to be sent to all targets
    message<size_t> * _M_pSendMessage;

    // The index of the _Order_node_base in the user's construct
    size_t _M_index;

private:
    //
    // Hide assignment operator and copy constructor
    //
    _Order_node_base const & operator=(_Order_node_base const &);  // no assignment operator
    _Order_node_base(_Order_node_base const &);                    // no copy constructor
};


/// <summary>
///     Helper class used in multi-type choice blocks
///     Ordered node is a single-target, single-source ordered propagator block
/// </summary>
///
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/**/
template<class _Type>
class _Reserving_node: public _Order_node_base<_Type>
{
private:
    typedef single_link_registry<ITarget<size_t>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;
    using typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::source_iterator;

    /// <summary>
    ///     Constructs a _Reserving_node within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Reserving_node(ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr) :
        _M_fIsInitialized(false),
        _M_savedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget);
    }

    /// <summary>
    ///     Constructs a _Reserving_node within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Reserving_node(ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter) :
        _M_fIsInitialized(false),
        _M_savedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget);
    }

    /// <summary>
    ///     Constructs a _Reserving_node within the specified scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A reference to a scheduler instance.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Reserving_node(Scheduler& _PScheduler, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr) :
        _M_fIsInitialized(false),
        _M_savedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget, &_PScheduler);
    }

    /// <summary>
    ///     Constructs a _Reserving_node within the specified scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A reference to a scheduler instance.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Reserving_node(Scheduler& _PScheduler, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter) :
        _M_fIsInitialized(false),
        _M_savedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget, &_PScheduler);
    }

    /// <summary>
    ///     Constructs a _Order_node within the specified schedule group.  The scheduler is implied
    ///     by the schedule group.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     A reference to a schedule group.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Reserving_node(ScheduleGroup& _PScheduleGroup, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr) :
        _M_fIsInitialized(false),
        _M_savedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget, nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Constructs a _Order_node within the specified schedule group.  The scheduler is implied
    ///     by the schedule group.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     A reference to a schedule group.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Reserving_node(ScheduleGroup& _PScheduleGroup, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter) :
        _M_fIsInitialized(false),
        _M_savedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget, nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Cleans up any resources that may have been created by the _Reserving_node.
    /// </summary>
    /**/
    ~_Reserving_node()
    {
        if (_M_pReservedSource != nullptr)
        {
            _M_pReservedSource = nullptr;
            this->_M_connectedSources.release();
        }

        // Remove all links
        this->remove_network_links();
    }


    /// <summary>
    ///     Resets the _Reserving_node and prepares it for the next propagation
    /// </summary>
    /// <remarks>
    ///     This function is not used in a _Reserving_node, which is only used for choice blocks
    /// </remarks>
    /**/
    virtual void _Reset()
    {
    }

protected:

    //
    // propagator_block protected function implementation
    //

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>ITarget</c> block. It is invoked
    ///     by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     It is important that calls to propagate do *not* take the same lock on the
    ///     internal structure that is used by Consume and the light-weight task.  Doing so could
    ///     result in a deadlock with the Consume call.
    /// </remarks>
    /**/
    virtual message_status propagate_message(message<_Type> * _PMessage, ISource<_Type> * _PSource)
    {
        message_status _Result = postponed;

        // _Order_node messaging block can be initialized only once, just like single_assignment.
        if (_M_fIsInitialized)
        {
            return declined;
        }

        // Reserve a message on the source until this _Order_node gets the feedback from
        // the single_assignment on whether it has been selected.
        _M_fIsInitialized = _PSource->reserve(_PMessage->msg_id(), this);

        //
        // If message was successfully reserved, set the member variables for
        // this messaging block and start the asynchronous propagation task.
        //
        if (_M_fIsInitialized)
        {
            _M_savedId = _PMessage->msg_id();
            this->async_send(nullptr);
        }
        else
        {
            _Result = missed;
        }

        return _Result;
    }

    /// <summary>
    ///     Accept the message by making a copy of the payload.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<size_t> * accept_message(runtime_object_identity _MsgId)
    {
        // This check is to prevent spoofing and verify that the propagated message is
        // the one that is accepted at the end.
        if (this->_M_pSendMessage == nullptr || _MsgId != this->_M_pSendMessage->msg_id())
        {
            return nullptr;
        }

        // If the source has disconnected then we can't allow for accept to succeed.
        source_iterator _Iter = this->_M_connectedSources.begin();
        ISource<_Type>* _PSource = *_Iter;

        if (_PSource == nullptr)
        {
            // source was disconnected. Fail accept.
            return nullptr;
        }

        this->_M_pReceiveMessage = _PSource->consume(_M_savedId, this);

        _CONCRT_ASSERT(this->_M_pReceiveMessage != nullptr);

        //
        // Instead of returning the internal message, we return a copy of the
        // message passed in.
        //
        // Because we are returning a copy, the accept routine for a _Order_node
        // does not need to grab the internal lock.
        //
        return (new message<size_t>(this->_M_pSendMessage->payload));
    }

    /// <summary>
    ///     Takes the message and propagates it to all the targets of this _Order_node
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to a new message.
    /// </param>
    /// <remarks>
    ///     This function packages its _M_index into a message and immediately sends it to the targets.
    /// </remarks>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<size_t> *)
    {
        if (this->_M_pSendMessage == nullptr)
        {
            this->_Create_send_message();
        }

        for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
        {
            ITarget<size_t> * _PTarget = *_Iter;
            _Propagate_to_target(_PTarget);
        }
    }

private:

    /// <summary>
    ///     Propagate messages to the given target
    /// </summary>
    /**/
    message_status _Propagate_to_target(ITarget<size_t> * _PTarget)
    {
        message_status _Status = _PTarget->propagate(this->_M_pSendMessage, this);

        // If the message got rejected we have to release the hold on the source message.
        if (_Status != accepted)
        {
            if (_M_savedId != -1)
            {
                // Release the reservation
                source_iterator _Iter = this->_M_connectedSources.begin();
                ISource<_Type> * _PSource = *_Iter;

                if (_PSource != nullptr)
                {
                    _PSource->release(_M_savedId, this);
                }

                // If the source was disconnected, then it would
                // automatically release any reservation. So we
                // should reset our savedId regardless.
                _M_savedId = -1;
            }

        }

        return _Status;
    }

    //
    //  Private Data Members
    //

    // The source where we have reserved a message
    ISource<_Type> * _M_pReservedSource;

    // For greedy order-nodes, the message ID of subsequent messages sent to this node
    // For non-greedy order nodes, the message ID of the message to reserve/consume
    runtime_object_identity _M_savedId;

    // The marker that indicates that _Reserving_node has reserved a message
    volatile bool _M_fIsInitialized;

private:
    //
    // Hide assignment operator and copy constructor
    //
    _Reserving_node const & operator=(_Reserving_node const &);  // no assignment operator
    _Reserving_node(_Reserving_node const &);                    // no copy constructor
};


/// <summary>
///     Helper class used in multi-type greedy join blocks
///     Ordered node is a single-target, single-source ordered propagator block
/// </summary>
///
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/**/
template<class _Type>
class _Greedy_node: public _Order_node_base<_Type>
{
private:
    typedef single_link_registry<ITarget<size_t>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;
    using typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::source_iterator;

    /// <summary>
    ///     Constructs a _Greedy_node within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Greedy_node(ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr)
        : _M_pGreedyMessage(nullptr)
        , _M_savedId(-1)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget);
    }

    /// <summary>
    ///     Constructs a _Greedy_node within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Greedy_node(ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter)
        : _M_pGreedyMessage(nullptr)
        , _M_savedId(-1)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget);
    }

    /// <summary>
    ///     Constructs a _Greedy_node within the specified scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A reference to a scheduler instance.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Greedy_node(Scheduler& _PScheduler, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr)
        : _M_pGreedyMessage(nullptr)
        , _M_savedId(-1)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget, &_PScheduler);
    }

    /// <summary>
    ///     Constructs a _Greedy_node within the specified scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A reference to a scheduler instance.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Greedy_node(Scheduler& _PScheduler, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter)
        : _M_pGreedyMessage(nullptr)
        , _M_savedId(-1)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget, &_PScheduler);
    }

    /// <summary>
    ///     Constructs a _Greedy_node within the specified schedule group.  The scheduler is implied
    ///     by the schedule group.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     A reference to a schedule group.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Greedy_node(ScheduleGroup& _PScheduleGroup, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr)
        : _M_pGreedyMessage(nullptr)
        , _M_savedId(-1)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget, nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Constructs a _Greedy_node within the specified schedule group.  The scheduler is implied
    ///     by the schedule group.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     A reference to a schedule group.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Greedy_node(ScheduleGroup& _PScheduleGroup, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter)
        : _M_pGreedyMessage(nullptr)
        , _M_savedId(-1)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget, nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Cleans up any resources that may have been created by the _Greedy_node.
    /// </summary>
    /**/
    ~_Greedy_node()
    {
        // Remove all links
        this->remove_network_links();

        if (_M_pGreedyMessage != this->_M_pReceiveMessage)
        {
            delete _M_pGreedyMessage;
        }
    }

    /// <summary>
    ///     Resets the _Greedy_node and prepares it for the next propagation
    /// </summary>
    /// <remarks>
    ///     _Reset is called from Populate_destination_tuple through propagate_to_any_targets()
    ///     thus, it always has the internal lock held.
    /// </remarks>
    /**/
    void _Reset()
    {
        _R_lock _Lock(_M_resetLock);

        delete this->_M_pReceiveMessage;
        this->_M_pReceiveMessage = nullptr;

        delete this->_M_pSendMessage;
        this->_M_pSendMessage = nullptr;

        //
        // For greedy type joins, look to see if any other messages have been
        // passed to this _Greedy_node while the join was waiting for other
        // messages to arrive.  This function is already called with _M_resetLock
        // held through propagate_to_any_targets().
        //
        for(;;)
        {
            // Set the current saved ID as -1.  Check to see if something was ready for consumption
            // (if _Saved_id != -1) and consume it if possible.
            runtime_object_identity _Saved_id;

            {
                _NR_lock lockHolder(_M_propagationLock);

                _Saved_id = _M_savedId;

                if (_Saved_id == -1)
                {
                    _M_pGreedyMessage = nullptr;
                    break;
                }
                else
                {
                    _M_savedId = -1;
                }
            }

            if (_Saved_id != -1)
            {
                source_iterator _Iter = this->_M_connectedSources.begin();

                ISource<_Type> * _PSource = *_Iter;
                if ((_PSource != nullptr) && _PSource->reserve(_Saved_id, this))
                {
                    _M_pGreedyMessage = _PSource->consume(_Saved_id, this);
                    this->async_send(nullptr);
                    break;
                }
            }
        }
    }

protected:

    //
    // propagator_block protected function implementation
    //

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>ITarget</c> block. It is invoked
    ///     by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     It is important that calls to propagate do *not* take the same lock on the
    ///     internal structure that is used by Consume and the light-weight task.  Doing so could
    ///     result in a deadlock with the Consume call.
    /// </remarks>
    /**/
    virtual message_status propagate_message(message<_Type> * _PMessage, ISource<_Type> * _PSource)
    {
        message_status _Result = postponed;

        bool _FDone = false;

        {
            _NR_lock lockHolder(_M_propagationLock);
            if (_M_pGreedyMessage != nullptr)
            {
                _M_savedId = _PMessage->msg_id();
                _Result = postponed;
                _FDone = true;
            }
        }

        if (!_FDone)
        {
            _M_pGreedyMessage = _PSource->accept(_PMessage->msg_id(), this);

            if (_M_pGreedyMessage != nullptr)
            {
                _Result = accepted;
                this->async_send(nullptr);
            }
            else
            {
                _Result = missed;
            }
        }

        return _Result;
    }

    /// <summary>
    ///     Accept the message by making a copy of the payload.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<size_t> * accept_message(runtime_object_identity _MsgId)
    {
        // This check is to prevent spoofing and verify that the propagated message is
        // the one that is accepted at the end.
        if (this->_M_pSendMessage == nullptr || _MsgId != this->_M_pSendMessage->msg_id())
        {
            return nullptr;
        }

        //
        // Instead of returning the internal message, we return a copy of the
        // message passed in.
        //
        // Because we are returning a copy, the accept routine for a _Greedy_node
        // does not need to grab the internal lock.
        //
        return (new message<size_t>(this->_M_pSendMessage->payload));
    }


    /// <summary>
    ///     Takes the message and propagates it to all the targets of this _Greedy_node
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to a new message.
    /// </param>
    /// <remarks>
    ///     This function packages its _M_index into a message and immediately sends it to the targets.
    /// </remarks>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<size_t> *)
    {
        _R_lock _Lock(_M_resetLock);

        if (this->_M_pSendMessage == nullptr)
        {
            // Save the incoming message so that it can be consumed in the accept function
            this->_M_pReceiveMessage = _M_pGreedyMessage;
            this->_Create_send_message();
        }

        for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
        {
            ITarget<size_t> * _PTarget = *_Iter;
            _PTarget->propagate(this->_M_pSendMessage, this);
        }
    }

private:

    //
    //  Private Data Members
    //

    // The message to be saved by a greedy order node
    message<_Type> * _M_pGreedyMessage;

    // The lock used to protect propagation
    ::Concurrency::details::_NonReentrantPPLLock _M_propagationLock;

    // The lock used to protect modification during a reset
    ::Concurrency::details::_ReentrantPPLLock _M_resetLock;

    // For greedy order-nodes, the message ID of subsequent messages sent to this node
    // For non-greedy order nodes, the message ID of the message to reserve/consume
    runtime_object_identity _M_savedId;

private:
    //
    // Hide assignment operator and copy constructor
    //
    _Greedy_node const & operator=(_Greedy_node const &);  // no assignment operator
    _Greedy_node(_Greedy_node const &);                    // no copy constructor
};


/// <summary>
///     Helper class used in multi-type non-greedy join blocks
///     Ordered node is a single-target, single-source ordered propagator block
/// </summary>
///
/// <typeparam name="_Type">
///     The payload type
/// </typeparam>
/**/
template<class _Type>
class _Non_greedy_node: public _Order_node_base<_Type>
{
private:
    typedef single_link_registry<ITarget<size_t>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<_Type>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename ITarget<typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::_Source_type>::filter_method;
    using typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::source_iterator;

    /// <summary>
    ///     Constructs a _Non_greedy_node within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Non_greedy_node(ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr) :
        _M_savedId(-1),
        _M_reservedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget);
    }

    /// <summary>
    ///     Constructs a _Non_greedy_node within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Non_greedy_node(ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter) :
        _M_savedId(-1),
        _M_reservedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget);
    }

    /// <summary>
    ///     Constructs a _Non_greedy_node within the specified scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A reference to a scheduler instance.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Non_greedy_node(Scheduler& _PScheduler, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr) :
        _M_savedId(-1),
        _M_reservedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget, &_PScheduler);
    }

    /// <summary>
    ///     Constructs a _Non_greedy_node within the specified scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A reference to a scheduler instance.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Non_greedy_node(Scheduler& _PScheduler, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter) :
        _M_savedId(-1),
        _M_reservedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget, &_PScheduler);
    }

    /// <summary>
    ///     Constructs a _Non_greedy_node within the specified schedule group.  The scheduler is implied
    ///     by the schedule group.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     A reference to a schedule group.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /**/
    _Non_greedy_node(ScheduleGroup& _PScheduleGroup, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget = nullptr) :
        _M_savedId(-1),
        _M_reservedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->_Initialize_order_node(_PSource, _Index, _PTarget, nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Constructs a _Non_greedy_node within the specified schedule group.  The scheduler is implied
    ///     by the schedule group.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     A reference to a schedule group.
    /// </param>
    /// <param name="_PSource">
    ///     The source of data passed into the node
    /// </param>
    /// <param name="_Index">
    ///     The node's index, assigned from the outside.
    /// </param>
    /// <param name="_PTarget">
    ///     The target to which the node will signal about having received its input data
    /// </param>
    /// <param name="_Filter">
    ///     A reference to a filter function.
    /// </param>
    /**/
    _Non_greedy_node(ScheduleGroup& _PScheduleGroup, ISource<_Type> * _PSource, size_t _Index, ITarget<size_t> * _PTarget, filter_method const& _Filter) :
        _M_savedId(-1),
        _M_reservedId(-1),
        _M_pReservedSource(nullptr)
    {
        this->register_filter(_Filter);
        this->_Initialize_order_node(_PSource, _Index, _PTarget, nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Cleans up any resources that may have been created by the _Order_node.
    /// </summary>
    /**/
    ~_Non_greedy_node()
    {
        if (_M_pReservedSource != nullptr)
        {
            _M_pReservedSource = nullptr;
            this->_M_connectedSources.release();
        }

        // Remove all links
        this->remove_network_links();
    }

    /// <summary>
    ///     Resets the _Order_node and prepares it for the next propagation
    /// </summary>
    /// <remarks>
    ///     _Reset is called from Populate_destination_tuple through propagate_to_any_targets()
    ///     thus, it always has the internal lock held.
    /// </remarks>
    /**/
    void _Reset()
    {
        _R_lock _Lock(_M_resetLock);

        delete this->_M_pReceiveMessage;
        this->_M_pReceiveMessage = nullptr;

        delete this->_M_pSendMessage;
        this->_M_pSendMessage = nullptr;
    }

    /// <summary>
    ///     Called for a non_greedy type join block in order to reserve the message
    ///     in this join block
    /// </summary>
    /// <returns>
    ///     A bool indicating whether the reservation worked
    /// </returns>
    /**/
    bool _Reserve_received_message()
    {
        bool _Ret_val = false;

        // Order node has only  a single source.
        // Obtain an iterator to the first source. It will guarantee that the reference
        // count on the source is maintained
        source_iterator _Iter = this->_M_connectedSources.begin();
        ISource<_Type> * _PSource = *_Iter;

        if (_PSource != nullptr)
        {
            // CAS out the current saved ID, in order to try and reserve it
            runtime_object_identity _SavedId = _InterlockedExchange((volatile long *) &_M_savedId, -1);

            _Ret_val = _PSource->reserve(_SavedId, this);
            //
            // If this reserved failed, that means we need to wait for another message
            // to come in on this link.  _M_savedID was set to -1 to indicate to the _Order_node
            // that it needs to async_send when that next message comes through
            //
            // If the reserve succeeds, save away the reserved ID.  This will be use later in
            // consume
            //
            if (_Ret_val)
            {
                _M_reservedId = _SavedId;

                // Acquire a reference on the source
                this->_M_connectedSources.reference();
                _M_pReservedSource = _PSource;
            }
        }

        return _Ret_val;
    }

    /// <summary>
    ///     Called for a non_greedy type join block in order to consume the message
    ///     in this join block that has been reserved
    /// </summary>
    /**/
    void _Consume_received_message()
    {
        if (_M_pReservedSource != nullptr)
        {
            runtime_object_identity _SavedId = _M_reservedId;
            this->_M_pReceiveMessage = _M_pReservedSource->consume(_SavedId, this);

            runtime_object_identity _OldId = _InterlockedExchange((volatile long *) &_M_reservedId, -1);

            _CONCRT_ASSERT(_OldId == _SavedId);

            // Release the reference on the source
            _M_pReservedSource = nullptr;
            this->_M_connectedSources.release();
        }
    }

    /// <summary>
    ///     Called for a non_greedy type join block release a reservation on this block
    /// </summary>
    /**/
    bool _Release_received_message()
    {
        bool retVal = false;

        if (_M_pReservedSource != nullptr)
        {
            runtime_object_identity _SavedId = _M_reservedId;
            // If the _M_savedId is still -1, then swap the succeeded one back
            _M_pReservedSource->release(_SavedId, this);

            if (_InterlockedCompareExchange((volatile long *) &_M_savedId, _SavedId, -1) == -1)
            {
                retVal = true;
            }

            // Release the reference on the source
            _M_pReservedSource = nullptr;
            this->_M_connectedSources.release();
        }

        return retVal;
    }

protected:

    //
    // propagator_block protected function implementation
    //

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>ITarget</c> block. It is invoked
    ///     by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /// <remarks>
    ///     It is important that calls to propagate do *not* take the same lock on the
    ///     internal structure that is used by Consume and the light-weight task.  Doing so could
    ///     result in a deadlock with the Consume call.
    /// </remarks>
    /**/
    virtual message_status propagate_message(message<_Type> * _PMessage, ISource<_Type> *)
    {
        // Change the message ID.  If it was -1, that means an async-send needs to occur
        if (_InterlockedExchange((volatile long *) &_M_savedId, _PMessage->msg_id()) == -1)
        {
            this->async_send(nullptr);
        }

        // Always return postponed.  This message will be consumed
        // in the LWT

        return postponed;
    }

    /// <summary>
    ///     Accept the message by making a copy of the payload.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<size_t> * accept_message(runtime_object_identity _MsgId)
    {
        // This check is to prevent spoofing and verify that the propagated message is
        // the one that is accepted at the end.
        if (this->_M_pSendMessage == nullptr || _MsgId != this->_M_pSendMessage->msg_id())
        {
            return nullptr;
        }

        //
        // Instead of returning the internal message, we return a copy of the
        // message passed in.
        //
        // Because we are returning a copy, the accept routine for a _Non_greedy_node
        // does not need to grab the internal lock.
        //
        return (new message<size_t>(this->_M_pSendMessage->payload));
    }

    /// <summary>
    ///     Takes the message and propagates it to all the targets of this _Order_node
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to a new message.
    /// </param>
    /// <remarks>
    ///     This function packages its _M_index into a message and immediately sends it to the targets.
    /// </remarks>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<size_t> *)
    {
        _R_lock _Lock(_M_resetLock);

        if (this->_M_pSendMessage == nullptr)
        {
            this->_Create_send_message();
        }

        for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
        {
            ITarget<size_t> * _PTarget = *_Iter;
            _PTarget->propagate(this->_M_pSendMessage, this);
        }
    }

private:

    //
    //  Private Data Members
    //

    // The source where we have reserved a message
    ISource<_Type> * _M_pReservedSource;

    // The lock used to protect modification during a reset
    ::Concurrency::details::_ReentrantPPLLock _M_resetLock;

    // For non-greedy order nodes, the message ID of the message to reserve/consume
    runtime_object_identity _M_savedId;

    // For non-greedy order nodes, the reserved ID of the message that was reserved
    runtime_object_identity _M_reservedId;

    // The marker that indicates that _Non_greedy_node has reserved a message
    volatile bool _M_fIsInitialized;

private:
    //
    // Hide assignment operator and copy constructor
    //
    _Non_greedy_node const & operator=(_Non_greedy_node const &);  // no assignment operator
    _Non_greedy_node(_Non_greedy_node const &);                    // no copy constructor
};

//**************************************************************************
// Choice:
//**************************************************************************

/// <summary>
///     A <c>choice</c> messaging block is a multi-source, single-target block that represents a control-flow
///     interaction with a set of sources. The choice block will wait for any one of multiple sources to
///     produce a message and will propagate the index of the source that produced the message.
/// </summary>
/// <typeparam name="_Type">
///     A <c>tuple</c>-based type representing the payloads of the input sources.
/// </typeparam>
/// <remarks>
///     The choice block ensures that only one of the incoming messages is consumed.
///     <para>For more information, see <see cref="Asynchronous Message Blocks"/>.</para>
/// </remarks>
/// <seealso cref="join Class"/>
/// <seealso cref="single_assignment Class"/>
/// <seealso cref="make_choice Function"/>
/// <seealso cref="tuple Class"/>
/**/
template<class _Type>
class choice: public ISource<size_t>
{
public:

    /// <summary>
    ///     Constructs a <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_Tuple">
    ///     A <c>tuple</c> of sources for the choice.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    explicit choice(_Type _Tuple) : _M_sourceTuple(_Tuple), _M_pScheduler(nullptr), _M_pScheduleGroup(nullptr)
    {
        _M_pSingleAssignment = new single_assignment<size_t>();
        _Initialize_choices<0>();
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs a <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>choice</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Tuple">
    ///     A <c>tuple</c> of sources for the choice.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    choice(Scheduler& _PScheduler, _Type _Tuple) : _M_sourceTuple(_Tuple), _M_pScheduler(&_PScheduler), _M_pScheduleGroup(nullptr)
    {
        _M_pSingleAssignment = new single_assignment<size_t>(_PScheduler);
        _Initialize_choices<0>();
    }

    /// <summary>
    ///     Constructs a <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>choice</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Tuple">
    ///     A <c>tuple</c> of sources for the choice.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    choice(ScheduleGroup& _PScheduleGroup, _Type _Tuple) : _M_sourceTuple(_Tuple), _M_pScheduler(nullptr), _M_pScheduleGroup(&_PScheduleGroup)
    {
        _M_pSingleAssignment = new single_assignment<size_t>(_PScheduleGroup);
        _Initialize_choices<0>();
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Constructs a <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_Choice">
    ///     A <c>choice</c> messaging block to copy from.
    ///     Note that the original object is orphaned, making this a move constructor.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    choice(choice && _Choice)
    {
        // Copy scheduler group or scheduler to the new object.
        _M_pScheduleGroup = _Choice._M_pScheduleGroup;
        _M_pScheduler = _Choice._M_pScheduler;

        // Single assignment is heap allocated, so simply copy the pointer. If it already has
        // a value, it will be preserved.
        _M_pSingleAssignment = _Choice._M_pSingleAssignment;
        _Choice._M_pSingleAssignment = nullptr;

        // Invoke copy assignment for tuple to copy pointers to message blocks.
        _M_sourceTuple = _Choice._M_sourceTuple;

        // Copy the pointers to order nodes to a new object and zero out in the old object.
        memcpy(_M_pSourceChoices, _Choice._M_pSourceChoices, sizeof(_M_pSourceChoices));
        memset(_Choice._M_pSourceChoices, 0, sizeof(_M_pSourceChoices));
    }

    /// <summary>
    ///     Destroys the <c>choice</c> messaging block.
    /// </summary>
    /**/
    ~choice()
    {
        delete _M_pSingleAssignment;
        _Delete_choices<0>();
    }

    /// <summary>
    ///     A type alias for <typeparamref name="_Type"/>.
    /// </summary>
    /**/
    typedef _Type type;

    /// <summary>
    ///     Checks whether this <c>choice</c> messaging block has been initialized with a value yet.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the block has received a value, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool has_value() const
    {
        return _M_pSingleAssignment->has_value();
    }

    /// <summary>
    ///     Returns an index into the <c>tuple</c> representing the element selected by the
    ///     <c>choice</c> messaging block.
    /// </summary>
    /// <returns>
    ///     The message index.
    /// </returns>
    /// <remarks>
    ///      The message payload can be extracted using the <c>get</c> method.
    /// </remarks>
    /**/
    size_t index()
    {
        return _M_pSingleAssignment->value();
    }

    /// <summary>
    ///     Gets the message whose index was selected by the <c>choice</c> messaging block.
    /// </summary>
    /// <typeparam name="_Payload_type">
    ///     The type of the message payload.
    /// </typeparam>
    /// <returns>
    ///     The payload of the message.
    /// </returns>
    /// <remarks>
    ///     Because a <c>choice</c> messaging block can take inputs with different payload types, you must specify
    ///     the type of the payload at the point of retrieval. You can determine the type based on the result of
    ///     the <c>index</c> method.
    /// </remarks>
    /**/
    template <typename _Payload_type>
    _Payload_type const & value()
    {
        return static_cast<_Reserving_node<_Payload_type> *>(_M_pSourceChoices[_M_pSingleAssignment->value()])->value();
    }

    //
    // ISource public function implementations
    //

    /// <summary>
    ///     Links a target block to this <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to an <c>ITarget</c> block to link to this <c>choice</c> messaging block.
    /// </param>
    /**/
    virtual void link_target(_Inout_ ITarget<size_t> * _PTarget)
    {
        _M_pSingleAssignment->link_target(_PTarget);
    }

    /// <summary>
    ///     Unlinks a target block from this <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to an <c>ITarget</c> block to unlink from this <c>choice</c> messaging block.
    /// </param>
    /**/
    virtual void unlink_target(_Inout_ ITarget<size_t> * _PTarget)
    {
        _M_pSingleAssignment->unlink_target(_PTarget);
    }

    /// <summary>
    ///     Unlinks all targets from this <c>choice</c> messaging block.
    /// </summary>
    /// <remarks>
    ///     This method does not need to be called from the destructor because destructor for the internal
    ///     <c>single_assignment</c> block will unlink properly.
    /// </remarks>
    /**/
    virtual void unlink_targets()
    {
        _M_pSingleAssignment->unlink_targets();
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>choice</c> block, transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>accept</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<size_t> * accept(runtime_object_identity _MsgId, _Inout_ ITarget<size_t> * _PTarget)
    {
        return _M_pSingleAssignment->accept(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>reserve</c> method.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise. Reservations can fail
    ///     for many reasons, including: the message was already reserved or accepted by another target, the source could
    ///     deny reservations, and so forth.
    /// </returns>
    /// <remarks>
    ///     After you call <c>reserve</c>, if it succeeds, you must call either <c>consume</c> or <c>release</c>
    ///     in order to take or give up possession of the message, respectively.
    /// </remarks>
    /**/
    virtual bool reserve(runtime_object_identity _MsgId, _Inout_ ITarget<size_t> * _PTarget)
    {
        return _M_pSingleAssignment->reserve(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Consumes a message previously offered by this <c>choice</c> messaging block and successfully reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the reserved <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>consume</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     The <c>consume</c> method is similar to <c>accept</c>, but must always be preceded by a call to <c>reserve</c> that
    ///     returned <c>true</c>.
    /// </remarks>
    /**/
    virtual message<size_t> * consume(runtime_object_identity _MsgId, _Inout_ ITarget<size_t> * _PTarget)
    {
        return _M_pSingleAssignment->consume(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Releases a previous successful message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>release</c> method.
    /// </param>
    /**/
    virtual void release(runtime_object_identity _MsgId, _Inout_ ITarget<size_t> * _PTarget)
    {
        _M_pSingleAssignment->release(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Acquires a reference count on this <c>choice</c> messaging block, to prevent deletion.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being linked to this source
    ///     during the <c>link_target</c> method.
    /// </remarks>
    /**/
    virtual void acquire_ref(_Inout_ ITarget<size_t> * _PTarget)
    {
        _M_pSingleAssignment->acquire_ref(_PTarget);
    }

    /// <summary>
    ///     Releases a reference count on this <c>choice</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being unlinked from this source.
    ///     The source block is allowed to release any resources reserved for the target block.
    /// </remarks>
    /**/
    virtual void release_ref(_Inout_ ITarget<size_t> * _PTarget)
    {
        _M_pSingleAssignment->release_ref(_PTarget);
    }

private:
    template<int _Index>
    using _Reserving_node_source_type = _Reserving_node<typename ::std::remove_pointer_t<::std::tuple_element_t<_Index, _Type>>::source_type>;

    /// <summary>
    ///     Constructs and initializes a _Reserving_node for each tuple messaging block passed in.
    /// </summary>
    /// <typeparam>The highest-number index of the choice's sources</typeparam>
    /**/
    template<int _Index>
    void _Initialize_choices()
    {
        ::std::tuple_element_t<_Index, _Type> _Item = ::std::get<_Index>(_M_sourceTuple);
        _Reserving_node_source_type<_Index> * _Order_node_element = nullptr;

        if (_M_pScheduleGroup != nullptr)
        {
            _Order_node_element = new _Reserving_node_source_type<_Index>(*_M_pScheduleGroup, _Item, _Index);
        }
        else if (_M_pScheduler != nullptr)
        {
            _Order_node_element = new _Reserving_node_source_type<_Index>(*_M_pScheduler, _Item, _Index);
        }
        else
        {
            _Order_node_element = new _Reserving_node_source_type<_Index>(_Item, _Index);
        }

        _M_pSourceChoices[_Index] = _Order_node_element;
        _Order_node_element->link_target(_M_pSingleAssignment);
        _Initialize_choices<_Index + 1>();
    }

    /// <summary>
    ///     Provides a sentinel template specialization for _Initialize_choices recursive
    ///     template expansion.
    /// </summary>
    /**/
    template<> void _Initialize_choices<::std::tuple_size_v<_Type>>()
    {
    }

    /// <summary>
    ///     Deletes all _Reserving_node elements that were created in _Initialize_choices.
    /// </summary>
    /// <typeparam>The highest-number index of the choice's sources</typeparam>
    /**/
    template<int _Index>
    void _Delete_choices()
    {
        delete static_cast<_Reserving_node_source_type<_Index> *>(_M_pSourceChoices[_Index]);
        _M_pSourceChoices[_Index] = nullptr;
        _Delete_choices<_Index + 1>();
    }

    /// <summary>
    ///     Provides a sentinel template specialization for _Delete_choices recursive
    ///     template expansion.
    /// </summary>
    /**/
    template<> void _Delete_choices<::std::tuple_size_v<_Type>>()
    {
    }

    // Array of pointers to _Reserving_node elements representing each source
    void * _M_pSourceChoices[::std::tuple_size_v<_Type>];

    // Single assignment which chooses between source messaging blocks
    single_assignment<size_t> * _M_pSingleAssignment;

    // Tuple of messaging blocks that are sources to this choice
    _Type _M_sourceTuple;

    // The scheduler to propagate messages on
    Scheduler * _M_pScheduler;

    // The schedule group to propagate messages on
    ScheduleGroup * _M_pScheduleGroup;

private:
    //
    // Hide assignment operator
    //
    choice const &operator =(choice const &);                      // no assignment operator
    choice(choice const &);                                        // no copy constructor
};

// Templated factory functions that create a choice, three flavors

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
/// <summary>
///     Constructs a <c>choice</c> messaging block from an optional <c>Scheduler</c> or <c>ScheduleGroup</c>
///     and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_PScheduler">
///     The <c>Scheduler</c> object within which the propagation task for the <c>choice</c> messaging block is scheduled.
/// </param>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>choice</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="choice Class"/>
/// <seealso cref="Scheduler Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
choice<::std::tuple<_Type1, _Type2, _Types...>>
make_choice(Scheduler& _PScheduler, _Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return choice<::std::tuple<_Type1, _Type2, _Types...>>(_PScheduler, ::std::make_tuple(_Item1, _Item2, _Items...));
}

/// <summary>
///     Constructs a <c>choice</c> messaging block from an optional <c>Scheduler</c> or <c>ScheduleGroup</c>
///     and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_PScheduleGroup">
///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>choice</c> messaging block is scheduled.
///     The <c>Scheduler</c> object used is implied by the schedule group.
/// </param>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>choice</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="choice Class"/>
/// <seealso cref="ScheduleGroup Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
choice<::std::tuple<_Type1, _Type2, _Types...>>
make_choice(ScheduleGroup& _PScheduleGroup, _Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return choice<::std::tuple<_Type1, _Type2, _Types...>>(_PScheduleGroup, ::std::make_tuple(_Item1, _Item2, _Items...));
}

#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

/// <summary>
///     Constructs a <c>choice</c> messaging block from an optional <c>Scheduler</c> or <c>ScheduleGroup</c>
///     and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>choice</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="choice Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
choice<::std::tuple<_Type1, _Type2, _Types...>>
make_choice(_Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return choice<::std::tuple<_Type1, _Type2, _Types...>>(::std::make_tuple(_Item1, _Item2, _Items...));
}

//**************************************************************************
// Join:
//**************************************************************************

// Template specialization used to unwrap the types from within a tuple.

/**/
template <typename _Tuple> struct _Unwrap;

/// <summary>
///     Template specialization used to unwrap the types from within a tuple.
/// </summary>
/// <typeparam name="_Types">
///     The types of the elements of the tuple.
/// </typeparam>
/**/
template <typename... _Types>
struct _Unwrap<::std::tuple<_Types...>>
{
    typedef ::std::tuple<typename ::std::remove_pointer_t<_Types>::source_type...> type;
};

/// <summary>
///     Defines a block allowing sources of distinct types to be joined.
///     Join node is a single-target, multi-source ordered propagator block
/// </summary>
/// <typeparam name="_Type">
///     The payload tuple type
/// </typeparam>
/// <typeparam name="_Jtype">
///     The kind of join this is, either 'greedy' or 'non-greedy'
/// </typeparam>
/**/
template<typename _Type, typename _Destination_type, join_type _Jtype>
class _Join_node: public propagator_block<single_link_registry<ITarget<_Destination_type>>, multi_link_registry<ISource<size_t>>>
{
private:
    typedef single_link_registry<ITarget<_Destination_type>> _TargetLinkRegistry;
    typedef multi_link_registry<ISource<size_t>> _SourceLinkRegistry;

public:
    using typename source_block<_TargetLinkRegistry>::_Target_type;
    using typename source_block<_TargetLinkRegistry>::target_iterator;
    using typename propagator_block<_TargetLinkRegistry, _SourceLinkRegistry>::source_iterator;

    /// <summary>
    ///     Constructs a join within the default scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /**/
    _Join_node() : _M_counter(::std::tuple_size_v<_Destination_type>)
    {
        this->initialize_source_and_target();
    }

    /// <summary>
    ///     Constructs a join within the specified scheduler, and places it on any schedule
    ///     group of the scheduler's choosing.
    /// </summary>
    /// <param name="_PScheduler">
    ///     A reference to a scheduler instance.
    /// </param>
    /**/
    _Join_node(Scheduler& _PScheduler) : _M_counter(::std::tuple_size_v<_Destination_type>)
    {
        this->initialize_source_and_target(&_PScheduler);
    }

    /// <summary>
    ///     Constructs a join within the specified schedule group.  The scheduler is implied
    ///     by the schedule group.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     A reference to a schedule group.
    /// </param>
    /**/
    _Join_node(ScheduleGroup& _PScheduleGroup) : _M_counter(::std::tuple_size_v<_Destination_type>)
    {
        this->initialize_source_and_target(nullptr, &_PScheduleGroup);
    }

    /// <summary>
    ///     Cleans up any resources that may have been created by the join.
    /// </summary>
    /**/
    ~_Join_node()
    {
        // Remove all links
        this->remove_network_links();

        // Clean up any messages left in this message block
        _Delete_stored_messages();
    }

protected:

    /// <summary>
    ///     Asynchronously passes a message from an <c>ISource</c> block to this <c>ITarget</c> block. It is invoked
    ///     by the <c>propagate</c> method, when called by a source block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to the <c>message</c> object.
    /// </param>
    /// <param name="_PSource">
    ///     A pointer to the source block offering the message.
    /// </param>
    /// <returns>
    ///     A <see cref="message_status Enumeration">message_status</see> indication of what
    ///     the target decided to do with the message.
    /// </returns>
    /**/
    virtual message_status propagate_message(message<size_t> * _PMessage, ISource<size_t> * _PSource)
    {
        // This join block is connected to the _Order_node sources, which know not to send
        // any more messages until join propagates them further. That is why join can
        // always accept the incoming messages.

        _PMessage = _PSource->accept(_PMessage->msg_id(), this);

        //
        // Source block created an int message only to notify join that the real
        // payload is available. There is no need to keep this message around.
        //
        _CONCRT_ASSERT(_PMessage != nullptr);
        delete _PMessage;

        long _Ret_val = _InterlockedDecrement(&_M_counter);

        _CONCRT_ASSERT(_Ret_val >= 0);

        if (_Ret_val == 0)
        {
            //
            // All source messages are now received so join can propagate them further
            //
            this->async_send(nullptr);
        }

        return accepted;
    }

    /// <summary>
    ///     Accepts an offered message by the source, transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<_Destination_type> * accept_message(runtime_object_identity _MsgId)
    {
        //
        // Peek at the head message in the message buffer.  If the IDs match
        // dequeue and transfer ownership
        //
        message<_Destination_type> * _Msg = nullptr;

        if (_M_messageBuffer._Is_head(_MsgId))
        {
            _Msg = _M_messageBuffer._Dequeue();
        }

        return _Msg;
    }

    /// <summary>
    ///     Reserves a message previously offered by the source.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A bool indicating whether the reservation worked or not.
    /// </returns>
    /// <remarks>
    ///     After <c>reserve</c> is called, if it returns <c>true</c>, either <c>consume</c> or <c>release</c> must be called
    ///     to either take or release ownership of the message.
    /// </remarks>
    /**/
    virtual bool reserve_message(runtime_object_identity _MsgId)
    {
        // Allow reservation if this is the head message
        return _M_messageBuffer._Is_head(_MsgId);
    }

    /// <summary>
    ///     Consumes a message previously offered by the source and reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     <c>consume_message</c> is similar to <c>accept</c>, but is always preceded by a call to <c>reserve</c>.
    /// </remarks>
    /**/
    virtual message<_Destination_type> * consume_message(runtime_object_identity _MsgId)
    {
        // By default, accept the message
        return accept_message(_MsgId);
    }

    /// <summary>
    ///     Releases a previous message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The runtime object identity of the message.
    /// </param>
    /**/
    virtual void release_message(runtime_object_identity _MsgId)
    {
        // The head message is the one reserved.
        if (!_M_messageBuffer._Is_head(_MsgId))
        {
            throw message_not_found();
        }
    }

    /// <summary>
    ///     Resumes propagation after a reservation has been released
    /// </summary>
    /**/
    virtual void resume_propagation()
    {
        // If there are any messages in the buffer, propagate them out
        if (_M_messageBuffer._Count() > 0)
        {
            this->async_send(nullptr);
        }
    }

    /// <summary>
    ///     Notification that a target was linked to this source.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the newly linked target.
    /// </param>
    /**/
    virtual void link_target_notification(_Inout_ ITarget<_Destination_type> *)
    {
        // There is only a single target.
        _Propagate_priority_order(_M_messageBuffer);
    }

    /// <summary>
    ///     Takes the message and propagates it to all the targets of this <c>join</c> block.
    /// </summary>
    /// <param name="_PMessage">
    ///     A pointer to a new message.
    /// </param>
    /// <remarks>
    ///     This function packages source payloads into a tuple message and immediately sends it to the targets.
    /// </remarks>
    /**/
    virtual void propagate_to_any_targets(_Inout_opt_ message<_Destination_type> *)
    {
        message<_Destination_type> * _Msg = nullptr;

        if (_M_counter == 0)
        {
            bool fIsNonGreedy = (_Jtype == non_greedy);

            if (fIsNonGreedy)
            {
                if (!_Non_greedy_acquire_messages())
                {
                    return;
                }
            }

            if (!fIsNonGreedy)
            {
                // Because a greedy join has captured all input, we can reset
                // the counter to the total number of inputs
                _InterlockedExchange(&_M_counter, ::std::tuple_size_v<_Destination_type>);
            }

            _Msg = _Create_send_message();
        }

        if (_Msg != nullptr)
        {
            _M_messageBuffer._Enqueue(_Msg);

            if (!_M_messageBuffer._Is_head(_Msg->msg_id()))
            {
                // another message is at the head of the outbound message queue and blocked
                // simply return
                return;
            }
        }

        _Propagate_priority_order(_M_messageBuffer);
    }

private:

    /// <summary>
    ///     Tries to reserve from all sources.  If successful, it will consume all the messages
    /// </summary>
    /// <returns>
    ///     A bool indicating whether the consumption attempt worked.
    /// </returns>
    /// <typeparam name="_Index">
    ///     The highest-number index of the join's sources
    /// </typeparam>
    /**/
    template<int _Index>
    bool _Try_consume_source_messages(_Destination_type & _Destination_tuple, ISource<size_t> ** _Sources)
    {
        typedef _Non_greedy_node<typename ::std::remove_pointer_t<::std::tuple_element_t<_Index, _Type>>::source_type> _Non_greedy_node_source_type;
        _Non_greedy_node_source_type * _Node = static_cast<_Non_greedy_node_source_type *>(_Sources[_Index]);

        // Increment the counter once for each reservation
        _InterlockedIncrement(&_M_counter);

        if (_Node->_Reserve_received_message())
        {
            bool _Ret_val = _Try_consume_source_messages<_Index + 1>(_Destination_tuple, _Sources);

            if (_Ret_val)
            {
                _Node->_Consume_received_message();
            }
            else
            {
                if (_Node->_Release_received_message())
                {
                    // If _Release_received_message() restored the ID, decrement the count for that
                    // restoration
                    if (_InterlockedDecrement(&_M_counter) == 0)
                    {
                        this->async_send(nullptr);
                    }
                }
            }

            return _Ret_val;
        }

        return false;
    }

    /// <summary>
    ///     Provides a sentinel template specialization for _Try_consume_source_messages recursive
    ///     template expansion.
    /// </summary>
    /// <returns>
    ///     A bool indicating whether the consumption attempt worked.
    /// </returns>
    /**/
    template<> bool _Try_consume_source_messages<::std::tuple_size_v<_Type>>(_Destination_type &, ISource<size_t> **)
    {
        return true;
    }

    /// <summary>
    ///     Tries to acquire all of the messages from the _Non_greedy_nodes.  Each node has already
    ///     indicated that it has received a message that it can try to reserve.  This function
    ///     starts the reservation and consume process.
    /// </summary>
    /// <returns>
    ///     A bool indicating whether the reserve/consume of all messages succeeded.
    /// </returns>
    /**/
    bool _Non_greedy_acquire_messages()
    {
        _Destination_type _Destination_tuple;

        // Populate the sources buffer
        ISource<size_t> * _Sources[::std::tuple_size_v<_Type>];
        size_t _Index = 0;

        // Get an iterator which will keep a reference on the connected sources
        source_iterator _Iter = this->_M_connectedSources.begin();

        while (*_Iter != nullptr)
        {
            ISource<size_t> * _PSource = *_Iter;

            if (_PSource == nullptr)
            {
                // One of the sources disconnected
                break;
            }

            if (_Index >= ::std::tuple_size_v<_Type>)
            {
                // More sources that we expect
                break;
            }

            _Sources[_Index] = _PSource;
            _Index++;
            ++_Iter;
        }

        // The order nodes should not have unlinked while the join node is
        // active.

        if (_Index != ::std::tuple_size_v<_Type>)
        {
            // On debug build assert to help debugging
            _CONCRT_ASSERT(_Index == ::std::tuple_size_v<_Type>);
            return false;
        }

        bool _IsAcquireSuccessful = _Try_consume_source_messages<0>(_Destination_tuple, _Sources);

        return _IsAcquireSuccessful;
    }

    /// <summary>
    ///     Propagate messages in priority order
    /// </summary>
    /// <param name="_MessageBuffer">
    ///     Reference to a message queue with messages to be propagated
    /// </param>
    /**/
    void _Propagate_priority_order(::Concurrency::details::_Queue<message<_Target_type>> & _MessageBuffer)
    {
        message<_Target_type> * _Msg = _MessageBuffer._Peek();

        // If someone has reserved the _Head message, don't propagate anymore
        if (this->_M_pReservedFor != nullptr)
        {
            return;
        }

        while (_Msg != nullptr)
        {
            message_status _Status = declined;

            // Always start from the first target that linked
            for (target_iterator _Iter = this->_M_connectedTargets.begin(); *_Iter != nullptr; ++_Iter)
            {
                ITarget<_Target_type> * _PTarget = *_Iter;
                _Status = _PTarget->propagate(_Msg, this);

                // Ownership of message changed. Do not propagate this
                // message to any other target.
                if (_Status == accepted)
                {
                    break;
                }

                // If the target just propagated to reserved this message, stop
                // propagating it to others
                if (this->_M_pReservedFor != nullptr)
                {
                    break;
                }
            }

            // If status is anything other than accepted, then the head message
            // was not propagated out.  Thus, nothing after it in the queue can
            // be propagated out.  Cease propagation.
            if (_Status != accepted)
            {
                break;
            }

            // Get the next message
            _Msg = _MessageBuffer._Peek();
        }
    }

    /// <summary>
    ///     Called when all the source messaging blocks have received their messages. The payloads are copied
    ///     into local tuple and then packaged into a message to be propagated: _M_pSendMessage.
    /// </summary>
    /**/
    message<_Destination_type> * _Create_send_message()
    {
        _Destination_type _Destination_tuple;

        // Populate the sources buffer
        ISource<size_t> * _Sources[::std::tuple_size_v<_Type>];
        size_t _Index = 0;

        // Get an iterator which will keep a reference on the connected sources
        source_iterator _Iter = this->_M_connectedSources.begin();

        while (*_Iter != nullptr)
        {
            ISource<size_t> * _PSource = *_Iter;

            if (_PSource == nullptr)
            {
                // One of the sources disconnected
                break;
            }

            // Avoid buffer overrun
            if (_Index >= ::std::tuple_size_v<_Type>)
            {
                // More sources that we expect
                break;
            }

            _Sources[_Index] = *_Iter;
            _Index++;
            ++_Iter;
        }

        // The order nodes should not have unlinked while the join node is
        // active.
        if (_Index != ::std::tuple_size_v<_Type>)
        {
            // On debug build assert to help debugging
            _CONCRT_ASSERT(_Index == ::std::tuple_size_v<_Type>);
            return nullptr;
        }

        _Populate_destination_tuple<0>(_Destination_tuple, _Sources);

        return new message<_Destination_type>(_Destination_tuple);
    }

    /// <summary>
    ///     Deletes all messages currently stored in this message block.  Should be called
    ///     by the destructor to ensure any messages propagated in are cleaned up.
    /// </summary>
    /**/
    void _Delete_stored_messages()
    {
        // Delete any messages remaining in the output queue
        for (;;)
        {
            message<_Destination_type> * _Msg = _M_messageBuffer._Dequeue();
            if (_Msg == nullptr)
            {
                break;
            }
            delete _Msg;
        }
    }

    /// <summary>
    ///     Copies payloads from all sources to destination tuple.
    /// </summary>
    /**/
    template<int _Index>
    void _Populate_destination_tuple(_Destination_type & _Destination_tuple, ISource<size_t> ** _Sources)
    {
        typedef _Order_node_base<typename ::std::remove_pointer_t<::std::tuple_element_t<_Index, _Type>>::source_type> _Order_node_base_source_type;
        _Order_node_base_source_type * _Node = static_cast<_Order_node_base_source_type *>(_Sources[_Index]);

        ::std::get<_Index>(_Destination_tuple) = _Node->value();
        _Node->_Reset();

        _Populate_destination_tuple<_Index + 1>(_Destination_tuple, _Sources);
    }

    /// <summary>
    ///     Provides a sentinel template specialization for _Populate_destination_tuple recursive
    ///     template expansion.
    /// </summary>
    /**/
    template<> void _Populate_destination_tuple<::std::tuple_size_v<_Type>>(_Destination_type &, ISource<size_t> **)
    {
    }

    // A tuple containing a collection of source messaging blocks
    _Type _M_sourceTuple;

    // Counts messages received by sources of this node and is used to trigger propagation to targets
    // This value starts at the total number of inputs and counts down to zero.  When it reaches zero,
    // a join of the inputs is started.
    volatile long _M_counter;

    // Buffer to hold outgoing messages
    ::Concurrency::details::_Queue<message<_Destination_type>> _M_messageBuffer;

private:
    //
    // Hide assignment operator and copy constructor
    //
    _Join_node(const _Join_node & _Join);                                  // no copy constructor
    _Join_node const &operator =(_Join_node const &);                      // no assignment operator
};

/// <summary>
///     A <c>multitype_join</c> messaging block is a multi-source, single-target messaging block that
///     combines together messages of different types from each of its sources and offers a tuple
///     of the combined messages to its targets.
/// </summary>
/// <typeparam name="_Type">
///     The <c>tuple</c> payload type of the messages joined and propagated by the block.
/// </typeparam>
/// <typeparam name="_Jtype">
///     The kind of <c>join</c> block this is, either <c>greedy</c> or <c>non_greedy</c>
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Asynchronous Message Blocks"/>.
/// </remarks>
/// <seealso cref="choice Class"/>
/// <seealso cref="join Class"/>
/// <seealso cref="join_type Enumeration"/>
/// <seealso cref="make_join Function"/>
/// <seealso cref="make_greedy_join Function"/>
/// <seealso cref="tuple Class"/>
/**/
template<typename _Type, join_type _Jtype = non_greedy>
class multitype_join: public ISource<typename _Unwrap<_Type>::type>
{
public:

    typedef typename _Unwrap<_Type>::type _Destination_type;

    /// <summary>
    ///     Constructs a <c>multitype_join</c> messaging block.
    /// </summary>
    /// <param name="_Tuple">
    ///     A <c>tuple</c> of sources for this <c>multitype_join</c> messaging block.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    explicit multitype_join(_Type _Tuple) : _M_sourceTuple(_Tuple), _M_pScheduler(nullptr), _M_pScheduleGroup(nullptr)
    {
        _M_pJoinNode = new _Join_node<_Type, _Destination_type, _Jtype>();
        _Initialize_joins<0>();
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs a <c>multitype_join</c> messaging block.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the propagation task for the <c>multitype_join</c> messaging block is scheduled.
    /// </param>
    /// <param name="_Tuple">
    ///     A <c>tuple</c> of sources for this <c>multitype_join</c> messaging block.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    multitype_join(Scheduler& _PScheduler, _Type _Tuple) : _M_sourceTuple(_Tuple), _M_pScheduler(&_PScheduler), _M_pScheduleGroup(nullptr)
    {
        _M_pJoinNode = new _Join_node<_Type, _Destination_type, _Jtype>(_PScheduler);
        _Initialize_joins<0>();
    }

    /// <summary>
    ///     Constructs a <c>multitype_join</c> messaging block.
    /// </summary>
    /// <param name="_PScheduleGroup">
    ///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>multitype_join</c> messaging block is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <param name="_Tuple">
    ///     A <c>tuple</c> of sources for this <c>multitype_join</c> messaging block.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    multitype_join(ScheduleGroup& _PScheduleGroup, _Type _Tuple) : _M_sourceTuple(_Tuple), _M_pScheduler(nullptr), _M_pScheduleGroup(&_PScheduleGroup)
    {
        _M_pJoinNode = new _Join_node<_Type, _Destination_type, _Jtype>(_PScheduleGroup);
        _Initialize_joins<0>();
    }
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Constructs a <c>multitype_join</c> messaging block.
    /// </summary>
    /// <param name="_Join">
    ///     A <c>multitype_join</c> messaging block to copy from.
    ///     Note that the original object is orphaned, making this a move constructor.
    /// </param>
    /// <remarks>
    ///     <para>
    ///         The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///         or <paramref name="_PScheduleGroup"/> parameters.
    ///     </para>
    ///     <para>
    ///         Move construction is not performed under a lock, which means that it is up to the user
    ///         to make sure that there are no light-weight tasks in flight at the time of moving.
    ///         Otherwise, numerous races can occur, leading to exceptions or inconsistent state.
    ///     </para>
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    multitype_join(multitype_join && _Join)
    {
        // Copy scheduler group or scheduler to the new object.
        _M_pScheduleGroup = _Join._M_pScheduleGroup;
        _M_pScheduler = _Join._M_pScheduler;

        // Single assignment is heap allocated, so simply copy the pointer. If it already has
        // a value, it will be preserved.
        _M_pJoinNode = _Join._M_pJoinNode;
        _Join._M_pJoinNode = nullptr;

        // Invoke copy assignment for tuple to copy pointers to message blocks.
        _M_sourceTuple = _Join._M_sourceTuple;

        // Copy the pointers to order nodes to a new object and zero out in the old object.
        memcpy(_M_pSourceJoins, _Join._M_pSourceJoins, sizeof(_M_pSourceJoins));
        memset(_Join._M_pSourceJoins, 0, sizeof(_M_pSourceJoins));
    }

    /// <summary>
    ///     Destroys the <c>multitype_join</c> messaging block.
    /// </summary>
    /**/
    ~multitype_join()
    {
        delete _M_pJoinNode;
        _Delete_joins<0>();
    }

    /// <summary>
    ///     A type alias for <typeparamref name="_Type"/>.
    /// </summary>
    /**/
    typedef _Type type;

    //
    // ISource public function implementations
    //

    /// <summary>
    ///     Links a target block to this <c>multitype_join</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to an <c>ITarget</c> block to link to this <c>multitype_join</c> messaging block.
    /// </param>
    /**/
    virtual void link_target(_Inout_ ITarget<_Destination_type> * _PTarget)
    {
        _M_pJoinNode->link_target(_PTarget);
    }

    /// <summary>
    ///     Unlinks a target block from this <c>multitype_join</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to an <c>ITarget</c> block to unlink from this <c>multitype_join</c> messaging block.
    /// </param>
    /**/
    virtual void unlink_target(_Inout_ ITarget<_Destination_type> * _PTarget)
    {
        _M_pJoinNode->unlink_target(_PTarget);
    }

    /// <summary>
    ///     Unlinks all targets from this <c>multitype_join</c> messaging block.
    /// </summary>
    /**/
    virtual void unlink_targets()
    {
        _M_pJoinNode->unlink_targets();
    }

    /// <summary>
    ///     Accepts a message that was offered by this <c>multitype_join</c> block, transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the offered <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>accept</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the message that the caller now has ownership of.
    /// </returns>
    /**/
    virtual message<_Destination_type> * accept(runtime_object_identity _MsgId, _Inout_ ITarget<_Destination_type> * _PTarget)
    {
        return _M_pJoinNode->accept(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Reserves a message previously offered by this <c>multitype_join</c> messaging block.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being reserved.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>reserve</c> method.
    /// </param>
    /// <returns>
    ///     <c>true</c> if the message was successfully reserved, <c>false</c> otherwise. Reservations can fail
    ///     for many reasons, including: the message was already reserved or accepted by another target, the source could
    ///     deny reservations, and so forth.
    /// </returns>
    /// <remarks>
    ///     After you call <c>reserve</c>, if it succeeds, you must call either <c>consume</c> or <c>release</c>
    ///     in order to take or give up possession of the message, respectively.
    /// </remarks>
    /**/
    virtual bool reserve(runtime_object_identity _MsgId, _Inout_ ITarget<_Destination_type> * _PTarget)
    {
        return _M_pJoinNode->reserve(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Consumes a message previously offered by the <c>multitype_join</c> messaging block and successfully reserved by the target,
    ///     transferring ownership to the caller.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the reserved <c>message</c> object.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>consume</c> method.
    /// </param>
    /// <returns>
    ///     A pointer to the <c>message</c> object that the caller now has ownership of.
    /// </returns>
    /// <remarks>
    ///     The <c>consume</c> method is similar to <c>accept</c>, but must always be preceded by a call to <c>reserve</c> that
    ///     returned <c>true</c>.
    /// </remarks>
    /**/
    virtual message<_Destination_type> * consume(runtime_object_identity _MsgId, _Inout_ ITarget<_Destination_type> * _PTarget)
    {
        return _M_pJoinNode->consume(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Releases a previous successful message reservation.
    /// </summary>
    /// <param name="_MsgId">
    ///     The <c>runtime_object_identity</c> of the <c>message</c> object being released.
    /// </param>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling the <c>release</c> method.
    /// </param>
    /**/
    virtual void release(runtime_object_identity _MsgId, _Inout_ ITarget<_Destination_type> * _PTarget)
    {
        _M_pJoinNode->release(_MsgId, _PTarget);
    }

    /// <summary>
    ///     Acquires a reference count on this <c>multitype_join</c> messaging block, to prevent deletion.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being linked to this source
    ///     during the <c>link_target</c> method.
    /// </remarks>
    /**/
    virtual void acquire_ref(_Inout_ ITarget<_Destination_type> * _PTarget)
    {
        _M_pJoinNode->acquire_ref(_PTarget);
    }

    /// <summary>
    ///     Releases a reference count on this <c>multiple_join</c> messaging block.
    /// </summary>
    /// <param name="_PTarget">
    ///     A pointer to the target block that is calling this method.
    /// </param>
    /// <remarks>
    ///     This method is called by an <c>ITarget</c> object that is being unlinked from this source.
    ///     The source block is allowed to release any resources reserved for the target block.
    /// </remarks>
    /**/
    virtual void release_ref(_Inout_ ITarget<_Destination_type> * _PTarget)
    {
        _M_pJoinNode->release_ref(_PTarget);
    }

private:
    template<int _Index>
    using _Source_type = typename ::std::remove_pointer_t<::std::tuple_element_t<_Index, _Type>>::source_type;

    template<int _Index>
    using _Order_node_base_source_type = _Order_node_base<_Source_type<_Index>>;

    /// <summary>
    ///     Constructs and initializes a _Order_node for each tuple messaging block passed in.
    /// </summary>
    /// <typeparam name="_Index">
    ///     The highest-number index of the multitype_join's sources
    /// </typeparam>
    /**/
    template<int _Index>
    void _Initialize_joins()
    {
        ::std::tuple_element_t<_Index, _Type> _Item = ::std::get<_Index>(_M_sourceTuple);
        _Order_node_base_source_type<_Index> * _Order_node_element = nullptr;

        if (_Jtype == non_greedy)
        {
            typedef _Non_greedy_node<_Source_type<_Index>> _Non_greedy_node_source_type;

            if (_M_pScheduleGroup != nullptr)
            {
                _Order_node_element = new _Non_greedy_node_source_type(*_M_pScheduleGroup, _Item, _Index);
            }
            else if (_M_pScheduler != nullptr)
            {
                _Order_node_element = new _Non_greedy_node_source_type(*_M_pScheduler, _Item, _Index);
            }
            else
            {
                _Order_node_element = new _Non_greedy_node_source_type(_Item, _Index);
            }
        }
        else
        {
            typedef _Greedy_node<_Source_type<_Index>> _Greedy_node_source_type;

            if (_M_pScheduleGroup != nullptr)
            {
                _Order_node_element = new _Greedy_node_source_type(*_M_pScheduleGroup, _Item, _Index);
            }
            else if (_M_pScheduler != nullptr)
            {
                _Order_node_element = new _Greedy_node_source_type(*_M_pScheduler, _Item, _Index);
            }
            else
            {
                _Order_node_element = new _Greedy_node_source_type(_Item, _Index);
            }
        }
        _M_pSourceJoins[_Index] = _Order_node_element;
        _Order_node_element->link_target(_M_pJoinNode);
        _Initialize_joins<_Index + 1>();
    }

    /// <summary>
    ///     Provides a sentinel template specialization for _Initialize_joins recursive
    ///     template expansion.
    /// </summary>
    /**/
    template<> void _Initialize_joins<::std::tuple_size_v<_Type>>()
    {
    }

    /// <summary>
    ///     Deletes all _Order_node elements that were created in _Initialize_joins.
    /// </summary>
    /// <typeparam name="_Index">
    ///     The highest-number index of the multitype_join's sources
    /// </typeparam>
    /**/
    template<int _Index>
    void _Delete_joins()
    {
        delete static_cast<_Order_node_base_source_type<_Index> *>(_M_pSourceJoins[_Index]);
        _M_pSourceJoins[_Index] = nullptr;
        _Delete_joins<_Index + 1>();
    }

    /// <summary>
    ///     Provides a sentinel template specialization for _Delete_joins recursive
    ///     template expansion.
    /// </summary>
    /**/
    template<> void _Delete_joins<::std::tuple_size_v<_Type>>()
    {
    }

    // Array of pointers to _Order_node elements representing each source
    void * _M_pSourceJoins[::std::tuple_size_v<_Type>];

    // Join node that collects source messaging block messages
    _Join_node<_Type, _Destination_type, _Jtype> * _M_pJoinNode;

    // Tuple of messaging blocks that are sources to this multitype_join
    _Type _M_sourceTuple;

    // The scheduler to propagate messages on
    Scheduler * _M_pScheduler;

    // The schedule group to propagate messages on
    ScheduleGroup * _M_pScheduleGroup;

private:
    //
    // Hide assignment operator
    //
    multitype_join const &operator =(multitype_join const &);                      // no assignment operator
    multitype_join(multitype_join const &);                                        // no copy constructor
};

// Templated factory functions that create a join, three flavors

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
/// <summary>
///     Constructs a <c>non_greedy multitype_join</c> messaging block from an optional <c>Scheduler</c>
///     or <c>ScheduleGroup</c> and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_PScheduler">
///     The <c>Scheduler</c> object within which the propagation task for the <c>multitype_join</c> messaging block is scheduled.
/// </param>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>non_greedy multitype_join</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="multitype_join Class"/>
/// <seealso cref="Scheduler Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
multitype_join<::std::tuple<_Type1, _Type2, _Types...>>
make_join(Scheduler& _PScheduler, _Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return multitype_join<::std::tuple<_Type1, _Type2, _Types...>>(_PScheduler, ::std::make_tuple(_Item1, _Item2, _Items...));
}

/// <summary>
///     Constructs a <c>non_greedy multitype_join</c> messaging block from an optional <c>Scheduler</c>
///     or <c>ScheduleGroup</c> and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_PScheduleGroup">
///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>multitype_join</c> messaging block is scheduled.
///     The <c>Scheduler</c> object used is implied by the schedule group.
/// </param>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>non_greedy multitype_join</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="multitype_join Class"/>
/// <seealso cref="ScheduleGroup Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
multitype_join<::std::tuple<_Type1, _Type2, _Types...>>
make_join(ScheduleGroup& _PScheduleGroup, _Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return multitype_join<::std::tuple<_Type1, _Type2, _Types...>>(_PScheduleGroup, ::std::make_tuple(_Item1, _Item2, _Items...));
}

#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

/// <summary>
///     Constructs a <c>non_greedy multitype_join</c> messaging block from an optional <c>Scheduler</c>
///     or <c>ScheduleGroup</c> and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>non_greedy multitype_join</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="multitype_join Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
multitype_join<::std::tuple<_Type1, _Type2, _Types...>>
make_join(_Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return multitype_join<::std::tuple<_Type1, _Type2, _Types...>>(::std::make_tuple(_Item1, _Item2, _Items...));
}

// Templated factory functions that create a *greedy* join, three flavors
#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
/// <summary>
///     Constructs a <c>greedy multitype_join</c> messaging block from an optional <c>Scheduler</c>
///     or <c>ScheduleGroup</c> and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_PScheduler">
///     The <c>Scheduler</c> object within which the propagation task for the <c>multitype_join</c> messaging block
///     is scheduled.
/// </param>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>greedy multitype_join</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="multitype_join Class"/>
/// <seealso cref="Scheduler Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
multitype_join<::std::tuple<_Type1, _Type2, _Types...>, greedy>
make_greedy_join(Scheduler& _PScheduler, _Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return multitype_join<::std::tuple<_Type1, _Type2, _Types...>, greedy>(_PScheduler, ::std::make_tuple(_Item1, _Item2, _Items...));
}

/// <summary>
///     Constructs a <c>greedy multitype_join</c> messaging block from an optional <c>Scheduler</c>
///     or <c>ScheduleGroup</c> and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_PScheduleGroup">
///     The <c>ScheduleGroup</c> object within which the propagation task for the <c>multitype_join</c> messaging block is scheduled.
///     The <c>Scheduler</c> object used is implied by the schedule group.
/// </param>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>greedy multitype_join</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="multitype_join Class"/>
/// <seealso cref="ScheduleGroup Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
multitype_join<::std::tuple<_Type1, _Type2, _Types...>, greedy>
make_greedy_join(ScheduleGroup& _PScheduleGroup, _Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return multitype_join<::std::tuple<_Type1, _Type2, _Types...>, greedy>(_PScheduleGroup, ::std::make_tuple(_Item1, _Item2, _Items...));
}

#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

/// <summary>
///     Constructs a <c>greedy multitype_join</c> messaging block from an optional <c>Scheduler</c>
///     or <c>ScheduleGroup</c> and two or more input sources.
/// </summary>
/// <typeparam name="_Type1">
///     The message block type of the first source.
/// </typeparam>
/// <typeparam name="_Type2">
///     The message block type of the second source.
/// </typeparam>
/// <typeparam name="_Types">
///     The message block types of additional sources.
/// </typeparam>
/// <param name="_Item1">
///     The first source.
/// </param>
/// <param name="_Item2">
///     The second source.
/// </param>
/// <param name="_Items">
///     Additional sources.
/// </param>
/// <returns>
///     A <c>greedy multitype_join</c> message block with two or more input sources.
/// </returns>
/// <seealso cref="multitype_join Class"/>
/**/
template<typename _Type1, typename _Type2, typename... _Types>
multitype_join<::std::tuple<_Type1, _Type2, _Types...>, greedy>
make_greedy_join(_Type1 _Item1, _Type2 _Item2, _Types... _Items)
{
    return multitype_join<::std::tuple<_Type1, _Type2, _Types...>, greedy>(::std::make_tuple(_Item1, _Item2, _Items...));
}

//**************************************************************************
// Agents:
//**************************************************************************

/// <summary>
///     The valid states for an <c>agent</c>.
/// </summary>
/// <remarks>
///     For more information, see <see cref="Asynchronous Agents"/>.
/// </remarks>
/**/
enum agent_status {
    /// <summary>
    ///     The <c>agent</c> has been created but not started.
    /// </summary>
    /**/
    agent_created,
    /// <summary>
    ///     The <c>agent</c> has been started, but not entered its <c>run</c> method.
    /// </summary>
    /**/
    agent_runnable,
    /// <summary>
    ///     The <c>agent</c> has started.
    /// </summary>
    /**/
    agent_started,
    /// <summary>
    ///     The <c>agent</c> finished without being canceled.
    /// </summary>
    /**/
    agent_done,
    /// <summary>
    ///     The <c>agent</c> was canceled.
    /// </summary>
    /**/
    agent_canceled
};

/// <summary>
///     A class intended to be used as a base class for all independent agents. It is used to hide
///     state from other agents and interact using message-passing.
/// </summary>
/// <remarks>
///     For more information, see <see cref="Asynchronous Agents"/>.
/// </remarks>
/**/
class agent
{
public:
    /// <summary>
    ///     Constructs an agent.
    /// </summary>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PGroup"/> parameters.
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    _CONCRTIMP agent();

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    /// <summary>
    ///     Constructs an agent.
    /// </summary>
    /// <param name="_PScheduler">
    ///     The <c>Scheduler</c> object within which the execution task of the agent is scheduled.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PGroup"/> parameters.
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    _CONCRTIMP agent(Scheduler& _PScheduler);

    /// <summary>
    ///     Constructs an agent.
    /// </summary>
    /// <param name="_PGroup">
    ///     The <c>ScheduleGroup</c> object within which the execution task of the agent is scheduled.
    ///     The <c>Scheduler</c> object used is implied by the schedule group.
    /// </param>
    /// <remarks>
    ///     The runtime uses the default scheduler if you do not specify the <paramref name="_PScheduler"/>
    ///     or <paramref name="_PGroup"/> parameters.
    /// </remarks>
    /// <seealso cref="Scheduler Class"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    _CONCRTIMP agent(ScheduleGroup& _PGroup);
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Destroys the agent.
    /// </summary>
    /// <remarks>
    ///     It is an error to destroy an agent that is not in a terminal state (either <c>agent_done</c> or
    ///     <c>agent_canceled</c>). This can be avoided by waiting for the agent to reach a terminal state
    ///     in the destructor of a class that inherits from the <c>agent</c> class.
    /// </remarks>
    /**/
    _CONCRTIMP virtual ~agent();

    /// <summary>
    ///     An asynchronous source of status information from the agent.
    /// </summary>
    /// <returns>
    ///     Returns a message source that can send messages about the current state of the agent.
    /// </returns>
    /**/
    _CONCRTIMP ISource<agent_status> * status_port();

    /// <summary>
    ///     A synchronous source of status information from the agent.
    /// </summary>
    /// <returns>
    ///     Returns the current state of the agent.  Note that this returned state could change
    ///     immediately after being returned.
    /// </returns>
    /// <seealso cref="agent_status Enumeration"/>
    /**/
    _CONCRTIMP agent_status status();

    /// <summary>
    ///     Moves an agent from the <c>agent_created</c> state to the <c>agent_runnable</c> state, and schedules it for execution.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the agent started correctly, <c>false</c> otherwise. An agent that has been canceled cannot be started.
    /// </returns>
    /// <seealso cref="agent_status Enumeration"/>
    /**/
    _CONCRTIMP bool start();

    /// <summary>
    ///     Moves an agent from either the <c>agent_created</c> or <c>agent_runnable</c> states to the <c>agent_canceled</c> state.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the agent was canceled, <c>false</c> otherwise. An agent cannot be canceled if it has already started
    ///     running or has already completed.
    /// </returns>
    /// <seealso cref="agent_status Enumeration"/>
    /**/
    _CONCRTIMP bool cancel();

    /// <summary>
    ///     Waits for an agent to complete its task.
    /// </summary>
    /// <param name="_PAgent">
    ///     A pointer to the agent to wait for.
    /// </param>
    /// <param name="_Timeout">
    ///     The maximum time for which to wait, in milliseconds.
    /// </param>
    /// <returns>
    ///     The <c>agent_status</c> of the agent when the wait completes. This can either be <c>agent_canceled</c>
    ///     or <c>agent_done</c>.
    /// </returns>
    /// <remarks>
    ///     An agent task is completed when the agent enters the <c>agent_canceled</c> or <c>agent_done</c> states.
    ///     <para>If the parameter <paramref name="_Timeout"/> has a value other than the constant <c>COOPERATIVE_TIMEOUT_INFINITE</c>,
    ///     the exception <see cref="operation_timed_out Class">operation_timed_out</see> is thrown if the specified amount
    ///     of time expires before the agent has completed its task.</para>
    /// </remarks>
    /// <seealso cref="agent::wait_for_all Method"/>
    /// <seealso cref="agent::wait_for_one Method"/>
    /// <seealso cref="agent_status Enumeration"/>
    /**/
    _CONCRTIMP static agent_status __cdecl wait(_Inout_ agent * _PAgent, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE);

    /// <summary>
    ///     Waits for all of the specified agents to complete their tasks.
    /// </summary>
    /// <param name="_Count">
    ///     The number of agent pointers present in the array <paramref name="_PAgents"/>.
    /// </param>
    /// <param name="_PAgents">
    ///     An array of pointers to the agents to wait for.
    /// </param>
    /// <param name="_PStatus">
    ///     A pointer to an array of agent statuses. Each status value will represent the status of the corresponding
    ///     agent when the method returns.
    /// </param>
    /// <param name="_Timeout">
    ///     The maximum time for which to wait, in milliseconds.
    /// </param>
    /// <remarks>
    ///     An agent task is completed when the agent enters the <c>agent_canceled</c> or <c>agent_done</c> states.
    ///     <para>If the parameter <paramref name="_Timeout"/> has a value other than the constant <c>COOPERATIVE_TIMEOUT_INFINITE</c>,
    ///     the exception <see cref="operation_timed_out Class">operation_timed_out</see> is thrown if the specified amount
    ///     of time expires before the agent has completed its task.</para>
    /// </remarks>
    /// <seealso cref="agent::wait Method"/>
    /// <seealso cref="agent::wait_for_one Method"/>
    /// <seealso cref="agent_status Enumeration"/>
    /**/
    _CONCRTIMP static void __cdecl wait_for_all(size_t _Count, _In_reads_(_Count) agent ** _PAgents,
        _Out_writes_opt_(_Count) agent_status * _PStatus = nullptr, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE);

    /// <summary>
    ///     Waits for any one of the specified agents to complete its task.
    /// </summary>
    /// <param name="_Count">
    ///     The number of agent pointers present in the array <paramref name="_PAgents"/>.
    /// </param>
    /// <param name="_PAgents">
    ///     An array of pointers to the agents to wait for.
    /// </param>
    /// <param name="_Status">
    ///     A reference to a variable where the agent status will be placed.
    /// </param>
    /// <param name="_Index">
    ///     A reference to a variable where the agent index will be placed.
    /// </param>
    /// <param name="_Timeout">
    ///     The maximum time for which to wait, in milliseconds.
    /// </param>
    /// <remarks>
    ///     An agent task is completed when the agent enters the <c>agent_canceled</c> or <c>agent_done</c> states.
    ///     <para>If the parameter <paramref name="_Timeout"/> has a value other than the constant <c>COOPERATIVE_TIMEOUT_INFINITE</c>,
    ///     the exception <see cref="operation_timed_out Class">operation_timed_out</see> is thrown if the specified amount
    ///     of time expires before the agent has completed its task.</para>
    /// </remarks>
    /// <seealso cref="agent::wait Method"/>
    /// <seealso cref="agent::wait_for_all Method"/>
    /// <seealso cref="agent_status Enumeration"/>
    /**/
    _CONCRTIMP static void __cdecl wait_for_one(size_t _Count, _In_reads_(_Count) agent ** _PAgents, agent_status& _Status,
                                      size_t& _Index, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE);

protected:
    /// <summary>
    ///     Represents the main task of an agent. <c>run</c> should be overridden in a derived class, and specifies what
    ///     the agent should do after it has been started.
    /// </summary>
    /// <remarks>
    ///     The agent status is changed to <c>agent_started</c> right before this method is invoked. The method should
    ///     invoke <c>done</c> on the agent with an appropriate status before returning, and may not throw any
    ///     exceptions.
    /// </remarks>
    /**/
    virtual void run() = 0;

    /// <summary>
    ///     Moves an agent into the <c>agent_done</c> state, indicating that the agent has completed.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the agent is moved to the <c>agent_done</c> state, <c>false</c> otherwise. An agent that has
    ///     been canceled cannot be moved to the <c>agent_done</c> state.
    /// </returns>
    /// <remarks>
    ///     This method should be called at the end of the <c>run</c> method, when you know the execution of your agent
    ///     has completed.
    /// </remarks>
    /// <seealso cref="agent_status Enumeration"/>
    /**/
    _CONCRTIMP bool done();

    /// <summary>
    ///     Holds the current status of the agent.
    /// </summary>
    /**/
    overwrite_buffer<agent_status> _M_status;

private:

    // A flag to check of whether the agent can be started
    // This is initialized to TRUE and there is a race between Start() and Cancel() to set it
    // to FALSE.  Once Started or Canceled, further calls to Start() or Cancel() will return false.
    /**/
    volatile long _M_fStartable;

    // A flag to check of whether the agent can be canceled
    // This is initialized to TRUE and there is a race between Cancel() and the LWT executing
    // a task that has been started to set it to FALSE.  If Cancel() wins, the task will not be
    // executed.  If the LWT wins, Cancel() will return false.
    /**/
    volatile long _M_fCancelable;

    // A static wrapper function that calls the Run() method.  Used for scheduling of the task
    /**/
    static void __cdecl _Agent_task_wrapper(void * data);

    Scheduler * _M_pScheduler;
    ScheduleGroup * _M_pScheduleGroup;

    //
    // Hide assignment operator and copy constructor
    //
    agent const &operator =(agent const&);  // no assignment operator
    agent(agent const &);                   // no copy constructor
};

/// <summary>
///     Associates the given name to the message block or agent in the ETW trace.
/// </summary>
/// <typeparam name="_Type">
///     The type of the object. This is typically a message block or an agent.
/// </typeparam>
/// <param name="_PObject">
///     A pointer to the message block or agent that is being named in the trace.
/// </param>
/// <param name="_Name">
///     The name for the given object.
/// </param>
template <class _Type>
void Trace_agents_register_name(_Inout_ _Type * _PObject, _In_z_ const wchar_t * _Name)
{
    _Trace_agents(AGENTS_EVENT_NAME, ::Concurrency::details::_Trace_agents_get_id(_PObject), _Name);
}

} // namespace Concurrency

namespace concurrency = ::Concurrency;

#pragma warning(pop)
#pragma pack(pop)
