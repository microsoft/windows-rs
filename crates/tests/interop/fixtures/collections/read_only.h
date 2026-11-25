#pragma once

#include <windows.h>
#include <algorithm>
#include <array>
#include <atomic>
#include <barrier>
#include <functional>
#include <memory>
#include <mutex>
#include <thread>
#include <vector>

using Inspectable = Windows::Foundation::IInspectable;
using Vector = IVector<Inspectable>;

template <typename F>
void fails_with(int32_t expected, F&& call,
    std::source_location const location = std::source_location::current())
{
    try
    {
        call();
    }
    catch (hresult_error const& error)
    {
        check(error.code() == expected, location);
        return;
    }
    catch (std::bad_alloc const&)
    {
        check(expected == E_OUTOFMEMORY, location);
        return;
    }
    check(false, location);
}

void read_only_contract(Widget const& widget)
{
    auto item = box_value(42);
    auto vector = widget.ReadOnly({ item, nullptr });
    auto abi = static_cast<impl::abi_t<Vector>*>(get_abi(vector));
    check(abi->SetAt(0, get_abi(item)) == E_NOTIMPL);
    check(abi->InsertAt(0, get_abi(item)) == E_NOTIMPL);
    check(abi->RemoveAt(0) == E_NOTIMPL);
    check(abi->Append(get_abi(item)) == E_NOTIMPL);
    check(abi->RemoveAtEnd() == E_NOTIMPL);
    check(abi->Clear() == E_NOTIMPL);
    check(abi->ReplaceAll(0, nullptr) == E_NOTIMPL);
    check(vector.Size() == 2);
    check(vector.GetAt(0) == item);
    check(vector.GetAt(1) == nullptr);
    fails_with(E_BOUNDS, [&] { static_cast<void>(vector.GetAt(2)); });
    uint32_t index = 99;
    check(vector.IndexOf(item, index) && index == 0);
    check(vector.IndexOf(nullptr, index) && index == 1);
    check(!vector.IndexOf(box_value(43), index) && index == 0);

    void* items[]{ get_abi(item), get_abi(item), get_abi(item) };
    uint32_t count = 99;
    check_hresult(abi->GetMany(0, 3, items, &count));
    Inspectable returned{ items[0], take_ownership_from_abi };
    check(count == 2 && returned == item);
    check(items[1] == nullptr && items[2] == nullptr);

    auto view = vector.GetView();
    auto iterator = view.First();
    vector = nullptr;
    check(view.GetAt(0) == item);
    view = nullptr;
    check(iterator.Current() == item);
    check(iterator.MoveNext());
    check(iterator.Current() == nullptr);
    check(!iterator.MoveNext());
    check(!iterator.MoveNext());
    fails_with(E_BOUNDS, [&] { static_cast<void>(iterator.Current()); });
}

struct query_state
{
    std::atomic<bool> fail{ false };
    std::atomic<bool> destroyed{ false };
    std::atomic<int32_t> failure{ S_OK };
    std::mutex lock;
    std::function<void()> reenter;
};

struct query_value final : impl::abi_t<Inspectable>
{
    std::atomic<uint32_t> references{ 1 };
    std::shared_ptr<query_state> state;
    bool const marshalable;

    explicit query_value(std::shared_ptr<query_state> value, bool agile = true)
        : state(std::move(value)), marshalable(agile) {}

    int32_t __stdcall QueryInterface(guid const& id, void** result) noexcept final
    {
        *result = nullptr;
        if (id == guid_of<Inspectable>() && state->fail)
        {
            return E_OUTOFMEMORY;
        }
        if (id != guid_of<Inspectable>() && id != guid_of<Windows::Foundation::IUnknown>() &&
            !(marshalable && id == guid_of<impl::IAgileObject>()) &&
            !(!marshalable && id == guid_of<::INoMarshal>()))
        {
            return E_NOINTERFACE;
        }
        *result = this;
        AddRef();
        return S_OK;
    }

    uint32_t __stdcall AddRef() noexcept final
    {
        auto count = ++references;
        try
        {
            std::function<void()> reenter;
            {
                std::lock_guard guard(state->lock);
                reenter = std::move(state->reenter);
            }
            if (reenter)
            {
                reenter();
            }
        }
        catch (...)
        {
            state->failure = to_hresult();
        }
        return count;
    }

