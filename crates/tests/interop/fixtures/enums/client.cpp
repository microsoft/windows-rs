#include <winrt/Enums.h>
#include <winrt/Windows.Foundation.Collections.h>
#include "check.h"

using namespace winrt;
using namespace Enums;
using namespace Windows::Foundation;
using namespace Windows::Foundation::Collections;

void test()
{
    Widget widget;
    check(widget.Next(Color::Red) == Color::Green);
    check(widget.Next(Color::Blue) == Color::Red);
    check(widget.Next(Color::Unknown) == Color::Unknown);
    check(widget.Next(static_cast<Color>(7)) == Color::Unknown);

    check(widget.Styles() == (Styles::Bold | Styles::Underline));
    widget.Styles(Styles::Italic);
    check(widget.Styles() == Styles::Italic);

    std::vector<Color> colors;
    for (Color color : widget.Colors())
    {
        colors.push_back(color);
    }
    check((colors == std::vector<Color>{ Color::Red, Color::Green, Color::Blue }));
    auto styles = single_threaded_vector<Styles>({ Styles::Bold, Styles::Underline });
    check(widget.Combine(styles) == (Styles::Bold | Styles::Underline));

    check(unbox_value<Color>(widget.BoxColor(Color::Blue)) == Color::Blue);
    check(widget.UnboxStyles(box_value(Styles::Italic | Styles::Underline)) == (Styles::Italic | Styles::Underline));
}
