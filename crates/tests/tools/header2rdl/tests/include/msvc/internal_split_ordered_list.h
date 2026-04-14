/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* internal_split_ordered_list.h
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/
#pragma once

// Needed for forward iterators
#include <forward_list>
#include <concrt.h>

#pragma pack(push,_CRT_PACKING)

#pragma warning(push)
#pragma warning(disable: 4510 4512 4610)  // disable warnings for compiler unable to generate constructor

namespace Concurrency
{
namespace details
{
// Split-order list iterators, needed to skip dummy elements
template<class _Mylist>
class _Solist_const_iterator : public ::std::_Flist_const_iterator<_Mylist>
{
public:
    typedef _Solist_const_iterator<_Mylist> _Myiter;
    typedef ::std::_Flist_const_iterator<_Mylist> _Mybase;
    typedef ::std::forward_iterator_tag iterator_category;

    typedef typename _Mylist::_Nodeptr _Nodeptr;
    typedef typename _Mylist::value_type value_type;
    typedef typename _Mylist::difference_type difference_type;
    typedef typename _Mylist::const_pointer pointer;
    typedef typename _Mylist::const_reference reference;

    _Solist_const_iterator()
    {
    }

    _Solist_const_iterator(_Nodeptr _Pnode, const _Mylist * _Plist) : _Mybase(_Pnode, _Plist)
    {
    }

    typedef _Solist_const_iterator _Unchecked_type;

    reference operator*() const
    {
        return ((reference)**(_Mybase *)this);
    }

    pointer operator->() const
    {
        return (&**this);
    }

    _Myiter& operator++()
    {
        do
        {
            ++(*(_Mybase *)this);
        }
        while (this->_Ptr != nullptr && this->_Ptr->_Is_dummy());

        return (*this);
    }

    _Myiter operator++(int)
    {
        _Myiter _Tmp = *this;
        do
        {
            ++*this;
        }
        while (this->_Ptr != nullptr && this->_Ptr->_Is_dummy());

        return (_Tmp);
    }
};

template<class _Mylist>
class _Solist_iterator : public _Solist_const_iterator<_Mylist>
{
public:
    typedef _Solist_iterator<_Mylist> _Myiter;
    typedef _Solist_const_iterator<_Mylist> _Mybase;
    typedef ::std::forward_iterator_tag iterator_category;

    typedef typename _Mylist::_Nodeptr _Nodeptr;
    typedef typename _Mylist::value_type value_type;
    typedef typename _Mylist::difference_type difference_type;
    typedef typename _Mylist::pointer pointer;
    typedef typename _Mylist::reference reference;

    _Solist_iterator()
    {
    }

    _Solist_iterator(_Nodeptr _Pnode, const _Mylist *_Plist) : _Mybase(_Pnode, _Plist)
    {
    }

    typedef _Solist_iterator _Unchecked_type;

    reference operator*() const
    {
        return ((reference)**(_Mybase *)this);
    }

    pointer operator->() const
    {
        return (&**this);
    }

    _Myiter& operator++()
    {
        do
        {
            ++(*(_Mybase *)this);
        }
        while (this->_Ptr != nullptr && this->_Ptr->_Is_dummy());

        return (*this);
    }

    _Myiter operator++(int)
    {
        _Myiter _Tmp = *this;
        do
        {
            ++*this;
        }
        while (this->_Ptr != nullptr && this->_Ptr->_Is_dummy());

        return (_Tmp);
    }
};

// Forward type and class definitions
typedef size_t _Map_key;
typedef _Map_key _Split_order_key;

template<typename _Element_type, typename _Element_allocator_type>
class _Split_order_list_node : public ::std::_Container_base
{
public:
    typedef typename ::std::allocator_traits<_Element_allocator_type>::template rebind_alloc<_Element_type> _Allocator_type;
    typedef ::std::allocator_traits<_Allocator_type> _Al_traits;
    typedef typename _Al_traits::size_type size_type;
    typedef _Element_type value_type;

