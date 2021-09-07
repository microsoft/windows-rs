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

        static uint8_t Signature2(uint8_t a, uint8_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint8_t> ArraySignature2(array_view<uint8_t const> a, array_view<uint8_t> b, com_array<uint8_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint8_t>(a.begin(), a.end());
            return com_array<uint8_t>(a.begin(), a.end());
        }
        static void CallSignature2(winrt::Component::Signatures::Signature2 const& handler)
        {
            uint8_t a = 123;
            uint8_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignature2(winrt::Component::Signatures::ArraySignature2 const& handler)
        {
            std::array<uint8_t, 3> a{ 1, 2, 3 };
            std::array<uint8_t, 3> b;
            com_array<uint8_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static uint16_t Signature3(uint16_t a, uint16_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint16_t> ArraySignature3(array_view<uint16_t const> a, array_view<uint16_t> b, com_array<uint16_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint16_t>(a.begin(), a.end());
            return com_array<uint16_t>(a.begin(), a.end());
        }
        static void CallSignature3(winrt::Component::Signatures::Signature3 const& handler)
        {
            uint16_t a = 123;
            uint16_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignature3(winrt::Component::Signatures::ArraySignature3 const& handler)
        {
            std::array<uint16_t, 3> a{ 1, 2, 3 };
            std::array<uint16_t, 3> b;
            com_array<uint16_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static uint32_t Signature4(uint32_t a, uint32_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint32_t> ArraySignature4(array_view<uint32_t const> a, array_view<uint32_t> b, com_array<uint32_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint32_t>(a.begin(), a.end());
            return com_array<uint32_t>(a.begin(), a.end());
        }
        static void CallSignature4(winrt::Component::Signatures::Signature4 const& handler)
        {
            uint32_t a = 123;
            uint32_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignature4(winrt::Component::Signatures::ArraySignature4 const& handler)
        {
            std::array<uint32_t, 3> a{ 1, 2, 3 };
            std::array<uint32_t, 3> b;
            com_array<uint32_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static uint64_t Signature5(uint64_t a, uint64_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint64_t> ArraySignature5(array_view<uint64_t const> a, array_view<uint64_t> b, com_array<uint64_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint64_t>(a.begin(), a.end());
            return com_array<uint64_t>(a.begin(), a.end());
        }
        static void CallSignature5(winrt::Component::Signatures::Signature5 const& handler)
        {
            uint64_t a = 123;
            uint64_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignature5(winrt::Component::Signatures::ArraySignature5 const& handler)
        {
            std::array<uint64_t, 3> a{ 1, 2, 3 };
            std::array<uint64_t, 3> b;
            com_array<uint64_t> c;
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
