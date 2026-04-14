/***
* collection.h - Windows Runtime Collection/Iterator Wrappers
*
* Copyright (c) Microsoft Corporation. All rights reserved.
****/

#pragma once

#ifndef _COLLECTION_H_
#define _COLLECTION_H_

#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#ifndef __cplusplus_winrt
    #error collection.h requires the /ZW compiler option.
#endif

#include <stddef.h>
#include <algorithm>
#include <array>
#include <exception>
#include <functional>
#include <iterator>
#include <map>
#include <memory>
#include <new>
#include <string> // for source compatibility, as <unordered_map> included most of <string> before microsoft/STL#2996
#include <type_traits>
#include <unordered_map>
#include <utility>
#include <vector>
#include <agile.h>

#define _COLLECTION_ATTRIBUTES [::Platform::Metadata::RuntimeClassName] [::Windows::Foundation::Metadata::Default]

#define _COLLECTION_TRANSLATE           \
} catch (const ::std::bad_alloc&) {     \
    throw ref new OutOfMemoryException; \
} catch (const ::std::exception&) {     \
    throw ref new FailureException;     \
}

#ifndef _COLLECTION_WUXI
    #define _COLLECTION_WUXI 1
#endif

#ifdef _WIN64
    #pragma pack(push, 16)
#else
    #pragma pack(push, 8)
#endif

#pragma warning(push, 4)

#pragma warning(disable: 4451) // Usage of ref class 'Meow' inside this context can lead to invalid marshaling of object across contexts

namespace Platform {
  namespace Collections {
    namespace Details {
        namespace WFC = ::Windows::Foundation::Collections;

#if _COLLECTION_WUXI
        namespace WUXI = ::Windows::UI::Xaml::Interop;
#endif // _COLLECTION_WUXI

        typedef ::Windows::Foundation::EventRegistrationToken Token;

        inline void ValidateBounds(bool b) {
            if (!b) {
                throw ref new OutOfBoundsException;
            }
        }

        inline void ValidateCounter(const ::std::shared_ptr<unsigned int>& ctr, unsigned int good_ctr) {
            if (*ctr != good_ctr) {
                throw ref new ChangedStateException;
            }
        }

        inline void ValidateSize(size_t n) {
            if (n > 0x7FFFFFFFUL) {
                throw ref new OutOfMemoryException;
            }
        }

        template <typename T> struct AlwaysFalse
            : public ::std::false_type { };

        template <typename T> struct Wrap {
            typedef T type;
        };

        template <typename T> struct Wrap<T^>
            : public ::std::conditional<__is_winrt_agile(T), T^, Agile<T^>> { };

        template <typename T> inline const T& MakeWrap(const T& t) {
            return t;
        }

        template <typename T> inline typename ::std::enable_if<!__is_winrt_agile(T), Agile<T^>>::type MakeWrap(T^ const & t) {
            return Agile<T^>(t);
        }

        template <typename T> inline const T& Unwrap(const T& t) {
            return t;
        }

        template <typename T> inline T^ Unwrap(const Agile<T^>& a) {
            return a.Get();
        }

        template <typename T, typename U> struct VectorEnableIf
            : public ::std::enable_if< ::std::is_same<T, U>::value || ::std::is_same<typename Wrap<T>::type, U>::value, void **> { };

