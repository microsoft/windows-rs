#include <winrt/Numerics.h>
#include <winrt/Windows.Foundation.Numerics.h>
#include "check.h"

using namespace winrt;
using namespace Numerics;
using namespace Windows::Foundation::Numerics;

void test()
{
    Widget widget;
    check(widget.NegateVector2({ 1.25f, -2.5f }) == float2{ -1.25f, 2.5f });
    check(widget.NegateVector3({ 1.25f, -2.5f, 3.75f }) == float3{ -1.25f, 2.5f, -3.75f });
    check(widget.NegateVector4({ 1.25f, -2.5f, 3.75f, -4.5f }) == float4{ -1.25f, 2.5f, -3.75f, 4.5f });
    check(widget.NegateMatrix3x2({ 1, -2, 3, -4, 5, -6 }) == float3x2{ -1, 2, -3, 4, -5, 6 });
    check(widget.NegateMatrix4x4({
        1, -2, 3, -4,
        5, -6, 7, -8,
        9, -10, 11, -12,
        13, -14, 15, -16
    }) == float4x4{
        -1, 2, -3, 4,
        -5, 6, -7, 8,
        -9, 10, -11, 12,
        -13, 14, -15, 16
    });
    check(widget.NegateQuaternion({ 1.25f, -2.5f, 3.75f, -4.5f }) == quaternion{ -1.25f, 2.5f, -3.75f, 4.5f });
    check(widget.NegatePlane({ { 1.25f, -2.5f, 3.75f }, -4.5f }) == plane{ { -1.25f, 2.5f, -3.75f }, 4.5f });
}
