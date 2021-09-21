#include "pch.h"
#include "query.h"

struct Class : winrt::implements<Class, IQueryInt32, IQuerySingle>
{
    int32_t m_value{};

    Class(int32_t value) : m_value(value) {}

    int32_t __stdcall GetInt32() noexcept
    {
        return m_value;
    }

    float __stdcall GetSingle() noexcept
    {
        return m_value / 100.f;
    }
};

HRESULT __stdcall QueryInterface(GUID const& iid, void** object) noexcept
{
    auto temp = winrt::make<Class>(123);
    return temp.as(iid, object);
}

HRESULT __stdcall QueryWithValue(int32_t value, GUID const& iid, void** object) noexcept
{
    auto temp = winrt::make<Class>(value);
    return temp.as(iid, object);
}
