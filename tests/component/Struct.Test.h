#pragma once
#include "Struct.Test.g.h"

namespace winrt::Component::Struct::implementation
{
    struct Test
    {
        Test() = default;

        static uint32_t SizeOfBlittable()
        {
            return sizeof(Blittable);
        }

        static Blittable ZeroBlittable()
        {
            return {};
        }

        static Blittable NonZeroBlittable()
        {
            Blittable b{};
            b.Bool = true;
            b.Char = L'B';
            b.UInt8 = 8;
            b.UInt16 = 16;
            b.UInt32 = 32;
            b.UInt64 = 64;
            b.Int16 = -16;
            b.Int32 = -32;
            b.Int64 = -64;
            b.Single = 1.23f;
            b.Double = 4.56;
            b.Guid = guid("97FCC68A-30DE-42C0-AD91-0ADB63F4E934");
            return b;
        }
    };
}
namespace winrt::Component::Struct::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
