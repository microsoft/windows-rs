#include "pch.h"
#include "query.h"

namespace
{
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

        HRESULT __stdcall Query(GUID const& iid, _COM_Outptr_ void** object) noexcept
        {
            return QueryInterface(iid, object);
        }

        HRESULT __stdcall QueryWithValue(int32_t value, GUID const& iid, _COM_Outptr_ void** object) noexcept
        {
            m_value = value;
            return QueryInterface(iid, object);
        }

        HRESULT __stdcall QueryOptionalWithValue(int32_t /*value*/, GUID const& iid, _COM_Outptr_opt_ void** optional) noexcept
        {
            winrt::com_ptr<::IUnknown> object;
            HRESULT hr = QueryInterface(iid, object.put_void());

            if (optional)
            {
                *optional = object.detach();
            }

            return hr;
        }

        HRESULT __stdcall QueryOptionalWithConvertible(IUnknown* /*value*/, GUID const& iid, _COM_Outptr_opt_ void** optional) noexcept
        {
            winrt::com_ptr<::IUnknown> object;
            HRESULT hr = QueryInterface(iid, object.put_void());

            if (optional)
            {
                *optional = object.detach();
            }

            return hr;
        }
    };
}

HRESULT __stdcall Query(GUID const& iid, void** object) noexcept
{
    auto temp = winrt::make<Class>(123);
    return temp.as(iid, object);
}

HRESULT __stdcall QueryWithValue(int32_t value, GUID const& iid, void** object) noexcept
{
    auto temp = winrt::make<Class>(value);
    return temp.as(iid, object);
}

HRESULT __stdcall QueryOptionalWithValue(int32_t /*value*/, GUID const& iid, void** optional) noexcept
{
    auto temp = winrt::make<Class>(123);

    winrt::com_ptr<::IUnknown> object;
    HRESULT hr = temp.as(iid, object.put_void());

    if (optional)
    {
        *optional = object.detach();
    }

    return hr;
}

HRESULT __stdcall QueryOptionalWithConvertible(IUnknown* /*value*/, GUID const& iid, void** optional) noexcept
{
    auto temp = winrt::make<Class>(123);

    winrt::com_ptr<::IUnknown> object;
    HRESULT hr = temp.as(iid, object.put_void());

    if (optional)
    {
        *optional = object.detach();
    }

    return hr;
}