    uint32_t __stdcall Release() noexcept final
    {
        auto remaining = --references;
        if (!remaining)
        {
            state->destroyed = true;
            delete this;
        }
        return remaining;
    }

    int32_t __stdcall GetIids(uint32_t* count, guid** values) noexcept final
    {
        *count = 0;
        *values = nullptr;
        return S_OK;
    }

    int32_t __stdcall GetRuntimeClassName(void** value) noexcept final
    {
        *value = nullptr;
        return S_OK;
    }

    int32_t __stdcall GetTrustLevel(Windows::Foundation::TrustLevel* value) noexcept final
    {
        *value = Windows::Foundation::TrustLevel::BaseTrust;
        return S_OK;
    }
};

template <typename F>
void vector_retains_values(F const& make)
{
    auto state = std::make_shared<query_state>();
    Inspectable value{ new query_value(state), take_ownership_from_abi };
    auto vector = make({ value });
    auto view = vector.GetView();
    auto iterator = vector.First();
    value = nullptr;
    vector = nullptr;
    check(!state->destroyed);
    check(view.GetAt(0) != nullptr);
    view = nullptr;
    check(!state->destroyed);
    check(iterator.Current() != nullptr);
    iterator = nullptr;
    check(state->destroyed);
}

void read_only_marshaling_failure(Widget const& widget)
{
    auto state = std::make_shared<query_state>();
    Inspectable failing{ new query_value(state, false), take_ownership_from_abi };
    auto first = box_value(1);
    com_ptr<impl::IAgileReference> control;
    auto error = impl::get_agile_reference(guid_of<Inspectable>(), get_abi(failing), control.put_void());
    check(error < 0 && !control);
    fails_with(error, [&] { static_cast<void>(widget.ReadOnly({ first, failing })); });
}

template <typename F>
void vector_resolution_failure(F const& make)
{
    auto state = std::make_shared<query_state>();
    Inspectable failing{ new query_value(state), take_ownership_from_abi };
    auto first_value = new query_value(std::make_shared<query_state>());
    Inspectable first{ first_value, take_ownership_from_abi };
    auto vector = make({ first, failing });
    auto view = vector.GetView();
    auto iterator = vector.First();
    com_ptr<impl::IAgileReference> control;
    check_hresult(impl::get_agile_reference(
        guid_of<Inspectable>(), get_abi(failing), control.put_void()));
    state->fail = true;
    void* resolved{};
    // agile_ref::get discards the HRESULT; check Resolve directly.
    check(control->Resolve(guid_of<Inspectable>(), &resolved) == E_OUTOFMEMORY);
    check(resolved == nullptr);
    fails_with(E_OUTOFMEMORY, [&] { static_cast<void>(vector.GetAt(1)); });
    fails_with(E_OUTOFMEMORY, [&] {
        uint32_t index = 99;
        vector.IndexOf(nullptr, index);
    });
    auto references = first_value->references.load();
    for (int kind = 0; kind != 3; ++kind)
    {
        void* items[]{ get_abi(first), get_abi(first) };
        uint32_t count = 99;
        int32_t result;
        if (kind == 0)
        {
            result = static_cast<impl::abi_t<Vector>*>(get_abi(vector))->GetMany(0, 2, items, &count);
        }
        else if (kind == 1)
        {
            result = static_cast<impl::abi_t<IVectorView<Inspectable>>*>(get_abi(view))->GetMany(0, 2, items, &count);
        }
        else
        {
            result = static_cast<impl::abi_t<IIterator<Inspectable>>*>(get_abi(iterator))->GetMany(2, items, &count);
        }
        check(result == E_OUTOFMEMORY);
        check(count == 0 && items[0] == nullptr && items[1] == nullptr);
        check(first_value->references == references);
        check(iterator.Current() == first);
    }
    state->fail = false;
    std::array<Inspectable, 2> items;
    check(iterator.GetMany(items) == 2);
    check(items[0] == first && items[1] == failing);
    check(!iterator.HasCurrent());
}

