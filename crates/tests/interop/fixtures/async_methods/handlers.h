#pragma once

#include <windows.h>
#include <condition_variable>
#include <cstdio>
#include <functional>
#include <memory>
#include <mutex>
#include <semaphore>
#include <thread>

struct completion_gate
{
    std::binary_semaphore ready{ 0 };
    std::atomic<bool> opened{ false };

    void open()
    {
        if (!opened.exchange(true))
        {
            ready.release();
        }
    }
};

IAsyncAction gated_action(std::shared_ptr<completion_gate> gate)
{
    co_await resume_background();
    gate->ready.acquire();
}

IAsyncActionWithProgress<uint32_t> gated_progress_action(std::shared_ptr<completion_gate> gate)
{
    co_await resume_background();
    gate->ready.acquire();
}

IAsyncOperation<int32_t> gated_operation(std::shared_ptr<completion_gate> gate)
{
    co_await resume_background();
    gate->ready.acquire();
    co_return 41;
}

IAsyncOperationWithProgress<int32_t, uint32_t> gated_progress_operation(std::shared_ptr<completion_gate> gate)
{
    co_await resume_background();
    gate->ready.acquire();
    co_return 41;
}

struct handler_observation
{
    handle destroyed{ CreateEventW(nullptr, TRUE, FALSE, nullptr) };
    std::atomic<uint32_t> calls{ 0 };
    std::atomic<DWORD> called_on{ 0 };
    std::atomic<DWORD> destroyed_on{ 0 };
    std::atomic<int32_t> failure{ 0 };
    std::mutex lock;
    std::function<void()> prepare;
    std::function<void()> release;

    handler_observation()
    {
        check(static_cast<bool>(destroyed));
    }

    void wait()
    {
        auto event = destroyed.get();
        DWORD index{};
        check_hresult(CoWaitForMultipleHandles(COWAIT_DISPATCH_CALLS, 5000, 1, &event, &index));
    }
};

template <typename Delegate>
struct observed_handler final : impl::abi_t<Delegate>
{
    std::atomic<uint32_t> references{ 1 };
    std::shared_ptr<handler_observation> observation;
    bool agile;

    observed_handler(std::shared_ptr<handler_observation> value, bool is_agile)
        : observation(std::move(value)), agile(is_agile)
    {
    }

    int32_t __stdcall QueryInterface(guid const& id, void** result) noexcept final
    {
        *result = nullptr;
        if (id == guid_of<impl::IAgileObject>())
        {
            try
            {
                std::function<void()> prepare;
                {
                    std::lock_guard guard(observation->lock);
                    prepare = std::move(observation->prepare);
                }
                if (prepare)
                {
                    prepare();
                }
            }
            catch (...)
            {
                return to_hresult();
            }
            if (!agile)
            {
                return E_NOINTERFACE;
            }
        }
        else if (id != guid_of<Delegate>() && id != guid_of<Windows::Foundation::IUnknown>())
        {
            return E_NOINTERFACE;
        }
        *result = this;
        AddRef();
        return S_OK;
    }

    uint32_t __stdcall AddRef() noexcept final
    {
        return ++references;
    }

    uint32_t __stdcall Release() noexcept final
    {
        try
        {
            std::function<void()> release;
            {
                std::lock_guard guard(observation->lock);
                release = observation->release;
            }
            if (release)
            {
                release();
            }
        }
        catch (...)
        {
            observation->failure = to_hresult();
        }
        auto remaining = --references;
        if (!remaining)
        {
            observation->destroyed_on = GetCurrentThreadId();
            SetEvent(observation->destroyed.get());
            delete this;
        }
        return remaining;
    }

    int32_t __stdcall Invoke(void*, int32_t status) noexcept final
    {
        observation->called_on = GetCurrentThreadId();
        ++observation->calls;
        if (status != static_cast<int32_t>(AsyncStatus::Completed))
        {
            observation->failure = E_UNEXPECTED;
        }
        return S_OK;
    }
};

