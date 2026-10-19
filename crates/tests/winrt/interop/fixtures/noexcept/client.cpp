#include <winrt/Noexcept.h>
#include "check.h"

using namespace winrt;
using namespace Noexcept;

struct Value : implements<Value, IValue>
{
    int32_t m_number{};
    hstring m_text;
    IValue m_next;

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

    void Clear() noexcept
    {
        m_number = {};
        m_text = {};
        m_next = nullptr;
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

    // Clearing releases the widget's reference to itself.
    widget.Clear();
    check(widget.Number() == 0);
    check(widget.Text().empty());
    check(!widget.Next());

    widget.Check(make<Value>());
}