        template <typename X> inline void Init(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp) {
            try {
                ctr = ::std::make_shared<unsigned int>(0);
                sp = ::std::make_shared<X>();
            _COLLECTION_TRANSLATE
        }

        template <typename X, typename A> inline void Init(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp, A&& a) {
            try {
                ctr = ::std::make_shared<unsigned int>(0);
                sp = ::std::make_shared<X>(::std::forward<A>(a));

                ValidateSize(sp->size());
            _COLLECTION_TRANSLATE
        }

        template <typename X, typename A, typename B> inline void Init(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp, A&& a, B&& b) {
            try {
                ctr = ::std::make_shared<unsigned int>(0);
                sp = ::std::make_shared<X>(::std::forward<A>(a), ::std::forward<B>(b));

                ValidateSize(sp->size());
            _COLLECTION_TRANSLATE
        }

        template <typename X, typename A, typename B, typename C> inline void Init(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp, A&& a, B&& b, C&& c) {
            try {
                ctr = ::std::make_shared<unsigned int>(0);
                sp = ::std::make_shared<X>(::std::forward<A>(a), ::std::forward<B>(b), ::std::forward<C>(c));

                ValidateSize(sp->size());
            _COLLECTION_TRANSLATE
        }

        template <typename X, typename A> inline void InitMoveVector(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp, A&& a) {
            Init(ctr, sp, a.begin(), a.end());
        }

        template <typename X> inline void InitMoveVector(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp, X&& x) {
            Init(ctr, sp, ::std::move(x));
        }

        template <typename K, typename V, typename M, typename InIt> inline void EmplaceWrappedRange(M& m, InIt first, InIt last) {
            for ( ; first != last; ++first) {
                ::std::pair<const K, V> p(*first);

                m.emplace(MakeWrap(p.first), MakeWrap(p.second));
            }
        }

        template <typename K, typename V, typename X, typename A> inline void InitMoveMap(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp, A&& a) {
            Init(ctr, sp, a.key_comp());

            EmplaceWrappedRange<K, V>(*sp, a.begin(), a.end());
        }

        template <typename K, typename V, typename X> inline void InitMoveMap(::std::shared_ptr<unsigned int>& ctr, ::std::shared_ptr<X>& sp, X&& x) {
            Init(ctr, sp, ::std::move(x));
        }

        inline void IncrementCounter(::std::shared_ptr<unsigned int>& ctr) {
            if (++*ctr == static_cast<unsigned int>(-1)) {
                // Wraparound is imminent! Create a fresh counter.
                ctr = ::std::make_shared<unsigned int>(0);
            }
        }

        ref class VectorChangedEventArgs sealed : public _COLLECTION_ATTRIBUTES WFC::IVectorChangedEventArgs {
        internal:
            VectorChangedEventArgs(WFC::CollectionChange change, unsigned int index)
                : m_change(change), m_index(index) { }

        public:
            virtual property WFC::CollectionChange CollectionChange {
                virtual WFC::CollectionChange get() {
                    return m_change;
                }
            }

            virtual property unsigned int Index {
                virtual unsigned int get() {
                    if (m_change == WFC::CollectionChange::Reset) {
                        throw ref new FailureException;
                    }

                    return m_index;
                }
            }

        private:
            WFC::CollectionChange m_change;
            unsigned int m_index;
        };

        template <typename K> ref class MapChangedEventArgsReset sealed : public _COLLECTION_ATTRIBUTES WFC::IMapChangedEventArgs<K> {
        public:
            virtual property WFC::CollectionChange CollectionChange {
                virtual WFC::CollectionChange get() {
                    return WFC::CollectionChange::Reset;
                }
            }

            virtual property K Key {
                virtual K get() {
                    throw ref new FailureException;
                }
            }
        };

        template <typename K> ref class MapChangedEventArgs sealed : public _COLLECTION_ATTRIBUTES WFC::IMapChangedEventArgs<K> {
        internal:
            MapChangedEventArgs(WFC::CollectionChange change, K key)
                : m_change(change), m_key(key) { }

        public:
            virtual property WFC::CollectionChange CollectionChange {
                virtual WFC::CollectionChange get() {
                    return m_change;
                }
            }

            virtual property K Key {
                virtual K get() {
                    return Unwrap(m_key);
                }
            }

        private:
            WFC::CollectionChange m_change;
            typename Wrap<K>::type m_key;
        };

        template <typename T, typename E> inline bool VectorIndexOf(const ::std::vector<typename Wrap<T>::type>& v, T value, unsigned int * index) {
            auto pred = [&](const typename Wrap<T>::type& elem) { return E()(Unwrap(elem), value); };

            *index = static_cast<unsigned int>(::std::find_if(v.begin(), v.end(), pred) - v.begin());

            return *index < v.size();
        }

#if _COLLECTION_WUXI
        template <typename T> struct is_hat : public ::std::false_type { };

        template <typename U> struct is_hat<U^> : public ::std::true_type { };

        template <typename T, typename E> inline bool VectorBindableIndexOf(::std::false_type, const ::std::vector<typename Wrap<T>::type>& v, Object^ o, unsigned int * index) {
            IBox<T>^ ib = dynamic_cast<IBox<T>^>(o);

            if (ib) {
                return VectorIndexOf<T, E>(v, ib->Value, index);
            } else {
                *index = static_cast<unsigned int>(v.size());
                return false;
            }
        }

        template <typename T, typename E> inline bool VectorBindableIndexOf(::std::true_type, const ::std::vector<typename Wrap<T>::type>& v, Object^ o, unsigned int * index) {
            T t = dynamic_cast<T>(o);

            if (!o || t) {
                return VectorIndexOf<T, E>(v, t, index);
            } else {
                *index = static_cast<unsigned int>(v.size());
                return false;
            }
        }

        template <typename T, typename E> inline bool VectorBindableIndexOf(const ::std::vector<typename Wrap<T>::type>& v, Object^ o, unsigned int * index) {
            return VectorBindableIndexOf<T, E>(is_hat<T>(), v, o, index);
        }
#endif // _COLLECTION_WUXI

        template <typename T> inline unsigned int VectorGetMany(const ::std::vector<typename Wrap<T>::type>& v, unsigned int startIndex, WriteOnlyArray<T>^ dest) {
            unsigned int capacity = dest->Length;

            unsigned int actual = static_cast<unsigned int>(v.size()) - startIndex;

            if (actual > capacity) {
                actual = capacity;
            }

            for (unsigned int i = 0; i < actual; ++i) {
                dest->set(i, Unwrap(v[startIndex + i]));
            }

            return actual;
        }

        template <typename T> ref class IteratorForVectorView sealed
            : public _COLLECTION_ATTRIBUTES WFC::IIterator<T>
#if _COLLECTION_WUXI
            , public WUXI::IBindableIterator
#endif // _COLLECTION_WUXI
        {
        private:
            typedef ::std::vector<typename Wrap<T>::type> WrappedVector;
            typedef WFC::IIterator<T> WFC_Base;

#if _COLLECTION_WUXI
            typedef WUXI::IBindableIterator WUXI_Base;
#endif // _COLLECTION_WUXI

        internal:
            IteratorForVectorView(const ::std::shared_ptr<unsigned int>& ctr, const ::std::shared_ptr<WrappedVector>& vec)
                : m_ctr(ctr), m_vec(vec), m_good_ctr(*ctr), m_index(0) { }

        public:
            virtual property T Current {
                virtual T get() = WFC_Base::Current::get {
                    ValidateCounter(m_ctr, m_good_ctr);

                    ValidateBounds(m_index < m_vec->size());

                    return Unwrap((*m_vec)[m_index]);
                }
            }

            virtual property bool HasCurrent {
                virtual bool get() {
                    ValidateCounter(m_ctr, m_good_ctr);

                    return m_index < m_vec->size();
                }
            }

            virtual bool MoveNext() {
                ValidateCounter(m_ctr, m_good_ctr);

                ValidateBounds(m_index < m_vec->size());

                ++m_index;
                return m_index < m_vec->size();
            }

            virtual unsigned int GetMany(WriteOnlyArray<T>^ dest) {
                ValidateCounter(m_ctr, m_good_ctr);

                unsigned int actual = VectorGetMany(*m_vec, m_index, dest);

                m_index += actual;

                return actual;
            }

        private:

#if _COLLECTION_WUXI
            virtual Object^ BindableCurrent() = WUXI_Base::Current::get {
                return Current;
            }
#endif // _COLLECTION_WUXI

            ::std::shared_ptr<unsigned int> m_ctr;
            ::std::shared_ptr<WrappedVector> m_vec;
            unsigned int m_good_ctr;
            unsigned int m_index;
        };
    } // namespace Details

    template <typename T, typename E = ::std::equal_to<T>, bool = __is_valid_winrt_type(T)> ref class Vector;
    template <typename T, typename E = ::std::equal_to<T>, bool = __is_valid_winrt_type(T)> ref class VectorView;

    template <typename T, typename E> ref class VectorView<T, E, false> {
        static_assert(Details::AlwaysFalse<T>::value, "Platform::Collections::VectorView<T, E> requires T to be a valid Windows Runtime type.");
    };

    template <typename T, typename E, bool> ref class VectorView sealed
        : public _COLLECTION_ATTRIBUTES Details::WFC::IVectorView<T>
#if _COLLECTION_WUXI
        , public Details::WUXI::IBindableVectorView
#endif // _COLLECTION_WUXI
    {
    private:
        typedef ::std::vector<typename Details::Wrap<T>::type> WrappedVector;
        typedef Details::WFC::IVectorView<T> WFC_Base;

#if _COLLECTION_WUXI
        typedef Details::WUXI::IBindableVectorView WUXI_Base;
#endif // _COLLECTION_WUXI

    internal:
        VectorView() {
            Details::Init(m_ctr, m_vec);

            m_good_ctr = 0;
        }

        explicit VectorView(unsigned int size) {
            Details::Init(m_ctr, m_vec, size);

            m_good_ctr = 0;
        }

        VectorView(unsigned int size, T value) {
            Details::Init(m_ctr, m_vec, size, Details::MakeWrap(value));

            m_good_ctr = 0;
        }

        template <typename U> explicit VectorView(const ::std::vector<U>& v, typename Details::VectorEnableIf<T, U>::type = nullptr) {
            Details::Init(m_ctr, m_vec, v.begin(), v.end());

            m_good_ctr = 0;
        }

        template <typename U> explicit VectorView(::std::vector<U>&& v, typename Details::VectorEnableIf<T, U>::type = nullptr) {
            Details::InitMoveVector(m_ctr, m_vec, ::std::move(v));

            m_good_ctr = 0;
        }

        VectorView(const T * ptr, unsigned int size) {
            Details::Init(m_ctr, m_vec, ptr, ptr + size);

            m_good_ctr = 0;
        }

        template <size_t N> explicit VectorView(const T (&arr)[N]) {
            Details::Init(m_ctr, m_vec, arr, arr + N);

            m_good_ctr = 0;
        }

        template <size_t N> explicit VectorView(const ::std::array<T, N>& a) {
            Details::Init(m_ctr, m_vec, a.begin(), a.end());

            m_good_ctr = 0;
        }

        explicit VectorView(const Array<T>^ arr) {
            Details::Init(m_ctr, m_vec, arr->begin(), arr->end());

            m_good_ctr = 0;
        }

        template <typename InIt> VectorView(InIt first, InIt last) {
            // SFINAE is unnecessary here.

            Details::Init(m_ctr, m_vec, first, last);

            m_good_ctr = 0;
        }

        VectorView(::std::initializer_list<T> il) {
            Details::Init(m_ctr, m_vec, il.begin(), il.end());

            m_good_ctr = 0;
        }

    public:
        virtual Details::WFC::IIterator<T>^ First() = WFC_Base::First {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            return ref new Details::IteratorForVectorView<T>(m_ctr, m_vec);
        }

        virtual T GetAt(unsigned int index) = WFC_Base::GetAt {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            Details::ValidateBounds(index < m_vec->size());

            return Details::Unwrap((*m_vec)[index]);
        }

        virtual property unsigned int Size {
            virtual unsigned int get() {
                Details::ValidateCounter(m_ctr, m_good_ctr);

                return static_cast<unsigned int>(m_vec->size());
            }
        }

        virtual bool IndexOf(T value, unsigned int * index) = WFC_Base::IndexOf {
            *index = 0;

            Details::ValidateCounter(m_ctr, m_good_ctr);

            return Details::VectorIndexOf<T, E>(*m_vec, value, index);
        }

        virtual unsigned int GetMany(unsigned int startIndex, WriteOnlyArray<T>^ dest) {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            Details::ValidateBounds(startIndex <= m_vec->size());

            return Details::VectorGetMany(*m_vec, startIndex, dest);
        }

    private:
        friend ref class Vector<T, E>;

        VectorView(const ::std::shared_ptr<unsigned int>& ctr, const ::std::shared_ptr<WrappedVector>& vec)
            : m_ctr(ctr), m_vec(vec), m_good_ctr(*ctr) { }

#if _COLLECTION_WUXI
        virtual Details::WUXI::IBindableIterator^ BindableFirst() = WUXI_Base::First {
            return safe_cast<Details::WUXI::IBindableIterator^>(First());
        }

        virtual Object^ BindableGetAt(unsigned int index) = WUXI_Base::GetAt {
            return GetAt(index);
        }

        virtual bool BindableIndexOf(Object^ value, unsigned int * index) = WUXI_Base::IndexOf {
            *index = 0;

            Details::ValidateCounter(m_ctr, m_good_ctr);

            return Details::VectorBindableIndexOf<T, E>(*m_vec, value, index);
        }
#endif // _COLLECTION_WUXI

        ::std::shared_ptr<unsigned int> m_ctr;
        ::std::shared_ptr<WrappedVector> m_vec;
        unsigned int m_good_ctr;
    };

    template <typename T, typename E> ref class Vector<T, E, false> {
        static_assert(Details::AlwaysFalse<T>::value, "Platform::Collections::Vector<T, E> requires T to be a valid Windows Runtime type.");
    };

    template <typename T, typename E, bool> ref class Vector sealed
        : public _COLLECTION_ATTRIBUTES Details::WFC::IObservableVector<T>
#if _COLLECTION_WUXI
        , public Details::WUXI::IBindableObservableVector
#endif // _COLLECTION_WUXI
    {
    private:
        typedef ::std::vector<typename Details::Wrap<T>::type> WrappedVector;
        typedef Details::WFC::IObservableVector<T> WFC_Base;
        typedef Details::WFC::VectorChangedEventHandler<T> WFC_Handler;

#if _COLLECTION_WUXI
        typedef Details::WUXI::IBindableObservableVector WUXI_Base;
        typedef Details::WUXI::BindableVectorChangedEventHandler WUXI_Handler;
#endif // _COLLECTION_WUXI

    internal:
        Vector() {
            Details::Init(m_ctr, m_vec);

            m_observed = false;
        }

        explicit Vector(unsigned int size) {
            Details::Init(m_ctr, m_vec, size);

            m_observed = false;
        }

        Vector(unsigned int size, T value) {
            Details::Init(m_ctr, m_vec, size, Details::MakeWrap(value));

            m_observed = false;
        }

        template <typename U> explicit Vector(const ::std::vector<U>& v, typename Details::VectorEnableIf<T, U>::type = nullptr) {
            Details::Init(m_ctr, m_vec, v.begin(), v.end());

            m_observed = false;
        }

        template <typename U> explicit Vector(::std::vector<U>&& v, typename Details::VectorEnableIf<T, U>::type = nullptr) {
            Details::InitMoveVector(m_ctr, m_vec, ::std::move(v));

            m_observed = false;
        }

        Vector(const T * ptr, unsigned int size) {
            Details::Init(m_ctr, m_vec, ptr, ptr + size);

            m_observed = false;
        }

        template <size_t N> explicit Vector(const T (&arr)[N]) {
            Details::Init(m_ctr, m_vec, arr, arr + N);

            m_observed = false;
        }

        template <size_t N> explicit Vector(const ::std::array<T, N>& a) {
            Details::Init(m_ctr, m_vec, a.begin(), a.end());

            m_observed = false;
        }

        explicit Vector(const Array<T>^ arr) {
            Details::Init(m_ctr, m_vec, arr->begin(), arr->end());

            m_observed = false;
        }

        template <typename InIt> Vector(InIt first, InIt last) {
            // SFINAE is unnecessary here.

            Details::Init(m_ctr, m_vec, first, last);

            m_observed = false;
        }

        Vector(::std::initializer_list<T> il) {
            Details::Init(m_ctr, m_vec, il.begin(), il.end());

            m_observed = false;
        }

    public:
        virtual Details::WFC::IIterator<T>^ First() = WFC_Base::First {
            return ref new Details::IteratorForVectorView<T>(m_ctr, m_vec);
        }

        virtual T GetAt(unsigned int index) = WFC_Base::GetAt {
            Details::ValidateBounds(index < m_vec->size());

            return Details::Unwrap((*m_vec)[index]);
        }

        virtual property unsigned int Size {
            virtual unsigned int get() {
                return static_cast<unsigned int>(m_vec->size());
            }
        }

        virtual bool IndexOf(T value, unsigned int * index) = WFC_Base::IndexOf {
            *index = 0;

            return Details::VectorIndexOf<T, E>(*m_vec, value, index);
        }

        virtual unsigned int GetMany(unsigned int startIndex, WriteOnlyArray<T>^ dest) {
            Details::ValidateBounds(startIndex <= m_vec->size());

            return Details::VectorGetMany(*m_vec, startIndex, dest);
        }

        virtual Details::WFC::IVectorView<T>^ GetView() = WFC_Base::GetView {
            return ref new VectorView<T, E>(m_ctr, m_vec);
        }

        virtual void SetAt(unsigned int index, T item) = WFC_Base::SetAt {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateBounds(index < m_vec->size());

                (*m_vec)[index] = item;

                NotifyChanged(index);
            _COLLECTION_TRANSLATE
        }

        virtual void InsertAt(unsigned int index, T item) = WFC_Base::InsertAt {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateBounds(index <= m_vec->size());

                Details::ValidateSize(m_vec->size() + 1);

                Emplace(m_vec->begin() + index, item, ::std::is_same<T, bool>());

                NotifyInserted(index);
            _COLLECTION_TRANSLATE
        }

        virtual void Append(T item) = WFC_Base::Append {
            try {
                Details::IncrementCounter(m_ctr);

                size_t n = m_vec->size();

                Details::ValidateSize(n + 1);

                EmplaceBack(item, ::std::is_same<T, bool>());

                NotifyInserted(static_cast<unsigned int>(n));
            _COLLECTION_TRANSLATE
        }

        virtual void RemoveAt(unsigned int index) {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateBounds(index < m_vec->size());

                m_vec->erase(m_vec->begin() + index);

                NotifyRemoved(index);
            _COLLECTION_TRANSLATE
        }

        virtual void RemoveAtEnd() {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateBounds(!m_vec->empty());

                m_vec->pop_back();

                NotifyRemoved(static_cast<unsigned int>(m_vec->size()));
            _COLLECTION_TRANSLATE
        }

        virtual void Clear() {
            try {
                Details::IncrementCounter(m_ctr);

                m_vec->clear();

                NotifyReset();
            _COLLECTION_TRANSLATE
        }

        virtual void ReplaceAll(const Array<T>^ arr) {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateSize(arr->Length);

                m_vec->assign(arr->begin(), arr->end());

                NotifyReset();
            _COLLECTION_TRANSLATE
        }

        virtual event WFC_Handler^ VectorChanged {
            virtual Details::Token add(WFC_Handler^ e) = WFC_Base::VectorChanged::add {
                m_observed = true;
                return m_wfc_event += e;
            }

            virtual void remove(Details::Token t) = WFC_Base::VectorChanged::remove {
                m_wfc_event -= t;
            }
        };

    private:
        template <typename A, typename B> void Emplace(A&& a, B&& b, ::std::false_type) {
            m_vec->emplace(::std::forward<A>(a), ::std::forward<B>(b));
        }

        template <typename A, typename B> void Emplace(A&& a, B&& b, ::std::true_type) {
            m_vec->insert(::std::forward<A>(a), ::std::forward<B>(b));
        }

        template <typename A> void EmplaceBack(A&& a, ::std::false_type) {
            m_vec->emplace_back(::std::forward<A>(a));
        }

        template <typename A> void EmplaceBack(A&& a, ::std::true_type) {
            m_vec->push_back(::std::forward<A>(a));
        }

        void Notify(Details::WFC::CollectionChange change, unsigned int index) {
            if (m_observed) {
                auto args = ref new Details::VectorChangedEventArgs(change, index);
                m_wfc_event(this, args);

#if _COLLECTION_WUXI
                m_wuxi_event(this, args);
#endif // _COLLECTION_WUXI

            }
        }

        void NotifyReset() {
            Notify(Details::WFC::CollectionChange::Reset, 0);
        }

        void NotifyInserted(unsigned int index) {
            Notify(Details::WFC::CollectionChange::ItemInserted, index);
        }

        void NotifyRemoved(unsigned int index) {
            Notify(Details::WFC::CollectionChange::ItemRemoved, index);
        }

        void NotifyChanged(unsigned int index) {
            Notify(Details::WFC::CollectionChange::ItemChanged, index);
        }

#if _COLLECTION_WUXI
        virtual Details::WUXI::IBindableIterator^ BindableFirst() = WUXI_Base::First {
            return safe_cast<Details::WUXI::IBindableIterator^>(First());
        }

        virtual Object^ BindableGetAt(unsigned int index) = WUXI_Base::GetAt {
            return GetAt(index);
        }

        virtual bool BindableIndexOf(Object^ value, unsigned int * index) = WUXI_Base::IndexOf {
            *index = 0;

            return Details::VectorBindableIndexOf<T, E>(*m_vec, value, index);
        }

        virtual Details::WUXI::IBindableVectorView^ BindableGetView() = WUXI_Base::GetView {
            return safe_cast<Details::WUXI::IBindableVectorView^>(GetView());
        }

        virtual void BindableSetAt(unsigned int index, Object^ item) = WUXI_Base::SetAt {
            SetAt(index, safe_cast<T>(item));
        }

        virtual void BindableInsertAt(unsigned int index, Object^ item) = WUXI_Base::InsertAt {
            InsertAt(index, safe_cast<T>(item));
        }

        virtual void BindableAppend(Object^ item) = WUXI_Base::Append {
            Append(safe_cast<T>(item));
        }

        virtual Details::Token BindableEventAdd(WUXI_Handler^ e) = WUXI_Base::VectorChanged::add {
            m_observed = true;
            return m_wuxi_event += e;
        }

        virtual void BindableEventRemove(Details::Token t) = WUXI_Base::VectorChanged::remove {
            m_wuxi_event -= t;
        }
#endif // _COLLECTION_WUXI

        ::std::shared_ptr<unsigned int> m_ctr;
        ::std::shared_ptr<WrappedVector> m_vec;
        bool m_observed;

        event WFC_Handler^ m_wfc_event;

#if _COLLECTION_WUXI
        event WUXI_Handler^ m_wuxi_event;
#endif // _COLLECTION_WUXI

    };


    namespace Details {
        template <typename K, typename V> ref class KeyValuePair sealed : public _COLLECTION_ATTRIBUTES WFC::IKeyValuePair<K, V> {
        internal:
            KeyValuePair(const typename Wrap<K>::type& key, const typename Wrap<V>::type& value)
                : m_key(key), m_value(value) { }

        public:
            virtual property K Key {
                virtual K get() {
                    return Unwrap(m_key);
                }
            }

            virtual property V Value {
                virtual V get() {
                    return Unwrap(m_value);
                }
            }

        private:
            typename Wrap<K>::type m_key;
            typename Wrap<V>::type m_value;
        };

        template <typename K, typename F, bool = ::std::is_same<typename Wrap<K>::type, K>::value> class WrapFunc {
        public:
            typedef F type;
        };

        template <typename K, typename F> class WrapFunc<K, F, false> {
        public:
            typedef WrapFunc type;

            WrapFunc(const F& func)
                : m_func(func) { }

            size_t operator()(const Agile<K>& k) const {
                return m_func(k.Get());
            }

            bool operator()(const Agile<K>& l, const Agile<K>& r) const {
                return m_func(l.Get(), r.Get());
            }

        private:
            F m_func;
        };

        template <typename K, typename V, typename C> struct WrapMap {
            typedef ::std::map<typename Wrap<K>::type, typename Wrap<V>::type, typename WrapFunc<K, C>::type> type;
        };

        template <typename K, typename V, typename H, typename P> struct WrapUnorderedMap {
            typedef ::std::unordered_map<typename Wrap<K>::type, typename Wrap<V>::type, typename WrapFunc<K, H>::type, typename WrapFunc<K, P>::type> type;
        };

        template <typename K, typename V, typename WrappedMap> ref class IteratorForAnyMapView sealed : public _COLLECTION_ATTRIBUTES WFC::IIterator<WFC::IKeyValuePair<K, V>^> {

        internal:
            IteratorForAnyMapView(const ::std::shared_ptr<unsigned int>& ctr, const ::std::shared_ptr<WrappedMap>& m)
                : m_ctr(ctr), m_map(m), m_good_ctr(*ctr), m_iter(m->begin()) { }

        public:
            virtual property WFC::IKeyValuePair<K, V>^ Current {
                virtual WFC::IKeyValuePair<K, V>^ get() {
                    ValidateCounter(m_ctr, m_good_ctr);

                    ValidateBounds(m_iter != m_map->end());

                    return ref new KeyValuePair<K, V>(m_iter->first, m_iter->second);
                }
            }

            virtual property bool HasCurrent {
                virtual bool get() {
                    ValidateCounter(m_ctr, m_good_ctr);

                    return m_iter != m_map->end();
                }
            }

            virtual bool MoveNext() {
                ValidateCounter(m_ctr, m_good_ctr);

                ValidateBounds(m_iter != m_map->end());

                ++m_iter;
                return m_iter != m_map->end();
            }

            virtual unsigned int GetMany(WriteOnlyArray<WFC::IKeyValuePair<K, V>^>^ dest) {
                ValidateCounter(m_ctr, m_good_ctr);

                unsigned int capacity = dest->Length;

                unsigned int actual = 0;

                while (capacity > 0 && m_iter != m_map->end()) {
                    dest->set(actual, ref new KeyValuePair<K, V>(m_iter->first, m_iter->second));
                    ++m_iter;
                    --capacity;
                    ++actual;
                }

                return actual;
            }

        private:
            ::std::shared_ptr<unsigned int> m_ctr;
            ::std::shared_ptr<WrappedMap> m_map;
            unsigned int m_good_ctr;
            typename WrappedMap::const_iterator m_iter;
        };
    } // namespace Details

    template <typename K, typename V, typename C = ::std::less<K>, bool = __is_valid_winrt_type(K), bool = __is_valid_winrt_type(V)> ref class Map;
    template <typename K, typename V, typename C = ::std::less<K>, bool = __is_valid_winrt_type(K), bool = __is_valid_winrt_type(V)> ref class MapView;
    template <typename K, typename V, typename H = ::std::hash<K>, typename P = ::std::equal_to<K>, bool = __is_valid_winrt_type(K), bool = __is_valid_winrt_type(V)> ref class UnorderedMap;
    template <typename K, typename V, typename H = ::std::hash<K>, typename P = ::std::equal_to<K>, bool = __is_valid_winrt_type(K), bool = __is_valid_winrt_type(V)> ref class UnorderedMapView;

    template <typename K, typename V, typename C> ref class MapView<K, V, C, false, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::MapView<K, V, C> requires K and V to be valid Windows Runtime types.");
    };

    template <typename K, typename V, typename C> ref class MapView<K, V, C, false, true> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::MapView<K, V, C> requires K to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename C> ref class MapView<K, V, C, true, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::MapView<K, V, C> requires V to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename C, bool, bool> ref class MapView sealed : public _COLLECTION_ATTRIBUTES Details::WFC::IMapView<K, V> {
    private:
        typedef      typename Details::WrapMap<K, V, C>::type    WrappedMap;
        typedef Details::IteratorForAnyMapView<K, V, WrappedMap> MyIterator;
                          friend ref class Map<K, V, C>;

    internal:
        explicit MapView(const C& comp = C()) {
            Details::Init(m_ctr, m_map, comp);

            m_good_ctr = 0;
        }

        explicit MapView(const ::std::map<K, V, C>& m) {
            Details::Init(m_ctr, m_map, m.key_comp());

            Details::EmplaceWrappedRange<K, V>(*m_map, m.begin(), m.end());

            m_good_ctr = 0;
        }

        explicit MapView(::std::map<K, V, C>&& m) {
            Details::InitMoveMap<K, V>(m_ctr, m_map, ::std::move(m));

            m_good_ctr = 0;
        }

        template <typename InIt> MapView(InIt first, InIt last, const C& comp = C()) {
            Details::Init(m_ctr, m_map, comp);

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_good_ctr = 0;
        }

        MapView(::std::initializer_list< ::std::pair<const K, V>> il, const C& comp = C()) {
            Details::Init(m_ctr, m_map, comp);

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_good_ctr = 0;
        }

    public:
        virtual Details::WFC::IIterator<Details::WFC::IKeyValuePair<K, V>^>^ First() {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            return ref new MyIterator(m_ctr, m_map);
        }

        virtual V Lookup(K key) {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            auto i = m_map->find(Details::MakeWrap(key));

            Details::ValidateBounds(i != m_map->end());

            return Details::Unwrap(i->second);
        }

        virtual property unsigned int Size {
            virtual unsigned int get() {
                Details::ValidateCounter(m_ctr, m_good_ctr);

                return static_cast<unsigned int>(m_map->size());
            }
        }

        virtual bool HasKey(K key) {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            return m_map->find(Details::MakeWrap(key)) != m_map->end();
        }

        virtual void Split(Details::WFC::IMapView<K, V>^ * firstPartition, Details::WFC::IMapView<K, V>^ * secondPartition) {
            *firstPartition = nullptr;
            *secondPartition = nullptr;

            Details::ValidateCounter(m_ctr, m_good_ctr);
        }

    private:
        MapView(const ::std::shared_ptr<unsigned int>& ctr, const ::std::shared_ptr<WrappedMap>& m)
            : m_ctr(ctr), m_map(m), m_good_ctr(*ctr) { }

        ::std::shared_ptr<unsigned int> m_ctr;
        ::std::shared_ptr<WrappedMap> m_map;
        unsigned int m_good_ctr;
    };

    template <typename K, typename V, typename H, typename P> ref class UnorderedMapView<K, V, H, P, false, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::UnorderedMapView<K, V, H, P> requires K and V to be valid Windows Runtime types.");
    };

    template <typename K, typename V, typename H, typename P> ref class UnorderedMapView<K, V, H, P, false, true> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::UnorderedMapView<K, V, H, P> requires K to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename H, typename P> ref class UnorderedMapView<K, V, H, P, true, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::UnorderedMapView<K, V, H, P> requires V to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename H, typename P, bool, bool> ref class UnorderedMapView sealed
        : public _COLLECTION_ATTRIBUTES Details::WFC::IMapView<K, V> {
    private:
        typedef typename Details::WrapUnorderedMap<K, V, H, P>::type    WrappedMap;
        typedef     Details::IteratorForAnyMapView<K, V, WrappedMap>    MyIterator;
                     friend ref class UnorderedMap<K, V, H, P>;

    internal:
        UnorderedMapView() {
            Details::Init(m_ctr, m_map);

            m_good_ctr = 0;
        }

        explicit UnorderedMapView(size_t n) {
            Details::Init(m_ctr, m_map, n, H(), P());

            m_good_ctr = 0;
        }

        UnorderedMapView(size_t n, const H& h) {
            Details::Init(m_ctr, m_map, n, h, P());

            m_good_ctr = 0;
        }

        UnorderedMapView(size_t n, const H& h, const P& p) {
            Details::Init(m_ctr, m_map, n, h, p);

            m_good_ctr = 0;
        }

        explicit UnorderedMapView(const ::std::unordered_map<K, V, H, P>& m) {
            Details::Init(m_ctr, m_map, m.bucket_count(), m.hash_function(), m.key_eq());

            Details::EmplaceWrappedRange<K, V>(*m_map, m.begin(), m.end());

            m_good_ctr = 0;
        }

        explicit UnorderedMapView(::std::unordered_map<K, V, H, P>&& m) {
            Details::InitMoveMap<K, V>(m_ctr, m_map, ::std::move(m));

            m_good_ctr = 0;
        }

        template <typename InIt> UnorderedMapView(InIt first, InIt last) {
            Details::Init(m_ctr, m_map);

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_good_ctr = 0;
        }

        template <typename InIt> UnorderedMapView(InIt first, InIt last, size_t n) {
            Details::Init(m_ctr, m_map, n, H(), P());

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_good_ctr = 0;
        }

        template <typename InIt> UnorderedMapView(InIt first, InIt last, size_t n, const H& h) {
            Details::Init(m_ctr, m_map, n, h, P());

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_good_ctr = 0;
        }

        template <typename InIt> UnorderedMapView(InIt first, InIt last, size_t n, const H& h, const P& p) {
            Details::Init(m_ctr, m_map, n, h, p);

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_good_ctr = 0;
        }

        UnorderedMapView(::std::initializer_list< ::std::pair<const K, V>> il) {
            Details::Init(m_ctr, m_map);

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_good_ctr = 0;
        }

        UnorderedMapView(::std::initializer_list< ::std::pair<const K, V>> il, size_t n) {
            Details::Init(m_ctr, m_map, n, H(), P());

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_good_ctr = 0;
        }

        UnorderedMapView(::std::initializer_list< ::std::pair<const K, V>> il, size_t n, const H& h) {
            Details::Init(m_ctr, m_map, n, h, P());

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_good_ctr = 0;
        }

        UnorderedMapView(::std::initializer_list< ::std::pair<const K, V>> il, size_t n, const H& h, const P& p) {
            Details::Init(m_ctr, m_map, n, h, p);

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_good_ctr = 0;
        }

    public:
        virtual Details::WFC::IIterator<Details::WFC::IKeyValuePair<K, V>^>^ First() {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            return ref new MyIterator(m_ctr, m_map);
        }

        virtual V Lookup(K key) {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            auto i = m_map->find(Details::MakeWrap(key));

            Details::ValidateBounds(i != m_map->end());

            return Details::Unwrap(i->second);
        }

        virtual property unsigned int Size {
            virtual unsigned int get() {
                Details::ValidateCounter(m_ctr, m_good_ctr);

                return static_cast<unsigned int>(m_map->size());
            }
        }

        virtual bool HasKey(K key) {
            Details::ValidateCounter(m_ctr, m_good_ctr);

            return m_map->find(Details::MakeWrap(key)) != m_map->end();
        }

        virtual void Split(Details::WFC::IMapView<K, V>^ * firstPartition, Details::WFC::IMapView<K, V>^ * secondPartition) {
            *firstPartition = nullptr;
            *secondPartition = nullptr;

            Details::ValidateCounter(m_ctr, m_good_ctr);
        }

    private:
        UnorderedMapView(const ::std::shared_ptr<unsigned int>& ctr, const ::std::shared_ptr<WrappedMap>& m)
            : m_ctr(ctr), m_map(m), m_good_ctr(*ctr) { }

        ::std::shared_ptr<unsigned int> m_ctr;
        ::std::shared_ptr<WrappedMap> m_map;
        unsigned int m_good_ctr;
    };

    template <typename K, typename V, typename C> ref class Map<K, V, C, false, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::Map<K, V, C> requires K and V to be valid Windows Runtime types.");
    };

    template <typename K, typename V, typename C> ref class Map<K, V, C, false, true> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::Map<K, V, C> requires K to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename C> ref class Map<K, V, C, true, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::Map<K, V, C> requires V to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename C, bool, bool> ref class Map sealed : public _COLLECTION_ATTRIBUTES Details::WFC::IObservableMap<K, V> {
    private:
        typedef             typename Details::WrapMap<K, V, C>::type        WrappedMap;
        typedef        Details::IteratorForAnyMapView<K, V, WrappedMap>     MyIterator;
        typedef                               MapView<K, V, C>              MyView;
        typedef  Details::WFC::MapChangedEventHandler<K, V>                 WFC_Handler;

    internal:
        explicit Map(const C& comp = C()) {
            Details::Init(m_ctr, m_map, comp);

            m_observed = false;
        }

        explicit Map(const ::std::map<K, V, C>& m) {
            Details::Init(m_ctr, m_map, m.key_comp());

            Details::EmplaceWrappedRange<K, V>(*m_map, m.begin(), m.end());

            m_observed = false;
        }

        explicit Map(::std::map<K, V, C>&& m) {
            Details::InitMoveMap<K, V>(m_ctr, m_map, ::std::move(m));

            m_observed = false;
        }

        template <typename InIt> Map(InIt first, InIt last, const C& comp = C()) {
            Details::Init(m_ctr, m_map, comp);

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_observed = false;
        }

        Map(::std::initializer_list< ::std::pair<const K, V>> il, const C& comp = C()) {
            Details::Init(m_ctr, m_map, comp);

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_observed = false;
        }

    public:
        virtual Details::WFC::IIterator<Details::WFC::IKeyValuePair<K, V>^>^ First() {
            return ref new MyIterator(m_ctr, m_map);
        }

        virtual V Lookup(K key) {
            auto i = m_map->find(Details::MakeWrap(key));

            Details::ValidateBounds(i != m_map->end());

            return Details::Unwrap(i->second);
        }

        virtual property unsigned int Size {
            virtual unsigned int get() {
                return static_cast<unsigned int>(m_map->size());
            }
        }

        virtual bool HasKey(K key) {
            return m_map->find(Details::MakeWrap(key)) != m_map->end();
        }

        virtual Details::WFC::IMapView<K, V>^ GetView() {
            return ref new MyView(m_ctr, m_map);
        }

        virtual bool Insert(K key, V value) {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateSize(m_map->size() + 1);

                auto p = m_map->emplace(Details::MakeWrap(key), Details::MakeWrap(value));

                if (p.second) {
                    NotifyInserted(key);
                } else {
                    p.first->second = value;
                    NotifyChanged(key);
                }

                return !p.second;
            _COLLECTION_TRANSLATE
        }

        virtual void Remove(K key) {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateBounds(m_map->erase(Details::MakeWrap(key)) == 1);

                NotifyRemoved(key);
            _COLLECTION_TRANSLATE
        }

        virtual void Clear() {
            try {
                Details::IncrementCounter(m_ctr);

                m_map->clear();

                NotifyReset();
            _COLLECTION_TRANSLATE
        }

        virtual event WFC_Handler^ MapChanged {
            virtual Details::Token add(WFC_Handler^ e) {
                m_observed = true;
                return m_wfc_event += e;
            }

            virtual void remove(Details::Token t) {
                m_wfc_event -= t;
            }
        };

    private:
        void NotifyReset() {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgsReset<K>);
            }
        }

        void NotifyInserted(K key) {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgs<K>(Details::WFC::CollectionChange::ItemInserted, key));
            }
        }

