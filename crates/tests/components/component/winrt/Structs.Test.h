#pragma once
#include "Structs.Test.g.h"

namespace winrt::Component::Structs::implementation
{
    struct Test
    {
        Test() = default;

        static uint32_t SizeOfBlittable()
        {
            return sizeof(Blittable);
        }

        static uint32_t SizeOfNonBlittable()
        {
            return sizeof(NonBlittable);
        }

        static uint32_t SizeOfNested()
        {
            return sizeof(Nested);
        }

        static guid GuidOfRefNested()
        {
            return guid_of<Windows::Foundation::IReference<Nested>>();
        }
    };
}
namespace winrt::Component::Structs::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
