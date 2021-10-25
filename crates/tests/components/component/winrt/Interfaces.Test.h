#pragma once
#include "Interfaces.Test.g.h"

namespace winrt::Component::Interfaces::implementation
{
    struct Test
    {
        Test() = default;

        static winrt::Component::Interfaces::IRequires GoodRequires()
        {
            struct Good : implements<Good, IRequires, IRequired>
            {
                void Required() {}
                void Requires() {}
            };

            return make<Good>();
        }

        static winrt::Component::Interfaces::IRequires BadRequires()
        {
            struct Bad : implements<Bad, IRequires>
            {
                void Requires() {}
            };

            return make<Bad>();
        }
    };
}
namespace winrt::Component::Interfaces::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
