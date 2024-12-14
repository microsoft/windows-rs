#include <windows.h>
#include <atomic>
#include "winrt/Test.h"

using namespace winrt;
using namespace winrt::Test;

struct Test : implements<Test, ITest>
{
    std::atomic_int32_t m_int32;

    Test(int32_t value) : m_int32(value)
    {
    }

    int32_t Input(ITest const& input)
    {
        if (!input)
        {
            throw hresult_error(E_POINTER);
        }

        return input.Current();
    }

    void Output(int32_t value, ITest& output)
    {
        output = make<Test>(value);
    }

    int32_t Current()
    {
        return m_int32;
    }

    void Current(int32_t value)
    {
        m_int32 = value;
    }
};

void test_interface(ITest const& test)
{
    assert(test.Current() == 0);
    test.Current(-321);
    assert(test.Current() == -321);

    ITest one_two_three = make<Test>(123);
    ITest four_five_six = make<Test>(456);

    assert(test.Input(one_two_three) == 123);
    assert(test.Input(four_five_six) == 456);

    ITest seven_eight_nine;
    test.Output(789, seven_eight_nine);
    assert(seven_eight_nine.Current() == 789);
}

extern "C"
{
    HRESULT __stdcall consume(void *abi) noexcept
    try
    {
        ITest const &test = *reinterpret_cast<ITest const *>(&abi);
        test_interface(test);
        return S_OK;
    }
    catch (...)
    {
        return to_hresult();
    }

    HRESULT __stdcall produce(void **abi) noexcept
    try
    {
        *abi = detach_abi(make<Test>(0));
        return S_OK;
    }
    catch (...)
    {
        return to_hresult();
    }
}
