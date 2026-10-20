#include <winrt/Primitives.h>
#include "check.h"

using namespace winrt;
using namespace Primitives;

void test()
{
    Widget widget;
    check(!widget.NotBool(true));
    check(widget.NotBool(false));
    check(widget.NextChar16(u'a') == u'b');
    check(widget.NextChar16(0xfffe) == 0xffff);
    check(widget.NextU8(0xfe) == 0xff);
    check(widget.NextI16(INT16_MIN) == INT16_MIN + 1);
    check(widget.NextU16(0xfffe) == 0xffff);
    check(widget.NextI32(INT32_MIN) == INT32_MIN + 1);
    check(widget.NextU32(0xffff'fffe) == 0xffff'ffff);
    check(widget.NextI64(INT64_MIN) == INT64_MIN + 1);
    check(widget.NextU64(UINT64_MAX - 1) == UINT64_MAX);
    check(widget.DoubleF32(1.5f) == 3.0f);
    check(widget.DoubleF64(1e300) == 2e300);

    guid const value{ 0x00112233, 0x4455, 0x6677, { 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff } };
    check(widget.FormatGuid(value) == L"00112233-4455-6677-8899-AABBCCDDEEFF");
    check(widget.ParseGuid(L"00112233-4455-6677-8899-AABBCCDDEEFF") == value);
}