    struct _Node;
    typedef _Node * _Nodeptr;
    typedef typename _Al_traits::template rebind_alloc<_Node> _Alnode;
    typedef ::std::allocator_traits<_Alnode> _Alnode_traits;

    // Node that holds the element in a split-ordered list
    struct _Node
    {
        // Initialize the node with the given order key
        void _Init(_Split_order_key _Order_key)
        {
            _M_order_key = _Order_key;
            _Next = nullptr;
        }

        // Return the order key (needed for hashing)
        _Split_order_key _Get_order_key() const
        {
            return _M_order_key;
        }

        // Inserts the new element in the list in an atomic fashion
        _Nodeptr _Atomic_set_next(_Nodeptr _New_node, _Nodeptr _Current_node)
        {
            // Try to change the next pointer on the current element to a new element, only if it still points to the cached next
            _Nodeptr _Exchange_node = (_Nodeptr) _InterlockedCompareExchangePointer((void * volatile *) &_Next, _New_node, _Current_node);

            if (_Exchange_node == _Current_node)
            {
                // Operation succeeded, return the new node
                return _New_node;
            }
            else
            {
                // Operation failed, return the "interfering" node
                return _Exchange_node;
            }
        }

        // Checks if this element in the list is a dummy, order-enforcing node. Dummy nodes are used by buckets
        // in the hash table to quickly index into the right subsection of the split-ordered list.
        bool _Is_dummy() const
        {
            return (_M_order_key & 0x1) == 0;
        }

        // dummy default constructor - never used but for suppress warning
        _Node();

        _Nodeptr         _Next;        // Next element in the list
        value_type       _Myval;       // Element storage
        _Split_order_key _M_order_key; // Order key for this element
    };

#if _ITERATOR_DEBUG_LEVEL == 0
    _Split_order_list_node(_Allocator_type _Allocator) : _M_node_allocator(_Allocator), _M_value_allocator(_Allocator)
    {
    }
#else  /* _ITERATOR_DEBUG_LEVEL == 0 */
    _Split_order_list_node(_Allocator_type _Allocator) : _M_node_allocator(_Allocator), _M_value_allocator(_Allocator)
    {
        using _Alproxy_traits = typename _Al_traits::template rebind_traits<::std::_Container_proxy>;
        typename _Alproxy_traits::allocator_type _Alproxy(_M_node_allocator);
        _Myproxy = _Alproxy.allocate(1);
        ::std::_Construct_in_place(*_Myproxy);
        _Myproxy->_Mycont = this;
    }

    ~_Split_order_list_node()
    {
        using _Alproxy_traits = typename _Al_traits::template rebind_traits<::std::_Container_proxy>;
        typename _Alproxy_traits::allocator_type _Alproxy(_M_node_allocator);
        _Orphan_all();
        ::std::_Destroy_in_place(*_Myproxy);
        _Alproxy.deallocate(_Myproxy, 1);
        _Myproxy = 0;
    }
#endif  /* _ITERATOR_DEBUG_LEVEL == 0 */

    _Nodeptr _Before_head() const noexcept
    {
        // return pointer to the "before begin" pseudo node
        return &(reinterpret_cast<_Node&>(const_cast<_Nodeptr&>(_Myhead)));
    }

    _Nodeptr         _Myhead;            // pointer to head node
    _Alnode          _M_node_allocator;  // allocator object for nodes
    _Allocator_type  _M_value_allocator; // allocator object for element values
};

template<typename _Element_type, typename _Element_allocator_type>
class _Split_order_list_value : public _Split_order_list_node<_Element_type, _Element_allocator_type>
{
public:
    typedef _Split_order_list_node<_Element_type, _Element_allocator_type> _Mybase;
    using typename _Mybase::_Nodeptr;
    using typename _Mybase::_Allocator_type;
    using typename _Mybase::_Al_traits;
    using typename _Mybase::_Alnode_traits;

