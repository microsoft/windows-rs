#include <windows.h>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <vector>
#include "winrt/LangPerf.h"
#include "winrt/Windows.Foundation.h"
#include "winrt/Windows.Foundation.Collections.h"

using namespace winrt;
using namespace winrt::LangPerf;

static long long elapsed_ms(std::chrono::high_resolution_clock::time_point const start)
{
    return std::chrono::duration_cast<std::chrono::milliseconds>(
               std::chrono::high_resolution_clock::now() - start)
        .count();
}

extern "C" int32_t __stdcall lang_perf_cpp(uint64_t iterations) noexcept
{
    try
    {
        init_apartment();
        Class object;
        printf("# C++ consumer -> %ls component - %llu iterations\n",
               object.Lang().c_str(), static_cast<unsigned long long>(iterations));

        auto start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            Class temp;
            (void)temp;
        }
        printf("Create: %lld ms\n", elapsed_ms(start));

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            object.Int32Property(123);
            auto value = object.Int32Property();
            (void)value;
        }
        printf("Int32: %lld ms\n", elapsed_ms(start));

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            object.StringProperty(L"value");
            auto value = object.StringProperty();
            (void)value;
        }
        printf("String: %lld ms\n", elapsed_ms(start));

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            object.ObjectProperty(object);
            auto value = object.ObjectProperty();
            (void)value;
        }
        printf("Object: %lld ms\n", elapsed_ms(start));

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            auto value = object.ObjectProperty().as<INonDefault>().Value();
            (void)value;
        }
        printf("Cast: %lld ms\n", elapsed_ms(start));

        auto token = object.Event([](Windows::Foundation::IInspectable const&, int32_t) {});
        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            object.Raise();
        }
        printf("Event: %lld ms\n", elapsed_ms(start));
        object.Event(token);

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            auto added = object.Event([](Windows::Foundation::IInspectable const&, int32_t) {});
            object.Event(added);
        }
        printf("AddRemove: %lld ms\n", elapsed_ms(start));

        {
            uint32_t const count = iterations > UINT32_MAX
                ? UINT32_MAX
                : static_cast<uint32_t>(iterations);
            auto vector = object.Items(count);

            start = std::chrono::high_resolution_clock::now();
            int32_t sum = 0;
            for (auto&& value : vector)
            {
                sum += value;
            }
            volatile int32_t sink = sum;
            (void)sink;
            printf("IterateVector: %lld ms\n", elapsed_ms(start));

            std::vector<int32_t> buffer(count);
            start = std::chrono::high_resolution_clock::now();
            vector.GetMany(0, buffer);
            printf("GetMany: %lld ms\n", elapsed_ms(start));

            auto map = object.Map(count);
            start = std::chrono::high_resolution_clock::now();
            int32_t msum = 0;
            for (auto&& pair : map)
            {
                msum += pair.Value();
            }
            volatile int32_t msink = msum;
            (void)msink;
            printf("Map: %lld ms\n", elapsed_ms(start));
        }

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            auto value = object.Operation().get();
            (void)value;
        }
        printf("Async: %lld ms\n", elapsed_ms(start));

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            auto value = object.Reference().Value();
            (void)value;
        }
        printf("Reference: %lld ms\n", elapsed_ms(start));

        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            try
            {
                (void)object.Next();
            }
            catch (hresult_error const&)
            {
            }
        }
        printf("Error: %lld ms\n", elapsed_ms(start));

        fflush(stdout);
        return 0;
    }
    catch (...)
    {
        return static_cast<int32_t>(winrt::to_hresult());
    }
}