template <typename Delegate>
Delegate make_observed(std::shared_ptr<handler_observation> const& observation, bool agile)
{
    return { static_cast<void*>(new observed_handler<Delegate>(observation, agile)), take_ownership_from_abi };
}

template <typename F>
void on_sta(F&& body)
{
    std::exception_ptr error;
    std::thread([&]
    {
        init_apartment(apartment_type::single_threaded);
        try
        {
            body();
        }
        catch (...)
        {
            error = std::current_exception();
        }
        uninit_apartment();
    }).join();
    if (error)
    {
        std::rethrow_exception(error);
    }
}

template <typename Delegate, typename Make>
void apartment_completion(Make&& make)
{
    auto gate = std::make_shared<completion_gate>();
    try
    {
        on_sta([&]
        {
            auto observation = std::make_shared<handler_observation>();
            auto operation = make(gate);
            auto handler = make_observed<Delegate>(observation, false);
            try
            {
                // Some parameterized delegate IIDs have no registered system proxy.
                (void)make_agile(handler);
            }
            catch (hresult_error const& error)
            {
                check(error.code() == REGDB_E_IIDNOTREG);
                try
                {
                    operation.Completed(handler);
                    check(false);
                }
                catch (hresult_error const& registration)
                {
                    check(registration.code() == error.code());
                }
                handler = nullptr;
                observation->wait();
                check(observation->calls == 0);
                check(observation->destroyed_on == GetCurrentThreadId());
                observation = std::make_shared<handler_observation>();
                handler = make_observed<Delegate>(observation, true);
                operation.Completed(handler);
                handler = nullptr;
                gate->open();
                observation->wait();
                check(observation->calls == 1);
                check(observation->failure == 0);
                (void)operation.GetResults();
                return;
            }
            operation.Completed(handler);
            handler = nullptr;
            gate->open();
            observation->wait();
            check(observation->calls == 1);
            check(observation->called_on == GetCurrentThreadId());
            check(observation->destroyed_on == GetCurrentThreadId());
            check(observation->failure == 0);
            (void)operation.GetResults();
        });
    }
    catch (...)
    {
        gate->open();
        throw;
    }
}

struct result_action final : impl::abi_t<IAsyncAction>, impl::abi_t<IAsyncInfo>
{
    std::atomic<uint32_t> references{ 1 };
    std::shared_ptr<handler_observation> observation;

    explicit result_action(std::shared_ptr<handler_observation> value)
        : observation(std::move(value)) {}

    int32_t __stdcall QueryInterface(guid const& id, void** result) noexcept final
    {
        *result = nullptr;
        if (id == guid_of<IAsyncInfo>())
        {
            *result = static_cast<impl::abi_t<IAsyncInfo>*>(this);
        }
        else if (id == guid_of<IAsyncAction>() || id == guid_of<IInspectable>() ||
            id == guid_of<Windows::Foundation::IUnknown>() || id == guid_of<impl::IAgileObject>())
        {
            *result = static_cast<impl::abi_t<IAsyncAction>*>(this);
        }
        else
        {
            return E_NOINTERFACE;
        }
        AddRef();
        return S_OK;
    }

    uint32_t __stdcall AddRef() noexcept final
    {
        auto count = ++references;
        ++observation->calls;
        try
        {
            std::function<void()> prepare;
            {
                std::lock_guard guard(observation->lock);
                prepare = std::move(observation->prepare);
            }
            if (prepare)
            {
                std::fputs("result AddRef entered\n", stderr);
                prepare();
            }
        }
        catch (...)
        {
            observation->failure = to_hresult();
        }
        return count;
    }

