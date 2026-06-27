#include <windows.h>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include "winrt/LangPerf.h"

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
        printf("# C++/WinRT - %llu iterations\n", static_cast<unsigned long long>(iterations));

        auto start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            Class object;
            (void)object;
        }
        printf("Create: %lld ms\n", elapsed_ms(start));

        Class object;

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