    typedef typename _Al_traits::size_type size_type;
    typedef typename _Al_traits::difference_type difference_type;
    typedef typename _Al_traits::pointer pointer;
    typedef typename _Al_traits::const_pointer const_pointer;
    typedef typename _Allocator_type::value_type value_type;
    typedef value_type& reference;
    typedef const value_type& const_reference;

    _Split_order_list_value(_Allocator_type _Allocator = _Allocator_type()) : _Mybase(_Allocator)
    {
        // Immediately allocate a dummy node with order key of 0. This node
        // will always be the head of the list.
        this->_Myhead = _Buynode(0);
    }

    ~_Split_order_list_value()
    {
    }

    // Allocate a new node with the given order key and value
    template<typename _ValTy>
    _Nodeptr _Buynode(_Split_order_key _Order_key, _ValTy&& _Value)
    {
        _Nodeptr _Pnode = this->_M_node_allocator.allocate(1);

        try
        {
            _Al_traits::construct(
                this->_M_value_allocator, ::std::addressof(_Pnode->_Myval), ::std::forward<_ValTy>(_Value));
            _Pnode->_Init(_Order_key);
        }
        catch(...)
        {
            this->_M_node_allocator.deallocate(_Pnode, 1);
            throw;
        }

        return (_Pnode);
    }

    // Allocate a new node with the given order key; used to allocate dummy nodes
    _Nodeptr _Buynode(_Split_order_key _Order_key)
    {
        _Nodeptr _Pnode = this->_M_node_allocator.allocate(1);
        _Pnode->_Init(_Order_key);

        return (_Pnode);
    }
};

// Forward list in which elements are sorted in a split-order
template <typename _Element_type, typename _Element_allocator_type = ::std::allocator<_Element_type>>
class _Split_ordered_list : _Split_order_list_value<_Element_type, _Element_allocator_type>
{
public:
    typedef _Split_ordered_list<_Element_type, _Element_allocator_type> _Mytype;
    typedef _Split_order_list_value<_Element_type, _Element_allocator_type> _Mybase;
    using typename _Mybase::_Allocator_type;
    using typename _Mybase::_Al_traits;
    using typename _Mybase::_Nodeptr;
    using typename _Mybase::_Alnode_traits;

    typedef _Allocator_type allocator_type;
    using typename _Mybase::size_type;
    using typename _Mybase::difference_type;
    using typename _Mybase::pointer;
    using typename _Mybase::const_pointer;
    using typename _Mybase::reference;
    using typename _Mybase::const_reference;
    using typename _Mybase::value_type;

    typedef _Solist_const_iterator<_Mybase> const_iterator;
    typedef _Solist_iterator<_Mybase> iterator;
    typedef ::std::_Flist_const_iterator<_Mybase> _Full_const_iterator;
    typedef ::std::_Flist_iterator<_Mybase> _Full_iterator;
    typedef ::std::pair<iterator, bool> _Pairib;
    using _Mybase::_Buynode;
    _Split_ordered_list(allocator_type _Allocator = allocator_type()) : _Mybase(_Allocator), _M_element_count(0)
    {
    }

    ~_Split_ordered_list()
    {
        // Clear the list
        clear();

        // Remove the head element which is not cleared by clear()
        _Nodeptr _Pnode = this->_Myhead;
        this->_Myhead = nullptr;

        _ASSERT_EXPR(_Pnode != nullptr && _Pnode->_Next == nullptr, L"Invalid head list node");

        _Erase(_Pnode);
    }

    // Common forward list functions

    allocator_type get_allocator() const
    {
        return (this->_M_value_allocator);
    }