template <typename F>
void vector_reentrant_iterator(F const& make)
{
    auto state = std::make_shared<query_state>();
    Inspectable value{ new query_value(state), take_ownership_from_abi };
    auto vector = make({ value, box_value(1), box_value(2) });
    auto iterator = vector.First();
    bool called = false;
    {
        std::lock_guard guard(state->lock);
        state->reenter = [&] {
            called = true;
            check(iterator.MoveNext());
        };
    }
    std::array<Inspectable, 3> items;
    check(iterator.GetMany(items) == 2);
    check(called);
    check(state->failure == S_OK);
    check(unbox_value<int32_t>(items[0]) == 1);
    check(unbox_value<int32_t>(items[1]) == 2);
    check(items[2] == nullptr);
    check(!iterator.HasCurrent());
}

template <typename F>
void vector_concurrent_iterator(F const& make)
{
    struct mta_scope
    {
        mta_scope() { init_apartment(apartment_type::multi_threaded); }
        ~mta_scope() { uninit_apartment(); }
    };
    std::vector<Inspectable> input;
    for (int32_t value = 0; value != 256; ++value)
    {
        input.push_back(box_value(value));
    }
    auto vector = make(input);
    agile_ref<IIterator<Inspectable>> reference{ vector.First() };
    std::barrier start{ 4 };
    std::array<std::thread, 4> workers;
    std::array<std::vector<int32_t>, 4> results;
    std::array<std::exception_ptr, 4> errors;
    for (size_t index = 0; index != workers.size(); ++index)
    {
        workers[index] = std::thread([&, index] {
            start.arrive_and_wait();
            try
            {
                mta_scope apartment;
                auto iterator = reference.get();
                check(iterator != nullptr);
                std::array<Inspectable, 7> items;
                while (auto count = iterator.GetMany(items))
                {
                    for (uint32_t i = 0; i != count; ++i)
                    {
                        results[index].push_back(unbox_value<int32_t>(items[i]));
                    }

                }
            }
            catch (...)
            {
                errors[index] = std::current_exception();
            }
        });
    }
    for (auto& worker : workers)
    {
        worker.join();
    }
    std::vector<int32_t> actual;
    for (size_t index = 0; index != workers.size(); ++index)
    {
        if (errors[index])
        {
            std::rethrow_exception(errors[index]);
        }
        actual.insert(actual.end(), results[index].begin(), results[index].end());
    }
    std::sort(actual.begin(), actual.end());
    check(actual.size() == input.size());
    for (size_t index = 0; index != actual.size(); ++index)
    {
        check(actual[index] == static_cast<int32_t>(index));
    }
}

template <typename F>
void vector_storage_contract(F const& make)
{
    vector_retains_values(make);
    vector_resolution_failure(make);
    vector_reentrant_iterator(make);
    vector_concurrent_iterator(make);
}

template <typename F>
void map_resolution_failure(F const& make)
{
    auto state = std::make_shared<query_state>();
    Inspectable failing{ new query_value(state), take_ownership_from_abi };
    auto first = box_value(1);
    auto map = make({ first, failing, nullptr });
    auto view = map.GetView();
    auto iterator = view.First();
    check(iterator.MoveNext());
    auto pair = iterator.Current();
    check(pair.Key() == 1);
    com_ptr<impl::IAgileReference> control;
    check_hresult(impl::get_agile_reference(
        guid_of<Inspectable>(), get_abi(failing), control.put_void()));
    state->fail = true;
    void* resolved{};
    check(control->Resolve(guid_of<Inspectable>(), &resolved) == E_OUTOFMEMORY);
    check(resolved == nullptr);
    fails_with(E_OUTOFMEMORY, [&] { static_cast<void>(map.Lookup(1)); });
    fails_with(E_OUTOFMEMORY, [&] { static_cast<void>(view.Lookup(1)); });
    fails_with(E_OUTOFMEMORY, [&] { static_cast<void>(pair.Value()); });
    check(view.Lookup(2) == nullptr);

    // Deriving owners does not resolve their apartment-bound values.
    auto snapshot = map.GetView();
    std::array<IKeyValuePair<uint32_t, Inspectable>, 3> pairs;
    check(map.First().GetMany(pairs) == 3);
    map.Clear();
    check(map.Size() == 0);
    map = nullptr;
    fails_with(E_OUTOFMEMORY, [&] { static_cast<void>(snapshot.Lookup(1)); });
    fails_with(E_OUTOFMEMORY, [&] { static_cast<void>(pairs[1].Value()); });
    state->fail = false;
    check(pair.Value() == failing);
    check(pairs[0].Value() == first);
    check(pairs[1].Value() == failing);
    check(pairs[2].Value() == nullptr);
}