        void NotifyRemoved(K key) {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgs<K>(Details::WFC::CollectionChange::ItemRemoved, key));
            }
        }

        void NotifyChanged(K key) {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgs<K>(Details::WFC::CollectionChange::ItemChanged, key));
            }
        }

        ::std::shared_ptr<unsigned int> m_ctr;
        ::std::shared_ptr<WrappedMap> m_map;
        bool m_observed;

        event WFC_Handler^ m_wfc_event;
    };

    template <typename K, typename V, typename H, typename P> ref class UnorderedMap<K, V, H, P, false, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::UnorderedMap<K, V, H, P> requires K and V to be valid Windows Runtime types.");
    };

    template <typename K, typename V, typename H, typename P> ref class UnorderedMap<K, V, H, P, false, true> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::UnorderedMap<K, V, H, P> requires K to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename H, typename P> ref class UnorderedMap<K, V, H, P, true, false> {
        static_assert(Details::AlwaysFalse<K>::value, "Platform::Collections::UnorderedMap<K, V, H, P> requires V to be a valid Windows Runtime type.");
    };

    template <typename K, typename V, typename H, typename P, bool, bool> ref class UnorderedMap sealed : public _COLLECTION_ATTRIBUTES Details::WFC::IObservableMap<K, V> {
    private:
        typedef   typename Details::WrapUnorderedMap<K, V, H, P>::type  WrappedMap;
        typedef       Details::IteratorForAnyMapView<K, V, WrappedMap>  MyIterator;
        typedef                     UnorderedMapView<K, V, H, P>        MyView;
        typedef Details::WFC::MapChangedEventHandler<K, V>              WFC_Handler;

    internal:
        UnorderedMap() {
            Details::Init(m_ctr, m_map);

            m_observed = false;
        }

        explicit UnorderedMap(size_t n) {
            Details::Init(m_ctr, m_map, n, H(), P());

            m_observed = false;
        }

        UnorderedMap(size_t n, const H& h) {
            Details::Init(m_ctr, m_map, n, h, P());

            m_observed = false;
        }

        UnorderedMap(size_t n, const H& h, const P& p) {
            Details::Init(m_ctr, m_map, n, h, p);

            m_observed = false;
        }

        explicit UnorderedMap(const ::std::unordered_map<K, V, H, P>& m) {
            Details::Init(m_ctr, m_map, m.bucket_count(), m.hash_function(), m.key_eq());

            Details::EmplaceWrappedRange<K, V>(*m_map, m.begin(), m.end());

            m_observed = false;
        }

        explicit UnorderedMap(::std::unordered_map<K, V, H, P>&& m) {
            Details::InitMoveMap<K, V>(m_ctr, m_map, ::std::move(m));

            m_observed = false;
        }

        template <typename InIt> UnorderedMap(InIt first, InIt last) {
            Details::Init(m_ctr, m_map);

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_observed = false;
        }

        template <typename InIt> UnorderedMap(InIt first, InIt last, size_t n) {
            Details::Init(m_ctr, m_map, n, H(), P());

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_observed = false;
        }

        template <typename InIt> UnorderedMap(InIt first, InIt last, size_t n, const H& h) {
            Details::Init(m_ctr, m_map, n, h, P());

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_observed = false;
        }

        template <typename InIt> UnorderedMap(InIt first, InIt last, size_t n, const H& h, const P& p) {
            Details::Init(m_ctr, m_map, n, h, p);

            Details::EmplaceWrappedRange<K, V>(*m_map, first, last);

            m_observed = false;
        }

        UnorderedMap(::std::initializer_list< ::std::pair<const K, V>> il) {
            Details::Init(m_ctr, m_map);

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_observed = false;
        }

        UnorderedMap(::std::initializer_list< ::std::pair<const K, V>> il, size_t n) {
            Details::Init(m_ctr, m_map, n, H(), P());

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_observed = false;
        }

        UnorderedMap(::std::initializer_list< ::std::pair<const K, V>> il, size_t n, const H& h) {
            Details::Init(m_ctr, m_map, n, h, P());

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_observed = false;
        }

        UnorderedMap(::std::initializer_list< ::std::pair<const K, V>> il, size_t n, const H& h, const P& p) {
            Details::Init(m_ctr, m_map, n, h, p);

            Details::EmplaceWrappedRange<K, V>(*m_map, il.begin(), il.end());

            m_observed = false;
        }

    public:
        virtual Details::WFC::IIterator<Details::WFC::IKeyValuePair<K, V>^>^ First() {
            return ref new MyIterator(m_ctr, m_map);
        }

        virtual V Lookup(K key) {
            auto i = m_map->find(Details::MakeWrap(key));

            Details::ValidateBounds(i != m_map->end());

            return Details::Unwrap(i->second);
        }

        virtual property unsigned int Size {
            virtual unsigned int get() {
                return static_cast<unsigned int>(m_map->size());
            }
        }

        virtual bool HasKey(K key) {
            return m_map->find(Details::MakeWrap(key)) != m_map->end();
        }

        virtual Details::WFC::IMapView<K, V>^ GetView() {
            return ref new MyView(m_ctr, m_map);
        }

        virtual bool Insert(K key, V value) {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateSize(m_map->size() + 1);

                auto p = m_map->emplace(Details::MakeWrap(key), Details::MakeWrap(value));

                if (p.second) {
                    NotifyInserted(key);
                } else {
                    p.first->second = value;
                    NotifyChanged(key);
                }

                return !p.second;
            _COLLECTION_TRANSLATE
        }

        virtual void Remove(K key) {
            try {
                Details::IncrementCounter(m_ctr);

                Details::ValidateBounds(m_map->erase(Details::MakeWrap(key)) == 1);

                NotifyRemoved(key);
            _COLLECTION_TRANSLATE
        }

        virtual void Clear() {
            try {
                Details::IncrementCounter(m_ctr);

                m_map->clear();

                NotifyReset();
            _COLLECTION_TRANSLATE
        }

        virtual event WFC_Handler^ MapChanged {
            virtual Details::Token add(WFC_Handler^ e) {
                m_observed = true;
                return m_wfc_event += e;
            }

            virtual void remove(Details::Token t) {
                m_wfc_event -= t;
            }
        };

    private:
        void NotifyReset() {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgsReset<K>);
            }
        }

        void NotifyInserted(K key) {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgs<K>(Details::WFC::CollectionChange::ItemInserted, key));
            }
        }

        void NotifyRemoved(K key) {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgs<K>(Details::WFC::CollectionChange::ItemRemoved, key));
            }
        }

        void NotifyChanged(K key) {
            if (m_observed) {
                m_wfc_event(this, ref new Details::MapChangedEventArgs<K>(Details::WFC::CollectionChange::ItemChanged, key));
            }
        }

        ::std::shared_ptr<unsigned int> m_ctr;
        ::std::shared_ptr<WrappedMap> m_map;
        bool m_observed;

        event WFC_Handler^ m_wfc_event;
    };


    template <typename X> class InputIterator;
    template <typename T> class VectorIterator;
    template <typename T> class VectorViewIterator;
    template <typename T> class BackInsertIterator;
  } // namespace Collections
} // namespace Platform

