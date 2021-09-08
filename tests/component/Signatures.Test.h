#pragma once
#include "Signatures.Test.g.h"
#include "winrt/Component.Simple.h"

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

        static float SignatureSingle(float a, float& b)
        {
            b = a;
            return a;
        }
        static com_array<float> ArraySignatureSingle(array_view<float const> a, array_view<float> b, com_array<float>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<float>(a.begin(), a.end());
            return com_array<float>(a.begin(), a.end());
        }
        static void CallSignatureSingle(winrt::Component::Signatures::SignatureSingle const& handler)
        {
            float a = 123.0f;
            float b = 0.0f;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureSingle(winrt::Component::Signatures::ArraySignatureSingle const& handler)
        {
            std::array<float, 3> a{ 1.0f, 2.0f, 3.0f };
            std::array<float, 3> b;
            com_array<float> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static double SignatureDouble(double a, double& b)
        {
            b = a;
            return a;
        }
        static com_array<double> ArraySignatureDouble(array_view<double const> a, array_view<double> b, com_array<double>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<double>(a.begin(), a.end());
            return com_array<double>(a.begin(), a.end());
        }
        static void CallSignatureDouble(winrt::Component::Signatures::SignatureDouble const& handler)
        {
            double a = 123.0;
            double b = 0.0;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureDouble(winrt::Component::Signatures::ArraySignatureDouble const& handler)
        {
            std::array<double, 3> a{ 1.0, 2.0, 3.0 };
            std::array<double, 3> b;
            com_array<double> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static char16_t SignatureChar(char16_t a, char16_t& b)
        {
            b = a;
            return a;
        }
        static com_array<char16_t> ArraySignatureChar(array_view<char16_t const> a, array_view<char16_t> b, com_array<char16_t>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<char16_t>(a.begin(), a.end());
            return com_array<char16_t>(a.begin(), a.end());
        }
        static void CallSignatureChar(winrt::Component::Signatures::SignatureChar const& handler)
        {
            char16_t a = L'A';
            char16_t b = L' ';
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureChar(winrt::Component::Signatures::ArraySignatureChar const& handler)
        {
            std::array<char16_t, 3> a{ L'A', 'b', 'c'};
            std::array<char16_t, 3> b;
            com_array<char16_t> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static hstring SignatureString(hstring const& a, hstring& b)
        {
            b = a;
            return a;
        }
        static com_array<hstring> ArraySignatureString(array_view<hstring const> a, array_view<hstring> b, com_array<hstring>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<hstring>(a.begin(), a.end());
            return com_array<hstring>(a.begin(), a.end());
        }
        static void CallSignatureString(winrt::Component::Signatures::SignatureString const& handler)
        {
            hstring a = L"string";
            hstring b;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureString(winrt::Component::Signatures::ArraySignatureString const& handler)
        {
            std::array<hstring, 3> a{ L"first", L"second", L"third" };
            std::array<hstring, 3> b;
            com_array<hstring> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static guid SignatureGuid(guid const& a, guid& b)
        {
            b = a;
            return a;
        }
        static com_array<guid> ArraySignatureGuid(array_view<guid const> a, array_view<guid> b, com_array<guid>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<guid>(a.begin(), a.end());
            return com_array<guid>(a.begin(), a.end());
        }
        static void CallSignatureGuid(winrt::Component::Signatures::SignatureGuid const& handler)
        {
            guid a("006A8569-0BC8-446B-9D1F-240FED17250D");
            guid b;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureGuid(winrt::Component::Signatures::ArraySignatureGuid const& handler)
        {
            std::array<guid, 3> a{ guid("B0180C8C-8FEB-448A-A915-AC92E05135FE"), guid("9E234A6E-DF89-4891-AAD5-632692BBB1DC"), guid("286F8B75-2DF4-49CF-841C-52438E2D5326") };
            std::array<guid, 3> b;
            com_array<guid> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static hresult SignatureHResult(hresult const& a, hresult& b)
        {
            b = a;
            return a;
        }
        static com_array<hresult> ArraySignatureHResult(array_view<hresult const> a, array_view<hresult> b, com_array<hresult>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<hresult>(a.begin(), a.end());
            return com_array<hresult>(a.begin(), a.end());
        }
        static void CallSignatureHResult(winrt::Component::Signatures::SignatureHResult const& handler)
        {
            hresult a = E_INVALIDARG;
            hresult b;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureHResult(winrt::Component::Signatures::ArraySignatureHResult const& handler)
        {
            std::array<hresult, 3> a{ E_NOINTERFACE, E_INVALIDARG, S_OK };
            std::array<hresult, 3> b;
            com_array<hresult> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static Windows::Foundation::IInspectable SignatureObject(Windows::Foundation::IInspectable const& a, Windows::Foundation::IInspectable& b)
        {
            b = a;
            return a;
        }
        static com_array<Windows::Foundation::IInspectable> ArraySignatureObject(array_view<Windows::Foundation::IInspectable const> a, array_view<Windows::Foundation::IInspectable> b, com_array<Windows::Foundation::IInspectable>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<Windows::Foundation::IInspectable>(a.begin(), a.end());
            return com_array<Windows::Foundation::IInspectable>(a.begin(), a.end());
        }
        static void CallSignatureObject(winrt::Component::Signatures::SignatureObject const& handler)
        {
            Windows::Foundation::IInspectable a = Simple::Class();
            Windows::Foundation::IInspectable b;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureObject(winrt::Component::Signatures::ArraySignatureObject const& handler)
        {
            std::array<Windows::Foundation::IInspectable, 3> a{ Simple::Class(), Simple::Class(), Simple::Class() };
            std::array<Windows::Foundation::IInspectable, 3> b;
            com_array<Windows::Foundation::IInspectable> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static Simple::Class SignatureClass(Simple::Class const& a, Simple::Class& b)
        {
            b = a;
            return a;
        }
        static com_array<Simple::Class> ArraySignatureClass(array_view<Simple::Class const> a, array_view<Simple::Class> b, com_array<Simple::Class>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array<Simple::Class>(a.begin(), a.end());
            return com_array<Simple::Class>(a.begin(), a.end());
        }
        static void CallSignatureClass(winrt::Component::Signatures::SignatureClass const& handler)
        {
            Simple::Class a = Simple::Class();
            Simple::Class b;
            auto c = handler(a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureClass(winrt::Component::Signatures::ArraySignatureClass const& handler)
        {
            std::array a{ Simple::Class(), Simple::Class(), Simple::Class() };
            std::array<Simple::Class, 3> b;
            com_array<Simple::Class> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static Structs::Blittable SignatureBlittable(Structs::Blittable const& a, Structs::Blittable const& b, Structs::Blittable& c)
        {
            check(a == b);
            c = a;
            return a;
        }
        static com_array<Structs::Blittable> ArraySignatureBlittable(array_view<Structs::Blittable const> a, array_view<Structs::Blittable> b, com_array<Structs::Blittable>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array(a.begin(), a.end());
            return com_array(a.begin(), a.end());
        }
        static void CallSignatureBlittable(winrt::Component::Signatures::SignatureBlittable const& handler)
        {
            Structs::Blittable a{ true, L'A', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("B0180C8C-8FEB-448A-A915-AC92E05135FE") };
            Structs::Blittable b;
            auto c = handler(a, a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureBlittable(winrt::Component::Signatures::ArraySignatureBlittable const& handler)
        {
            std::array a{
                Structs::Blittable{ true, L'A', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("B0180C8C-8FEB-448A-A915-AC92E05135FE") },
                Structs::Blittable{ false, L'B', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("9E234A6E-DF89-4891-AAD5-632692BBB1DC") },
                Structs::Blittable{ true, L'C', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("286F8B75-2DF4-49CF-841C-52438E2D5326") },
            };

            std::array<Structs::Blittable, 3> b;
            com_array<Structs::Blittable> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static Structs::NonBlittable SignatureNonBlittable(Structs::NonBlittable const& a, Structs::NonBlittable const& b, Structs::NonBlittable& c)
        {
            check(a == b);
            c = a;
            return a;
        }
        static com_array<Structs::NonBlittable> ArraySignatureNonBlittable(array_view<Structs::NonBlittable const> a, array_view<Structs::NonBlittable> b, com_array<Structs::NonBlittable>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array(a.begin(), a.end());
            return com_array(a.begin(), a.end());
        }
        static void CallSignatureNonBlittable(winrt::Component::Signatures::SignatureNonBlittable const& handler)
        {
            Structs::NonBlittable a{ L"string", 1234 };
            Structs::NonBlittable b;
            auto c = handler(a, a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureNonBlittable(winrt::Component::Signatures::ArraySignatureNonBlittable const& handler)
        {
            std::array a{
                Structs::NonBlittable{ L"first", 1, },
                Structs::NonBlittable{ L"second", 2, },
                Structs::NonBlittable{ L"third", 3, },
            };

            std::array<Structs::NonBlittable, 3> b;
            com_array<Structs::NonBlittable> c;
            com_array d = handler(a, b, c);
            check(a == b);
            check(std::equal(a.begin(), a.end(), c.begin(), c.end()));
            check(std::equal(a.begin(), a.end(), d.begin(), d.end()));
        }

        static Structs::Nested SignatureNested(Structs::Nested const& a, Structs::Nested const& b, Structs::Nested& c)
        {
            check(a == b);
            c = a;
            return a;
        }
        static com_array<Structs::Nested> ArraySignatureNested(array_view<Structs::Nested const> a, array_view<Structs::Nested> b, com_array<Structs::Nested>& c)
        {
            check(a.size() == b.size());
            check(c.size() == 0);
            std::copy(a.begin(), a.end(), b.begin());
            c = com_array(a.begin(), a.end());
            return com_array(a.begin(), a.end());
        }
        static void CallSignatureNested(winrt::Component::Signatures::SignatureNested const& handler)
        {
            Structs::Nested a{ { true, L'A', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("B0180C8C-8FEB-448A-A915-AC92E05135FE") }, { L"string", 1234 } };
            Structs::Nested b;
            auto c = handler(a, a, b);
            check(a == b);
            check(a == c);
        }
        static void CallArraySignatureNested(winrt::Component::Signatures::ArraySignatureNested const& handler)
        {
            std::array a{
                Structs::Nested{ { true, L'A', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("B0180C8C-8FEB-448A-A915-AC92E05135FE") }, { L"string", 123 } },
                Structs::Nested{ { false, L'B', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("9E234A6E-DF89-4891-AAD5-632692BBB1DC") }, { L"string", 456 } },
                Structs::Nested{ { true, L'C', 1,2,3,4, -1, -2, -3, 1.0f, 0.1, guid("286F8B75-2DF4-49CF-841C-52438E2D5326") }, { L"string", 789 } }
            };

            std::array<Structs::Nested, 3> b;
            com_array<Structs::Nested> c;
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
