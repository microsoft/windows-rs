#pragma once
#include "Simple.Class.g.h"

namespace winrt::Component::Simple::implementation
{
    struct Class : ClassT<Class>
    {
        Class() = default;

        void Method()
        {
        }
    };
}

namespace winrt::Component::Simple::factory_implementation
{
    struct Class : ClassT<Class, implementation::Class>
    {
    };
}
