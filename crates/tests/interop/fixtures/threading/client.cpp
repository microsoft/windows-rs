#include <windows.h>
#include <winrt/Windows.Foundation.h>
#include <winrt/Threading.h>
#include <latch>
#include <mutex>
#include <set>
#include <thread>
#include <vector>
#include "check.h"

using namespace winrt;
using namespace Windows::Foundation;
using namespace Threading;

constexpr int threads = 8;
constexpr int calls = 1000;

void test()
{
    Widget widget;

    // The widget is created on this thread but called directly from threads in other apartments.
    for (auto apartment : { apartment_type::single_threaded, apartment_type::multi_threaded })
    {
        uint32_t thread = 0;
        uint32_t current = 0;

        std::thread([&]
        {
            init_apartment(apartment);
            thread = GetCurrentThreadId();
            current = widget.CurrentThread();
            uninit_apartment();
        }).join();

        check(thread == current);
    }

    // Threads start together so calls overlap, and every call gets a unique count.
    {
        std::latch start(threads);
        std::vector<std::vector<int32_t>> counts(threads);
        std::vector<std::jthread> workers;

        for (auto& thread_counts : counts)
        {
            workers.emplace_back([&]
            {
                start.arrive_and_wait();

                for (int i = 0; i < calls; ++i)
                {
                    thread_counts.push_back(widget.Increment());
                }
            });
        }

        workers.clear();
        std::set<int32_t> unique;

        for (auto& thread_counts : counts)
        {
            unique.insert(thread_counts.begin(), thread_counts.end());
        }

        check(unique.size() == threads * calls);
        check(*unique.begin() == 1 && *unique.rbegin() == threads * calls);
        check(widget.Count() == threads * calls);
    }

    // The component calls handlers on background threads.
    {
        std::atomic<int32_t> sum = 0;
        std::mutex lock;
        std::set<uint32_t> handler_threads;

        Handler handler = [&](int32_t value)
        {
            sum += value;
            std::lock_guard guard(lock);
            handler_threads.insert(GetCurrentThreadId());
        };

        std::vector<IAsyncAction> actions;

        for (int32_t value = 1; value <= threads; ++value)
        {
            actions.push_back(widget.InvokeOnBackground(handler, value));
        }

        for (auto&& action : actions)
        {
            action.get();
        }

        check(sum == 36);
        check(!handler_threads.contains(GetCurrentThreadId()));
    }

    // A handler's error fails the action.
    try
    {
        widget.InvokeOnBackground([](int32_t) { throw hresult_out_of_bounds(); }, 0).get();
        check(false);
    }
    catch (hresult_out_of_bounds const&)
    {
    }
}