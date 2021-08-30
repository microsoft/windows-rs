#pragma once
#include "Simple.SimpleClass.g.h"

namespace winrt::component::Simple::implementation
{
    struct SimpleClass : SimpleClassT<SimpleClass>
    {
        SimpleClass() = default;

        void Method()
        {
        }
    };
}

namespace winrt::component::Simple::factory_implementation
{
    struct SimpleClass : SimpleClassT<SimpleClass, implementation::SimpleClass>
    {
    };
}