namespace Platform {
  namespace Collections {
    template <typename X> class InputIterator {
    public:
        typedef ::std::input_iterator_tag iterator_category;
        typedef                         X value_type;
        typedef                 ptrdiff_t difference_type;
        typedef                 const X * pointer;
        typedef                 const X & reference;

        InputIterator() { }

        explicit InputIterator(Details::WFC::IIterator<X>^ iter) {
            if (iter->HasCurrent) {
                m_iter = iter;
                m_val = iter->Current;
            }
        }

        bool operator==(const InputIterator& other) const {
            return !!m_iter == !!other.m_iter;
        }

        bool operator!=(const InputIterator& other) const {
            return !(*this == other);
        }

        reference operator*() const {
            return m_val;
        }

        pointer operator->() const {
            return &m_val;
        }

        InputIterator& operator++() {
            if (m_iter->MoveNext()) {
                m_val = m_iter->Current;
            } else {
                m_iter = nullptr;
            }

            return *this;
        }

        InputIterator operator++(int) {
            InputIterator old(*this);
            ++*this;
            return old;
        }

    private:
        Details::WFC::IIterator<X>^ m_iter;
        X m_val;
    };

    namespace Details {
        template <typename T> class VectorProxy {
        public:
            VectorProxy(WFC::IVector<T>^ v, ptrdiff_t n)
                : m_v(v), m_i(static_cast<unsigned int>(n)) { }

            VectorProxy& operator=(const VectorProxy& other) {
                m_v->SetAt(m_i, other.m_v->GetAt(other.m_i));
                return *this;
            }

            VectorProxy& operator=(T t) {
                m_v->SetAt(m_i, t);
                return *this;
            }

            operator T() const {
                return m_v->GetAt(m_i);
            }

            T operator->() const {
                return m_v->GetAt(m_i);
            }

            void swap(const VectorProxy& other) const {
                T t1(m_v->GetAt(m_i));
                T t2(other.m_v->GetAt(other.m_i));

                m_v->SetAt(m_i, t2);
                other.m_v->SetAt(other.m_i, t1);
            }

            void swap(T& t) const {
                T temp(t);
                t = m_v->GetAt(m_i);
                m_v->SetAt(m_i, temp);
            }

        private:
            WFC::IVector<T>^ m_v;
            unsigned int m_i;
        };

