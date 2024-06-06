#include <windows.h>
#include "winrt/Test.h"

using namespace winrt;
using namespace winrt::Test;

void test_except(ITest const &test)
{
    test.MethodString(L"abc");
    assert(test.String() == L"abc");

    test.MethodInt32(123);
    assert(test.Int32() == 123);

    test.MethodTest(test);
    assert(test.Test() == test);
}

void test_noexcept(ITest const &test)
{
    test.MethodStringN(L"abc");
    assert(test.StringN() == L"abc");

    test.MethodInt32N(123);
    assert(test.Int32N() == 123);

    test.MethodTestN(test);
    assert(test.TestN() == test);
}

struct Implementation : implements<Implementation, ITest>
{
    hstring m_string;
    int32_t m_int32;
    ITest m_test;

    void MethodString(hstring const &value)
    {
        m_string = value;
    }
    void MethodInt32(int32_t value)
    {
        m_int32 = value;
    }
    void MethodTest(ITest const &value)
    {
        m_test = value;
    }
    hstring String() const
    {
        return m_string;
    }
    void String(hstring const &value)
    {
        m_string = value;
    }
    int32_t Int32() const
    {
        return m_int32;
    }
    void Int32(int32_t value)
    {
        m_int32 = value;
    }
    ITest Test() const
    {
        return m_test;
    }
    void Test(ITest const &value)
    {
        m_test = value;
    }

    void MethodStringN(hstring const &value) noexcept
    {
        m_string = value;
    }
    void MethodInt32N(int32_t value) noexcept
    {
        m_int32 = value;
    }
    void MethodTestN(ITest const &value) noexcept
    {
        m_test = value;
    }
    hstring StringN() const noexcept
    {
        return m_string;
    }
    void StringN(hstring const &value) noexcept
    {
        m_string = value;
    }
    int32_t Int32N() const noexcept
    {
        return m_int32;
    }
    void Int32N(int32_t value) noexcept
    {
        m_int32 = value;
    }
    ITest TestN() const noexcept
    {
        return m_test;
    }
    void TestN(ITest const &value) noexcept
    {
        m_test = value;
    }
};

extern "C"
{
    HRESULT __stdcall consume(void *abi) noexcept
    try
    {
        ITest const &test = *reinterpret_cast<ITest const *>(&abi);

        test_noexcept(test);
        test_except(test);

        return S_OK;
    }
    catch (...)
    {
        return to_hresult();
    }

    HRESULT __stdcall produce(void **abi) noexcept
    try
    {
        *abi = detach_abi(make<Implementation>());

        return S_OK;
    }
    catch (...)
    {
        return to_hresult();
    }
}