    void clear()
    {
#if _ITERATOR_DEBUG_LEVEL == 2
        _Orphan_ptr(*this, 0);
#endif  /* _ITERATOR_DEBUG_LEVEL == 2 */

        _Nodeptr _Pnext;
        _Nodeptr _Pnode = this->_Myhead;

        _ASSERT_EXPR(this->_Myhead != nullptr, L"Invalid head list node");
        _Pnext = _Pnode->_Next;
        _Pnode->_Next = nullptr;
        _Pnode = _Pnext;

        while (_Pnode != nullptr)
        {
            _Pnext = _Pnode->_Next;
            _Erase(_Pnode);
            _Pnode = _Pnext;
        }

        _M_element_count = 0;
    }

    // Returns a first non-dummy element in the SOL
    iterator begin()
    {
        _Full_iterator _Iterator = _Begin();
        return _Get_first_real_iterator(_Iterator);
    }

    // Returns a first non-dummy element in the SOL
    const_iterator begin() const
    {
        _Full_const_iterator _Iterator = _Begin();
        return _Get_first_real_iterator(_Iterator);
    }

    iterator end()
    {
        return (iterator(0, this));
    }

    const_iterator end() const
    {
        return (const_iterator(0, this));
    }

    const_iterator cbegin() const
    {
        return (((const _Mytype *)this)->begin());
    }

    const_iterator cend() const
    {
        return (((const _Mytype *)this)->end());
    }

    // Checks if the number of elements (non-dummy) is 0
    bool empty() const
    {
        return (_M_element_count == 0);
    }

    // Returns the number of non-dummy elements in the list
    size_type size() const
    {
        return _M_element_count;
    }

    // Returns the maximum size of the list, determined by the allocator
    size_type max_size() const
    {
        return _Al_traits::max_size(this->_M_value_allocator);
    }

    // Swaps 'this' list with the passed in one
    void swap(_Mytype& _Right)
    {
        if (this == &_Right)
        {
            // Nothing to do
            return;
        }

        if (this->_M_value_allocator == _Right._M_value_allocator)
        {
            this->_Swap_proxy_and_iterators(_Right);
            ::std::swap(this->_Myhead, _Right._Myhead);
            ::std::swap(_M_element_count, _Right._M_element_count);
        }
        else
        {
            _Mytype _Temp_list(this->get_allocator());
            _Temp_list._Move_all(_Right);
            _Right._Move_all(*this);
            _Move_all(_Temp_list);
        }
    }

    // Split-order list functions

    // Returns a first element in the SOL, which is always a dummy
    _Full_iterator _Begin()
    {
        return _Full_iterator(this->_Myhead, this);
    }

    // Returns a first element in the SOL, which is always a dummy
    _Full_const_iterator _Begin() const
    {
        return _Full_const_iterator(this->_Myhead, this);
    }

    _Full_iterator _End()
    {
        return _Full_iterator(0, this);
    }

    _Full_const_iterator _End() const
    {
        return _Full_const_iterator(0, this);
    }

    static _Split_order_key _Get_key(const _Full_const_iterator& _Iterator)
    {
        return _Iterator._Ptr->_Get_order_key();
    }

    // Returns a public iterator version of the internal iterator. Public iterator must not
    // be a dummy private iterator.
    iterator _Get_iterator(_Full_iterator _Iterator)
    {
        _ASSERT_EXPR(_Iterator._Ptr != nullptr && !_Iterator._Ptr->_Is_dummy(), L"Invalid user node (dummy)");
        return iterator(_Iterator._Ptr, this);
    }

    // Returns a public iterator version of the internal iterator. Public iterator must not
    // be a dummy private iterator.
    const_iterator _Get_iterator(_Full_const_iterator _Iterator) const
    {
        _ASSERT_EXPR(_Iterator._Ptr != nullptr && !_Iterator._Ptr->_Is_dummy(), L"Invalid user node (dummy)");
        return const_iterator(_Iterator._Ptr, this);
    }

    // Returns a non-const version of the _Full_iterator
    _Full_iterator _Get_iterator(_Full_const_iterator _Iterator)
    {
        return _Full_iterator(_Iterator._Ptr, this);
    }