        template <typename T> inline void swap(const VectorProxy<T>& l, const VectorProxy<T>& r) {
            l.swap(r);
        }

        template <typename T> inline void swap(const VectorProxy<T>& p, T& t) {
            p.swap(t);
        }

        template <typename T> inline void swap(T& t, const VectorProxy<T>& p) {
            p.swap(t);
        }

        template <typename T> inline bool operator==(const VectorProxy<T>& l, const VectorProxy<T>& r) {
            return static_cast<T>(l) == static_cast<T>(r);
        }

        template <typename T> inline bool operator==(const VectorProxy<T>& l, const T& t) {
            return static_cast<T>(l) == t;
        }

        template <typename T> inline bool operator==(const T& t, const VectorProxy<T>& r) {
            return t == static_cast<T>(r);
        }

        template <typename T> inline bool operator!=(const VectorProxy<T>& l, const VectorProxy<T>& r) {
            return static_cast<T>(l) != static_cast<T>(r);
        }

        template <typename T> inline bool operator!=(const VectorProxy<T>& l, const T& t) {
            return static_cast<T>(l) != t;
        }

        template <typename T> inline bool operator!=(const T& t, const VectorProxy<T>& r) {
            return t != static_cast<T>(r);
        }

        template <typename T> inline bool operator<(const VectorProxy<T>& l, const VectorProxy<T>& r) {
            return static_cast<T>(l) < static_cast<T>(r);
        }

