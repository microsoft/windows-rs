#include <winerror.h>
#include <winrt/Callback.h>
#include "check.h"

using namespace winrt;
using namespace Callback;

struct Value : implements<Value, IValue>
{
    int32_t m_current;

    Value(int32_t current) : m_current(current)
    {
    }

    int32_t Current()
    {
        return m_current;
    }

    void Current(int32_t current)
    {
        m_current = current;
    }

    int32_t Add(IValue const& other)
    {
        if (!other)
        {
            throw hresult_error(E_POINTER);
        }

        return m_current + other.Current();
    }

    void Create(int32_t current, IValue& result)
    {
        result = make<Value>(current);
    }
};

void test()
{
    Widget().Check(make<Value>(0));
}
