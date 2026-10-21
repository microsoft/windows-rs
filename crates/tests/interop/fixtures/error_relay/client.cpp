#include <winrt/ErrorRelay.h>
#include <chrono>
#include <functional>
#include <future>
#include "check.h"

using namespace winrt;
using namespace ErrorRelay;

struct Callback : implements<Callback, ICallback>
{
    Callback(std::function<void()> wait = {}) : checkpoint(std::move(wait)) {}

    int calls = 0;
    std::function<void()> checkpoint;

    void Fail(int32_t code, hstring const& message)
    {
        ++calls;
        throw hresult_error(hresult{ code }, message);
    }

    void Checkpoint()
    {
        check(static_cast<bool>(checkpoint));
        checkpoint();
    }
};

void test()
{
    Widget widget;
    auto callback = make<Callback>();
    auto verify = [&](hresult code, hstring const& message)
    {
        try
        {
            widget.Relay(callback, code, message);
            check(false);
        }
        catch (hresult_error const& error)
        {
            check(error.code() == code);
            check(error.message() == message);
        }
    };
    verify(hresult{ static_cast<int32_t>(0x8000000B) }, L"relay-bounds");
    verify(hresult{ static_cast<int32_t>(0x8004A123) }, L"relay-custom");

    std::array<std::promise<void>, 2> signals;
    std::array<std::future<void>, 2> ready{ signals[0].get_future(), signals[1].get_future() };
    std::array<std::future<void>, 2> workers;
    for (size_t index = 0; index < workers.size(); ++index)
    {
        workers[index] = std::async(std::launch::async, [&, index]
        {
            init_apartment();
            auto concurrent = make_self<Callback>([&, index]
            {
                signals[index].set_value();
                check(ready[1 - index].wait_for(std::chrono::seconds(10)) == std::future_status::ready);
            });
            hresult code{ static_cast<int32_t>(0x8004A123) + static_cast<int32_t>(index) };
            hstring message = L"thread-" + to_hstring(index);
            try
            {
                widget.RelayPreserved(concurrent.as<ICallback>(), code, message);
                check(false);
            }
            catch (hresult_error const& error)
            {
                check(error.code() == code);
                check(error.message() == message);
            }
            check(concurrent->calls == 2);
            concurrent = nullptr;
            uninit_apartment();
        });
    }
    for (auto& worker : workers)
    {
        worker.get();
    }
    for (auto& signal : ready)
    {
        check(signal.wait_for(std::chrono::seconds(0)) == std::future_status::ready);
    }
}
