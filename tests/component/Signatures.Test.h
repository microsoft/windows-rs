#pragma once
#include "Signatures.Test.g.h"

inline void check(bool expression)
{
    if (!expression)
    {
        throw winrt::hresult_invalid_argument();
    }
}

namespace winrt::Component::Signatures::implementation
{
    struct Test
    {
        Test() = default;

        static bool Signature1(bool a, bool& b)
        {
            b = a;
            return a;
        }

        static com_array<bool> ArraySignature1(array_view<bool const> a, array_view<bool> b, com_array<bool>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<bool>(a.begin(), a.end());
            return com_array<bool>(a.begin(), a.end());
        }

        static void CallSignature1(winrt::Component::Signatures::Signature1 const& handler)
        {
            auto a = true;
            auto b = false;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }

        static void CallArraySignature1(winrt::Component::Signatures::ArraySignature1 const& handler)
        {
            std::array a{ true, false, true };
            std::array<bool, 3> b;
            com_array<bool> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }
    };
}
namespace winrt::Component::Signatures::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