    // Returns a non-const version of the iterator
    iterator _Get_iterator(const_iterator _Iterator)
    {
        return iterator(_Iterator._Ptr, this);
    }

    // Returns a public iterator version of a first non-dummy internal iterator at or after
    // the passed in internal iterator.
    iterator _Get_first_real_iterator(_Full_iterator _Iterator)
    {
        // Skip all dummy, internal only iterators
        while (_Iterator != _End() && _Iterator._Ptr->_Is_dummy())
        {
            ++_Iterator;
        }

        return iterator(_Iterator._Ptr, this);
    }

    // Returns a public iterator version of a first non-dummy internal iterator at or after
    // the passed in internal iterator.
    const_iterator _Get_first_real_iterator(_Full_const_iterator _Iterator) const
    {
        // Skip all dummy, internal only iterators
        while (_Iterator != _End() && _Iterator._Ptr->_Is_dummy())
        {
            ++_Iterator;
        }

        return const_iterator(_Iterator._Ptr, this);
    }

    // Erase an element using the allocator
    void _Erase(_Nodeptr _Delete_node)
    {
        if (!_Delete_node->_Is_dummy())
        {
            // Dummy nodes have nothing constructed, thus should not be destroyed.
            _Alnode_traits::destroy(this->_M_node_allocator, _Delete_node);
        }
        this->_M_node_allocator.deallocate(_Delete_node, 1);
    }

    // Try to insert a new element in the list. If insert fails, return the node that
    // was inserted instead.
    _Nodeptr _Insert(_Nodeptr _Previous, _Nodeptr _New_node, _Nodeptr _Current_node)
    {
        _New_node->_Next = _Current_node;
        return _Previous->_Atomic_set_next(_New_node, _Current_node);
    }

    // Insert a new element between passed in iterators
    _Pairib _Insert(_Full_iterator _Iterator, _Full_iterator _Next, _Nodeptr _List_node, long * _New_count)
    {
        _Nodeptr _Inserted_node = _Insert(_Iterator._Ptr, _List_node, _Next._Ptr);

        if (_Inserted_node == _List_node)
        {
            // If the insert succeeded, increment the element count
            *_New_count = _InterlockedIncrement(&_M_element_count);
            return _Pairib(iterator(_List_node, this), true);
        }
        else
        {
            return _Pairib(end(), false);
        }
    }

    // Insert a new dummy element, starting search at a parent dummy element
    _Full_iterator _Insert_dummy(_Full_iterator _Iterator, _Split_order_key _Order_key)
    {
        _Full_iterator _Last = _End();
        _Full_iterator _Where = _Iterator;

        _ASSERT_EXPR(_Where != _Last, L"Invalid head node");

        ++_Where;

        // Create a dummy element up front, even though it may be discarded (due to concurrent insertion)
        _Nodeptr _Dummy_node = _Buynode(_Order_key);

        for (;;)
        {
            _ASSERT_EXPR(_Iterator != _Last, L"Invalid head list node");

            // If the head iterator is at the end of the list, or past the point where this dummy
            // node needs to be inserted, then try to insert it.
            if (_Where == _Last || _Get_key(_Where) > _Order_key)
            {
                _ASSERT_EXPR(_Get_key(_Iterator) < _Order_key, L"Invalid node order in the list");

                // Try to insert it in the right place
                _Nodeptr _Inserted_node = _Insert(_Iterator._Ptr, _Dummy_node, _Where._Ptr);

                if (_Inserted_node == _Dummy_node)
                {
                    // Insertion succeeded
                    return _Full_iterator(_Dummy_node, this);
                }
                else
                {
                    // Insertion failed: either dummy node was inserted by another thread, or
                    // a real element was inserted at exactly the same place as dummy node.
                    // Proceed with the search from the previous location where order key was
                    // known to be larger (note: this is legal only because there is no safe
                    // concurrent erase operation supported).
                    _Where = _Iterator;
                    ++_Where;
                    continue;
                }
            }
            else if (_Get_key(_Where) == _Order_key)
            {
                // Another dummy node with the same value found, discard the new one.
                _Erase(_Dummy_node);
                return _Where;
            }

            // Move the iterator forward
            _Iterator = _Where;
            ++_Where;
        }

    }