        template <typename T> inline bool operator<(const VectorProxy<T>& l, const T& t) {
            return static_cast<T>(l) < t;
        }

        template <typename T> inline bool operator<(const T& t, const VectorProxy<T>& r) {
            return t < static_cast<T>(r);
        }

        template <typename T> inline bool operator<=(const VectorProxy<T>& l, const VectorProxy<T>& r) {
            return static_cast<T>(l) <= static_cast<T>(r);
        }

        template <typename T> inline bool operator<=(const VectorProxy<T>& l, const T& t) {
            return static_cast<T>(l) <= t;
        }

        template <typename T> inline bool operator<=(const T& t, const VectorProxy<T>& r) {
            return t <= static_cast<T>(r);
        }

        template <typename T> inline bool operator>(const VectorProxy<T>& l, const VectorProxy<T>& r) {
            return static_cast<T>(l) > static_cast<T>(r);
        }

        template <typename T> inline bool operator>(const VectorProxy<T>& l, const T& t) {
            return static_cast<T>(l) > t;
        }

        template <typename T> inline bool operator>(const T& t, const VectorProxy<T>& r) {
            return t > static_cast<T>(r);
        }

        template <typename T> inline bool operator>=(const VectorProxy<T>& l, const VectorProxy<T>& r) {
            return static_cast<T>(l) >= static_cast<T>(r);
        }

