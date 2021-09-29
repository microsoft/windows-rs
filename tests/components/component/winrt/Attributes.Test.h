#pragma once
#include "Attributes.Test.g.h"

namespace winrt::Component::Attributes::implementation
{
    struct Test : TestT<Test>
    {
        Test() = default;

        void Method()
        {
        }
    };
}
namespace winrt::Component::Attributes::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
