using System;
using Primitives;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(!widget.NotBool(true));
        Check(widget.NotBool(false));
        Check(widget.NextChar16('a') == 'b');
        Check(widget.NextChar16('\ufffe') == '\uffff');
        Check(widget.NextU8(0xfe) == 0xff);
        Check(widget.NextI16(short.MinValue) == short.MinValue + 1);
        Check(widget.NextU16(0xfffe) == 0xffff);
        Check(widget.NextI32(int.MinValue) == int.MinValue + 1);
        Check(widget.NextU32(0xffff_fffe) == 0xffff_ffff);
        Check(widget.NextI64(long.MinValue) == long.MinValue + 1);
        Check(widget.NextU64(ulong.MaxValue - 1) == ulong.MaxValue);
        Check(widget.DoubleF32(1.5f) == 3.0f);
        Check(widget.DoubleF64(1e300) == 2e300);

        var guid = new Guid("00112233-4455-6677-8899-aabbccddeeff");
        Check(widget.FormatGuid(guid) == "00112233-4455-6677-8899-AABBCCDDEEFF");
        Check(widget.ParseGuid("00112233-4455-6677-8899-AABBCCDDEEFF") == guid);
    }
}