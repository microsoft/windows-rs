// xnode_handle.h internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XNODE_HANDLE_H
#define _XNODE_HANDLE_H
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#if !_HAS_CXX17
#error Node handles are only available with C++17. (Also, you should not include this internal header.)
#endif // _HAS_CXX17

#include <xmemory>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new
_STD_BEGIN

template <class _Iterator, class _NodeType>
struct _Insert_return_type {
    _Iterator position;
    bool inserted;
    _NodeType node;
};

template <class _Derived_type, class _KeyTy, class _ValueTy>
struct _Node_handle_map_base { // map-specific node handle behavior
    using key_type    = _KeyTy;
    using mapped_type = _ValueTy;

    key_type& key() const noexcept /* strengthened */ {
        return _Datum().first;
    }
    mapped_type& mapped() const noexcept /* strengthened */ {
        return _Datum().second;
    }

private:
    using _Pair_type = pair<key_type, mapped_type>;
    _Pair_type& _Datum() const {
        const auto& _Self = static_cast<const _Derived_type&>(*this);
        auto& _Data       = _Self._Getptr()->_Myval;
        // NB: Node handle requires formally-undefined behavior *somewhere*, this
        // implementation chooses to type-pun pair<const K, T> as pair<K, T>.
        return reinterpret_cast<_Pair_type&>(_Data);
    }
};

template <class _Derived_type, class _ValueTy>
struct _Node_handle_set_base { // set-specific node handle behavior
    using value_type = _ValueTy;

    value_type& value() const noexcept /* strengthened */ {
        const auto& _Self = static_cast<const _Derived_type&>(*this);
        return _Self._Getptr()->_Myval;
    }
};

template <class _Node, class _Alloc, template <class...> class _Base, class... _Types>
class _Node_handle : public _Base<_Node_handle<_Node, _Alloc, _Base, _Types...>, _Types...> {
    // storage for a node from one of the node-based standard containers
public:
    using allocator_type = _Alloc;

private:
    using _Alty_traits   = allocator_traits<_Alloc>;
    using _Alnode        = _Rebind_alloc_t<_Alloc, _Node>;
    using _Alnode_traits = allocator_traits<_Alnode>;
    using _Nodeptr       = typename _Alnode_traits::pointer;

    _Nodeptr _Ptr{};
    _Aligned_storage_t<sizeof(_Alloc), alignof(_Alloc)>
        _Alloc_storage; // Invariant: contains a live _Alloc iff _Ptr != nullptr

    void _Clear() noexcept { // destroy any contained node and return to the empty state
        if (_Ptr != nullptr) {
            _Alloc& _Al = _Getal();
            _Alty_traits::destroy(_Al, _STD addressof(_Ptr->_Myval));
            _Alnode _Node_alloc{_Al};
            _Node::_Freenode0(_Node_alloc, _Ptr);
            _Destroy_in_place(_Al);
            _Ptr = nullptr;
        }
    }

    _Node_handle(const _Nodeptr _Ptr_, const _Alloc& _Al) noexcept
        : _Ptr{_Ptr_} { // Initialize a _Node_handle that holds the specified node
                        // pre: _Alloc can release _Ptr
        _STL_INTERNAL_CHECK(_Ptr_ != nullptr);
        _Construct_in_place(_Getal(), _Al);
    }

public:
    constexpr _Node_handle() noexcept : _Alloc_storage{} {}

    ~_Node_handle() noexcept {
        _Clear();
    }

    _Node_handle(_Node_handle&& _That) noexcept : _Ptr{_That._Ptr} { // steal node and allocator (if any) from _That
        if (_Ptr != nullptr) {
            _That._Ptr       = nullptr;
            _Alloc& _That_al = _That._Getal();
            _Construct_in_place(_Getal(), _STD move(_That_al));
            _Destroy_in_place(_That_al);
        }
    }

    _Node_handle& operator=(_Node_handle&& _That) noexcept /* strengthened */ {
        // steal state from _That
        if (_Ptr == nullptr) {
            if (_That._Ptr != nullptr) {
                _Alloc& _That_al = _That._Getal();
                _Construct_in_place(_Getal(), _STD move(_That_al));
                _Destroy_in_place(_That_al);
                _Ptr = _STD exchange(_That._Ptr, nullptr);
            }

            return *this;
        }

        if (_That._Ptr == nullptr || this == _STD addressof(_That)) {
            _Clear();
            return *this;
        }

        _Alloc& _Al = _Getal();
        _Alty_traits::destroy(_Al, _STD addressof(_Ptr->_Myval));
        _Alnode _Node_alloc{_Al};
        _Alnode_traits::deallocate(_Node_alloc, _Ptr, 1);

        _Alloc& _That_al = _That._Getal();
        _Pocma(_Al, _That_al);
        _Destroy_in_place(_That_al);

        _Ptr = _STD exchange(_That._Ptr, nullptr);
        return *this;
    }

    _Nodeptr _Getptr() const noexcept {
        return _Ptr;
    }

    _Alloc& _Getal() noexcept {
        return reinterpret_cast<_Alloc&>(_Alloc_storage);
    }
    const _Alloc& _Getal() const noexcept {
        _STL_INTERNAL_CHECK(!empty());
        return reinterpret_cast<const _Alloc&>(_Alloc_storage);
    }

    _NODISCARD allocator_type get_allocator() const noexcept /* strengthened */ {
        _STL_INTERNAL_CHECK(!empty());
        return _Getal();
    }

    explicit operator bool() const noexcept {
        return _Ptr != nullptr;
    }

    _NODISCARD bool empty() const noexcept {
        return _Ptr == nullptr;
    }

    _Nodeptr _Release() noexcept { // extract the node from *this
        _STL_INTERNAL_CHECK(!empty());
        _Destroy_in_place(_Getal());
        return _STD exchange(_Ptr, nullptr);
    }

    void swap(_Node_handle& _That) noexcept /* strengthened */ {
        using _STD swap;
        if (_Ptr != nullptr) {
            if (_That._Ptr != nullptr) {
                _Pocs(_Getal(), _That._Getal());
            } else {
                _Alloc& _Al = _Getal();
                _Construct_in_place(_That._Getal(), _STD move(_Al));
                _Destroy_in_place(_Al);
            }
        } else {
            if (_That._Ptr == nullptr) {
                return;
            }

            _Alloc& _That_al = _That._Getal();
            _Construct_in_place(_Getal(), _STD move(_That_al));
            _Destroy_in_place(_That_al);
        }
        swap(_Ptr, _That._Ptr); // intentional ADL
    }
    friend void swap(_Node_handle& _Left, _Node_handle& _Right) noexcept /* strengthened */ {
        _Left.swap(_Right);
    }

    static _Node_handle _Make(const _Nodeptr _Ptr, const allocator_type& _Al) {
        // initialize a _Node_handle that holds _Ptr and _Al
        // pre: _Al can release _Ptr
        _STL_INTERNAL_CHECK(_Ptr != nullptr);
        return _Node_handle{_Ptr, _Al};
    }
};

_STD_END
#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XNODE_HANDLE_H