        template <typename T> inline bool operator>=(const VectorProxy<T>& l, const T& t) {
            return static_cast<T>(l) >= t;
        }

        template <typename T> inline bool operator>=(const T& t, const VectorProxy<T>& r) {
            return t >= static_cast<T>(r);
        }

        template <typename T> class ArrowProxy {
        public:
            explicit ArrowProxy(T t)
                : m_val(t) { }

            const T * operator->() const {
                return &m_val;
            }

        private:
            T m_val;
        };
    } // namespace Details

    template <typename T> class VectorIterator {
    public:
        typedef ::std::random_access_iterator_tag iterator_category;
        typedef                                 T value_type;
        typedef                         ptrdiff_t difference_type;
        typedef         Details::VectorProxy<T> * pointer;
        typedef         Details::VectorProxy<T>   reference;

        VectorIterator()
            : m_v(nullptr), m_i(0) { }

        explicit VectorIterator(Details::WFC::IVector<T>^ v)
            : m_v(v), m_i(0) { }

        reference operator*() const {
            return reference(m_v, m_i);
        }

        Details::ArrowProxy<T> operator->() const {
            return Details::ArrowProxy<T>(m_v->GetAt(static_cast<unsigned int>(m_i)));
        }

        reference operator[](difference_type n) const {
            return reference(m_v, m_i + n);
        }

        VectorIterator& operator++() {
            ++m_i;
            return *this;
        }

        VectorIterator& operator--() {
            --m_i;
            return *this;
        }

        VectorIterator operator++(int) {
            VectorIterator old(*this);
            ++*this;
            return old;
        }

        VectorIterator operator--(int) {
            VectorIterator old(*this);
            --*this;
            return old;
        }

        VectorIterator& operator+=(difference_type n) {
            m_i += n;
            return *this;
        }

        VectorIterator& operator-=(difference_type n) {
            m_i -= n;
            return *this;
        }

        VectorIterator operator+(difference_type n) const {
            VectorIterator ret(*this);
            ret += n;
            return ret;
        }

        VectorIterator operator-(difference_type n) const {
            VectorIterator ret(*this);
            ret -= n;
            return ret;
        }

        difference_type operator-(const VectorIterator& other) const {
            return m_i - other.m_i;
        }

        bool operator==(const VectorIterator& other) const {
            return m_i == other.m_i;
        }

        bool operator!=(const VectorIterator& other) const {
            return m_i != other.m_i;
        }

        bool operator<(const VectorIterator& other) const {
            return m_i < other.m_i;
        }

        bool operator>(const VectorIterator& other) const {
            return m_i > other.m_i;
        }

        bool operator<=(const VectorIterator& other) const {
            return m_i <= other.m_i;
        }

        bool operator>=(const VectorIterator& other) const {
            return m_i >= other.m_i;
        }

    private:
        Details::WFC::IVector<T>^ m_v;
        difference_type m_i;
    };

