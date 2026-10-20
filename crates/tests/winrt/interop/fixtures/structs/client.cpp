#include <winrt/Structs.h>
#include <winrt/Windows.Foundation.Collections.h>
#include "check.h"

using namespace winrt;
using namespace Structs;
using namespace Windows::Foundation::Collections;

void test()
{
    Widget widget;
    check(widget.Offset({ 1, 2 }, 3) == Point{ 4, 5 });

    Mixed mixed = widget.Step({ true, 0x1'0000'0005, 254, 1.5f });
    check(!mixed.Flag && mixed.Value == 0x1'0000'0006 && mixed.Code == 255 && mixed.Ratio == 2.5f);

    Label label = widget.Label();
    check(label.Text == L"label" && label.Position == Point{ 1, 2 } && label.Width == nullptr);
    widget.Label({ L"text", { 3, 4 }, Windows::Foundation::IReference<int32_t>{ 5 } });
    check(widget.Describe(widget.Label()) == L"text (3, 4) 5");
    check(widget.Describe({}) == L" (0, 0) none");

    std::vector<Point> points;
    for (Point const& point : widget.Points())
    {
        points.push_back(point);
    }
    check((points == std::vector<Point>{ { 1, 2 }, { 3, 4 } }));
    check(widget.Sum(single_threaded_vector<Point>({ { 1, 2 }, { 3, 4 } })) == Point{ 4, 6 });

    com_array<Label> labels = widget.Labels(2);
    check(labels.size() == 2);
    check(labels[1].Text == L"1" && labels[1].Position == Point{ 1, 1 } && labels[1].Width.Value() == 1);
    check(widget.Labels(0).empty());

    check(unbox_value<Point>(widget.BoxPoint({ 5, 6 })) == Point{ 5, 6 });
    check(widget.UnboxPoint(box_value(Point{ 7, 8 })) == Point{ 7, 8 });
}
