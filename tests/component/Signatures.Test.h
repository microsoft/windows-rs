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

        static bool SignatureBoolean(bool a, bool& b)
        {
            b = a;
            return a;
        }
        static com_array<bool> ArraySignatureBoolean(array_view<bool const> a, array_view<bool> b, com_array<bool>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<bool>(a.begin(), a.end());
            return com_array<bool>(a.begin(), a.end());
        }
        static void CallSignatureBoolean(winrt::Component::Signatures::SignatureBoolean const& handler)
        {
            auto a = true;
            auto b = false;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureBoolean(winrt::Component::Signatures::ArraySignatureBoolean const& handler)
        {
            std::array a{ true, false, true };
            std::array<bool, 3> b;
            com_array<bool> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static uint8_t SignatureUInt8(uint8_t a, uint8_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint8_t> ArraySignatureUInt8(array_view<uint8_t const> a, array_view<uint8_t> b, com_array<uint8_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint8_t>(a.begin(), a.end());
            return com_array<uint8_t>(a.begin(), a.end());
        }
        static void CallSignatureUInt8(winrt::Component::Signatures::SignatureUInt8 const& handler)
        {
            uint8_t a = 123;
            uint8_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureUInt8(winrt::Component::Signatures::ArraySignatureUInt8 const& handler)
        {
            std::array<uint8_t, 3> a{ 1, 2, 3 };
            std::array<uint8_t, 3> b;
            com_array<uint8_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static uint16_t SignatureUInt16(uint16_t a, uint16_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint16_t> ArraySignatureUInt16(array_view<uint16_t const> a, array_view<uint16_t> b, com_array<uint16_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint16_t>(a.begin(), a.end());
            return com_array<uint16_t>(a.begin(), a.end());
        }
        static void CallSignatureUInt16(winrt::Component::Signatures::SignatureUInt16 const& handler)
        {
            uint16_t a = 123;
            uint16_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureUInt16(winrt::Component::Signatures::ArraySignatureUInt16 const& handler)
        {
            std::array<uint16_t, 3> a{ 1, 2, 3 };
            std::array<uint16_t, 3> b;
            com_array<uint16_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static uint32_t SignatureUInt32(uint32_t a, uint32_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint32_t> ArraySignatureUInt32(array_view<uint32_t const> a, array_view<uint32_t> b, com_array<uint32_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint32_t>(a.begin(), a.end());
            return com_array<uint32_t>(a.begin(), a.end());
        }
        static void CallSignatureUInt32(winrt::Component::Signatures::SignatureUInt32 const& handler)
        {
            uint32_t a = 123;
            uint32_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureUInt32(winrt::Component::Signatures::ArraySignatureUInt32 const& handler)
        {
            std::array<uint32_t, 3> a{ 1, 2, 3 };
            std::array<uint32_t, 3> b;
            com_array<uint32_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static uint64_t SignatureUInt64(uint64_t a, uint64_t& b)
        {
            b = a;
            return a;
        }
        static com_array<uint64_t> ArraySignatureUInt64(array_view<uint64_t const> a, array_view<uint64_t> b, com_array<uint64_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<uint64_t>(a.begin(), a.end());
            return com_array<uint64_t>(a.begin(), a.end());
        }
        static void CallSignatureUInt64(winrt::Component::Signatures::SignatureUInt64 const& handler)
        {
            uint64_t a = 123;
            uint64_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureUInt64(winrt::Component::Signatures::ArraySignatureUInt64 const& handler)
        {
            std::array<uint64_t, 3> a{ 1, 2, 3 };
            std::array<uint64_t, 3> b;
            com_array<uint64_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static int16_t SignatureInt16(int16_t a, int16_t& b)
        {
            b = a;
            return a;
        }
        static com_array<int16_t> ArraySignatureInt16(array_view<int16_t const> a, array_view<int16_t> b, com_array<int16_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<int16_t>(a.begin(), a.end());
            return com_array<int16_t>(a.begin(), a.end());
        }
        static void CallSignatureInt16(winrt::Component::Signatures::SignatureInt16 const& handler)
        {
            int16_t a = 123;
            int16_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureInt16(winrt::Component::Signatures::ArraySignatureInt16 const& handler)
        {
            std::array<int16_t, 3> a{ 1, 2, 3 };
            std::array<int16_t, 3> b;
            com_array<int16_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static int32_t SignatureInt32(int32_t a, int32_t& b)
        {
            b = a;
            return a;
        }
        static com_array<int32_t> ArraySignatureInt32(array_view<int32_t const> a, array_view<int32_t> b, com_array<int32_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<int32_t>(a.begin(), a.end());
            return com_array<int32_t>(a.begin(), a.end());
        }
        static void CallSignatureInt32(winrt::Component::Signatures::SignatureInt32 const& handler)
        {
            int32_t a = 123;
            int32_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureInt32(winrt::Component::Signatures::ArraySignatureInt32 const& handler)
        {
            std::array<int32_t, 3> a{ 1, 2, 3 };
            std::array<int32_t, 3> b;
            com_array<int32_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static int64_t SignatureInt64(int64_t a, int64_t& b)
        {
            b = a;
            return a;
        }
        static com_array<int64_t> ArraySignatureInt64(array_view<int64_t const> a, array_view<int64_t> b, com_array<int64_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<int64_t>(a.begin(), a.end());
            return com_array<int64_t>(a.begin(), a.end());
        }
        static void CallSignatureInt64(winrt::Component::Signatures::SignatureInt64 const& handler)
        {
            int64_t a = 123;
            int64_t b = 0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureInt64(winrt::Component::Signatures::ArraySignatureInt64 const& handler)
        {
            std::array<int64_t, 3> a{ 1, 2, 3 };
            std::array<int64_t, 3> b;
            com_array<int64_t> c;
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
