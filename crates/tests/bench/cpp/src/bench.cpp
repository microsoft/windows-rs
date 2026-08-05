#include <windows.h>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <vector>
#include "winrt/Bench.h"
#include "winrt/Windows.Foundation.Collections.h"

using namespace winrt;
using namespace winrt::Bench;

static long long elapsed_ms(std::chrono::high_resolution_clock::time_point const start)
{
    return std::chrono::duration_cast<std::chrono::milliseconds>(
               std::chrono::high_resolution_clock::now() - start)
        .count();
}

extern "C" int32_t __stdcall test_bench_cpp(uint64_t iterations) noexcept
{
    try
    {
        init_apartment();
        Widget object;
        printf("# C++/WinRT consumer -> Bench component - %llu iterations\n",
               static_cast<unsigned long long>(iterations));

        auto start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            Widget temp;
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
        int32_t sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            sum += object.Add(static_cast<int32_t>(i), 1);
        }
        volatile int32_t sink = sum;
        (void)sink;
        printf("Add: %lld ms\n", elapsed_ms(start));

        // Cast: QueryInterface from the default interface to the non-default INonDefault and
        // call a method on it -- the per-cast cost the projection adds over a raw as<>().
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            sum += object.as<INonDefault>().Value();
        }
        volatile int32_t csink = sum;
        (void)csink;
        printf("Cast: %lld ms\n", elapsed_ms(start));

        // CastOwned: C++/WinRT's normal cast is already a stack-value owner, so this repeats the
        // same QI/call/destructor shape under the explicit ownership label.
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            auto value = object.as<INonDefault>();
            sum += value.Value();
        }
        volatile int32_t cosink = sum;
        (void)cosink;
        printf("CastOwned: %lld ms\n", elapsed_ms(start));

        // Interface: acquire the non-default interface once, then measure steady calls.
        auto non_default = object.as<INonDefault>();
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            sum += non_default.Value();
        }
        volatile int32_t interface_sink = sum;
        (void)interface_sink;
        printf("Interface: %lld ms\n", elapsed_ms(start));

        // Object: set and get metadata Object (IInspectable). The setter borrows the input and the
        // getter returns a reference released by the projected value's destructor.
        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            object.ObjectProperty(object);
            auto value = object.ObjectProperty();
            (void)value;
        }
        printf("Object: %lld ms\n", elapsed_ms(start));

        // Event: subscribe one handler and raise the event N times. Each Signal calls from the
        // component into this consumer's delegate. C++/WinRT invokes the delegate through one
        // interface pointer with no per-raise heap allocation.
        ChangedHandler handler([](Windows::Foundation::IInspectable const&, int32_t) {});
        auto token = object.Changed(handler);
        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            object.Signal(static_cast<int32_t>(i));
        }
        printf("Event: %lld ms\n", elapsed_ms(start));
        object.Changed(token);

        // AddRemove: subscribe and unsubscribe the same handler N times, measuring
        // event-registration churn (each add stores a delegate reference, each remove drops it).
        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            auto t = object.Changed(handler);
            object.Changed(t);
        }
        printf("AddRemove: %lld ms\n", elapsed_ms(start));

        // Vector: read elements from a projected generic collection IVector<int32_t>. The vector is
        // built once, then GetAt reads one element per iteration -- the per-element cost of a
        // generic collection call across the ABI. C++/WinRT dispatches the call through one
        // interface pointer with no per-element allocation.
        const uint32_t vector_len = 1024;
        auto vector = object.Items(vector_len);
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            sum += vector.GetAt(static_cast<uint32_t>(i % vector_len));
        }
        volatile int32_t vsink = sum;
        (void)vsink;
        printf("Vector: %lld ms\n", elapsed_ms(start));

        // IterateVector: a range-for over the whole collection, repeated a bounded number of
        // passes. C++/WinRT's projected iterator reads each element through GetAt, with no
        // per-element allocation.
        uint64_t iterate_passes = iterations > 100000 ? 100000u : iterations;
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterate_passes; i++)
        {
            for (auto&& v : vector)
            {
                sum += v;
            }
        }
        volatile int32_t isink = sum;
        (void)isink;
        printf("IterateVector: %lld ms\n", elapsed_ms(start));

        // GetMany: copy a vector sized to the requested iteration count into one caller-owned
        // buffer. Construction and allocation happen before the timer.
        uint32_t const bulk_count = iterations > INT32_MAX
            ? INT32_MAX
            : static_cast<uint32_t>(iterations);
        auto bulk_vector = object.Items(bulk_count);
        std::vector<int32_t> buffer(bulk_count);
        start = std::chrono::high_resolution_clock::now();
        uint32_t actual = bulk_vector.GetMany(0, buffer);
        volatile uint32_t gsink = actual;
        (void)gsink;
        printf("GetMany: %lld ms\n", elapsed_ms(start));

        const uint32_t map_len = 1024;

        // Map: enumerate an IMap<hstring, int32_t>. Repeat a bounded 1024-entry map enough times to
        // keep total entry visits near the requested count.
        auto string_map = object.StringMap(map_len);
        uint64_t map_passes = iterations / map_len;
        if (map_passes == 0) map_passes = 1;
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < map_passes; i++)
        {
            for (auto&& pair : string_map)
            {
                sum += pair.Value();
            }
        }
        volatile int32_t mesink = sum;
        (void)mesink;
        printf("Map: %lld ms\n", elapsed_ms(start));

        // Lookup: read values from a projected generic dictionary IMap<int32_t, int32_t> by key.
        // The map is built once, then Lookup reads one value per iteration through one interface
        // pointer with no per-lookup allocation.
        auto map = object.Map(map_len);
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            sum += map.Lookup(static_cast<int32_t>(i % map_len));
        }
        volatile int32_t lsink = sum;
        (void)lsink;
        printf("Lookup: %lld ms\n", elapsed_ms(start));

        // VectorView: read elements from the read-only view IVectorView<int32_t>. Same per-element
        // cost as Vector - one interface-pointer call - confirming the view projects like the
        // collection.
        auto vector_view = object.ItemsView(vector_len);
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            sum += vector_view.GetAt(static_cast<uint32_t>(i % vector_len));
        }
        volatile int32_t vvsink = sum;
        (void)vvsink;
        printf("VectorView: %lld ms\n", elapsed_ms(start));

        // MapView: read values from the read-only view IMapView<int32_t, int32_t> by key. Same
        // per-lookup cost as Lookup.
        auto map_view = object.MapView(map_len);
        start = std::chrono::high_resolution_clock::now();
        sum = 0;
        for (uint64_t i = 0; i < iterations; i++)
        {
            sum += map_view.Lookup(static_cast<int32_t>(i % map_len));
        }
        volatile int32_t mvsink = sum;
        (void)mvsink;
        printf("MapView: %lld ms\n", elapsed_ms(start));

        // Reference: box a nullable Int32 input as IReference<int> and unbox the returned value.
        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            object.ReferenceProperty(0);
            auto value = object.ReferenceProperty().Value();
            (void)value;
        }
        printf("Reference: %lld ms\n", elapsed_ms(start));

        // Async: obtain an already-completed IAsyncOperation<int32_t> and read its result.
        start = std::chrono::high_resolution_clock::now();
        for (uint64_t i = 0; i < iterations; i++)
        {
            auto value = object.Operation().get();
            (void)value;
        }
        printf("Async: %lld ms\n", elapsed_ms(start));

        // Error: call a method that always returns a failing HRESULT. C++/WinRT throws
        // hresult_error on the failed check; the caller catches it. Throwing and catching a C++
        // exception costs orders of magnitude more than the scalar calls above, so this loop runs
        // a reduced count.
        uint64_t fail_iterations = iterations > 1000000 ? 1000000u : iterations;
        start = std::chrono::high_resolution_clock::now();
        uint64_t errors = 0;
        for (uint64_t i = 0; i < fail_iterations; i++)
        {
            try { object.Fail(); } catch (winrt::hresult_error const&) { errors++; }
        }
        volatile uint64_t esink = errors;
        (void)esink;
        printf("Error: %lld ms\n", elapsed_ms(start));

        // Leak check: activate, cast, and release N objects, then confirm the component's live
        // instance count returns to the baseline. C++/WinRT's Widget and INonDefault are RAII
        // handles that Release in their destructors, so every AddRef (activation, as<>) balances.
        int32_t baseline = object.LiveCount();
        for (uint64_t i = 0; i < iterations; i++)
        {
            Widget scratch;
            (void)scratch.as<INonDefault>().Value();
        }
        printf("Leak: %d\n", object.LiveCount() - baseline);

        // Scalability: hold N live objects and report the client-side bytes per object. A
        // winrt::Bench::Widget is one interface pointer, matching windows-rs.
        uint32_t live = iterations > 1000000 ? 1000000u : static_cast<uint32_t>(iterations);
        std::vector<Widget> widgets;
        widgets.reserve(live);
        for (uint32_t i = 0; i < live; i++)
        {
            widgets.push_back(Widget());
        }
        size_t bytes = widgets.capacity() * sizeof(Widget);
        printf("Live-%u: %zu bytes (%.1f bytes/object)\n",
               live, bytes, static_cast<double>(bytes) / live);

        fflush(stdout);
        return 0;
    }
    catch (...)
    {
        return static_cast<int32_t>(winrt::to_hresult());
    }
}