    uint32_t __stdcall Release() noexcept final
    {
        auto remaining = --references;
        if (!remaining)
        {
            SetEvent(observation->destroyed.get());
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

    int32_t __stdcall GetTrustLevel(TrustLevel* value) noexcept final
    {
        *value = TrustLevel::BaseTrust;
        return S_OK;
    }

    int32_t __stdcall put_Completed(void*) noexcept final { return E_NOTIMPL; }
    int32_t __stdcall get_Completed(void** value) noexcept final
    {
        *value = nullptr;
        return S_OK;
    }
    int32_t __stdcall GetResults() noexcept final { return S_OK; }
    int32_t __stdcall get_Id(uint32_t* value) noexcept final { *value = 1; return S_OK; }
    int32_t __stdcall get_Status(int32_t* value) noexcept final
    {
        *value = static_cast<int32_t>(AsyncStatus::Completed);
        return S_OK;
    }
    int32_t __stdcall get_ErrorCode(hresult* value) noexcept final { *value = S_OK; return S_OK; }
    int32_t __stdcall Cancel() noexcept final { return S_OK; }
    int32_t __stdcall Close() noexcept final { return S_OK; }
};

template <typename Make>
void transfer_result(Make&& make)
{
    auto observation = std::make_shared<handler_observation>();
    IAsyncAction value{ static_cast<impl::abi_t<IAsyncAction>*>(new result_action(observation)),
        take_ownership_from_abi };
    auto identity = get_abi(value);
    auto operation = make(value);
    value = nullptr;
    auto completed = std::make_shared<std::binary_semaphore>(0);
    operation.Completed([completed](auto&&, auto status)
    {
        check(status == AsyncStatus::Completed);
        completed->release();
    });
    check(completed->try_acquire_for(std::chrono::seconds(5)));
    check(WaitForSingleObject(observation->destroyed.get(), 0) == WAIT_TIMEOUT);
    auto copies = observation->calls.load();
    {
        std::lock_guard guard(observation->lock);
        observation->prepare = [&]
        {
            check(operation.Status() == AsyncStatus::Completed);
            check(operation.ErrorCode() == S_OK);
        };
    }
    auto result = operation.GetResults();
    check(get_abi(result) == identity);
    check(observation->calls == copies);
    check(observation->failure == S_OK);
    {
        std::lock_guard guard(observation->lock);
        observation->prepare = nullptr;
    }
    try
    {
        (void)operation.GetResults();
        check(false);
    }
    catch (hresult_illegal_method_call const&)
    {
    }
    check(operation.Status() == AsyncStatus::Completed);
    check(operation.ErrorCode() == S_OK);
    result = nullptr;
    check(WaitForSingleObject(observation->destroyed.get(), 0) == WAIT_OBJECT_0);

    observation = std::make_shared<handler_observation>();
    value = { static_cast<impl::abi_t<IAsyncAction>*>(new result_action(observation)),
        take_ownership_from_abi };
    operation = make(value);
    value = nullptr;
    operation.Completed([completed](auto&&, auto) { completed->release(); });
    check(completed->try_acquire_for(std::chrono::seconds(5)));
    operation = nullptr;
    check(WaitForSingleObject(observation->destroyed.get(), 5000) == WAIT_OBJECT_0);
}

void test_handlers(Widget const& widget)
{
    std::jthread watchdog([](std::stop_token stop)
    {
        std::mutex mutex;
        std::unique_lock lock(mutex);
        std::condition_variable_any wake;
        wake.wait_for(lock, stop, std::chrono::seconds(15), [] { return false; });
        if (!stop.stop_requested())
        {
            std::fputs("completion handler contract timed out\n", stderr);
            std::abort();
        }
    });

    for (bool ready : { false, true })
    {
        transfer_result([&](auto const& value) { return widget.ReturnAction(value, ready); });
        transfer_result([&](auto const& value) { return widget.ReturnActionWithProgress(value, ready); });
    }

    apartment_completion<AsyncActionCompletedHandler>(
        [&](auto gate) { return widget.AwaitAction(gated_action(gate)); });
    apartment_completion<AsyncActionWithProgressCompletedHandler<uint32_t>>(
        [&](auto gate) { return widget.AwaitActionWithProgress(gated_progress_action(gate)); });
    apartment_completion<AsyncOperationCompletedHandler<int32_t>>(
        [&](auto gate) { return widget.AwaitOperation(gated_operation(gate)); });
    apartment_completion<AsyncOperationWithProgressCompletedHandler<int32_t, uint32_t>>(
        [&](auto gate) { return widget.AwaitOperationWithProgress(gated_progress_operation(gate)); });

    // Native preparation and release must not run under the operation's lock.
    for (int race = 0; race != 3; ++race)
    {
        bool const assign_other = race != 2;
        bool const complete_during_preparation = race != 0;
        auto gate = std::make_shared<completion_gate>();
        auto operation = widget.AwaitAction(gated_action(gate));
        auto winner_state = std::make_shared<handler_observation>();
        auto winner = make_observed<AsyncActionCompletedHandler>(winner_state, true);
        auto loser_state = std::make_shared<handler_observation>();
        auto loser = make_observed<AsyncActionCompletedHandler>(loser_state, true);
        loser_state->prepare = [&]
        {
            check(operation.Status() == AsyncStatus::Started);
            if (assign_other)
            {
                operation.Completed(winner);
                winner = nullptr;
            }
            if (complete_during_preparation)
            {
                gate->open();
                while (operation.Status() == AsyncStatus::Started)
                {
                    std::this_thread::yield();
                }
            }
        };
        loser_state->release = [&] { (void)operation.Status(); };
        try
        {
            operation.Completed(loser);
            check(!assign_other);
        }
        catch (hresult_illegal_delegate_assignment const&)
        {
            check(assign_other);
        }
        catch (...)
        {
            gate->open();
            throw;
        }
        loser = nullptr;
        loser_state->wait();
        check(loser_state->calls == (assign_other ? 0u : 1u));
        check(loser_state->failure == 0);
        gate->open();
        if (assign_other)
        {
            winner_state->wait();
            check(winner_state->calls == 1);
        }
        operation.GetResults();
    }

    on_sta([&]
    {
        auto operation = widget.OperationWithProgress(1);
        while (operation.Status() == AsyncStatus::Started)
        {
            std::this_thread::yield();
        }
        auto observation = std::make_shared<handler_observation>();
        auto handler = make_observed<AsyncOperationWithProgressCompletedHandler<int32_t, uint32_t>>(
            observation, false);
        operation.Completed(handler);
        handler = nullptr;
        observation->wait();
        check(observation->calls == 1);
        check(observation->called_on == GetCurrentThreadId());
        check(observation->destroyed_on == GetCurrentThreadId());
    });

    // Shut down the owning STA before completing the operation.
    auto gate = std::make_shared<completion_gate>();
    auto observation = std::make_shared<handler_observation>();
    IAsyncAction pending{ nullptr };
    com_ptr<impl::IAgileReference> reference;
    DWORD owner{};
    try
    {
        on_sta([&]
        {
            owner = GetCurrentThreadId();
            pending = widget.AwaitAction(gated_action(gate));
            auto handler = make_observed<AsyncActionCompletedHandler>(observation, false);
            check_hresult(impl::get_agile_reference(
                guid_of<AsyncActionCompletedHandler>(), get_abi(handler), reference.put_void()));
            pending.Completed(handler);
        });
        observation->wait();
        check(observation->destroyed_on == owner);
        check(observation->calls == 0);
        try
        {
            AsyncActionCompletedHandler proxy{ nullptr };
            check_hresult(reference->Resolve(guid_of<AsyncActionCompletedHandler>(), put_abi(proxy)));
            proxy(pending, AsyncStatus::Completed);
            check(false);
        }
        catch (hresult_error const& error)
        {
            check(error.code() == RPC_E_DISCONNECTED || error.code() == RPC_E_SERVER_DIED_DNE);
        }
        gate->open();
        while (pending.Status() == AsyncStatus::Started)
        {
            std::this_thread::yield();
        }
        pending.GetResults();
        check(observation->calls == 0);
    }
    catch (...)
    {
        gate->open();
        throw;
    }
}
