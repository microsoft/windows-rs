#include <winrt/StructDirections.h>
#include <winrt/Windows.Foundation.h>
#include "check.h"

using namespace winrt;
using namespace StructDirections;
using namespace Windows::Foundation;

Pair process_pair(Pair const& value, Pair const& borrowed, Pair& output)
{
    output = { value.X + borrowed.X, value.Y + borrowed.Y };
    return { value.X - borrowed.X, value.Y - borrowed.Y };
}

Label process_label(Label const& value, Label const& borrowed, Label& output)
{
    auto position = process_pair(value.Position, borrowed.Position, output.Position);
    output.Text = value.Text + borrowed.Text;
    output.Width = value.Width;
    return { borrowed.Text + value.Text, position, borrowed.Width };
}

void test()
{
    Widget widget;
    Pair const value{ 3, 0x1'0000'0000 };
    Pair const borrowed{ -5, 7 };
    Pair output;
    auto returned = widget.ProcessPair(value, borrowed, output);
    check(output == Pair{ -2, 0x1'0000'0007 });
    check(returned == Pair{ 8, 0xffff'fff9 });

    Label label_output, label_returned;
    {
        Label const left{ hstring{ std::wstring_view{ L"left\0tail", 9 } }, value, IReference<int32_t>{ 17 } };
        Label const right{ L"right", borrowed, nullptr };
        label_returned = widget.ProcessLabel(left, right, label_output);
    }
    check(label_output.Text == std::wstring_view{ L"left\0tailright", 14 });
    check(label_output.Position == Pair{ -2, 0x1'0000'0007 });
    check(label_output.Width.Value() == 17);
    check(label_returned.Text == std::wstring_view{ L"rightleft\0tail", 14 });
    check(label_returned.Position == Pair{ 8, 0xffff'fff9 });
    check(label_returned.Width == nullptr);

    widget.CheckPair(process_pair);
    widget.CheckLabel(process_label);
}