    template <typename T> inline VectorIterator<T> operator+(ptrdiff_t n, const VectorIterator<T>& i) {
        return i + n;
    }

    template <typename T> class VectorViewIterator {
    public:
        typedef ::std::random_access_iterator_tag iterator_category;
        typedef                                 T value_type;
        typedef                         ptrdiff_t difference_type;
        typedef                               T * pointer;
        typedef                               T   reference;

        VectorViewIterator()
            : m_v(nullptr), m_i(0) { }

        explicit VectorViewIterator(Details::WFC::IVectorView<T>^ v)
            : m_v(v), m_i(0) { }

        reference operator*() const {
            return m_v->GetAt(static_cast<unsigned int>(m_i));
        }

        Details::ArrowProxy<T> operator->() const {
            return Details::ArrowProxy<T>(m_v->GetAt(static_cast<unsigned int>(m_i)));
        }

        reference operator[](difference_type n) const {
            return m_v->GetAt(static_cast<unsigned int>(m_i + n));
        }

        VectorViewIterator& operator++() {
            ++m_i;
            return *this;
        }

        VectorViewIterator& operator--() {
            --m_i;
            return *this;
        }

        VectorViewIterator operator++(int) {
            VectorViewIterator old(*this);
            ++*this;
            return old;
        }

        VectorViewIterator operator--(int) {
            VectorViewIterator old(*this);
            --*this;
            return old;
        }

        VectorViewIterator& operator+=(difference_type n) {
            m_i += n;
            return *this;
        }

        VectorViewIterator& operator-=(difference_type n) {
            m_i -= n;
            return *this;
        }

        VectorViewIterator operator+(difference_type n) const {
            VectorViewIterator ret(*this);
            ret += n;
            return ret;
        }

        VectorViewIterator operator-(difference_type n) const {
            VectorViewIterator ret(*this);
            ret -= n;
            return ret;
        }

        difference_type operator-(const VectorViewIterator& other) const {
            return m_i - other.m_i;
        }

        bool operator==(const VectorViewIterator& other) const {
            return m_i == other.m_i;
        }

        bool operator!=(const VectorViewIterator& other) const {
            return m_i != other.m_i;
        }

        bool operator<(const VectorViewIterator& other) const {
            return m_i < other.m_i;
        }

        bool operator>(const VectorViewIterator& other) const {
            return m_i > other.m_i;
        }

        bool operator<=(const VectorViewIterator& other) const {
            return m_i <= other.m_i;
        }

        bool operator>=(const VectorViewIterator& other) const {
            return m_i >= other.m_i;
        }

    private:
        Details::WFC::IVectorView<T>^ m_v;
        difference_type m_i;
    };

    template <typename T> inline VectorViewIterator<T> operator+(ptrdiff_t n, const VectorViewIterator<T>& i) {
        return i + n;
    }

    template <typename T> class BackInsertIterator {
    public:
        using iterator_category = ::std::output_iterator_tag;
        using value_type = void;
        using difference_type = void;
        using pointer = void;
        using reference = void;

        explicit BackInsertIterator(Details::WFC::IVector<T>^ v) : m_v(v) { }

        BackInsertIterator& operator=(const T& t) {
            m_v->Append(t);
            return *this;
        }

        BackInsertIterator& operator*() {
            return *this;
        }

        BackInsertIterator& operator++() {
            return *this;
        }

        BackInsertIterator operator++(int) {
            return *this;
        }

    private:
        Details::WFC::IVector<T>^ m_v;
    };

    namespace Details {
        template <typename T, typename I> inline ::std::vector<T> ToVector(I^ v) {
            unsigned int size = v->Size;

            ::std::vector<T> ret(size);

            for (unsigned int actual = 0; actual < size; ) {
                Array<T>^ arr = ref new Array<T>(size - actual);

                unsigned int n = v->GetMany(actual, arr);

                if (n == 0) {
                    throw ref new FailureException;
                }

                ::std::copy_n(arr->begin(), n, ret.begin() + actual);

                actual += n;
            }

            return ret;
        }
    } // namespace Details
  } // namespace Collections
} // namespace Platform

namespace Windows {
  namespace Foundation {
    namespace Collections {
        template <typename X> inline ::Platform::Collections::InputIterator<X> begin(IIterable<X>^ i) {
            return ::Platform::Collections::InputIterator<X>(i->First());
        }

        template <typename X> inline ::Platform::Collections::InputIterator<X> end(IIterable<X>^) {
            return ::Platform::Collections::InputIterator<X>();
        }

        template <typename T> inline ::Platform::Collections::VectorIterator<T> begin(IVector<T>^ v) {
            return ::Platform::Collections::VectorIterator<T>(v);
        }

        template <typename T> inline ::Platform::Collections::VectorIterator<T> end(IVector<T>^ v) {
            return ::Platform::Collections::VectorIterator<T>(v) + v->Size;
        }

        template <typename T> inline ::Platform::Collections::VectorViewIterator<T> begin(IVectorView<T>^ v) {
            return ::Platform::Collections::VectorViewIterator<T>(v);
        }

        template <typename T> inline ::Platform::Collections::VectorViewIterator<T> end(IVectorView<T>^ v) {
            return ::Platform::Collections::VectorViewIterator<T>(v) + v->Size;
        }

        template <typename T> inline ::std::vector<T> to_vector(IVector<T>^ v) {
            return ::Platform::Collections::Details::ToVector<T>(v);
        }

        template <typename T> inline ::std::vector<T> to_vector(IVectorView<T>^ v) {
            return ::Platform::Collections::Details::ToVector<T>(v);
        }

        template <typename T> inline ::Platform::Collections::BackInsertIterator<T> back_inserter(IVector<T>^ v) {
            return ::Platform::Collections::BackInsertIterator<T>(v);
        }

        template <typename T> inline ::Platform::Collections::BackInsertIterator<T> back_inserter(IObservableVector<T>^ v) {
            return ::Platform::Collections::BackInsertIterator<T>(v);
        }
    } // namespace Collections
  } // namespace Foundation
} // namespace Windows

namespace Platform {
  namespace Collections {
    template <typename T, typename E> inline BackInsertIterator<T> back_inserter(Vector<T, E>^ v) {
        return BackInsertIterator<T>(v);
    }
  } // namespace Collections
} // namespace Platform

template <> struct ::std::hash< ::Platform::String^> {
    typedef ::Platform::String^ argument_type;
    typedef size_t result_type;
    size_t operator()(::Platform::String^ s) const {
        return ::std::_Hash_array_representation(s->Data(), s->Length());
    }
};

#undef _COLLECTION_ATTRIBUTES
#undef _COLLECTION_TRANSLATE
#undef _COLLECTION_WUXI

#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR

#endif // _COLLECTION_H_
