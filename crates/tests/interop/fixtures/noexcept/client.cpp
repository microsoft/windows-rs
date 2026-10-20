#include <winrt/Noexcept.h>
#include "check.h"

using namespace winrt;
using namespace Noexcept;

struct Value : implements<Value, IValue>
{
    int32_t m_number{};
    hstring m_text;
    IValue m_next;
    int32_t m_limit{};

    int32_t Number() noexcept
    {
        return m_number;
    }

    void Number(int32_t value) noexcept
    {
        m_number = value;
    }

    hstring Text() noexcept
    {
        return m_text;
    }

    void Text(hstring const& value) noexcept
    {
        m_text = value;
    }

    IValue Next() noexcept
    {
        return m_next;
    }

    void Next(IValue const& value) noexcept
    {
        m_next = value;
    }

    int32_t Limit() noexcept
    {
        return m_limit;
    }

    void Limit(int32_t value)
    {
        if (value < 0)
        {
            throw hresult_invalid_argument();
        }

        m_limit = value;
    }

    void Clear() noexcept
    {
        m_number = {};
        m_text = {};
        m_next = nullptr;
        m_limit = {};
    }
};

void test()
{
    Widget widget;

    check(widget.Number() == 0);
    widget.Number(123);
    check(widget.Number() == 123);

    check(widget.Text().empty());
    widget.Text(L"abc");
    check(widget.Text() == L"abc");

    check(!widget.Next());
    widget.Next(widget);
    check(widget.Next() == widget);

    check(widget.Limit() == 0);
    widget.Limit(5);
    check(widget.Limit() == 5);

    try
    {
        widget.Limit(-1);
        check(false);
    }
    catch (hresult_invalid_argument const&)
    {
    }

    check(widget.Limit() == 5);

    // Clearing releases the widget's reference to itself.
    widget.Clear();
    check(widget.Number() == 0);
    check(widget.Text().empty());
    check(!widget.Next());
    check(widget.Limit() == 0);

    widget.Check(make<Value>());
}
