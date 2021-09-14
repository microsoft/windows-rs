#pragma once
#include "Conversion.Test.g.h"

namespace winrt::Component::Conversion::implementation
{
    struct Test
    {
        static uint32_t ExpectTimeSpan(winrt::Windows::Foundation::TimeSpan const& value)
        {
            auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(value);
            return static_cast<uint32_t>(ms.count());
        }

        static hstring ExpectObject(winrt::Windows::Foundation::IInspectable const& value)
        {
            return get_class_name(value);
        }
    };
}
namespace winrt::Component::Conversion::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