    // This erase function can handle both real and dummy nodes
    void _Erase(_Full_iterator _Previous, _Full_const_iterator& _Where)
    {
#if _ITERATOR_DEBUG_LEVEL == 2
        _STL_VERIFY(_Where._Getcont() == this && _Where._Ptr != this->_Myhead, "list erase iterator outside range");
        _Nodeptr _Pnode = (_Where++)._Ptr;
        _Orphan_ptr(*this, _Pnode);
#else  /* _ITERATOR_DEBUG_LEVEL == 2 */
        _Nodeptr _Pnode = (_Where++)._Ptr;
#endif  /* _ITERATOR_DEBUG_LEVEL == 2 */

        _Nodeptr _Prevnode = _Previous._Ptr;
        _ASSERT_EXPR(_Prevnode->_Next == _Pnode, L"Erase must take consecutive iterators");
        _Prevnode->_Next = _Pnode->_Next;

        _Erase(_Pnode);
    }

    // Erase the element (previous node needs to be passed because this is a forward only list)
    iterator _Erase(_Full_iterator _Previous, const_iterator _Where)
    {
        _Full_const_iterator _Iterator = _Where;
        _Erase(_Previous, _Iterator);
        _M_element_count--;

        return _Get_iterator(_Get_first_real_iterator(_Iterator));
    }

    // Move all elements from the passed in split-ordered list to this one
    void _Move_all(_Mytype& _Source_list)
    {
        _Full_const_iterator _First = _Source_list._Begin();
        _Full_const_iterator _Last = _Source_list._End();

        if (_First == _Last)
        {
            return;
        }

        _Nodeptr _Previous_node = this->_Myhead;
        _Full_const_iterator _Begin_iterator = _First++;

        // Move all elements one by one, including dummy ones
        for (_Full_const_iterator _Iterator = _First; _Iterator != _Last;)
        {
            _Nodeptr _Node = _Iterator._Ptr;

            _Nodeptr _Dummy_node = _Node->_Is_dummy() ? _Buynode(_Node->_Get_order_key()) : _Buynode(_Node->_Get_order_key(), _Node->_Myval);
            _Previous_node = _Insert(_Previous_node, _Dummy_node, nullptr);
            _ASSERT_EXPR(_Previous_node != nullptr, L"Insertion must succeed");
            _Full_const_iterator _Where = _Iterator++;
            _Source_list._Erase(_Get_iterator(_Begin_iterator), _Where);
        }
    }

private:
#if _ITERATOR_DEBUG_LEVEL == 2
    void _Orphan_ptr(_Mytype& _Cont, _Nodeptr _Ptr) const
    {
        const auto _BHead = this->_Before_head();
        ::std::_Lockit _Lock(_LOCK_DEBUG);
        ::std::_Iterator_base12** _Pnext = &_Cont._Myproxy->_Myfirstiter;
        while (*_Pnext)
        {
            const auto _Pnextptr = static_cast<const_iterator&>(**_Pnext)._Ptr;
            if (_Pnextptr == _BHead || (_Ptr && _Pnextptr != _Ptr))
            {
                _Pnext = &(*_Pnext)->_Mynextiter;
            }
            else
            {
                (*_Pnext)->_Myproxy = nullptr;
                *_Pnext = (*_Pnext)->_Mynextiter;
            }
        }
    }
#endif  /* _ITERATOR_DEBUG_LEVEL == 2 */

    volatile long _M_element_count; // Total item count, not counting dummy nodes
};

} // namespace details;
} // namespace Concurrency

namespace concurrency = ::Concurrency;

#pragma warning(pop)
#pragma pack(pop)
