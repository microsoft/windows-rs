#include <windows.h>
#include <weakreference.h>
#include <atomic>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <future>
#include <winrt/Windows.Foundation.h>
#include <winrt/WeakReferences.h>
#include "check.h"

#pragma comment(lib, "user32.lib")

using namespace winrt;
using namespace WeakReferences;

struct Observer : implements<Observer, IObserver>
{
    int32_t* m_value;

    Observer(int32_t* value) : m_value(value)
    {
    }

    void Notify(int32_t value)
    {
        *m_value = value;
    }
};

struct Sta
{
    Sta() { init_apartment(apartment_type::single_threaded); }
    ~Sta() { uninit_apartment(); }
};

struct ThreadProbe
{
    std::atomic<DWORD> called = 0;
    std::atomic<DWORD> dropped = 0;
};

// Only thread-safe state: the raw-pointer case intentionally bypasses COM marshaling.
struct ApartmentStringable : implements<ApartmentStringable, Windows::Foundation::IStringable, non_agile>
{
    ThreadProbe& m_probe;

    ApartmentStringable(ThreadProbe& probe) : m_probe(probe) {}
    ~ApartmentStringable() { m_probe.dropped = GetCurrentThreadId(); }

    hstring ToString()
    {
        m_probe.called = GetCurrentThreadId();
        return {};
    }
};

template <typename T>
T receive_with_dispatch(std::future<T>& future)
{
    auto deadline = std::chrono::steady_clock::now() + std::chrono::seconds(10);
    while (future.wait_for(std::chrono::seconds(0)) != std::future_status::ready)
    {
        if (std::chrono::steady_clock::now() >= deadline)
        {
            // Unwinding would join the worker without dispatching its pending COM calls.
            std::fputs("weak-reference COM dispatch timed out\n", stderr);
            std::fflush(stderr);
            std::_Exit(1);
        }
        MSG message{};
        while (PeekMessageW(&message, nullptr, 0, 0, PM_REMOVE))
        {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        Sleep(0);
    }
    return future.get();
}

void weak_apartments(bool marshal)
{
    auto owner_task = std::async(std::launch::async, [marshal]
    {
        Sta apartment;
        DWORD owner = GetCurrentThreadId();
        ThreadProbe probe;
        Windows::Foundation::IStringable object = make<ApartmentStringable>(probe);
        auto identity = get_abi(object);
        com_ptr<::IWeakReference> native;
        check_hresult(object.as<::IWeakReferenceSource>()->GetWeakReference(native.put()));
        check(!native.try_as<::IAgileObject>());
        check(!native.try_as<::IMarshal>());

        agile_ref<::IWeakReference> reference;
        com_ptr<::IWeakReference> raw;
        if (marshal)
        {
            reference = agile_ref<::IWeakReference>(native);
        }
        else
        {
            raw = native;
        }

        std::promise<void> ready;
        auto ready_future = ready.get_future();
        std::promise<void> release;
        auto worker = std::async(std::launch::async,
            [marshal, raw, reference, identity, ready = std::move(ready),
                release = release.get_future()]() mutable
        {
            auto ready_signal = std::move(ready);
            Sta apartment;
            auto weak = marshal ? reference.get() : raw;
            Windows::Foundation::IStringable resolved{ nullptr };
            check_hresult(weak->Resolve(guid_of<Windows::Foundation::IStringable>(),
                reinterpret_cast<::IInspectable**>(put_abi(resolved))));
            check(resolved != nullptr);
            check((get_abi(resolved) == identity) == !marshal);
            resolved.ToString();
            ready_signal.set_value();
            check(release.wait_for(std::chrono::seconds(10)) == std::future_status::ready);
            release.get();
            resolved = nullptr;
            check_hresult(weak->Resolve(guid_of<Windows::Foundation::IStringable>(),
                reinterpret_cast<::IInspectable**>(put_abi(resolved))));
            check(resolved == nullptr);
            return GetCurrentThreadId();
        });

        try
        {
            receive_with_dispatch(ready_future);
        }
        catch (...)
        {
            receive_with_dispatch(worker);
            throw;
        }
        object = nullptr;
        release.set_value();
        DWORD caller = receive_with_dispatch(worker);
        check(caller != owner);
        check(probe.called == (marshal ? owner : caller));
        check(probe.dropped == (marshal ? owner : caller));
    });
    owner_task.get();
}

void test()
{
    weak_apartments(false);
    weak_apartments(true);

    Widget widget;
    int32_t value = 0;

    IObserver observer = make<Observer>(&value);
    widget.Observe(observer);
    check(widget.Notify(1));
    check(value == 1);

    observer = nullptr;
    check(!widget.Notify(2));
    check(value == 1);

    weak_ref<Widget> weak = make_weak(widget);
    check(weak.get() == widget);
    widget = nullptr;
    check(!weak.get());
}