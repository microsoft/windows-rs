using System;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;

namespace GeneratedComBench;

// Source-generated COM only understands IUnknown inheritance. Declaring the three IInspectable
// members as a generated base interface gives derived WinRT interfaces their required slot-6 base.
[GeneratedComInterface]
[Guid("af86e2e0-b12d-4c6a-9c5a-d7aa65101e90")]
internal partial interface IInspectableAbi
{
    void GetIids(out uint count, out nint iids);
    nint GetRuntimeClassName();
    int GetTrustLevel();
}

[GeneratedComInterface]
[Guid("ad1e055d-7338-521c-a6f1-650e23a87d3c")]
internal partial interface IWidgetAbi : IInspectableAbi
{
    int GetInt32Property();
    void SetInt32Property(int value);

    [return: MarshalUsing(typeof(HStringMarshaller))]
    string GetStringProperty();

    void SetStringProperty([MarshalUsing(typeof(HStringMarshaller))] string value);

    nint GetObjectProperty();
    void SetObjectProperty(nint value);
    nint GetReferenceProperty();
    void SetReferenceProperty(nint value);
    nint Operation();
    nint StringOperation();
    nint ObjectOperation();
    int Add(int a, int b);
}

[CustomMarshaller(typeof(string), MarshalMode.Default, typeof(HStringMarshaller))]
internal static unsafe class HStringMarshaller
{
    // A returned nint cannot refer to a stack HSTRING_HEADER, so generated COM input strings need
    // an owned HSTRING rather than the direct projection's call-scoped string reference.
    public static nint ConvertToUnmanaged(string managed)
    {
        fixed (char* value = managed)
        {
            nint result;
            WindowsCsharp.Com.Check(
                HStringInterop.WindowsCreateString(value, (uint)managed.Length, &result));
            return result;
        }
    }

    public static string ConvertToManaged(nint unmanaged)
    {
        char* value = HStringInterop.WindowsGetStringRawBuffer(unmanaged, out uint length);
        return new string(value, 0, checked((int)length));
    }

    public static void Free(nint unmanaged)
    {
        if (unmanaged != 0)
        {
            _ = HStringInterop.WindowsDeleteString(unmanaged);
        }
    }
}

internal static unsafe partial class HStringInterop
{
    [LibraryImport("combase.dll")]
    public static partial int WindowsCreateString(char* source, uint length, nint* value);

    [LibraryImport("combase.dll")]
    public static partial char* WindowsGetStringRawBuffer(nint value, out uint length);

    [LibraryImport("combase.dll")]
    public static partial int WindowsDeleteString(nint value);
}

[GeneratedComInterface]
[Guid("ad1e055d-7338-521c-a6f1-650e23a87d3c")]
internal partial interface IWidgetPreserveSigAbi : IInspectableAbi
{
    [PreserveSig]
    int GetInt32Property(out int value);

    [PreserveSig]
    int SetInt32Property(int value);

    [PreserveSig]
    int GetStringProperty(out nint value);

    [PreserveSig]
    int SetStringProperty(nint value);

    [PreserveSig]
    int GetObjectProperty(out nint value);

    [PreserveSig]
    int SetObjectProperty(nint value);

    [PreserveSig]
    int GetReferenceProperty(out nint value);

    [PreserveSig]
    int SetReferenceProperty(nint value);

    [PreserveSig]
    int Operation(out nint value);

    [PreserveSig]
    int StringOperation(out nint value);

    [PreserveSig]
    int ObjectOperation(out nint value);

    [PreserveSig]
    int Add(int a, int b, out int value);
}

internal static class GeneratedWidget
{
    private static readonly ComWrappers s_wrappers = new StrategyBasedComWrappers();
    private static nint s_module;
    private static nint s_factory;

    public static IWidgetAbi Create()
    {
        return Create<IWidgetAbi>();
    }

    public static IWidgetPreserveSigAbi CreatePreserveSig()
    {
        return Create<IWidgetPreserveSigAbi>();
    }

    private static T Create<T>()
    {
        nint value = WindowsCsharp.WinRT.Activate(
            ref s_module,
            ref s_factory,
            "Bench.Widget",
            typeof(T).GUID);
        try
        {
            return (T)s_wrappers.GetOrCreateObjectForComInstance(
                value,
                CreateObjectFlags.UniqueInstance);
        }
        finally
        {
            _ = WindowsCsharp.Com.Release(value);
        }
    }
}
