//! Tests for the `windows-csharp` C# projection generator.
//!
//! Golden tests: `build.rs` emits one `#[test]` per `input/*.rdl` fixture, each calling
//! `golden(name)`, which authors a winmd from the fixture and writes the projection fragment to
//! `expected/{name}.cs`. Like the windows-bindgen harness, the golden is rewritten on every run, so
//! CI's `gen`/`test` workflows fail if a checked-in golden is stale.
//!
//! `compile_goldens` generates one combined projection over every fixture (single header, single
//! runtime support, all namespaces) and compiles it with `dotnet`, proving the emitted C# builds.
//!
//! `round_trip` generates the full `Bench` projection from the live `test_bench_component` winmd,
//! compiles it with a small harness, and runs it against that same WinRT component (staged as
//! `Bench.dll`), exercising a scalar property, a string property, and a method end to end.
//!
//! The `dotnet` tests skip cleanly when the SDK is not installed.

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

use std::path::{Path, PathBuf};

/// The bindgen default winmds, used to resolve the `Windows.Foundation.Metadata` attributes the
/// fixtures reference (`Activatable`, `ExclusiveTo`, ...).
const REFERENCE: &str = "../../../libs/bindgen/default";

/// The system WinRT metadata that defines the projected generic collections (`IVector`1` and its
/// PIID), needed to compute the parameterized IIDs of any `IVector<...>` a fixture names. Callers
/// of `windows-csharp` that use WinRT generics supply this the same way windows-rs consumes the
/// WinRT metadata.
const FOUNDATION: &str = r"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd";

/// The pinned Windows App SDK metadata used by windows-reactor. The direct WinUI projection test
/// consumes this same input so the C# slice cannot drift onto a separate metadata pipeline.
const WINUI: &str = "../../../tools/reactor/winmd";

/// Reads the `--filter` namespaces declared in a fixture's leading `//!` argument lines.
fn filters(rdl: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(rdl).unwrap();
    let mut filters = Vec::new();
    for line in contents.lines() {
        let Some(rest) = line.strip_prefix("//!") else {
            break;
        };
        let mut args = rest.split_whitespace();
        while let Some(arg) = args.next() {
            if arg == "--filter"
                && let Some(value) = args.next()
            {
                filters.push(value.to_string());
            }
        }
    }
    filters
}

/// Authors `input/{name}.rdl` (plus the reference winmds) into `{scratch}/{name}.winmd`.
fn author(name: &str, scratch: &Path) -> PathBuf {
    let winmd = scratch.join(format!("{name}.winmd"));
    windows_rdl::reader()
        .input(format!("input/{name}.rdl").as_str())
        .input(REFERENCE)
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap();
    winmd
}

/// Returns a fresh scratch directory under `OUT_DIR`.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("OUT_DIR")).join(name);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Authors the fixture, generates the projection fragment, and writes the self-updating golden.
fn golden(name: &str) {
    let scratch = scratch(name);
    let winmd = author(name, &scratch);

    let generated = scratch.join(format!("{name}.cs"));
    let mut builder = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .output(generated.to_str().unwrap())
        .fragment();
    for filter in filters(&format!("input/{name}.rdl")) {
        builder = builder.filter(filter);
    }
    builder.write().unwrap();

    let actual = std::fs::read_to_string(&generated).unwrap();
    std::fs::write(format!("expected/{name}.cs"), actual).unwrap();
}

/// Enumerates the fixture names (`input/*.rdl` stems), sorted.
fn fixtures() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir("input")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "rdl"))
        .map(|p| p.file_stem().unwrap().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn compile_goldens() {
    compile_goldens_with("compile", false);
}

#[test]
fn compile_synchronized_goldens() {
    compile_goldens_with("compile_synchronized", true);
}

#[test]
fn async_operation_round_trip() {
    async_operation_round_trip_with("async_operation_raw", false);
    async_operation_round_trip_with("async_operation_synchronized", true);
}

fn async_operation_round_trip_with(name: &str, synchronized: bool) {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch(name);
    let winmd = author("breadth", &scratch);
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    let mut builder = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .filter("Breadth")
        .output(project.join("Generated.cs").to_str().unwrap());
    if synchronized {
        builder = builder.synchronized();
    }
    builder.write().unwrap();

    let source = std::fs::read_to_string(project.join("Generated.cs")).unwrap();
    for absent in [
        "ThreadPool.QueueUserWorkItem",
        "Thread.Yield()",
        "while (_operation.Status()",
    ] {
        assert!(
            !source.contains(absent),
            "generated async awaiter retained polling `{absent}`"
        );
    }
    for expected in [
        "(*(void***)self)[6])(self, handler)",
        "private static int CompletedInvoke(nint self, nint operation, int status)",
        "public T GetResult() => _operation.GetResults();",
        "new Guid(0xd60cae9d, 0x88cb, 0x59f1, 0x85, 0x76, 0x3f, 0xba, 0x44, 0x79, 0x6b, 0xe8)",
    ] {
        assert!(
            source.contains(expected),
            "generated async awaiter omitted `{expected}`"
        );
    }

    std::fs::write(project.join("project.csproj"), EXE_CSPROJ).unwrap();
    std::fs::write(project.join("Program.cs"), ASYNC_PROGRAM).unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("run")
        .args(["--project", project.to_str().unwrap()])
        .output()
        .expect("failed to run synthetic async operation");

    assert!(
        output.status.success(),
        "synthetic {name} async operation failed\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn delegate_marshalling_round_trip() {
    delegate_marshalling_round_trip_with("delegate_marshalling_raw", false);
    delegate_marshalling_round_trip_with("delegate_marshalling_synchronized", true);
}

fn delegate_marshalling_round_trip_with(name: &str, synchronized: bool) {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch(name);
    let winmd = author("delegate_marshalling", &scratch);
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    let mut builder = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .filter("DelegateMarshalling")
        .output(project.join("Generated.cs").to_str().unwrap());
    if synchronized {
        builder = builder.synchronized();
    }
    builder.write().unwrap();

    let source = std::fs::read_to_string(project.join("Generated.cs")).unwrap();
    for expected in [
        "delegate* unmanaged<nint, nint, nint*, int>",
        "FromHstringBorrowed(value)",
        "Interop.CreateString(callback(",
        "new DelegateMarshalling.IDelegatePeer.Borrowed(value)",
        "WindowsCsharp.Com.AddRef(resultValue)",
        "WindowsCsharp.Interop.DeleteHstring(ref ownedResult)",
        "public bool IsNull => _this == 0;",
    ] {
        assert!(
            source.contains(expected),
            "generated delegate marshalling omitted `{expected}`"
        );
    }

    std::fs::write(project.join("project.csproj"), EXE_CSPROJ).unwrap();
    std::fs::write(project.join("Program.cs"), DELEGATE_MARSHALLING_PROGRAM).unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("run")
        .args(["--project", project.to_str().unwrap()])
        .output()
        .expect("failed to run synthetic delegate marshalling");

    assert!(
        output.status.success(),
        "synthetic {name} delegate marshalling failed\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    eprint!("{}", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn array_round_trip() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch("array_boolean_enum_round_trip");
    let winmd = author("arrays", &scratch);
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .filter("Sample")
        .output(project.join("Generated.cs").to_str().unwrap())
        .synchronized()
        .write()
        .unwrap();
    std::fs::write(project.join("project.csproj"), EXE_CSPROJ).unwrap();
    std::fs::write(
        project.join("Program.cs"),
        r#"using System;
using System.Runtime.InteropServices;
using Sample;

internal static unsafe class Program
{
    private static nint* s_peerVtable;
    private static Instance* s_returnPeer0;
    private static Instance* s_returnPeer2;
    private static bool s_failPeers;

    [StructLayout(LayoutKind.Sequential)]
    private struct Instance
    {
        public nint Vtable;
        public int References;
        public int Id;
    }

    private static void Main()
    {
        nint* vtable = stackalloc nint[23];
        new Span<nint>(vtable, 23).Clear();
        vtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&QueryInterface;
        vtable[1] = (nint)(delegate* unmanaged<nint, uint>)&AddRef;
        vtable[2] = (nint)(delegate* unmanaged<nint, uint>)&Release;
        vtable[9] = (nint)(delegate* unmanaged<nint, uint, byte*, uint*, int>)&CountTrue;
        vtable[10] = (nint)(delegate* unmanaged<nint, uint*, byte**, int>)&Booleans;
        vtable[11] = (nint)(delegate* unmanaged<nint, uint*, byte**, int>)&GetBooleans;
        vtable[12] = (nint)(delegate* unmanaged<nint, uint, int*, int*, int>)&SumModes;
        vtable[13] = (nint)(delegate* unmanaged<nint, uint*, int**, int>)&Modes;
        vtable[14] = (nint)(delegate* unmanaged<nint, uint*, int**, int>)&GetModes;
        vtable[15] = (nint)(delegate* unmanaged<nint, StateAbi, byte*, int>)&InspectState;
        vtable[16] = (nint)(delegate* unmanaged<nint, StateAbi*, int>)&CurrentState;
        vtable[17] = (nint)(delegate* unmanaged<nint, uint, nint*, uint*, int>)&CountStringUnits;
        vtable[18] = (nint)(delegate* unmanaged<nint, uint*, nint**, int>)&Strings;
        vtable[19] = (nint)(delegate* unmanaged<nint, uint*, nint**, int>)&GetStrings;
        vtable[20] = (nint)(delegate* unmanaged<nint, uint, nint*, uint*, int>)&CountPeers;
        vtable[21] = (nint)(delegate* unmanaged<nint, uint*, nint**, int>)&Peers;
        vtable[22] = (nint)(delegate* unmanaged<nint, uint*, nint**, int>)&GetPeers;

        s_peerVtable = (nint*)NativeMemory.Alloc(7, (nuint)sizeof(nint));
        new Span<nint>(s_peerVtable, 7).Clear();
        s_peerVtable[0] = vtable[0];
        s_peerVtable[1] = vtable[1];
        s_peerVtable[2] = vtable[2];

        Instance* arraysInstance = CreateInstance(vtable, 0);
        Instance* peer0 = CreateInstance(s_peerVtable, 1);
        Instance* peer2 = CreateInstance(s_peerVtable, 2);
        Arrays arrays = new((nint)arraysInstance);
        IArrayPeer first = new((nint)peer0);
        IArrayPeer second = new((nint)peer2);
        bool[] booleans = [true, false, true];
        Mode[] modes = [Mode.One, Mode.None, Mode.One];
        string[] strings = ["one", null!, "two"];
        IArrayPeer?[] peers = [first, null, second];
        if (arrays.CountTrue(booleans) != 2 || arrays.SumModes(modes) != 2)
        {
            throw new InvalidOperationException("array input conversion failed");
        }
        if (arrays.CountStringUnits(strings) != 6 || arrays.CountPeers(peers) != 3 ||
            peer0->References != 1 || peer2->References != 1)
        {
            throw new InvalidOperationException("string/object array input conversion failed");
        }
        State state = new() { enabled = true, mode = Mode.One };
        if (sizeof(StateAbi) != 8 || Marshal.OffsetOf<StateAbi>(nameof(StateAbi.mode)).ToInt32() != 4 ||
            !arrays.InspectState(state))
        {
            throw new InvalidOperationException("Boolean struct input layout failed");
        }
        State returnedState = arrays.CurrentState();
        if (!returnedState.enabled || returnedState.mode != Mode.One)
        {
            throw new InvalidOperationException("Boolean struct output layout failed");
        }

        bool[] returnedBooleans = arrays.Booleans();
        arrays.GetBooleans(out bool[] outputBooleans);
        Mode[] returnedModes = arrays.Modes();
        arrays.GetModes(out Mode[] outputModes);
        string[] returnedStrings = arrays.Strings();
        arrays.GetStrings(out string[] outputStrings);
        if (!returnedBooleans[0] || returnedBooleans[1] || !outputBooleans[2] ||
            returnedModes[0] != Mode.One || returnedModes[1] != Mode.None ||
            outputModes[2] != Mode.One || returnedStrings[0] != "red" ||
            returnedStrings[1] != string.Empty || returnedStrings[2] != "blue" ||
            outputStrings[0] != "red" || outputStrings[1] != string.Empty ||
            outputStrings[2] != "blue")
        {
            throw new InvalidOperationException("array output conversion failed");
        }
        IArrayPeer?[] returnedPeers = arrays.Peers();
        AssertPeerArray(returnedPeers);
        DisposePeers(returnedPeers);
        AssertReturnedPeersReleased();
        arrays.GetPeers(out IArrayPeer?[] outputPeers);
        AssertPeerArray(outputPeers);
        DisposePeers(outputPeers);
        AssertReturnedPeersReleased();

        s_failPeers = true;
        try
        {
            _ = arrays.Peers();
            throw new InvalidOperationException("failed array call should have thrown");
        }
        catch (COMException)
        {
        }
        finally
        {
            s_failPeers = false;
        }
        AssertReturnedPeersReleased();

        TestObjectArrayExceptionCleanup();

        _ = arrays.CountTrue(booleans);
        _ = arrays.SumModes(modes);
        _ = arrays.CountStringUnits(strings);
        _ = arrays.CountPeers(peers);
        _ = arrays.InspectState(state);
        _ = arrays.CurrentState();
        long allocatedBefore = GC.GetAllocatedBytesForCurrentThread();
        for (int i = 0; i < 1_000; i++)
        {
            _ = arrays.CountTrue(booleans);
            _ = arrays.SumModes(modes);
            _ = arrays.CountStringUnits(strings);
            _ = arrays.CountPeers(peers);
            _ = arrays.InspectState(state);
            _ = arrays.CurrentState();
        }
        long allocated = GC.GetAllocatedBytesForCurrentThread() - allocatedBefore;
        if (allocated != 0)
        {
            throw new InvalidOperationException($"array input calls allocated {allocated} bytes");
        }

        first.Dispose();
        second.Dispose();
        arrays.Dispose();
        if (peer0->References != 0 || peer2->References != 0 || arraysInstance->References != 0)
        {
            throw new InvalidOperationException("projected object reference balance failed");
        }
        NativeMemory.Free(peer0);
        NativeMemory.Free(peer2);
        NativeMemory.Free(arraysInstance);
        NativeMemory.Free(s_peerVtable);
    }

    [UnmanagedCallersOnly]
    private static int QueryInterface(nint self, Guid* iid, nint* result)
    {
        _ = iid;
        _ = AddRefCore(self);
        *result = self;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static uint AddRef(nint self)
    {
        return AddRefCore(self);
    }

    [UnmanagedCallersOnly]
    private static uint Release(nint self)
    {
        return (uint)--((Instance*)self)->References;
    }

    private static uint AddRefCore(nint self) => (uint)++((Instance*)self)->References;

    [UnmanagedCallersOnly]
    private static int CountTrue(nint self, uint length, byte* values, uint* result)
    {
        _ = self;
        uint count = 0;
        for (uint i = 0; i < length; i++)
        {
            count += values[i] != 0 ? 1u : 0u;
        }
        *result = count;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int SumModes(nint self, uint length, int* values, int* result)
    {
        _ = self;
        int sum = 0;
        for (uint i = 0; i < length; i++)
        {
            sum += values[i];
        }
        *result = sum;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int Booleans(nint self, uint* length, byte** values) =>
        ReturnBooleans(self, length, values);

    [UnmanagedCallersOnly]
    private static int GetBooleans(nint self, uint* length, byte** values) =>
        ReturnBooleans(self, length, values);

    private static int ReturnBooleans(nint self, uint* length, byte** values)
    {
        _ = self;
        *length = 3;
        *values = (byte*)Marshal.AllocCoTaskMem(3);
        (*values)[0] = 1;
        (*values)[1] = 0;
        (*values)[2] = 1;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int Modes(nint self, uint* length, int** values) =>
        ReturnModes(self, length, values);

    [UnmanagedCallersOnly]
    private static int GetModes(nint self, uint* length, int** values) =>
        ReturnModes(self, length, values);

    private static int ReturnModes(nint self, uint* length, int** values)
    {
        _ = self;
        *length = 3;
        *values = (int*)Marshal.AllocCoTaskMem(3 * sizeof(int));
        (*values)[0] = 1;
        (*values)[1] = 0;
        (*values)[2] = 1;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int InspectState(nint self, StateAbi value, byte* result)
    {
        _ = self;
        *result = value.enabled != 0 && value.mode == (int)Mode.One ? (byte)1 : (byte)0;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int CurrentState(nint self, StateAbi* result)
    {
        _ = self;
        *result = new StateAbi { enabled = 1, mode = (int)Mode.One };
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int CountStringUnits(nint self, uint length, nint* values, uint* result)
    {
        _ = self;
        uint count = 0;
        for (uint i = 0; i < length; i++)
        {
            _ = WindowsCsharp.Interop.WindowsGetStringRawBuffer(values[i], out uint current);
            count += current;
        }
        *result = count;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int Strings(nint self, uint* length, nint** values) =>
        ReturnStrings(self, length, values);

    [UnmanagedCallersOnly]
    private static int GetStrings(nint self, uint* length, nint** values) =>
        ReturnStrings(self, length, values);

    private static int ReturnStrings(nint self, uint* length, nint** values)
    {
        _ = self;
        *length = 3;
        *values = (nint*)Marshal.AllocCoTaskMem(3 * sizeof(nint));
        (*values)[0] = WindowsCsharp.Interop.CreateString("red");
        (*values)[1] = 0;
        (*values)[2] = WindowsCsharp.Interop.CreateString("blue");
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int CountPeers(nint self, uint length, nint* values, uint* result)
    {
        _ = self;
        uint count = 0;
        for (uint i = 0; i < length; i++)
        {
            if (values[i] != 0)
            {
                Instance* peer = (Instance*)values[i];
                if (peer->References != 2)
                {
                    return unchecked((int)0x80004005);
                }
                count += (uint)peer->Id;
            }
        }
        *result = count;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int Peers(nint self, uint* length, nint** values) =>
        ReturnPeers(self, length, values);

    [UnmanagedCallersOnly]
    private static int GetPeers(nint self, uint* length, nint** values) =>
        ReturnPeers(self, length, values);

    private static int ReturnPeers(nint self, uint* length, nint** values)
    {
        _ = self;
        *length = 3;
        *values = (nint*)Marshal.AllocCoTaskMem(3 * sizeof(nint));
        s_returnPeer0 = CreateInstance(s_peerVtable, 10);
        s_returnPeer2 = CreateInstance(s_peerVtable, 20);
        (*values)[0] = (nint)s_returnPeer0;
        (*values)[1] = 0;
        (*values)[2] = (nint)s_returnPeer2;
        return s_failPeers ? unchecked((int)0x80004005) : 0;
    }

    private static Instance* CreateInstance(nint* vtable, int id)
    {
        Instance* result = (Instance*)NativeMemory.Alloc((nuint)sizeof(Instance));
        result->Vtable = (nint)vtable;
        result->References = 1;
        result->Id = id;
        return result;
    }

    private static void AssertPeerArray(IArrayPeer?[] values)
    {
        if (values.Length != 3 || values[0] is null || values[1] is not null || values[2] is null)
        {
            throw new InvalidOperationException("projected object array output failed");
        }
    }

    private static void DisposePeers(IArrayPeer?[] values)
    {
        values[0]?.Dispose();
        values[2]?.Dispose();
    }

    private static void AssertReturnedPeersReleased()
    {
        if (s_returnPeer0->References != 0 || s_returnPeer2->References != 0)
        {
            throw new InvalidOperationException("returned object references were not consumed");
        }
        NativeMemory.Free(s_returnPeer0);
        NativeMemory.Free(s_returnPeer2);
        s_returnPeer0 = null;
        s_returnPeer2 = null;
    }

    private static void TestObjectArrayExceptionCleanup()
    {
        Instance* first = CreateInstance(s_peerVtable, 30);
        Instance* second = CreateInstance(s_peerVtable, 31);
        Instance* third = CreateInstance(s_peerVtable, 32);
        uint length = 3;
        nint* values = (nint*)Marshal.AllocCoTaskMem(3 * sizeof(nint));
        values[0] = (nint)first;
        values[1] = (nint)second;
        values[2] = (nint)third;
        try
        {
            _ = WindowsCsharp.Interop.FromObjectArray<ThrowingObject>(ref length, ref values);
            throw new InvalidOperationException("conversion should have failed");
        }
        catch (InvalidOperationException error) when (error.Message == "expected conversion failure")
        {
        }
        if (length != 0 || values != null || first->References != 0 ||
            second->References != 0 || third->References != 0)
        {
            throw new InvalidOperationException("object array exception cleanup failed");
        }
        NativeMemory.Free(first);
        NativeMemory.Free(second);
        NativeMemory.Free(third);
    }

    private sealed class ThrowingObject : WindowsCsharp.ComObject,
        WindowsCsharp.IComInterface<ThrowingObject>
    {
        public static Guid Iid { get; } = IArrayPeer.Iid;

        private ThrowingObject(nint self) : base(self, Iid) {}
        private ThrowingObject(nint self, bool trustedAgile) : base(self, trustedAgile) {}

        static ThrowingObject WindowsCsharp.IComInterface<ThrowingObject>.FromAbi(nint self)
        {
            if (((Instance*)self)->Id == 31)
            {
                _ = WindowsCsharp.Com.Release(self);
                throw new InvalidOperationException("expected conversion failure");
            }
            return new ThrowingObject(self);
        }

        static ThrowingObject WindowsCsharp.IComInterface<ThrowingObject>.FromAgileAbi(nint self) =>
            new ThrowingObject(self, true);
    }
}
"#,
    )
    .unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("run")
        .args(["--project", project.to_str().unwrap()])
        .output()
        .expect("failed to run array projection");
    assert!(
        output.status.success(),
        "array projection failed\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn string_struct_round_trip() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch("string_struct_round_trip");
    let winmd = author("structs", &scratch);
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .filter("Sample")
        .output(project.join("Generated.cs").to_str().unwrap())
        .write()
        .unwrap();
    let source = std::fs::read_to_string(project.join("Generated.cs")).unwrap();
    for expected in [
        "public string Title;",
        "public nint Title;",
        "result.Title = WindowsCsharp.Interop.CreateString(value.Title);",
        "result.Detail = Sample.TextAbi.FromSurface(value.Detail);",
        "result.Title = WindowsCsharp.Interop.TakeHstring(ref Title);",
        "result.Detail = Detail.ToSurface();",
        "WindowsCsharp.Interop.DeleteHstring(ref Title);",
        "Sample.DescriptionAbi _abi0 = default;",
        "_abi0 = Sample.DescriptionAbi.FromSurface(value);",
        "_abi0.Dispose();",
        "Sample.DescriptionAbi result = default;",
        "return result.ToSurface();",
        "result.Dispose();",
    ] {
        assert!(
            source.contains(expected),
            "string-struct projection omitted `{expected}`"
        );
    }

    std::fs::write(project.join("project.csproj"), EXE_CSPROJ).unwrap();
    std::fs::write(
        project.join("Program.cs"),
        r#"using System;
using System.Runtime.InteropServices;
using Sample;

internal static unsafe class Program
{
    [StructLayout(LayoutKind.Sequential)]
    private struct Instance
    {
        public nint Vtable;
        public int References;
    }

    private static void Main()
    {
        nint* vtable = stackalloc nint[14];
        new Span<nint>(vtable, 14).Clear();
        vtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&QueryInterface;
        vtable[1] = (nint)(delegate* unmanaged<nint, uint>)&AddRef;
        vtable[2] = (nint)(delegate* unmanaged<nint, uint>)&Release;
        vtable[8] = (nint)(delegate* unmanaged<nint, DescriptionAbi*, int>)&GetCaption;
        vtable[9] = (nint)(delegate* unmanaged<nint, DescriptionAbi, int>)&SetCaption;
        vtable[11] = (nint)(delegate* unmanaged<nint, DescriptionAbi, byte*, int>)&Inspect;
        vtable[12] = (nint)(delegate* unmanaged<nint, DescriptionAbi*, int>)&Current;
        vtable[13] = (nint)(delegate* unmanaged<nint, DescriptionAbi*, int>)&Failing;

        Instance* instance = (Instance*)NativeMemory.AllocZeroed((nuint)sizeof(Instance));
        instance->Vtable = (nint)vtable;
        instance->References = 1;
        Shape shape = new((nint)instance);
        Description input = new()
        {
            Title = "input",
            Detail = new Text { Value = "nested" },
            Visible = true,
        };

        if (!shape.InspectDescription(input))
        {
            throw new InvalidOperationException("string struct input failed");
        }
        shape.Caption = input;
        AssertDescription(shape.Caption, "caption", "property", false);
        AssertDescription(shape.CurrentDescription(), "return", "nested return", true);

        try
        {
            _ = shape.FailingDescription();
            throw new InvalidOperationException("failed struct call should have thrown");
        }
        catch (COMException)
        {
        }

        shape.Dispose();
        if (instance->References != 0)
        {
            throw new InvalidOperationException("string struct owner reference balance failed");
        }
        NativeMemory.Free(instance);
        Console.WriteLine("windows-csharp string struct OK");
    }

    private static void AssertDescription(
        Description value,
        string title,
        string detail,
        bool visible)
    {
        if (value.Title != title || value.Detail.Value != detail || value.Visible != visible)
        {
            throw new InvalidOperationException("string struct output failed");
        }
    }

    private static DescriptionAbi Create(string title, string detail, bool visible) =>
        DescriptionAbi.FromSurface(new Description
        {
            Title = title,
            Detail = new Text { Value = detail },
            Visible = visible,
        });

    [UnmanagedCallersOnly]
    private static int QueryInterface(nint self, Guid* iid, nint* result)
    {
        _ = iid;
        _ = AddRefCore(self);
        *result = self;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static uint AddRef(nint self) => AddRefCore(self);

    private static uint AddRefCore(nint self) => (uint)++((Instance*)self)->References;

    [UnmanagedCallersOnly]
    private static uint Release(nint self) => (uint)--((Instance*)self)->References;

    [UnmanagedCallersOnly]
    private static int GetCaption(nint self, DescriptionAbi* result)
    {
        _ = self;
        *result = Create("caption", "property", false);
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int SetCaption(nint self, DescriptionAbi value)
    {
        _ = self;
        return value.FromAbi() is { Title: "input", Detail: { Value: "nested" }, Visible: true }
            ? 0
            : unchecked((int)0x80004005);
    }

    [UnmanagedCallersOnly]
    private static int Inspect(nint self, DescriptionAbi value, byte* result)
    {
        _ = self;
        Description surface = value.FromAbi();
        *result = surface is { Title: "input", Detail: { Value: "nested" }, Visible: true }
            ? (byte)1
            : (byte)0;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int Current(nint self, DescriptionAbi* result)
    {
        _ = self;
        *result = Create("return", "nested return", true);
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int Failing(nint self, DescriptionAbi* result)
    {
        _ = self;
        *result = Create("failure", "cleanup", true);
        return unchecked((int)0x80004005);
    }
}
"#,
    )
    .unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("run")
        .args(["--project", project.to_str().unwrap()])
        .output()
        .expect("failed to run string struct projection");
    assert!(
        output.status.success(),
        "generated string struct projection failed to run\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("windows-csharp string struct OK"),
        "unexpected string struct projection output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

fn compile_goldens_with(name: &str, synchronized: bool) {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch(name);

    // Author every fixture and feed all winmds (and all filters) into a single generation so the
    // combined output has one header and one copy of the runtime support.
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    let mut builder = windows_csharp::builder();
    for name in fixtures() {
        let winmd = author(&name, &scratch);
        builder = builder.input(winmd.to_str().unwrap());
        for filter in filters(&format!("input/{name}.rdl")) {
            builder = builder.filter(filter);
        }
    }
    let mut builder = builder
        .input(FOUNDATION)
        .output(project.join("Generated.cs").to_str().unwrap());
    if synchronized {
        builder = builder.synchronized();
    }
    builder.write().unwrap();
    std::fs::write(project.join("project.csproj"), LIBRARY_CSPROJ).unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("build")
        .arg(project.to_str().unwrap())
        .output()
        .expect("failed to run dotnet build");

    assert!(
        output.status.success(),
        "combined {name} projection failed to compile\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn default_owner_policy() {
    let scratch = scratch("default_owner_policy");
    let winmd = author("thin", &scratch);
    let output = scratch.join("Generated.cs");
    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .filter("Thin")
        .output(output.to_str().unwrap())
        .write()
        .unwrap();

    let source = std::fs::read_to_string(output).unwrap();
    for expected in [
        "private nint _this;",
        "public void Dispose()",
        "return new ComLease(self);",
        "internal bool TrustedAgile => Handle != 0;",
        "private static readonly Guid s_iagile",
        "return new FactoryLease(requested, requested);",
    ] {
        assert!(
            source.contains(expected),
            "default runtime omitted `{expected}`:\n{source}"
        );
    }
    for absent in [
        "ApartmentReference",
        "private const int Disposed",
        "~ComObject()",
        "ReleaseLease",
    ] {
        assert!(
            !source.contains(absent),
            "default runtime retained `{absent}`:\n{source}"
        );
    }
}

#[test]
fn runtime_support_policies() {
    let raw = windows_csharp::runtime_support();
    assert!(raw.contains("private nint _this;"));
    assert!(!raw.contains("ApartmentReference"));
    assert!(!raw.contains("~ComObject()"));

    let synchronized = windows_csharp::synchronized_runtime_support();
    assert!(synchronized.contains("ApartmentReference"));
    assert!(synchronized.contains("~ComObject()"));
    assert!(synchronized.contains("ReleaseLease"));
    assert!(synchronized.contains("RevokeInContext"));
}

#[test]
fn win32_round_trip() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch("win32_roundtrip");
    let winmd = author("win32_foundation", &scratch);
    let project = scratch.join("project");
    let _ = std::fs::remove_dir_all(&project);
    std::fs::create_dir_all(&project).unwrap();

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .filter("Win32Test")
        .output(project.join("Generated.cs").to_str().unwrap())
        .write()
        .unwrap();
    let source = std::fs::read_to_string(project.join("Generated.cs")).unwrap();
    for expected in [
        "private static partial int CreateStreamOnHGlobalAbi(nint global, int delete_on_release, nint* stream);",
        "public static Win32Test.IStream CreateStreamOnHGlobal(nint global, bool delete_on_release)",
        "CreateStreamOnHGlobalAbi(global, (delete_on_release ? 1 : 0), &stream)",
        "private static partial int IsReadyAbi();",
        "public static bool IsReady()",
        "return result != 0;",
        "public unsafe struct NativeState",
        "public int ready;",
        "public byte* data;",
        "public static partial uint InspectState(Win32Test.NativeState value);",
        "public void Read(Span<byte> value, out uint read)",
        "(*(void***)self)[3])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2)",
        "public void Write(ReadOnlySpan<byte> value, out uint written)",
        "(*(void***)self)[4])(self, (byte*)_abi0, checked((uint)value.Length), &_abi2)",
        "public void Seek(long offset, uint origin, out ulong position)",
        "(*(void***)self)[6])(self, size)",
        "[System.Runtime.InteropServices.Marshalling.GeneratedComInterface]",
        "public unsafe partial interface IStreamAbi : Win32Test.ISequentialStreamAbi",
        "[PreserveSig]\n        int Read(byte* value, uint count, uint* read);",
        // `out`/`ref` pointer sugar: an out scalar, an in/out scalar, an out struct, and an in/out
        // struct, each combined with the existing `BOOL`/`HRESULT` wrapper.
        "private static partial int QueryPerformanceCounterAbi(long* value);",
        "public static bool QueryPerformanceCounter(out long value)",
        "private static partial int GetCountAbi(uint* value);",
        "public static bool GetCount(out uint value)",
        "private static partial int AdjustCountAbi(uint* value);",
        "public static bool AdjustCount(ref uint value)",
        "uint _abi0 = value;",
        "private static partial int GetPointAbi(Win32Test.Point* value);",
        "public static void GetPoint(out Win32Test.Point value)",
        "private static partial int AdjustPointAbi(Win32Test.Point* value);",
        "public static void AdjustPoint(ref Win32Test.Point value)",
        "private static partial uint GetModeAbi(int* value);",
        "public static uint GetMode(out Win32Test.Mode value)",
        "value = (Win32Test.Mode)_abi0;",
        // Deferred: an optional `[out]` pointer and a mutable pointer with an ambiguous (`[in]`
        // only) direction both keep their raw ABI pointer on the public surface.
        "public static bool TryGetOptional(uint* value)",
        "public static bool PeekValue(uint* value)",
        // A genuine Win32 opaque handle (see `native_handle_value`): an explicit blittable
        // `readonly struct` wrapping a single `nint` field, equatable, default/null capable (no
        // hand-written `Default` needed), exposing its raw value with no `unsafe` pointer field
        // and no `Close`/`Dispose`/invalid-handle convenience.
        "public readonly struct HWND : IEquatable<HWND>",
        "public readonly nint Value;",
        "public static implicit operator nint(HWND value) => value.Value;",
        "public static explicit operator HWND(nint value) => new(value);",
        "public static bool operator ==(HWND left, HWND right) => left.Value == right.Value;",
        "public static bool operator !=(HWND left, HWND right) => !(left == right);",
        "public bool Equals(HWND other) => Value == other.Value;",
        "public override bool Equals(object? obj) => obj is HWND other && Equals(other);",
        "public override int GetHashCode() => Value.GetHashCode();",
        "[StructLayout(LayoutKind.Explicit)]",
        "public struct Number",
        "[FieldOffset(0)]\n        public int signed;",
        "[FieldOffset(0)]\n        public uint unsigned;",
        "[FieldOffset(0)]\n        public Win32Test.Point point;",
        "public Win32Test.Variant.Variant_1 data;",
        "public struct Variant_1",
        "public struct ArchValue",
        "public ulong value;",
        // `HWND` crosses `LibraryImport` directly by value (no ABI decomposition), matching a
        // blittable struct; the returned handle from one real user32 export feeds directly into
        // the `HWND` parameter of another, and the out-`Rect` sugar applies exactly as it does for
        // any other blittable struct pointer.
        "public static partial Win32Test.HWND GetDesktopWindow();",
        "private static partial int GetWindowRectAbi(Win32Test.HWND hwnd, Win32Test.Rect* rect);",
        "public static bool GetWindowRect(Win32Test.HWND hwnd, out Win32Test.Rect rect)",
        "private static partial int EnumWindowsAbi(delegate* unmanaged[Stdcall]<Win32Test.HWND, nint, int> callback, nint lparam);",
        "public static bool EnumWindows(delegate* unmanaged[Stdcall]<Win32Test.HWND, nint, int> callback, nint lparam)",
        "private static partial Win32Test.HMODULE GetModuleHandleWAbi(ushort* module_name);",
        "public static Win32Test.HMODULE GetModuleHandleW(string? module_name)",
        "fixed (char* _abi0 = module_name)",
        "GetModuleHandleWAbi((ushort*)_abi0);",
        "private static partial uint GetTempPathWAbi(uint buffer_length, ushort* buffer);",
        "public static uint GetTempPathW(Span<char> buffer)",
        "fixed (char* _abi1 = buffer)",
        "GetTempPathWAbi(checked((uint)buffer.Length), (ushort*)_abi1);",
        "private static partial nuint RtlCompareMemoryUlongAbi(void* source, nuint length, uint pattern);",
        "public static nuint RtlCompareMemoryUlong(ReadOnlySpan<byte> source, uint pattern)",
        "fixed (byte* _abi0 = source)",
        "RtlCompareMemoryUlongAbi((void*)_abi0, checked((nuint)source.Length), pattern);",
        "public static int SumPoints(ReadOnlySpan<Win32Test.Point> points)",
        "SumPointsAbi((Win32Test.Point*)_abi0, checked((uint)points.Length));",
        "public static bool FillValues(Span<uint> values)",
        "FillValuesAbi((uint*)_abi0, checked((uint)values.Length));",
        "public static bool FillOptional(Span<uint> values)",
        "public static bool CompareBuffers(byte* left, byte* right, uint count)",
        "public static bool CompareOptionalBuffer(byte* left, byte* right, uint count)",
        "public uint Sum(ReadOnlySpan<byte> values)",
        "public void Fill(Span<uint> values)",
        "public unsafe partial interface IBufferOpsAbi",
        "uint Sum(byte* values, uint count);",
        "int Fill(uint* values, uint count);",
        // The generated COM source generator validates the same handle wrapper in a raw vtable
        // signature when the compile/run project includes this complete interface.
        "public unsafe partial interface IHandleConsumerAbi",
        "int SetWindow(Win32Test.HWND value);",
        // HRESULT native COM methods with one interface out pointer return an owning projected
        // object, matching the Win32 free-function surface instead of exposing a raw pointer.
        "public Win32Test.ICounter CreateCounter(int seed)",
        "(*(void***)self)[4])(self, seed, &counter)",
        "return WindowsCsharp.Com.Wrap<Win32Test.ICounter>(counter)!;",
        "int CreateCounter(int seed, nint* counter);",
        // Native C++ COM record returns use an explicit result pointer immediately after `this`,
        // including the 8-byte integer aggregate and the Arm64 HFA-shaped float pair.
        "public Win32Test.Point GetPoint()",
        "delegate* unmanaged<nint, Win32Test.Point*, void>)(*(void***)self)[3])(self, &result)",
        "public Win32Test.FloatPair GetFloatPair()",
        "delegate* unmanaged<nint, Win32Test.FloatPair*, void>)(*(void***)self)[4])(self, &result)",
        "void GetPoint(Win32Test.Point* result__);",
        "void GetFloatPair(Win32Test.FloatPair* result__);",
        // Only a required output-only, unambiguous interface double pointer is promoted. In/out,
        // optional, reserved, counted, and multiple candidates retain their literal `nint*` ABI.
        "public Win32Test.ICounter CreateCounter(int seed)",
        "public void UpdateCounter(nint* counter)",
        "public void TryCreateCounter(nint* counter)",
        "public void ReservedCounter(nint* counter)",
        "public void CreateCounterBuffer(nint* counters, uint count)",
        "public void CreateTwoCounters(nint* first, nint* second)",
        // An explicit retval attribute resolves a candidate even with another output parameter.
        "public Win32Test.ICounter CreateCounterWithStatus(out uint status)",
        "delegate* unmanaged<nint, uint*, nint*, int>)(*(void***)self)[11])",
        // Promoted outputs start null, release a non-null failure result, and reject a success-null
        // violation with E_POINTER before constructing a non-nullable owner.
        "nint counter = 0;",
        "if (_comOutHr < 0)",
        "_ = WindowsCsharp.Com.Release(counter);",
        "WindowsCsharp.Com.Check(unchecked((int)0x80004003));",
    ] {
        assert!(
            source.contains(expected),
            "missing Win32 stream projection `{expected}`"
        );
    }
    std::fs::write(project.join("project.csproj"), EXE_CSPROJ).unwrap();
    std::fs::write(
        project.join("Program.cs"),
        r#"using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.Marshalling;
using Win32Test;

internal static unsafe class Program
{
    private static nint s_counter;
    private static int s_releaseCount;

    private static void Main()
    {
        if (Apis.MAGIC != 0x12345678u)
        {
            throw new InvalidOperationException("constant projection failed");
        }
        Point point = new() { x = 10, y = 20 };
        if (point.x != 10 || point.y != 20 || Mode.Offset != (Mode)1)
        {
            throw new InvalidOperationException("Win32 value type projection failed");
        }
        Number number = new() { unsigned = uint.MaxValue };
        if (number.signed != -1)
        {
            throw new InvalidOperationException("Win32 union projection failed");
        }
        Variant variant = new() { data = new() { unsigned = uint.MaxValue } };
        if (variant.data.signed != -1)
        {
            throw new InvalidOperationException("nested Win32 union projection failed");
        }
        uint actual = Apis.GetCurrentProcessId();
        if (actual != (uint)Environment.ProcessId)
        {
            throw new InvalidOperationException($"process id mismatch: {actual}");
        }

        // A real Win32 out-scalar export (`QueryPerformanceCounter`): the `out long` sugar must
        // observe the same counter value the BCL's own P/Invoke of the same function would.
        if (!Apis.QueryPerformanceCounter(out long perfCounter) || perfCounter <= 0)
        {
            throw new InvalidOperationException($"QueryPerformanceCounter failed: {perfCounter}");
        }

        // A real, no-UI handle round trip: `GetDesktopWindow` never fails and never shows or owns
        // a window, so its returned `HWND` must be usable, unchanged, as `GetWindowRect`'s first
        // argument, and the filled `out Rect` must describe a non-empty screen rectangle.
        Win32Test.HWND desktop = Apis.GetDesktopWindow();
        if (desktop == default)
        {
            throw new InvalidOperationException("GetDesktopWindow returned a null handle");
        }
        if (!Apis.GetWindowRect(desktop, out Win32Test.Rect desktopRect))
        {
            throw new InvalidOperationException("GetWindowRect failed");
        }
        if (desktopRect.right <= desktopRect.left || desktopRect.bottom <= desktopRect.top)
        {
            throw new InvalidOperationException($"GetWindowRect returned an empty rectangle: {desktopRect.left},{desktopRect.top},{desktopRect.right},{desktopRect.bottom}");
        }

        int callbackCount = 0;
        _ = Apis.EnumWindows(&EnumWindow, (nint)(&callbackCount));
        if (callbackCount != 1)
        {
            throw new InvalidOperationException($"EnumWindows callback count mismatch: {callbackCount}");
        }
        _ = Apis.EnumWindows(&EnumWindow, (nint)(&callbackCount));
        long callbackAllocatedBefore = GC.GetAllocatedBytesForCurrentThread();
        for (int i = 0; i < 1_000; i++)
        {
            _ = Apis.EnumWindows(&EnumWindow, (nint)(&callbackCount));
        }
        long callbackAllocated =
            GC.GetAllocatedBytesForCurrentThread() - callbackAllocatedBefore;
        if (callbackAllocated != 0)
        {
            throw new InvalidOperationException($"EnumWindows callbacks allocated {callbackAllocated} bytes");
        }

        if (Apis.GetModuleHandleW(null) == default ||
            Apis.GetModuleHandleW("kernel32.dll") == default)
        {
            throw new InvalidOperationException("GetModuleHandleW string projection failed");
        }

        Span<char> tempPath = stackalloc char[260];
        if (Apis.GetTempPathW(Span<char>.Empty) == 0)
        {
            throw new InvalidOperationException("GetTempPathW empty-span query failed");
        }
        uint tempPathLength = Apis.GetTempPathW(tempPath);
        if (tempPathLength == 0 || tempPathLength >= (uint)tempPath.Length ||
            tempPath[(int)tempPathLength - 1] != '\\')
        {
            throw new InvalidOperationException("GetTempPathW output string projection failed");
        }
        _ = Apis.GetTempPathW(tempPath);
        long stringBufferAllocatedBefore = GC.GetAllocatedBytesForCurrentThread();
        for (int i = 0; i < 1_000; i++)
        {
            _ = Apis.GetTempPathW(tempPath);
        }
        long stringBufferAllocated =
            GC.GetAllocatedBytesForCurrentThread() - stringBufferAllocatedBefore;
        if (stringBufferAllocated != 0)
        {
            throw new InvalidOperationException($"GetTempPathW allocated {stringBufferAllocated} bytes");
        }

        byte[] repeated = [0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12];
        if (Apis.RtlCompareMemoryUlong(repeated, 0x12345678) != (nuint)repeated.Length)
        {
            throw new InvalidOperationException("RtlCompareMemoryUlong span projection failed");
        }
        _ = Apis.RtlCompareMemoryUlong(repeated, 0x12345678);
        long bufferAllocatedBefore = GC.GetAllocatedBytesForCurrentThread();
        for (int i = 0; i < 1_000; i++)
        {
            _ = Apis.RtlCompareMemoryUlong(repeated, 0x12345678);
        }
        long bufferAllocated = GC.GetAllocatedBytesForCurrentThread() - bufferAllocatedBefore;
        if (bufferAllocated != 0)
        {
            throw new InvalidOperationException($"RtlCompareMemoryUlong allocated {bufferAllocated} bytes");
        }

        ManagedCounter managedCounter = new();
        ComWrappers wrappers = new StrategyBasedComWrappers();
        nint unknown = wrappers.GetOrCreateComInterfaceForObject(
            managedCounter,
            CreateComInterfaceFlags.None);
        try
        {
            using ICounter projected = WindowsCsharp.Com.As<ICounter>(unknown, false);
            if (projected.GetValue() != 40 || projected.Add(2) != 42)
            {
                throw new InvalidOperationException("generated COM implementation failed");
            }
        }
        finally
        {
            _ = WindowsCsharp.Com.Release(unknown);
            GC.KeepAlive(managedCounter);
        }

        ManagedBufferOps managedBufferOps = new();
        unknown = wrappers.GetOrCreateComInterfaceForObject(
            managedBufferOps,
            CreateComInterfaceFlags.None);
        try
        {
            using IBufferOps projected = WindowsCsharp.Com.As<IBufferOps>(unknown, false);
            ReadOnlySpan<byte> inputValues = stackalloc byte[4] { 1, 2, 3, 4 };
            if (projected.Sum(inputValues) != 10)
            {
                throw new InvalidOperationException("generated COM input buffer failed");
            }
            Span<uint> outputValues = stackalloc uint[4];
            projected.Fill(outputValues);
            if (outputValues[0] != 1 || outputValues[3] != 4)
            {
                throw new InvalidOperationException("generated COM output buffer failed");
            }
            _ = projected.Sum(inputValues);
            projected.Fill(outputValues);
            long comBufferAllocatedBefore = GC.GetAllocatedBytesForCurrentThread();
            for (int i = 0; i < 1_000; i++)
            {
                _ = projected.Sum(inputValues);
                projected.Fill(outputValues);
            }
            long comBufferAllocated =
                GC.GetAllocatedBytesForCurrentThread() - comBufferAllocatedBefore;
            if (comBufferAllocated != 0)
            {
                throw new InvalidOperationException($"COM buffer calls allocated {comBufferAllocated} bytes");
            }
        }
        finally
        {
            _ = WindowsCsharp.Com.Release(unknown);
            GC.KeepAlive(managedBufferOps);
        }

        ManagedStream managedStream = new();
        unknown = wrappers.GetOrCreateComInterfaceForObject(
            managedStream,
            CreateComInterfaceFlags.None);
        try
        {
            using IStream projected = WindowsCsharp.Com.As<IStream>(unknown, false);
            projected.SetSize(64);
            using ISequentialStream sequential = projected.As<ISequentialStream>();
            ReadOnlySpan<byte> generatedInput = stackalloc byte[7];
            sequential.Write(generatedInput, out uint written);
            if (managedStream.Size != 64 || written != 7)
            {
                throw new InvalidOperationException("generated COM inheritance failed");
            }
        }
        finally
        {
            _ = WindowsCsharp.Com.Release(unknown);
            GC.KeepAlive(managedStream);
        }

        using (IStream stream = Apis.CreateStreamOnHGlobal(0, true))
        {
            ReadOnlySpan<byte> input = stackalloc byte[4] { 10, 20, 30, 40 };
            stream.Write(input, out uint written);
            if (written != 4)
            {
                throw new InvalidOperationException($"stream write mismatch: {written}");
            }

            stream.Seek(0, 0, out ulong position);
            Span<byte> output = stackalloc byte[4];
            stream.Read(output, out uint read);
            if (position != 0 || read != 4 || output[0] != 10 || output[3] != 40)
            {
                throw new InvalidOperationException("IStream projection failed");
            }
        }

        nint* vtable = stackalloc nint[5];
        vtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&QueryInterface;
        vtable[1] = (nint)(delegate* unmanaged<nint, uint>)&AddRef;
        vtable[2] = (nint)(delegate* unmanaged<nint, uint>)&Release;
        vtable[3] = (nint)(delegate* unmanaged<nint, int>)&GetValue;
        vtable[4] = (nint)(delegate* unmanaged<nint, int, int>)&Add;
        nint* instance = stackalloc nint[1];
        instance[0] = (nint)vtable;
        using ICounter counter = new((nint)instance);
        if (counter.GetValue() != 41 || counter.Add(1) != 42)
        {
            throw new InvalidOperationException("IUnknown vtable projection failed");
        }

        nint* trackedCounterVtable = stackalloc nint[5];
        trackedCounterVtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&QueryInterface;
        trackedCounterVtable[1] = (nint)(delegate* unmanaged<nint, uint>)&AddRef;
        trackedCounterVtable[2] = (nint)(delegate* unmanaged<nint, uint>)&TrackedRelease;
        trackedCounterVtable[3] = (nint)(delegate* unmanaged<nint, int>)&GetValue;
        trackedCounterVtable[4] = (nint)(delegate* unmanaged<nint, int, int>)&Add;
        nint* trackedCounter = stackalloc nint[1];
        trackedCounter[0] = (nint)trackedCounterVtable;
        s_counter = (nint)trackedCounter;

        nint* nativeAbiVtable = stackalloc nint[12];
        nativeAbiVtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&QueryInterface;
        nativeAbiVtable[1] = (nint)(delegate* unmanaged<nint, uint>)&AddRef;
        nativeAbiVtable[2] = (nint)(delegate* unmanaged<nint, uint>)&Release;
        nativeAbiVtable[3] = (nint)(delegate* unmanaged<nint, Point*, void>)&GetPointResult;
        nativeAbiVtable[4] = (nint)(delegate* unmanaged<nint, FloatPair*, void>)&GetFloatPairResult;
        nativeAbiVtable[5] = (nint)(delegate* unmanaged<nint, int, nint*, int>)&CreateCounterResult;
        nativeAbiVtable[6] = (nint)(delegate* unmanaged<nint, nint*, int>)&UpdateCounterRaw;
        nativeAbiVtable[7] = (nint)(delegate* unmanaged<nint, nint*, int>)&TryCreateCounterRaw;
        nativeAbiVtable[8] = (nint)(delegate* unmanaged<nint, nint*, int>)&ReservedCounterRaw;
        nativeAbiVtable[9] = (nint)(delegate* unmanaged<nint, nint*, uint, int>)&CreateCounterBufferRaw;
        nativeAbiVtable[10] = (nint)(delegate* unmanaged<nint, nint*, nint*, int>)&CreateTwoCountersRaw;
        nativeAbiVtable[11] = (nint)(delegate* unmanaged<nint, uint*, nint*, int>)&CreateCounterWithStatusResult;
        nint* nativeAbiInstance = stackalloc nint[1];
        nativeAbiInstance[0] = (nint)nativeAbiVtable;

        using INativeAbiCases nativeAbi = new((nint)nativeAbiInstance);
        Point returnedPoint = nativeAbi.GetPoint();
        FloatPair returnedPair = nativeAbi.GetFloatPair();
        if (returnedPoint.x != 17 || returnedPoint.y != 29 ||
            returnedPair.x != 1.25f || returnedPair.y != 2.5f)
        {
            throw new InvalidOperationException("native COM hidden result pointer failed");
        }

        int releases = s_releaseCount;
        using (ICounter owned = nativeAbi.CreateCounter(7))
        {
            if (owned.GetValue() != 41)
            {
                throw new InvalidOperationException("COM out ownership transfer failed");
            }
        }
        if (s_releaseCount != releases + 1)
        {
            throw new InvalidOperationException("COM out owner did not release exactly once");
        }

        releases = s_releaseCount;
        try
        {
            using ICounter unexpected = nativeAbi.CreateCounter(-1);
            throw new InvalidOperationException("failing COM out unexpectedly succeeded");
        }
        catch (COMException error) when (error.HResult == unchecked((int)0x80004005))
        {
        }
        if (s_releaseCount != releases + 1)
        {
            throw new InvalidOperationException("failing COM out did not release its pointer");
        }

        releases = s_releaseCount;
        try
        {
            using ICounter unexpected = nativeAbi.CreateCounter(-2);
            throw new InvalidOperationException("success-null COM out unexpectedly succeeded");
        }
        catch (COMException error) when (error.HResult == unchecked((int)0x80004003))
        {
        }
        if (s_releaseCount != releases)
        {
            throw new InvalidOperationException("success-null COM out released a null pointer");
        }

        nint optional = 1;
        nativeAbi.TryCreateCounter(&optional);
        if (optional != 0)
        {
            throw new InvalidOperationException("optional raw COM out did not preserve null");
        }

        nint inOut = s_counter;
        nativeAbi.UpdateCounter(&inOut);
        if (inOut != s_counter)
        {
            throw new InvalidOperationException("in/out COM pointer was not preserved");
        }

        using (ICounter explicitResult = nativeAbi.CreateCounterWithStatus(out uint status))
        {
            if (status != 99 || explicitResult.GetValue() != 41)
            {
                throw new InvalidOperationException("explicit COM retval selection failed");
            }
        }

        Console.WriteLine($"windows-csharp Win32 OK: ProcessId={actual}");
    }

    [UnmanagedCallersOnly]
    private static int QueryInterface(nint self, Guid* iid, nint* result)
    {
        *result = self;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static uint AddRef(nint self) => 2;

    [UnmanagedCallersOnly]
    private static uint Release(nint self) => 1;

    [UnmanagedCallersOnly]
    private static uint TrackedRelease(nint self)
    {
        s_releaseCount++;
        return 1;
    }

    [UnmanagedCallersOnly]
    private static int GetValue(nint self) => 41;

    [UnmanagedCallersOnly]
    private static int Add(nint self, int value) => 41 + value;

    [UnmanagedCallersOnly]
    private static void GetPointResult(nint self, Point* result)
    {
        result->x = 17;
        result->y = 29;
    }

    [UnmanagedCallersOnly]
    private static void GetFloatPairResult(nint self, FloatPair* result)
    {
        result->x = 1.25f;
        result->y = 2.5f;
    }

    [UnmanagedCallersOnly]
    private static int CreateCounterResult(nint self, int seed, nint* counter)
    {
        if (seed == -2)
        {
            *counter = 0;
            return 0;
        }
        *counter = s_counter;
        return seed == -1 ? unchecked((int)0x80004005) : 0;
    }

    [UnmanagedCallersOnly]
    private static int UpdateCounterRaw(nint self, nint* counter) => 0;

    [UnmanagedCallersOnly]
    private static int TryCreateCounterRaw(nint self, nint* counter)
    {
        *counter = 0;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int ReservedCounterRaw(nint self, nint* counter)
    {
        *counter = 0;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int CreateCounterBufferRaw(nint self, nint* counters, uint count)
    {
        for (uint i = 0; i < count; i++)
        {
            counters[i] = 0;
        }
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int CreateTwoCountersRaw(nint self, nint* first, nint* second)
    {
        *first = 0;
        *second = 0;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int CreateCounterWithStatusResult(nint self, uint* status, nint* counter)
    {
        *status = 99;
        *counter = s_counter;
        return 0;
    }

    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvStdcall)])]
    private static int EnumWindow(HWND hwnd, nint lparam)
    {
        _ = hwnd;
        (*(int*)lparam)++;
        return 0;
    }
}

[GeneratedComClass]
internal sealed partial class ManagedCounter : ICounterAbi
{
    public int GetValue() => 40;

    public int Add(int value) => 40 + value;
}

[GeneratedComClass]
internal sealed unsafe partial class ManagedBufferOps : IBufferOpsAbi
{
    public uint Sum(byte* values, uint count)
    {
        uint result = 0;
        for (uint i = 0; i < count; i++)
        {
            result += values[i];
        }
        return result;
    }

    public int Fill(uint* values, uint count)
    {
        for (uint i = 0; i < count; i++)
        {
            values[i] = i + 1;
        }
        return 0;
    }
}

[GeneratedComClass]
internal sealed unsafe partial class ManagedStream : IStreamAbi
{
    internal ulong Size { get; private set; }

    public int Read(byte* value, uint count, uint* read)
    {
        *read = count;
        return 0;
    }

    public int Write(byte* value, uint count, uint* written)
    {
        *written = count;
        return 0;
    }

    public int Seek(long offset, uint origin, ulong* position)
    {
        *position = (ulong)offset;
        return 0;
    }

    public int SetSize(ulong size)
    {
        Size = size;
        return 0;
    }
}
"#,
    )
    .unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("run")
        .args(["--project", project.to_str().unwrap()])
        .output()
        .expect("failed to run Win32 projection");
    assert!(
        output.status.success(),
        "generated Win32 projection failed to run\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("windows-csharp Win32 OK"),
        "unexpected Win32 projection output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn round_trip() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let staged = stage_component();

    let scratch = scratch("roundtrip");

    // Generate the projection from the live test_bench component's winmd (a build dependency, so
    // cargo has already authored it) and run it against that same component, so the round trip
    // validates the generator against a real WinRT component rather than a fixture.
    let winmd = "../../../samples/test_bench/component/bench.winmd";

    let project = scratch.join("project");
    let _ = std::fs::remove_dir_all(&project);
    std::fs::create_dir_all(&project).unwrap();
    windows_csharp::builder()
        .input(winmd)
        .input(REFERENCE)
        .input(FOUNDATION)
        .filter("Bench")
        .output(project.join("Bench.cs").to_str().unwrap())
        .write()
        .unwrap();
    std::fs::write(project.join("project.csproj"), EXE_CSPROJ).unwrap();
    std::fs::write(project.join("Program.cs"), PROGRAM).unwrap();

    // Run the harness with its working directory set to the freshly staged component so the
    // generated `LoadLibrary("Bench.dll")` resolves it by the current-directory search, which the
    // loader checks before `PATH`. Cargo puts both `target/debug` and `target/debug/deps` on the
    // inherited `PATH`, and a stale `Bench.dll` left in `target/debug` by an earlier run would
    // otherwise win the bare-name lookup -- loading an old component whose signature-derived
    // interface IIDs no longer match the regenerated projection (a QI then fails with E_NOINTERFACE).
    let mut command = std::process::Command::new("dotnet.exe");
    command
        .arg("run")
        .args(["--project", project.to_str().unwrap()]);
    if let Some(dir) = &staged {
        command.current_dir(dir);
    }
    let output = command.output().expect("failed to run dotnet");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "generated projection failed to run\nstdout:\n{stdout}\n\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains(
            "windows-csharp OK: Int32=123 Add=5 String=widget Echo=round-echo Cast=123 Fail=threw Event=77 Revoke=99"
        ),
        "unexpected output:\n{stdout}"
    );
}

/// Stages the Rust WinRT component cdylib as `Bench.dll` -- the module name WinRT activation probes
/// for the `Bench` namespace -- beside the test binary, and returns that directory. The caller runs
/// the harness with this as its working directory so the generated `LoadLibrary("Bench.dll")`
/// resolves this freshly staged copy by the current-directory search (which the loader checks before
/// `PATH`) rather than a stale `Bench.dll` a previous run may have left elsewhere on `PATH`.
fn stage_component() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    std::fs::copy(dir.join("bench_component.dll"), dir.join("Bench.dll")).ok()?;
    Some(dir.to_path_buf())
}

/// Returns whether `dotnet` is available, so the `dotnet`-backed tests can skip where it is absent.
fn have_dotnet() -> bool {
    std::process::Command::new("dotnet.exe")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

/// Authors `input/selection.rdl` and generates a projection fragment from a builder already
/// pointed at that winmd plus the reference metadata, letting the caller add `.select`/`.member`
/// calls before writing.
fn selection_builder(scratch_name: &str) -> (windows_csharp::Builder, PathBuf) {
    let scratch = scratch(scratch_name);
    let winmd = author("selection", &scratch);
    let generated = scratch.join("generated.cs");
    let builder = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .output(generated.to_str().unwrap())
        .fragment();
    (builder, generated)
}

fn delegate_selection_builder(scratch_name: &str) -> (windows_csharp::Builder, PathBuf) {
    let scratch = scratch(scratch_name);
    let winmd = author("delegate_marshalling", &scratch);
    let generated = scratch.join("generated.cs");
    let builder = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .output(generated.to_str().unwrap())
        .fragment();
    (builder, generated)
}

fn diagnostic_selection_builder(scratch_name: &str) -> (windows_csharp::Builder, PathBuf) {
    let scratch = scratch(scratch_name);
    let winmd = author("selection_diagnostics", &scratch);
    let generated = scratch.join("generated.cs");
    let builder = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .output(generated.to_str().unwrap())
        .fragment();
    (builder, generated)
}

#[test]
fn selection_delegate_marshalling_dependencies() {
    let (builder, generated) =
        delegate_selection_builder("selection_delegate_marshalling_dependencies");
    builder
        .member("DelegateMarshalling.IDelegateHost", "UseString")
        .member("DelegateMarshalling.IDelegateHost", "UseObject")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "class StringCallback",
        "class ObjectCallback",
        "class IDelegatePeer",
        "FromHstringBorrowed(value)",
        "WindowsCsharp.Com.AddRef(resultValue)",
    ] {
        assert!(
            source.contains(expected),
            "delegate dependency closure omitted `{expected}`:\n{source}"
        );
    }
    assert!(!source.contains("UnsupportedCallback"));
}

#[test]
fn selection_unsupported_delegate_errors() {
    let (builder, _) = delegate_selection_builder("selection_unsupported_delegate");
    let error = builder
        .member("DelegateMarshalling.IDelegateHost", "UseUnsupported")
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("not supported"));
}

#[test]
fn selection_supported_value_type_roots() {
    let (builder, generated) = selection_builder("selection_supported_value_type_roots");
    builder
        .select("Selection.ChangedHandler")
        .select("Selection.Mode")
        .select("Selection.Point")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in ["class ChangedHandler", "enum Mode", "struct Point"] {
        assert!(source.contains(expected), "missing `{expected}`:\n{source}");
    }
    assert!(!source.contains("class Widget"));
}

#[test]
fn selection_preserves_forwarded_overloads() {
    let (builder, generated) = selection_builder("selection_forwarded_overloads");
    builder
        .member("Selection.Widget", "Transform")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();
    assert!(source.contains("public int Transform()"));
    assert!(source.contains("public int Transform(int value)"));
}

#[test]
fn selection_supported_activation_and_static_root() {
    let scratch = scratch("selection_supported_activation");
    let winmd = author("activation", &scratch);
    let generated = scratch.join("generated.cs");
    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .select("Activation.Widget")
        .output(generated.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();
    for expected in [
        "public Widget()",
        "public Widget(int seed)",
        "public static int Count",
    ] {
        assert!(source.contains(expected), "missing `{expected}`:\n{source}");
    }
}

#[test]
fn selection_allows_empty_composable_factory() {
    let scratch = scratch("selection_empty_composable_factory");
    let winmd = author("activation", &scratch);
    let generated = scratch.join("generated.cs");
    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .member("Activation.Hosted", "Value")
        .output(generated.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();
    assert!(source.contains("public sealed unsafe class Hosted"));
    assert!(source.contains("public int Value"));
    assert!(!source.contains("public Hosted("));
}

#[test]
fn selection_reverse_delegate_shape_errors() {
    for (name, detail) in [
        ("BooleanCallback", "reverse-delegate shape `Bool`"),
        ("ArrayCallback", "reverse-delegate shape `Array(I32)`"),
        ("ConvertedStructCallback", "ConvertedStructCallback"),
    ] {
        let (builder, _) = diagnostic_selection_builder(&format!("selection_{name}"));
        let error = builder
            .select(format!("SelectionDiagnostics.{name}"))
            .write()
            .unwrap_err();
        let message = error.to_string();
        assert!(
            message.contains(detail),
            "missing `{detail}` in diagnostic:\n{message}"
        );
    }
}

#[test]
fn selection_unsupported_delegate_dependencies_error() {
    for (member, delegate) in [
        ("UseBoolean", "BooleanCallback"),
        ("UseArray", "ArrayCallback"),
        ("UseConvertedStruct", "ConvertedStructCallback"),
    ] {
        let (builder, _) = diagnostic_selection_builder(&format!("selection_dependency_{member}"));
        let error = builder
            .member("SelectionDiagnostics.Diagnostics", member)
            .write()
            .unwrap_err();
        let message = error.to_string();
        assert!(message.contains(member), "{message}");
        assert!(message.contains(delegate), "{message}");
        assert!(message.contains("not supported"), "{message}");
    }
}

#[test]
fn selection_unsupported_generic_dependency_errors() {
    let (builder, _) = diagnostic_selection_builder("selection_unsupported_generic");
    let message = builder
        .member("SelectionDiagnostics.Diagnostics", "UnsupportedGeneric")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("UnsupportedGeneric"), "{message}");
    assert!(message.contains("IVector"), "{message}");
    assert!(message.contains("not supported"), "{message}");
}

#[test]
fn selection_missing_required_generic_metadata_errors() {
    let scratch = scratch("selection_missing_generic_metadata");
    let winmd = author("selection_diagnostics", &scratch);
    let message = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .member("SelectionDiagnostics.Diagnostics", "GoodVector")
        .output(scratch.join("generated.cs").to_str().unwrap())
        .fragment()
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("required generic"), "{message}");
    assert!(message.contains("IVector"), "{message}");
    assert!(message.contains("GuidAttribute"), "{message}");
}

#[test]
fn selection_nested_generic_dependency_closure() {
    let (builder, generated) = diagnostic_selection_builder("selection_nested_generic_dependency");
    builder
        .member("SelectionDiagnostics.Diagnostics", "VectorArray")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();
    assert!(
        source.contains("public sealed unsafe class IVector<T>"),
        "{source}"
    );
    assert!(
        source.contains("IVector<int>?[] values"),
        "nested generic surface missing:\n{source}"
    );
}

#[test]
fn selection_unsupported_struct_root_errors() {
    let (builder, _) = diagnostic_selection_builder("selection_unsupported_struct_root");
    let message = builder
        .select("SelectionDiagnostics.ConvertedUnion")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("ConvertedUnion"), "{message}");
    assert!(
        message.contains("explicit layout")
            || message.contains("unsupported shape")
            || message.contains("ABI conversion"),
        "{message}"
    );
}

#[test]
fn selection_class_forwarder_factory_and_static_errors() {
    for (type_name, member, detail) in [
        (
            "SelectionDiagnostics.Diagnostics",
            "ForwardedBoolean",
            "BooleanCallback",
        ),
        (
            "SelectionDiagnostics.StaticDiagnostics",
            "BadStatic",
            "BooleanCallback",
        ),
    ] {
        let (builder, _) =
            diagnostic_selection_builder(&format!("selection_{}", member.to_lowercase()));
        let message = builder
            .member(type_name, member)
            .write()
            .unwrap_err()
            .to_string();
        assert!(message.contains(member), "{message}");
        assert!(message.contains(detail), "{message}");
    }

    let (builder, _) = diagnostic_selection_builder("selection_unsupported_factory");
    let message = builder
        .member("SelectionDiagnostics.Constructed", "Value")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("constructor"), "{message}");
    assert!(message.contains("BooleanCallback"), "{message}");
}

#[test]
fn selection_diagnostic_order_is_stable() {
    fn error(reverse: bool) -> String {
        let (builder, _) = diagnostic_selection_builder(if reverse {
            "selection_diagnostic_order_reverse"
        } else {
            "selection_diagnostic_order_forward"
        });
        let builder = if reverse {
            builder
                .member("SelectionDiagnostics.Diagnostics", "UseBoolean")
                .member("SelectionDiagnostics.Diagnostics", "UnsupportedGeneric")
        } else {
            builder
                .member("SelectionDiagnostics.Diagnostics", "UnsupportedGeneric")
                .member("SelectionDiagnostics.Diagnostics", "UseBoolean")
        };
        builder.write().unwrap_err().to_string()
    }

    let forward = error(false);
    let reverse = error(true);
    assert_eq!(forward, reverse);
    assert!(forward.contains("UnsupportedGeneric"), "{forward}");
}

#[test]
fn exact_selection_ignores_namespace_filters() {
    let (builder, generated) = diagnostic_selection_builder("selection_ignores_filter");
    builder
        .filter("Unrelated")
        .member("SelectionDiagnostics.Diagnostics", "Good")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();
    assert!(source.contains("public int Good("));
    assert!(!source.contains("UseBoolean"));
}

/// Selecting an entire class projects its default-interface and forwarder members in full, and
/// pulls in every referenced class/interface/delegate/enum/struct as a dependency -- a referenced
/// class or interface that is not itself selected comes through as a marker (castable, but with no
/// members of its own).
#[test]
fn selection_whole_class() {
    let (builder, generated) = selection_builder("selection_whole_class");
    builder.select("Selection.Widget").write().unwrap();
    let source = std::fs::read_to_string(&generated).unwrap();

    // The class's own default-interface and forwarder members are all present.
    for expected in ["Value", "Location", "State", "Peer", "Compute", "Changed"] {
        assert!(
            source.contains(expected),
            "missing member `{expected}` in:\n{source}"
        );
    }
    assert!(
        source.contains("Extra"),
        "missing forwarded member `Extra` in:\n{source}"
    );

    // The dependency closure pulls in the delegate, struct, and enum in full...
    assert!(
        source.contains("class ChangedHandler"),
        "missing dependency delegate:\n{source}"
    );
    assert!(
        source.contains("struct Point"),
        "missing dependency struct:\n{source}"
    );
    assert!(
        source.contains("enum Mode"),
        "missing dependency enum:\n{source}"
    );

    // ...but the referenced `Gadget` class comes through as a marker: castable (it has an `Iid` and
    // internal ABI constructors), but its own members and public activation were never selected.
    assert!(
        source.contains("class Gadget"),
        "missing marker class Gadget:\n{source}"
    );
    assert!(
        !source.contains("int Id"),
        "marker class Gadget should not project Id:\n{source}"
    );
    assert!(
        !source.contains("public Gadget()"),
        "marker class Gadget should not project activation:\n{source}"
    );

    // `IStandalone` was never referenced by anything selected, so it must not appear at all.
    assert!(
        !source.contains("IStandalone"),
        "unrelated interface leaked in:\n{source}"
    );
}

/// Narrowing a class selection to specific members keeps only those members (dropping every
/// unselected one, including ones that would otherwise pull in a dependency), while a selected
/// forwarder member from a non-default interface still comes through. The real ABI vtable slot
/// numbers are preserved regardless of which members were requested.
#[test]
fn selection_narrowed_members() {
    let (builder, generated) = selection_builder("selection_narrowed_members");
    builder
        .member("Selection.Widget", "Value")
        .member("Selection.Widget", "Extra")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(&generated).unwrap();

    assert!(
        source.contains("Value"),
        "missing selected member `Value` in:\n{source}"
    );
    assert!(
        source.contains("Extra"),
        "missing selected forwarder member `Extra` in:\n{source}"
    );
    for unselected in ["Location", "State", "Peer", "Compute", "Changed"] {
        assert!(
            !source.contains(unselected),
            "unselected member `{unselected}` leaked in:\n{source}"
        );
    }

    // Nothing selected references Gadget/Point/Mode/ChangedHandler, so the dependency closure must
    // not pull any of them in.
    for absent in [
        "class Gadget",
        "struct Point",
        "enum Mode",
        "class ChangedHandler",
    ] {
        assert!(
            !source.contains(absent),
            "unrelated dependency `{absent}` leaked in:\n{source}"
        );
    }

    // `Value`'s getter is still the interface's first method (slot 6), matching the whole-class
    // selection -- narrowing members never renumbers a kept member's vtable slot.
    assert!(
        source.contains("(*(void***)self)[6])(self, &value")
            || source.contains("(*(void***)self)[6])(self, &result"),
        "Value getter did not keep vtable slot 6 in:\n{source}"
    );
}

#[test]
fn selection_async_dependency_closure() {
    let scratch = scratch("selection_async_dependency_closure");
    let winmd = author("breadth", &scratch);
    let generated = scratch.join("generated.cs");

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .select("Breadth.Store")
        .output(generated.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();

    let source = std::fs::read_to_string(generated).unwrap();
    for expected in [
        "typeof(T) == typeof(int)",
        "typeof(T) == typeof(string)",
        "typeof(T) == typeof(Breadth.Item)",
        "ComputeCompletedIid",
        "new Guid(0x6bab735e, 0x7cbe, 0x5aca, 0x9f, 0x08, 0x42, 0xba, 0x13, 0xc3, 0xe8, 0x58)",
        "new Guid(0xd60cae9d, 0x88cb, 0x59f1, 0x85, 0x76, 0x3f, 0xba, 0x44, 0x79, 0x6b, 0xe8)",
        "new Guid(0xb79a741f, 0x7fb5, 0x50ae, 0x9e, 0x99, 0x91, 0x12, 0x01, 0xec, 0x3d, 0x41)",
    ] {
        assert!(
            source.contains(expected),
            "selected async dependency omitted `{expected}`:\n{source}"
        );
    }
}

#[test]
fn object_vector_enumerator_is_disposable() {
    let scratch = scratch("object_vector_enumerator_is_disposable");
    let winmd = author("generics", &scratch);
    let generated = scratch.join("generated.cs");

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .filter("Sample")
        .output(generated.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();

    let source = std::fs::read_to_string(generated).unwrap();
    assert!(
        source.contains("public struct Enumerator : IDisposable"),
        "object-vector foreach cannot dispose buffered owners:\n{source}"
    );
}

/// A standalone interface (no default-interface class) can be selected on its own.
#[test]
fn selection_standalone_interface() {
    let (builder, generated) = selection_builder("selection_standalone_interface");
    builder.select("Selection.IStandalone").write().unwrap();
    let source = std::fs::read_to_string(&generated).unwrap();

    assert!(
        source.contains("class IStandalone"),
        "missing selected interface:\n{source}"
    );
    assert!(
        source.contains("Ping"),
        "missing selected interface member:\n{source}"
    );
    assert!(
        !source.contains("class Widget"),
        "unrelated class leaked in:\n{source}"
    );
}

/// Selecting a type name that does not exist in the metadata is a hard error, not a silent no-op.
#[test]
fn selection_unknown_type_errors() {
    let (builder, _generated) = selection_builder("selection_unknown_type_errors");
    let error = builder
        .select("Selection.DoesNotExist")
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

/// Selecting a member name that does not exist on the selected type is a hard error, not a
/// silently empty projection.
#[test]
fn selection_unknown_member_errors() {
    let (builder, _generated) = selection_builder("selection_unknown_member_errors");
    let error = builder
        .member("Selection.Widget", "DoesNotExist")
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

/// Selecting a metadata item through the wrong exact-selection entry point reports its kind.
#[test]
fn selection_non_class_or_interface_root_errors() {
    let (builder, _generated) = selection_builder("selection_non_class_or_interface_root_errors");
    let error = builder.member("Selection.Point", "X").write().unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("metadata kind `struct`"));
}

/// The generated projection for an exact selection compiles standalone with `dotnet`.
fn win32_selection_builder(name: &str) -> (windows_csharp::Builder, PathBuf) {
    let scratch = scratch(name);
    let winmd = author("win32_foundation", &scratch);
    let generated = scratch.join("Generated.cs");
    (
        windows_csharp::builder()
            .input(winmd.to_str().unwrap())
            .input(FOUNDATION)
            .output(generated.to_str().unwrap())
            .fragment(),
        generated,
    )
}

#[test]
fn namespace_filter_omits_variadic_functions() {
    let (builder, generated) = win32_selection_builder("namespace_filter_omits_variadic");
    builder.filter("Win32Test").write().unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    assert!(source.contains("GetCurrentProcessId"));
    assert!(!source.contains("VariadicFunction"));
}

#[test]
fn exact_variadic_function_selection_errors() {
    let (builder, _) = win32_selection_builder("exact_variadic_function");
    assert_eq!(
        builder
            .function("Win32Test.VariadicFunction")
            .write()
            .unwrap_err()
            .to_string(),
        "windows-csharp: selected function `Win32Test.VariadicFunction` is variadic, and variadic \
         functions are unsupported"
    );
}

#[test]
fn real_variadic_function_selection_errors() {
    let scratch = scratch("real_variadic_function");
    let input = Path::new("../../../libs/bindgen/default/Windows.Win32.winmd");
    let output = scratch.join("Generated.cs");
    let error = windows_csharp::builder()
        .input(input.to_str().unwrap())
        .function("Windows.Win32.AuthzReportSecurityEvent")
        .output(output.to_str().unwrap())
        .fragment()
        .write()
        .unwrap_err();

    assert_eq!(
        error.to_string(),
        "windows-csharp: selected function `Windows.Win32.AuthzReportSecurityEvent` is variadic, \
         and variadic functions are unsupported"
    );
}

#[test]
fn real_inout_buffer_projects_as_span() {
    let scratch = scratch("real_inout_buffer");
    let input = Path::new("../../../libs/bindgen/default/Windows.Win32.winmd");
    let output = scratch.join("Generated.cs");
    windows_csharp::builder()
        .input(input.to_str().unwrap())
        .function("Windows.Win32.DPtoLP")
        .output(output.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();
    let source = std::fs::read_to_string(output).unwrap();

    for expected in [
        "private static partial int DPtoLPAbi(Windows.Win32.HDC hdc, Windows.Win32.POINT* lppt, int c);",
        "public static bool DPtoLP(Windows.Win32.HDC hdc, Span<Windows.Win32.POINT> lppt)",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
}

#[test]
fn selection_win32_function_and_dependencies() {
    let (builder, generated) = win32_selection_builder("selection_win32_function");
    builder.function("Win32Test.GetWindowRect").write().unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "public readonly struct HWND",
        "public struct Rect",
        "public static bool GetWindowRect(Win32Test.HWND hwnd, out Win32Test.Rect rect)",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
    for excluded in ["GetDesktopWindow", "MAGIC", "class ICounter"] {
        assert!(!source.contains(excluded), "unexpected `{excluded}`");
    }
}

#[test]
fn selection_win32_callback_and_dependencies() {
    let (builder, generated) = win32_selection_builder("selection_win32_callback");
    builder.function("Win32Test.EnumWindows").write().unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "public readonly struct HWND",
        "public static bool EnumWindows(delegate* unmanaged[Stdcall]<Win32Test.HWND, nint, int> callback, nint lparam)",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
    for excluded in [
        "GetDesktopWindow",
        "GetWindowRect",
        "MAGIC",
        "class ICounter",
    ] {
        assert!(!source.contains(excluded), "unexpected `{excluded}`");
    }
}

#[test]
fn selection_win32_union_dependency() {
    let (builder, generated) = win32_selection_builder("selection_win32_union");
    builder
        .function("Win32Test.TransformNumber")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "[StructLayout(LayoutKind.Explicit)]",
        "public struct Number",
        "[FieldOffset(0)]",
        "public Win32Test.Point point;",
        "public static partial Win32Test.Number TransformNumber(Win32Test.Number value);",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
    assert!(!source.contains("GetCurrentProcessId"));
}

#[test]
fn selection_win32_nested_union_dependency() {
    let (builder, generated) = win32_selection_builder("selection_win32_nested_union");
    builder
        .function("Win32Test.TransformVariant")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "public struct Variant",
        "public Win32Test.Variant.Variant_1 data;",
        "public struct Variant_1",
        "[StructLayout(LayoutKind.Explicit)]",
        "public static partial Win32Test.Variant TransformVariant(Win32Test.Variant value);",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
    assert!(!source.contains("namespace  "));
}

#[test]
fn selection_win32_architecture_specific_layout() {
    let scratch = scratch("selection_win32_architecture");
    let winmd = author("win32_foundation", &scratch);

    for (architecture, target, field_type, excluded, layout) in [
        (
            windows_csharp::Architecture::X86,
            "x86",
            "uint",
            "ulong",
            "[StructLayout(LayoutKind.Sequential, Pack = 4)]",
        ),
        (
            windows_csharp::Architecture::X64,
            "x64",
            "ulong",
            "uint",
            "[StructLayout(LayoutKind.Sequential)]",
        ),
        (
            windows_csharp::Architecture::Arm64,
            "arm64",
            "ulong",
            "uint",
            "[StructLayout(LayoutKind.Sequential)]",
        ),
    ] {
        let generated = scratch.join(format!("{target}.cs"));
        windows_csharp::builder()
            .input(winmd.to_str().unwrap())
            .input(FOUNDATION)
            .architecture(architecture)
            .function("Win32Test.TransformArch")
            .member("Win32Test.INativeAbiCases", "GetFloatPair")
            .output(generated.to_str().unwrap())
            .fragment()
            .write()
            .unwrap();
        let source = std::fs::read_to_string(generated).unwrap();
        assert!(source.contains(layout));
        assert!(source.contains(&format!("public {field_type} value;")));
        assert!(!source.contains(&format!("public {excluded} value;")));
        assert_eq!(source.matches("public struct ArchValue").count(), 1);
        assert_eq!(source.matches("TransformArch(").count(), 1);
        assert!(source.contains("public Win32Test.FloatPair GetFloatPair()"));
        assert!(source.contains(
            "delegate* unmanaged<nint, Win32Test.FloatPair*, void>)(*(void***)self)[4])"
        ));
        assert!(!source.contains("delegate* unmanaged<nint, Win32Test.FloatPair>)"));
    }
}

#[test]
fn selection_real_native_com_record_returns() {
    let scratch = scratch("selection_real_native_com_record_returns");
    let input = Path::new("../../../libs/bindgen/default/Windows.Win32.winmd");

    for (architecture, target) in [
        (windows_csharp::Architecture::X86, "x86"),
        (windows_csharp::Architecture::X64, "x64"),
        (windows_csharp::Architecture::Arm64, "arm64"),
    ] {
        let generated = scratch.join(format!("{target}.cs"));
        windows_csharp::builder()
            .input(input.to_str().unwrap())
            .architecture(architecture)
            .member("Windows.Win32.ID2D1Bitmap", "GetSize")
            .member("Windows.Win32.ID2D1Bitmap", "GetPixelFormat")
            .member("Windows.Win32.ID3D12Device", "GetAdapterLuid")
            .output(generated.to_str().unwrap())
            .fragment()
            .write()
            .unwrap();
        let source = std::fs::read_to_string(generated).unwrap();
        for expected in [
            "public Windows.Win32.D2D_SIZE_F GetSize()",
            "delegate* unmanaged<nint, Windows.Win32.D2D_SIZE_F*, void>",
            "public Windows.Win32.D2D1_PIXEL_FORMAT GetPixelFormat()",
            "delegate* unmanaged<nint, Windows.Win32.D2D1_PIXEL_FORMAT*, void>",
            "public Windows.Win32.LUID GetAdapterLuid()",
            "delegate* unmanaged<nint, Windows.Win32.LUID*, void>",
        ] {
            assert!(
                source.contains(expected),
                "{target} real native COM projection omitted `{expected}`"
            );
        }
    }

    let d2d =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/d2d/mod.rs").unwrap();
    assert!(d2d.contains(
        "pub GetSize: unsafe extern \"system\" fn(*mut core::ffi::c_void, *mut \
         super::D2D_SIZE_F)"
    ));
    assert!(d2d.contains(
        "pub GetPixelFormat: unsafe extern \"system\" fn(*mut core::ffi::c_void, *mut \
         super::D2D1_PIXEL_FORMAT)"
    ));
    let d3d12 =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/d3d12/mod.rs").unwrap();
    assert!(d3d12.contains(
        "pub GetAdapterLuid: unsafe extern \"system\" fn(*mut core::ffi::c_void, *mut super::LUID)"
    ));
}

#[test]
fn selection_win32_com_return_marker() {
    let (builder, generated) = win32_selection_builder("selection_win32_com_return");
    builder
        .function("Win32Test.CreateStreamOnHGlobal")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    assert!(source.contains("public static Win32Test.IStream CreateStreamOnHGlobal"));
    assert!(source.contains("public sealed unsafe class IStream"));
    assert!(!source.contains("public void Read("));
}

#[test]
fn selection_win32_com_out_safety_shapes() {
    let (builder, generated) = win32_selection_builder("selection_win32_com_out_safety");
    builder
        .member("Win32Test.INativeAbiCases", "CreateCounter")
        .member("Win32Test.INativeAbiCases", "UpdateCounter")
        .member("Win32Test.INativeAbiCases", "TryCreateCounter")
        .member("Win32Test.INativeAbiCases", "ReservedCounter")
        .member("Win32Test.INativeAbiCases", "CreateCounterBuffer")
        .member("Win32Test.INativeAbiCases", "CreateTwoCounters")
        .member("Win32Test.INativeAbiCases", "CreateCounterWithStatus")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "public Win32Test.ICounter CreateCounter(int seed)",
        "public void UpdateCounter(nint* counter)",
        "public void TryCreateCounter(nint* counter)",
        "public void ReservedCounter(nint* counter)",
        "public void CreateCounterBuffer(nint* counters, uint count)",
        "public void CreateTwoCounters(nint* first, nint* second)",
        "public Win32Test.ICounter CreateCounterWithStatus(out uint status)",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
    assert!(!source.contains("INativeAbiCasesAbi"));
}

#[test]
fn selection_win32_interface() {
    let (builder, generated) = win32_selection_builder("selection_win32_interface");
    builder.select("Win32Test.IBufferOps").write().unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "public sealed unsafe class IBufferOps",
        "public uint Sum(ReadOnlySpan<byte> values)",
        "public void Fill(Span<uint> values)",
        "public unsafe partial interface IBufferOpsAbi",
        "uint Sum(byte* values, uint count);",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
    for excluded in ["class ICounter", "GetCurrentProcessId", "MAGIC"] {
        assert!(!source.contains(excluded), "unexpected `{excluded}`");
    }
}

#[test]
fn malformed_buffer_relationships_remain_raw() {
    let (builder, generated) = win32_selection_builder("malformed_buffer_relationships");
    builder
        .function("Win32Test.ValidBuffer")
        .function("Win32Test.NegativeBuffer")
        .function("Win32Test.OutOfRangeBuffer")
        .function("Win32Test.SelfRelativeBuffer")
        .function("Win32Test.NegativeByteBuffer")
        .function("Win32Test.OutOfRangeByteBuffer")
        .function("Win32Test.SelfByteBuffer")
        .function("Win32Test.NegativeConstantBuffer")
        .function("Win32Test.ValidConstantBuffer")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    assert!(source.contains("public static uint ValidBuffer(ReadOnlySpan<uint> values)"));
    for expected in [
        "public static partial uint NegativeBuffer(uint count, uint* values)",
        "public static partial uint OutOfRangeBuffer(uint count, uint* values)",
        "public static partial uint SelfRelativeBuffer(uint* values)",
        "public static partial uint NegativeByteBuffer(uint count, byte* values)",
        "public static partial uint OutOfRangeByteBuffer(uint count, byte* values)",
        "public static partial uint SelfByteBuffer(byte* values)",
        "public static partial uint NegativeConstantBuffer(uint* values)",
        "public static partial uint ValidConstantBuffer(uint* values)",
    ] {
        assert!(
            source.contains(expected),
            "malformed relationship did not retain `{expected}`"
        );
    }
}

#[test]
fn selection_win32_inherited_interface_dependencies() {
    let (builder, generated) = win32_selection_builder("selection_win32_inherited_interface");
    builder
        .select("Win32Test.IDerivedDependency")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "public sealed unsafe class IDerivedDependency",
        "public void UsePoint(Win32Test.Point value)",
        "public void UseMode(Win32Test.Mode value)",
        "public struct Point",
        "public enum Mode",
    ] {
        assert!(source.contains(expected), "missing `{expected}`");
    }
}

#[test]
fn selection_win32_interface_member() {
    let (builder, generated) = win32_selection_builder("selection_win32_interface_member");
    builder
        .member("Win32Test.IDerivedDependency", "UsePoint")
        .write()
        .unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    assert!(source.contains("public void UsePoint(Win32Test.Point value)"));
    assert!(source.contains("public struct Point"));
    assert!(!source.contains("UseMode"));
    assert!(!source.contains("IDerivedDependencyAbi"));
}

#[test]
fn selection_win32_constant() {
    let (builder, generated) = win32_selection_builder("selection_win32_constant");
    builder.constant("Win32Test.MAGIC").write().unwrap();
    let source = std::fs::read_to_string(generated).unwrap();

    assert!(source.contains("public const uint MAGIC = 305419896u;"));
    assert!(!source.contains("GetCurrentProcessId"));
}

#[test]
fn selection_win32_handle_root() {
    let (builder, generated) = win32_selection_builder("selection_win32_handle_root");
    builder.select("Win32Test.HWND").write().unwrap();
    let source = std::fs::read_to_string(generated).unwrap();
    assert!(source.contains("public readonly struct HWND"));
    assert!(!source.contains("GetDesktopWindow"));
}

#[test]
fn selection_unknown_win32_item_errors() {
    let (builder, _) = win32_selection_builder("selection_unknown_win32_function");
    let error = builder
        .function("Win32Test.DoesNotExist")
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);

    let (builder, _) = win32_selection_builder("selection_unknown_win32_constant");
    let error = builder
        .constant("Win32Test.DoesNotExist")
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn selection_wrong_win32_item_kind_errors() {
    let (builder, _) = win32_selection_builder("selection_function_wrong_kind");
    assert_eq!(
        builder
            .function("Win32Test.MAGIC")
            .write()
            .unwrap_err()
            .to_string(),
        "windows-csharp: selected function `Win32Test.MAGIC` has metadata kind `constant`, \
         expected `function`"
    );

    let (builder, _) = win32_selection_builder("selection_constant_wrong_kind");
    assert_eq!(
        builder
            .constant("Win32Test.GetWindowRect")
            .write()
            .unwrap_err()
            .to_string(),
        "windows-csharp: selected constant `Win32Test.GetWindowRect` has metadata kind `function`, \
         expected `constant`"
    );
}

#[test]
fn selection_unsupported_win32_item_errors() {
    let (builder, _) = win32_selection_builder("selection_unsupported_win32_function");
    let message = builder
        .function("Win32Test.UnsupportedString")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("unsupported shape `String`"), "{message}");

    let (builder, _) = win32_selection_builder("selection_unsupported_win32_constant");
    let message = builder
        .constant("Win32Test.FLOAT_MAGIC")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("unsupported literal kind"), "{message}");
}

#[test]
fn selection_architecture_unavailable_errors() {
    for (kind, message) in [
        (
            "function",
            "windows-csharp: selected function `Win32Test.OnlyX86` is unavailable on the selected \
             x64 architecture",
        ),
        (
            "constant",
            "windows-csharp: selected constant `Win32Test.X86_MAGIC` is unavailable on the selected \
             x64 architecture",
        ),
        (
            "type",
            "windows-csharp: selected type `Win32Test.X86Only` is unavailable on the selected x64 \
             architecture",
        ),
    ] {
        let (builder, _) = win32_selection_builder(&format!("selection_arch_{kind}"));
        let builder = builder.architecture(windows_csharp::Architecture::X64);
        let error = match kind {
            "function" => builder.function("Win32Test.OnlyX86").write(),
            "constant" => builder.constant("Win32Test.X86_MAGIC").write(),
            _ => builder.select("Win32Test.X86Only").write(),
        }
        .unwrap_err();
        assert_eq!(error.to_string(), message);
    }

    let (builder, _) = win32_selection_builder("selection_arch_dependency");
    let message = builder
        .architecture(windows_csharp::Architecture::X64)
        .function("Win32Test.UsesUnavailableType")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("Win32Test.X86Only"), "{message}");
    assert!(
        message.contains("unavailable on the selected x64 architecture"),
        "{message}"
    );
}

#[test]
fn selection_unsupported_win32_member_errors() {
    let (builder, _) = win32_selection_builder("selection_unsupported_win32_member");
    let error = builder
        .member("Win32Test.IUnsupported", "Text")
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("is not supported"));

    let (builder, _) = win32_selection_builder("selection_unsupported_win32_interface");
    let error = builder
        .select("Win32Test.IUnsupported")
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("is not supported"));

    let (builder, _) = win32_selection_builder("selection_unsupported_inherited_win32_member");
    let message = builder
        .member("Win32Test.IUnsupportedDerived", "Text")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("Text"), "{message}");
    assert!(message.contains("String"), "{message}");

    let (builder, _) = win32_selection_builder("selection_unsupported_inherited_win32_interface");
    let message = builder
        .select("Win32Test.IUnsupportedDerived")
        .write()
        .unwrap_err()
        .to_string();
    assert!(message.contains("Text"), "{message}");
    assert!(message.contains("String"), "{message}");
}

#[test]
fn selection_compiles() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch("selection_compiles");
    let winmd = author("selection", &scratch);
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .select("Selection.Widget")
        .member("Selection.IStandalone", "Ping")
        .output(project.join("Generated.cs").to_str().unwrap())
        .write()
        .unwrap();
    std::fs::write(project.join("project.csproj"), LIBRARY_CSPROJ).unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("build")
        .arg(project.to_str().unwrap())
        .output()
        .expect("failed to run dotnet build");

    assert!(
        output.status.success(),
        "exact-selection projection failed to compile\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn selection_win32_compiles() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch("selection_win32_compiles");
    let winmd = author("win32_foundation", &scratch);
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .select("Win32Test.IBufferOps")
        .function("Win32Test.GetWindowRect")
        .function("Win32Test.EnumWindows")
        .function("Win32Test.TransformNumber")
        .function("Win32Test.TransformVariant")
        .function("Win32Test.TransformArch")
        .constant("Win32Test.MAGIC")
        .output(project.join("Generated.cs").to_str().unwrap())
        .write()
        .unwrap();
    std::fs::write(project.join("project.csproj"), LIBRARY_CSPROJ).unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("build")
        .arg(project.to_str().unwrap())
        .output()
        .expect("failed to run dotnet build");

    assert!(
        output.status.success(),
        "Win32 exact-selection projection failed to compile\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

/// The direct WinUI language slice compiles from the same pinned Windows App SDK metadata used by
/// windows-reactor.
#[test]
fn winui_slice_compiles() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let project = scratch("winui_slice_compiles").join("project");
    std::fs::create_dir_all(&project).unwrap();
    let generated = project.join("Generated.cs");

    windows_csharp::builder()
        .input(WINUI)
        .input(REFERENCE)
        .input(FOUNDATION)
        .member("Microsoft.UI.Xaml.Application", "Start")
        .member("Microsoft.UI.Xaml.Window", "Content")
        .member("Microsoft.UI.Xaml.Window", "Activate")
        .member("Microsoft.UI.Xaml.Controls.Canvas", "SetLeft")
        .member("Microsoft.UI.Xaml.Controls.StackPanel", "Children")
        .member("Microsoft.UI.Xaml.FrameworkElement", "Width")
        .member("Microsoft.UI.Xaml.FrameworkElement", "Height")
        .member("Microsoft.UI.Xaml.Controls.TextBlock", "Text")
        .member("Microsoft.UI.Xaml.Controls.TextBlock", "Width")
        .member("Microsoft.UI.Xaml.Controls.TextBlock", "Height")
        .output(generated.to_str().unwrap())
        .write()
        .unwrap();

    let source = std::fs::read_to_string(&generated).unwrap();
    assert!(
        source.len() < 180_000,
        "selected WinUI projection unexpectedly grew to {} bytes",
        source.len()
    );
    for expected in [
        "public static void Start(",
        "public Window()",
        "public Microsoft.UI.Xaml.UIElement? Content",
        "public void Activate()",
        "public static void SetLeft<T0>",
        "public sealed unsafe class IVector<T>",
        "IObjectParameter<Microsoft.UI.Xaml.UIElement._Parameter>",
        "public TextBlock()",
        "public string Text",
        "public double Width",
        "public void BorrowAs(Microsoft.UI.Xaml.FrameworkElement.BorrowAction action)",
        "using WindowsCsharp.ComLease source = Acquire();",
        "InterfaceLease.From(source.Handle, Microsoft.UI.Xaml.FrameworkElement.Iid)",
    ] {
        assert!(
            source.contains(expected),
            "missing WinUI surface `{expected}`"
        );
    }

    std::fs::write(
        project.join("Use.cs"),
        r#"
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

internal static class Use
{
    internal static void Position(TextBlock item)
    {
        Canvas.SetLeft(item, 1);
        item.BorrowAs(static (FrameworkElement.Borrowed element) =>
        {
            element.Width = 2;
            element.Height = 3;
        });
    }
}
"#,
    )
    .unwrap();
    std::fs::write(project.join("project.csproj"), LIBRARY_CSPROJ).unwrap();
    let output = std::process::Command::new("dotnet.exe")
        .arg("build")
        .arg(project.to_str().unwrap())
        .output()
        .expect("failed to run dotnet build");

    assert!(
        output.status.success(),
        "WinUI slice failed to compile\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

/// WinUI collection runtime classes whose default interface is a closed generic use that
/// interface's parameterized IID. The same projection also treats the WinRT
/// `Windows.Foundation.HResult` spelling in reactor's bootstrap metadata as HRESULT.
#[test]
fn winui_generic_default_collections_and_bootstrap_compile() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let project =
        scratch("winui_generic_default_collections_and_bootstrap_compile").join("project");
    std::fs::create_dir_all(&project).unwrap();
    let generated = project.join("Generated.cs");

    windows_csharp::builder()
        .input(WINUI)
        .input(REFERENCE)
        .input(FOUNDATION)
        .member("Microsoft.UI.Xaml.Controls.Grid", "RowDefinitions")
        .member("Microsoft.UI.Xaml.Controls.Grid", "ColumnDefinitions")
        .member("Microsoft.UI.Xaml.Controls.RowDefinition", "Height")
        .member("Microsoft.UI.Xaml.Controls.ColumnDefinition", "Width")
        .function("extras.MddBootstrapInitialize2")
        .function("extras.MddBootstrapShutdown")
        .constant("extras.WINDOWSAPPSDK_RELEASE_MAJORMINOR")
        .constant("extras.WINDOWSAPPSDK_RUNTIME_VERSION_UINT64")
        .output(generated.to_str().unwrap())
        .write()
        .unwrap();

    let source = std::fs::read_to_string(&generated).unwrap();
    for expected in [
        "class RowDefinitionCollection",
        "new Guid(0x5ddd9577, 0x3f94, 0x567f, 0xbe, 0xef, 0x54, 0x05, 0x68, 0x52, 0x22, 0x89)",
        "class ColumnDefinitionCollection",
        "new Guid(0x749bc47c, 0x1743, 0x5c21, 0x9c, 0xed, 0xc8, 0xa1, 0x13, 0x4c, 0x7b, 0xa7)",
        "public static void MddBootstrapInitialize2(",
        "WindowsCsharp.Com.Check(MddBootstrapInitialize2Abi(",
    ] {
        assert!(
            source.contains(expected),
            "missing WinUI collection/bootstrap surface `{expected}`"
        );
    }
    assert!(
        !source.contains("public struct HResult"),
        "bootstrap HRESULT spelling leaked as a value struct"
    );

    std::fs::write(
        project.join("Use.cs"),
        r#"
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Windows.Foundation.Collections;
using Windows.Win32;

internal static unsafe class Use
{
    internal static void Configure(Grid grid)
    {
        GridLength star = new() { Value = 1, GridUnitType = GridUnitType.Star };
        using RowDefinitionCollection rowCollection = grid.RowDefinitions!;
        using IVector<RowDefinition?> rows = rowCollection.As<IVector<RowDefinition?>>();
        using RowDefinition row = new() { Height = star };
        rows.Append(row);

        using ColumnDefinitionCollection columnCollection = grid.ColumnDefinitions!;
        using IVector<ColumnDefinition?> columns =
            columnCollection.As<IVector<ColumnDefinition?>>();
        using ColumnDefinition column = new() { Width = star };
        columns.Append(column);
    }

    internal static void BootstrapSignature()
    {
        PACKAGE_VERSION version = new()
        {
            Anonymous = new PACKAGE_VERSION.PACKAGE_VERSION_0
            {
                Version = extras.Apis.WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
            },
        };
        extras.Apis.MddBootstrapInitialize2(
            (uint)extras.Apis.WINDOWSAPPSDK_RELEASE_MAJORMINOR,
            null,
            version,
            extras.MddBootstrapInitializeOptions.MddBootstrapInitializeOptions_None);
        extras.Apis.MddBootstrapShutdown();
    }
}
"#,
    )
    .unwrap();
    std::fs::write(project.join("project.csproj"), LIBRARY_CSPROJ).unwrap();
    let output = std::process::Command::new("dotnet.exe")
        .arg("build")
        .arg(project.to_str().unwrap())
        .output()
        .expect("failed to run dotnet build");

    assert!(
        output.status.success(),
        "WinUI generic-default/bootstrap projection failed to compile\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

/// Authors `input/activation.rdl` and returns the whole-namespace projection fragment, exercising
/// default activation, a typed custom factory, a composable factory with `outer`/`inner`, and a
/// static interface.
fn activation_source(scratch_name: &str) -> String {
    let scratch = scratch(scratch_name);
    let winmd = author("activation", &scratch);
    let generated = scratch.join("generated.cs");
    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .filter("Activation")
        .output(generated.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();
    std::fs::read_to_string(&generated).unwrap()
}

/// The activation model projects default activation, custom/composable factory constructors, and
/// static members from metadata attributes, using the real factory/static interface IIDs and the
/// metadata vtable slots (from 6).
#[test]
fn activation_projection() {
    let source = activation_source("activation_projection");

    // Default activation (`#[Activatable(1)]`) yields a parameterless constructor through the
    // shared registration-free activation path.
    assert!(
        source.contains(
            "public Widget() : base(WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, \"Activation.Widget\", Iid), Iid) {}"
        ),
        "missing default-activation constructor in:\n{source}"
    );

    // A typed custom factory (`#[Activatable(IWidgetFactory, 1)]`) yields a public constructor whose
    // parameter is the factory method's parameter, and the returned pointer is handed to the safe
    // `ComObject` base as the class default interface (`base(..., Iid)`).
    assert!(
        source.contains("public Widget(int seed) : base(FactoryCreate0(seed), Iid) {}"),
        "missing custom-factory constructor in:\n{source}"
    );
    for expected in [
        "public Widget(Activation.Options options) : base(FactoryCreate1(options), Iid) {}",
        "_abi0 = Activation.OptionsAbi.FromSurface(options);",
        "_abi0.Dispose();",
        "public static Activation.Options Normalize(Activation.Options options)",
        "Activation.OptionsAbi result = default;",
        "return result.ToSurface();",
        "result.Dispose();",
    ] {
        assert!(
            source.contains(expected),
            "owned struct factory/static path omitted `{expected}`"
        );
    }

    // The factory is acquired by its own metadata IID (not the class IID) through a `FactoryLease`.
    assert!(
        source.contains(
            "private static readonly Guid s_factory0_iid = new Guid(0xd5c218ec, 0xdcb1, 0x528f, 0x97, 0x0a, 0xaa, 0xb9, 0x82, 0x3e, 0xbd, 0xb9);"
        ),
        "custom factory did not use the real IWidgetFactory IID in:\n{source}"
    );
    assert!(
        source.contains(
            "WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_factory0, \"Activation.Widget\", s_factory0_iid)"
        ),
        "custom factory did not acquire its factory interface by IID in:\n{source}"
    );

    // The factory creation method is the interface's first slot after `IInspectable` (slot 6).
    assert!(
        source.contains("(*(void***)self)[6])(self, seed, &_instance)"),
        "custom factory create did not call vtable slot 6 in:\n{source}"
    );

    // A composable factory (`#[Composable(IControlFactory, Public, 1)]`) drops the trailing
    // `outer`/`inner` ABI parameters from the public constructor and passes null for both
    // (non-aggregating construction), calling the composition slot 6.
    assert!(
        source.contains("public Control() : base(FactoryCreate0(), Iid) {}"),
        "missing composable-factory constructor in:\n{source}"
    );
    assert!(
        source.contains(
            "(delegate* unmanaged<nint, nint, nint, nint*, int>)(*(void***)self)[6])(self, 0, 0, &_instance)"
        ),
        "composable factory did not pass null outer/inner at slot 6 in:\n{source}"
    );

    // A static interface (`#[Static(IWidgetStatics, 1)]`) projects as static members reached through
    // the class's activation factory, marshalling with the same adapters as instance members.
    assert!(
        source.contains("public static int Count"),
        "missing static property `Count` in:\n{source}"
    );
    assert!(
        source.contains("public static void Start(Activation.StartCallback? callback)"),
        "missing static method `Start` in:\n{source}"
    );
    assert!(
        source.contains(
            "WindowsCsharp.WinRT.GetActivationFactory(ref s_module, ref s_static0, \"Activation.Widget\", s_static0_iid)"
        ),
        "static member did not acquire its static interface by IID in:\n{source}"
    );
    // The static method's callback argument marshals as a call-scoped object lease, at slot 8.
    assert!(
        source.contains(
            "using WindowsCsharp.ComLease _olease0 = WindowsCsharp.ComLease.From(callback);"
        ),
        "static method did not marshal its object parameter through a lease in:\n{source}"
    );
    assert!(
        source.contains("(*(void***)self)[8])(self, _olease0.Handle)"),
        "static method did not call its metadata vtable slot 8 in:\n{source}"
    );

    // The consumed factory/static interfaces are inlined into the class, not projected standalone.
    for absent in [
        "class IWidgetFactory",
        "class IWidgetStatics",
        "class IControlFactory",
    ] {
        assert!(
            !source.contains(absent),
            "consumed factory/static interface `{absent}` leaked as a standalone type in:\n{source}"
        );
    }
}

/// Selecting a class pulls in its activation metadata (constructors are always emitted) and its
/// static surface members participate in selection and dependency closure, while an unknown member
/// still errors.
#[test]
fn activation_selection() {
    let scratch = scratch("activation_selection");
    let winmd = author("activation", &scratch);

    // Selecting only the class projects its constructors even though no member was named, and pulls
    // the factory parameter and static surface's referenced types into the closure.
    let whole = scratch.join("whole.cs");
    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .select("Activation.Widget")
        .output(whole.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();
    let source = std::fs::read_to_string(&whole).unwrap();
    assert!(
        source.contains("public Widget()") && source.contains("public Widget(int seed)"),
        "selecting the class did not emit its constructors in:\n{source}"
    );
    assert!(
        source.contains("public static void Start"),
        "selecting the class did not emit its static members in:\n{source}"
    );
    // `Start` names `StartCallback`, so the delegate is pulled into the dependency closure.
    assert!(
        source.contains("class StartCallback"),
        "static member's referenced delegate was not pulled into the closure in:\n{source}"
    );

    // A static surface member can be selected by name, narrowing the static surface like an
    // instance member while the always-emitted constructors remain.
    let member = scratch.join("member.cs");
    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .member("Activation.Widget", "Start")
        .output(member.to_str().unwrap())
        .fragment()
        .write()
        .unwrap();
    let source = std::fs::read_to_string(&member).unwrap();
    assert!(
        source.contains("public static void Start"),
        "selecting a static member did not emit it in:\n{source}"
    );
    assert!(
        !source.contains("public static int Count"),
        "narrowing to `Start` leaked the unselected static member `Count` in:\n{source}"
    );

    // Selecting an unknown member still errors, having searched the static surface too.
    let error = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .member("Activation.Widget", "DoesNotExist")
        .output(scratch.join("err.cs").to_str().unwrap())
        .fragment()
        .write()
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

/// The activation projection (default, custom, composable, and static surfaces) compiles standalone
/// with `dotnet`.
#[test]
fn activation_compiles() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    let scratch = scratch("activation_compiles");
    let winmd = author("activation", &scratch);
    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();

    windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .filter("Activation")
        .output(project.join("Generated.cs").to_str().unwrap())
        .write()
        .unwrap();
    std::fs::write(project.join("project.csproj"), LIBRARY_CSPROJ).unwrap();

    let output = std::process::Command::new("dotnet.exe")
        .arg("build")
        .arg(project.to_str().unwrap())
        .output()
        .expect("failed to run dotnet build");

    assert!(
        output.status.success(),
        "activation projection failed to compile\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

const DELEGATE_MARSHALLING_PROGRAM: &str = r#"using System;
using System.Runtime.InteropServices;
using System.Threading;
using DelegateMarshalling;

internal static unsafe class Program
{
    private static nint* s_peerVtable;

    [StructLayout(LayoutKind.Sequential)]
    private struct PeerInstance
    {
        public nint Vtable;
        public int References;
        public int AddRefs;
        public int Releases;
        public int Id;
    }

    private static void Main()
    {
        s_peerVtable = (nint*)NativeMemory.AllocZeroed(8, (nuint)sizeof(nint));
        s_peerVtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&PeerQueryInterface;
        s_peerVtable[1] = (nint)(delegate* unmanaged<nint, uint>)&PeerAddRef;
        s_peerVtable[2] = (nint)(delegate* unmanaged<nint, uint>)&PeerRelease;
        s_peerVtable[6] = (nint)(delegate* unmanaged<nint, int*, int>)&PeerGetId;
        s_peerVtable[7] = (nint)(delegate* unmanaged<nint, int, int>)&PeerSetId;

        IDelegatePeer peer = CreatePeer(42, out PeerInstance* peerNative);
        try
        {
            long scalarAllocated = TestScalar();
            (long nullStringAllocated, long copiedStringAllocated) = TestStrings();
            long objectAllocated = TestObjects(peer, peerNative);

            Console.WriteLine(
                $"delegate allocations: scalar={scalarAllocated} B, " +
                $"null-string={nullStringAllocated} B, " +
                $"copied-string={copiedStringAllocated} B, object={objectAllocated} B");
        }
        finally
        {
            peer.Dispose();
            Check(peerNative->References == 0, "peer owner did not release its reference");
            NativeMemory.Free(peerNative);
            NativeMemory.Free(s_peerVtable);
        }
    }

    private static long TestScalar()
    {
        using ScalarCallback callback = ScalarCallback.Create(static value => value + 1);
        int result = 0;
        Check(InvokeScalar(callback, 41, &result) == 0 && result == 42, "scalar callback");

        for (int i = 0; i < 32; i++)
        {
            Check(InvokeScalar(callback, i, &result) == 0, "scalar warmup");
        }

        long before = GC.GetAllocatedBytesForCurrentThread();
        for (int i = 0; i < 10_000; i++)
        {
            int hr = InvokeScalar(callback, i, &result);
            Check(hr == 0 && result == i + 1, "scalar repeated callback");
        }
        long allocated = GC.GetAllocatedBytesForCurrentThread() - before;
        Check(allocated == 0, $"scalar callback allocated {allocated} bytes");
        return allocated;
    }

    private static (long NullAllocated, long CopiedAllocated) TestStrings()
    {
        using (StringCallback callback =
            StringCallback.Create(static value => value + "-return"))
        {
            nint input = WindowsCsharp.Interop.CreateString("alpha");
            nint result = 0;
            try
            {
                Check(InvokeString(callback, input, &result) == 0, "string callback HRESULT");
                Check(
                    WindowsCsharp.Interop.FromHstringBorrowed(input) == "alpha",
                    "string callback consumed its borrowed input");
                Check(
                    WindowsCsharp.Interop.FromHstringBorrowed(result) == "alpha-return",
                    "string callback return");
            }
            finally
            {
                WindowsCsharp.Interop.DeleteHstring(ref result);
                WindowsCsharp.Interop.DeleteHstring(ref input);
            }
        }

        using (StringCallback callback = StringCallback.Create(static value =>
        {
            Check(value.Length == 0, "null HSTRING did not project as empty");
            return null!;
        }))
        {
            nint result = 1;
            Check(InvokeString(callback, 0, &result) == 0, "null string callback HRESULT");
            Check(result == 0, "null managed string did not return a null HSTRING");

            for (int i = 0; i < 32; i++)
            {
                Check(InvokeString(callback, 0, &result) == 0 && result == 0, "null warmup");
            }
            long before = GC.GetAllocatedBytesForCurrentThread();
            for (int i = 0; i < 10_000; i++)
            {
                int hr = InvokeString(callback, 0, &result);
                Check(hr == 0 && result == 0, "null string repeated callback");
            }
            long allocated = GC.GetAllocatedBytesForCurrentThread() - before;
            Check(allocated == 0, $"null string callback allocated {allocated} bytes");

            using StringCallback copied = StringCallback.Create(static _ => null!);
            nint input = WindowsCsharp.Interop.CreateString("copy");
            try
            {
                for (int i = 0; i < 32; i++)
                {
                    Check(InvokeString(copied, input, &result) == 0, "copied string warmup");
                }
                before = GC.GetAllocatedBytesForCurrentThread();
                for (int i = 0; i < 1_000; i++)
                {
                    int hr = InvokeString(copied, input, &result);
                    Check(hr == 0 && result == 0, "copied string repeated callback");
                }
                long copiedAllocated = GC.GetAllocatedBytesForCurrentThread() - before;
                Check(copiedAllocated > 0, "non-empty string callback did not copy its input");

                using StringCallback failing =
                    StringCallback.Create(static _ => throw new InvalidOperationException("fail"));
                result = 123;
                Check(InvokeString(failing, input, &result) < 0, "string exception HRESULT");
                Check(result == 0, "failed string callback retained a return owner");
                Check(
                    WindowsCsharp.Interop.FromHstringBorrowed(input) == "copy",
                    "failed string callback consumed its borrowed input");
                return (allocated, copiedAllocated);
            }
            finally
            {
                WindowsCsharp.Interop.DeleteHstring(ref input);
            }
        }
    }

    private static long TestObjects(IDelegatePeer peer, PeerInstance* peerNative)
    {
        using ObjectCallback callback = ObjectCallback.Create((IDelegatePeer.Borrowed value) =>
        {
            Check(!value.IsNull, "non-null object projected as null");
            Check(value.Id == 42, "borrowed object callback value");
            return peer;
        });

        int beforeReferences = peerNative->References;
        int beforeAddRefs = peerNative->AddRefs;
        int beforeReleases = peerNative->Releases;
        nint result = 123;
        Check(
            InvokeObject(callback, (nint)peerNative, &result) == 0,
            "object callback HRESULT");
        Check(result == (nint)peerNative, "object callback return pointer");
        Check(
            peerNative->AddRefs == beforeAddRefs + 1,
            "object callback did not transfer exactly one owned reference");
        Check(
            peerNative->Releases == beforeReleases,
            "object input released caller-owned reference");
        Check(
            peerNative->References == beforeReferences + 1,
            "object return reference count");
        _ = PeerReleaseCore(result);
        Check(peerNative->References == beforeReferences, "native object return release");

        using (ObjectCallback nullCallback =
            ObjectCallback.Create(static (IDelegatePeer.Borrowed value) =>
            {
                Check(value.IsNull, "null object input was not observable");
                return null;
            }))
        {
            result = 123;
            Check(InvokeObject(nullCallback, 0, &result) == 0, "null object callback HRESULT");
            Check(result == 0, "null object return was not null");
        }

        using (ObjectCallback failing =
            ObjectCallback.Create(static _ => throw new InvalidOperationException("fail")))
        {
            result = 123;
            beforeAddRefs = peerNative->AddRefs;
            Check(
                InvokeObject(failing, (nint)peerNative, &result) < 0,
                "object exception HRESULT");
            Check(result == 0, "failed object callback retained a return owner");
            Check(peerNative->AddRefs == beforeAddRefs, "failed callback added a reference");
        }

        IDelegatePeer disposed = CreatePeer(7, out PeerInstance* disposedNative);
        disposed.Dispose();
        using (ObjectCallback disposedReturn = ObjectCallback.Create(_ => disposed))
        {
            result = 123;
            beforeAddRefs = disposedNative->AddRefs;
            Check(
                InvokeObject(disposedReturn, (nint)peerNative, &result) < 0,
                "disposed object conversion HRESULT");
            Check(result == 0, "disposed object conversion retained a return owner");
            Check(
                disposedNative->AddRefs == beforeAddRefs,
                "disposed object conversion added a reference");
        }
        Check(disposedNative->References == 0, "disposed test object reference count");
        NativeMemory.Free(disposedNative);

        ObjectCallback? reentrant = null;
        reentrant = ObjectCallback.Create((IDelegatePeer.Borrowed _) =>
        {
            reentrant!.Dispose();
            return peer;
        });
        result = 0;
        Check(
            InvokeObject(reentrant, (nint)peerNative, &result) == 0,
            "reentrant callback disposal");
        Check(result == (nint)peerNative, "reentrant object return");
        _ = PeerReleaseCore(result);
        reentrant.Dispose();

        for (int i = 0; i < 32; i++)
        {
            Check(
                InvokeObject(callback, (nint)peerNative, &result) == 0,
                "object warmup");
            _ = PeerReleaseCore(result);
        }
        long before = GC.GetAllocatedBytesForCurrentThread();
        for (int i = 0; i < 10_000; i++)
        {
            int hr = InvokeObject(callback, (nint)peerNative, &result);
            Check(hr == 0 && result == (nint)peerNative, "object repeated callback");
            _ = PeerReleaseCore(result);
        }
        long allocated = GC.GetAllocatedBytesForCurrentThread() - before;
        Check(allocated == 0, $"object callback allocated {allocated} bytes");
        return allocated;
    }

    private static int InvokeScalar(ScalarCallback callback, int value, int* result)
    {
        using WindowsCsharp.ComLease lease = callback.Acquire();
        nint self = lease.Handle;
        _ = WindowsCsharp.Com.AddRef(self);
        try
        {
            return ((delegate* unmanaged<nint, int, int*, int>)(*(void***)self)[3])(
                self,
                value,
                result);
        }
        finally
        {
            _ = WindowsCsharp.Com.Release(self);
        }
    }

    private static int InvokeString(StringCallback callback, nint value, nint* result)
    {
        using WindowsCsharp.ComLease lease = callback.Acquire();
        nint self = lease.Handle;
        _ = WindowsCsharp.Com.AddRef(self);
        try
        {
            return ((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[3])(
                self,
                value,
                result);
        }
        finally
        {
            _ = WindowsCsharp.Com.Release(self);
        }
    }

    private static int InvokeObject(ObjectCallback callback, nint value, nint* result)
    {
        using WindowsCsharp.ComLease lease = callback.Acquire();
        nint self = lease.Handle;
        _ = WindowsCsharp.Com.AddRef(self);
        try
        {
            return ((delegate* unmanaged<nint, nint, nint*, int>)(*(void***)self)[3])(
                self,
                value,
                result);
        }
        finally
        {
            _ = WindowsCsharp.Com.Release(self);
        }
    }

    private static IDelegatePeer CreatePeer(int id, out PeerInstance* instance)
    {
        instance = (PeerInstance*)NativeMemory.AllocZeroed((nuint)sizeof(PeerInstance));
        instance->Vtable = (nint)s_peerVtable;
        instance->References = 1;
        instance->Id = id;
        return new IDelegatePeer((nint)instance);
    }

    [UnmanagedCallersOnly]
    private static int PeerQueryInterface(nint self, Guid* iid, nint* result)
    {
        if (result == null)
        {
            return unchecked((int)0x80004003);
        }
        *result = self;
        _ = PeerAddRefCore(self);
        return 0;
    }

    [UnmanagedCallersOnly]
    private static uint PeerAddRef(nint self) => PeerAddRefCore(self);

    private static uint PeerAddRefCore(nint self)
    {
        PeerInstance* instance = (PeerInstance*)self;
        _ = Interlocked.Increment(ref instance->AddRefs);
        return (uint)Interlocked.Increment(ref instance->References);
    }

    [UnmanagedCallersOnly]
    private static uint PeerRelease(nint self) => PeerReleaseCore(self);

    private static uint PeerReleaseCore(nint self)
    {
        PeerInstance* instance = (PeerInstance*)self;
        _ = Interlocked.Increment(ref instance->Releases);
        return (uint)Interlocked.Decrement(ref instance->References);
    }

    [UnmanagedCallersOnly]
    private static int PeerGetId(nint self, int* value)
    {
        if (value == null)
        {
            return unchecked((int)0x80004003);
        }
        *value = ((PeerInstance*)self)->Id;
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int PeerSetId(nint self, int value)
    {
        ((PeerInstance*)self)->Id = value;
        return 0;
    }

    private static void Check(bool condition, string message)
    {
        if (!condition)
        {
            throw new InvalidOperationException(message);
        }
    }
}
"#;

const ASYNC_PROGRAM: &str = r#"using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Windows.Foundation;

internal static unsafe class Program
{
    private static readonly Guid s_iunknown = new(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
    private static readonly Guid s_iinspectable = new(0xaf86e2e0, 0xb12d, 0x4c6a, 0x9c, 0x5a, 0xd7, 0xaa, 0x65, 0x10, 0x1e, 0x90);
    private static readonly Guid s_iagile = new(0x94ea2b94, 0xe9cc, 0x49e0, 0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90);
    private static readonly Guid s_asyncInfo = new(0x00000036, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
    private static readonly Guid s_operationInt = new(0x968b9665, 0x06ed, 0x5774, 0x8f, 0x53, 0x8e, 0xde, 0xab, 0xd5, 0xf7, 0xb5);
    private static readonly Guid s_completedInt = new(0xd60cae9d, 0x88cb, 0x59f1, 0x85, 0x76, 0x3f, 0xba, 0x44, 0x79, 0x6b, 0xe8);
    private static readonly nint* s_operationVtable = BuildOperationVtable();
    private static readonly nint* s_infoVtable = BuildInfoVtable();
    private static int s_liveInstances;

    [StructLayout(LayoutKind.Sequential)]
    private struct Instance
    {
        public nint OperationVtable;
        public nint InfoVtable;
        public int References;
        public int Status;
        public int Result;
        public nint Completed;
        public int StatusCalls;
        public int CallbackCalls;
        public int LastCallbackHr;
        public int InvokeTwice;
        public int CompleteAfterStatus;
    }

    private static int Main()
    {
        Instance* pendingNative;
        IAsyncOperation<int> pending = Create(0, 42, false, out pendingNative);
        Check(AsyncHelpers.AwaitAndDispose(pending).GetAwaiter().GetResult() == 42, "pending await result");
        WaitForObservation(pendingNative);
        Check(pendingNative->StatusCalls == 1, "pending await polled status");
        Check(pendingNative->CallbackCalls == 1, "pending callback count");
        ReleaseObservation(pendingNative);

        Instance* readyNative;
        IAsyncOperation<int> ready = Create(1, 84, false, out readyNative);
        Check(AsyncHelpers.AwaitAndDispose(ready).GetAwaiter().GetResult() == 84, "ready await result");
        Check(readyNative->StatusCalls == 1, "ready await status count");
        Check(readyNative->CallbackCalls == 0, "ready await unexpectedly registered");
        ReleaseObservation(readyNative);

        Check(
            AsyncHelpers.AwaitAndDispose(CreateUnobserved(-1, 63, false)).GetAwaiter().GetResult() ==
                63,
            "inline race await result");
        Check(Volatile.Read(ref s_liveInstances) == 0, "inline race operation leaked");

        Instance* duplicateNative;
        using (IAsyncOperation<int> duplicate = Create(1, 7, true, out duplicateNative))
        {
            int continuations = 0;
            using ManualResetEventSlim signal = new();
            IAsyncOperation<int>.Awaiter awaiter = duplicate.GetAwaiter();
            awaiter.UnsafeOnCompleted(() =>
            {
                _ = Interlocked.Increment(ref continuations);
                signal.Set();
            });
            Check(signal.IsSet, "ready registration was not synchronous");
            Check(continuations == 1, "continuation ran more than once");
            Check(duplicateNative->CallbackCalls == 2, "fake did not invoke twice");
            Check(awaiter.GetResult() == 7, "ready registration result");
        }
        ReleaseObservation(duplicateNative);

        Instance* errorNative;
        using (IAsyncOperation<int> error = Create(1, 9, false, out errorNative))
        {
            IAsyncOperation<int>.Awaiter awaiter = error.GetAwaiter();
            awaiter.UnsafeOnCompleted(static () => throw new InvalidOperationException("callback"));
            Check(errorNative->LastCallbackHr < 0, "callback exception crossed the ABI");
            Check(awaiter.GetResult() == 9, "exception callback result");
        }
        ReleaseObservation(errorNative);

        Instance* prematureNative;
        using (IAsyncOperation<int> premature = Create(0, 0, false, out prematureNative))
        {
            try
            {
                _ = premature.GetAwaiter().GetResult();
                throw new InvalidOperationException("premature GetResult succeeded");
            }
            catch (COMException error) when (error.HResult == unchecked((int)0x8000000e))
            {
            }
        }
        ReleaseObservation(prematureNative);
        Check(Volatile.Read(ref s_liveInstances) == 0, "fake operation leaked");
        return 0;
    }

    private static IAsyncOperation<int> Create(
        int status,
        int result,
        bool invokeTwice,
        out Instance* instance)
    {
        return CreateCore(status, result, invokeTwice, 2, out instance);
    }

    private static IAsyncOperation<int> CreateUnobserved(
        int status,
        int result,
        bool invokeTwice)
    {
        Instance* instance;
        return CreateCore(status, result, invokeTwice, 1, out instance);
    }

    private static IAsyncOperation<int> CreateCore(
        int status,
        int result,
        bool invokeTwice,
        int references,
        out Instance* instance)
    {
        instance = (Instance*)NativeMemory.AllocZeroed((nuint)sizeof(Instance));
        instance->OperationVtable = (nint)s_operationVtable;
        instance->InfoVtable = (nint)s_infoVtable;
        instance->References = references;
        instance->Status = status < 0 ? 0 : status;
        instance->Result = result;
        instance->InvokeTwice = invokeTwice ? 1 : 0;
        instance->CompleteAfterStatus = status < 0 ? 1 : 0;
        _ = Interlocked.Increment(ref s_liveInstances);
        return new IAsyncOperation<int>((nint)instance);
    }

    private static nint* BuildOperationVtable()
    {
        nint* vtable = (nint*)NativeMemory.AllocZeroed(9, (nuint)sizeof(nint));
        vtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&OperationQueryInterface;
        vtable[1] = (nint)(delegate* unmanaged<nint, uint>)&OperationAddRef;
        vtable[2] = (nint)(delegate* unmanaged<nint, uint>)&OperationRelease;
        vtable[6] = (nint)(delegate* unmanaged<nint, nint, int>)&PutCompleted;
        vtable[7] = (nint)(delegate* unmanaged<nint, nint*, int>)&GetCompleted;
        vtable[8] = (nint)(delegate* unmanaged<nint, int*, int>)&GetResults;
        return vtable;
    }

    private static nint* BuildInfoVtable()
    {
        nint* vtable = (nint*)NativeMemory.AllocZeroed(11, (nuint)sizeof(nint));
        vtable[0] = (nint)(delegate* unmanaged<nint, Guid*, nint*, int>)&InfoQueryInterface;
        vtable[1] = (nint)(delegate* unmanaged<nint, uint>)&InfoAddRef;
        vtable[2] = (nint)(delegate* unmanaged<nint, uint>)&InfoRelease;
        vtable[7] = (nint)(delegate* unmanaged<nint, int*, int>)&Status;
        return vtable;
    }

    [UnmanagedCallersOnly]
    private static int OperationQueryInterface(nint self, Guid* iid, nint* result) =>
        QueryInterface((Instance*)self, iid, result);

    private static int QueryInterface(Instance* instance, Guid* iid, nint* result)
    {
        if (result == null)
        {
            return unchecked((int)0x80004003);
        }
        if (*iid == s_asyncInfo)
        {
            *result = (nint)(&instance->InfoVtable);
        }
        else if (*iid == s_operationInt || *iid == s_iunknown || *iid == s_iinspectable ||
                 *iid == s_iagile)
        {
            *result = (nint)instance;
        }
        else
        {
            *result = 0;
            return unchecked((int)0x80004002);
        }
        _ = AddRef(instance);
        return 0;
    }

    [UnmanagedCallersOnly]
    private static uint OperationAddRef(nint self) => AddRef((Instance*)self);

    [UnmanagedCallersOnly]
    private static uint OperationRelease(nint self) => Release((Instance*)self);

    [UnmanagedCallersOnly]
    private static int InfoQueryInterface(nint self, Guid* iid, nint* result)
    {
        Instance* instance = FromInfo(self);
        return QueryInterface(instance, iid, result);
    }

    [UnmanagedCallersOnly]
    private static uint InfoAddRef(nint self) => AddRef(FromInfo(self));

    [UnmanagedCallersOnly]
    private static uint InfoRelease(nint self) => Release(FromInfo(self));

    [UnmanagedCallersOnly]
    private static int Status(nint self, int* value)
    {
        if (value == null)
        {
            return unchecked((int)0x80004003);
        }
        Instance* instance = FromInfo(self);
        _ = Interlocked.Increment(ref instance->StatusCalls);
        *value = Volatile.Read(ref instance->Status);
        if (Interlocked.Exchange(ref instance->CompleteAfterStatus, 0) != 0)
        {
            Volatile.Write(ref instance->Status, 1);
        }
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int PutCompleted(nint self, nint handler)
    {
        if (handler == 0)
        {
            return unchecked((int)0x80004003);
        }
        int hr = QueryAndRelease(handler, s_completedInt);
        if (hr < 0)
        {
            return hr;
        }
        hr = QueryAndRelease(handler, s_iunknown);
        if (hr < 0)
        {
            return hr;
        }
        hr = QueryAndRelease(handler, s_iagile);
        if (hr < 0)
        {
            return hr;
        }

        Instance* instance = (Instance*)self;
        _ = CallbackAddRef(handler);
        if (Interlocked.CompareExchange(ref instance->Completed, handler, 0) != 0)
        {
            _ = CallbackRelease(handler);
            return unchecked((int)0x80000018);
        }

        if (Volatile.Read(ref instance->Status) != 0)
        {
            InvokeCompletion(instance);
        }
        else
        {
            _ = AddRef(instance);
            _ = ThreadPool.QueueUserWorkItem(static state =>
            {
                Instance* value = (Instance*)(nint)state!;
                Thread.Sleep(20);
                Volatile.Write(ref value->Status, 1);
                InvokeCompletion(value);
                _ = Release(value);
            }, (nint)instance);
        }
        return 0;
    }

    [UnmanagedCallersOnly]
    private static int GetCompleted(nint self, nint* value)
    {
        if (value != null)
        {
            *value = 0;
        }
        return unchecked((int)0x80004001);
    }

    [UnmanagedCallersOnly]
    private static int GetResults(nint self, int* value)
    {
        if (value == null)
        {
            return unchecked((int)0x80004003);
        }
        Instance* instance = (Instance*)self;
        if (Volatile.Read(ref instance->Status) == 0)
        {
            return unchecked((int)0x8000000e);
        }
        *value = instance->Result;
        return 0;
    }

    private static void InvokeCompletion(Instance* instance)
    {
        nint handler = Volatile.Read(ref instance->Completed);
        int count = instance->InvokeTwice != 0 ? 2 : 1;
        for (int i = 0; i < count; i++)
        {
            int hr = ((delegate* unmanaged<nint, nint, int, int>)(*(void***)handler)[3])(
                handler,
                (nint)instance,
                1);
            instance->LastCallbackHr = hr;
            _ = Interlocked.Increment(ref instance->CallbackCalls);
        }
        handler = Interlocked.Exchange(ref instance->Completed, 0);
        _ = CallbackRelease(handler);
    }

    private static int QueryAndRelease(nint value, Guid iid)
    {
        nint result;
        int hr = ((delegate* unmanaged<nint, Guid*, nint*, int>)(*(void***)value)[0])(
            value,
            &iid,
            &result);
        if (hr >= 0)
        {
            _ = CallbackRelease(result);
        }
        return hr;
    }

    private static uint CallbackAddRef(nint value) =>
        ((delegate* unmanaged<nint, uint>)(*(void***)value)[1])(value);

    private static uint CallbackRelease(nint value) =>
        ((delegate* unmanaged<nint, uint>)(*(void***)value)[2])(value);

    private static Instance* FromInfo(nint self) =>
        (Instance*)((byte*)self - sizeof(nint));

    private static uint AddRef(Instance* instance) =>
        (uint)Interlocked.Increment(ref instance->References);

    private static uint Release(Instance* instance)
    {
        int count = Interlocked.Decrement(ref instance->References);
        if (count == 0)
        {
            nint completed = Interlocked.Exchange(ref instance->Completed, 0);
            if (completed != 0)
            {
                _ = CallbackRelease(completed);
            }
            NativeMemory.Free(instance);
            _ = Interlocked.Decrement(ref s_liveInstances);
        }
        return (uint)count;
    }

    private static void WaitForObservation(Instance* instance)
    {
        SpinWait spinner = default;
        long timeout = Environment.TickCount64 + 5000;
        while (Volatile.Read(ref instance->References) != 1 &&
               Environment.TickCount64 < timeout)
        {
            spinner.SpinOnce();
        }
        Check(Volatile.Read(ref instance->References) == 1, "operation callback did not unwind");
    }

    private static void ReleaseObservation(Instance* instance)
    {
        WaitForObservation(instance);
        Check(Release(instance) == 0, "operation reference leak");
    }

    private static void Check(bool condition, string message)
    {
        if (!condition)
        {
            throw new InvalidOperationException(message);
        }
    }
}

internal static class AsyncHelpers
{
    internal static async Task<int> AwaitAndDispose(IAsyncOperation<int> operation)
    {
        using (operation)
        {
            return await operation;
        }
    }
}
"#;

const LIBRARY_CSPROJ: &str = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net10.0</TargetFramework>
    <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
    <Optimize>true</Optimize>
    <Nullable>enable</Nullable>
    <ImplicitUsings>disable</ImplicitUsings>
    <InvariantGlobalization>true</InvariantGlobalization>
  </PropertyGroup>
</Project>
"#;

const EXE_CSPROJ: &str = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
    <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
    <Optimize>true</Optimize>
    <Nullable>enable</Nullable>
    <ImplicitUsings>disable</ImplicitUsings>
    <AssemblyName>test_csharp_harness</AssemblyName>
    <InvariantGlobalization>true</InvariantGlobalization>
  </PropertyGroup>
</Project>
"#;

// Idiomatic harness against the generated projection. The component stores Int32Property, adds in
// Add, and its StringProperty getter always returns "widget" (the setter is a no-op), so the string
// round trip validates the HSTRING marshalling path rather than value storage.
const PROGRAM: &str = r#"using System;
using Bench;

internal static class Program
{
    private static int Main()
    {
        using Widget widget = new();

        // Projected objects are reference types over one COM pointer. Aliases share one dispose
        // state, repeated disposal is harmless, and subsequent calls fail instead of dereferencing
        // a released COM pointer.
        int ownershipBaseline = widget.LiveCount();
        Widget owned = new();
        Widget ownedAlias = owned;
        owned.Dispose();
        ownedAlias.Dispose();
        try
        {
            _ = ownedAlias.Int32Property;
            Console.Error.WriteLine("Disposed alias remained callable");
            return 15;
        }
        catch (ObjectDisposedException)
        {
        }
        if (widget.LiveCount() != ownershipBaseline)
        {
            Console.Error.WriteLine("Alias disposal leaked or over-released its source");
            return 16;
        }

        // Null projected references marshal as a null ABI pointer rather than failing in the
        // projection. This component rejects null with E_POINTER.
        try
        {
            _ = widget.Echo(null);
            Console.Error.WriteLine("Null object input unexpectedly succeeded");
            return 19;
        }
        catch (Exception ex) when (ex.HResult == unchecked((int)0x80004003))
        {
        }

        widget.Int32Property = 123;
        if (widget.Int32Property != 123)
        {
            Console.Error.WriteLine($"Int32Property mismatch: {widget.Int32Property}");
            return 1;
        }

        int sum = widget.Add(2, 3);
        if (sum != 5)
        {
            Console.Error.WriteLine($"Add mismatch: {sum}");
            return 2;
        }

        if (widget.SumArray(new[] { 1, 2, 3 }) != 6)
        {
            Console.Error.WriteLine("Array input mismatch");
            return 28;
        }
        int[] returned = widget.Values();
        if (returned.Length != 3 || returned[2] != 3)
        {
            Console.Error.WriteLine("Array return mismatch");
            return 29;
        }
        widget.GetValues(out int[] output);
        if (output.Length != 3 || output[0] != 4)
        {
            Console.Error.WriteLine("Array output mismatch");
            return 30;
        }

        widget.StringProperty = "hello";
        string value = widget.StringProperty;
        if (value != "widget")
        {
            Console.Error.WriteLine($"StringProperty mismatch: {value}");
            return 3;
        }
        widget.StringProperty = null!;

        // HSTRING as a method parameter and return (not just a property): the component appends
        // "-echo", proving the input string marshals in and the transformed result marshals out.
        string echoed = widget.EchoString("round");
        if (echoed != "round-echo")
        {
            Console.Error.WriteLine($"EchoString mismatch: {echoed}");
            return 11;
        }
        if (widget.EchoString(null!) != "-echo")
        {
            Console.Error.WriteLine("Null EchoString input did not marshal as an empty HSTRING");
            return 12;
        }

        using (Windows.Foundation.Collections.IVector<string> strings = widget.StringItems(3))
        {
            if (strings.GetAt(1) != "1")
            {
                Console.Error.WriteLine("String vector GetAt mismatch");
                return 20;
            }
            string[] copied = new string[3];
            if (strings.GetMany(0, copied) != 3 || copied[2] != "2")
            {
                Console.Error.WriteLine("String vector GetMany mismatch");
                return 21;
            }
            strings.Append("3");
            if (strings.Count != 4 || strings.GetAt(3) != "3")
            {
                Console.Error.WriteLine("String vector Append mismatch");
                return 22;
            }
            strings.RemoveAtEnd();
            if (strings.Count != 3)
            {
                Console.Error.WriteLine("String vector RemoveAtEnd mismatch");
                return 25;
            }
        }

        using (Windows.Foundation.Collections.IMap<int, string> values = widget.StringValues(3))
        {
            if (values.Lookup(2) != "2")
            {
                Console.Error.WriteLine("String map value mismatch");
                return 23;
            }
            bool replaced = values.Insert(2, "two");
            if (!replaced || values.Lookup(2) != "two")
            {
                Console.Error.WriteLine("String map insert mismatch");
                return 24;
            }
            bool found = false;
            foreach (Windows.Foundation.Collections.IMap<int, string>.Entry entry in values)
            {
                using (entry)
                {
                    if (entry.Key == 2 && entry.Value == "two")
                    {
                        found = true;
                    }
                }
            }
            if (!found)
            {
                Console.Error.WriteLine("Map entry key/value mismatch");
                return 25;
            }
        }

        using (Windows.Foundation.IAsyncOperation<string> operation = widget.StringOperation())
        {
            if (operation.GetResults() != "async")
            {
                Console.Error.WriteLine("String async result mismatch");
                return 26;
            }
        }

        using (Windows.Foundation.IAsyncOperation<INonDefault?> operation = widget.ObjectOperation())
        using (INonDefault result = operation.GetResults())
        {
            if (result.Value() != 123)
            {
                Console.Error.WriteLine("Object async result mismatch");
                return 27;
            }
        }

        int forwarded = widget.Value();
        if (forwarded != 123)
        {
            Console.Error.WriteLine($"Widget.Value mismatch: {forwarded}");
            return 4;
        }

        int borrowedAs = widget.BorrowAs(
            static (INonDefault.Borrowed extra) => extra.Value() + extra.Value());
        if (borrowedAs != 246)
        {
            Console.Error.WriteLine($"Widget.BorrowAs mismatch: {borrowedAs}");
            return 28;
        }

        using (INonDefault extra = widget.As<INonDefault>())
        {
            int cast = extra.Value();
            if (cast != 123)
            {
                Console.Error.WriteLine($"As<INonDefault>().Value mismatch: {cast}");
                return 15;
            }
        }

        INonDefault.Borrowed invalidBorrow = default;
        try
        {
            _ = invalidBorrow.Value();
            throw new Exception("Default borrowed interface remained usable");
        }
        catch (ObjectDisposedException)
        {
        }

        try
        {
            widget.Fail();
            Console.Error.WriteLine("Fail did not throw");
            return 5;
        }
        catch (System.Runtime.InteropServices.COMException ex)
        {
            const int E_BOUNDS = unchecked((int)0x8000000B);
            if (ex.HResult != E_BOUNDS)
            {
                Console.Error.WriteLine($"Fail threw wrong HRESULT: 0x{ex.HResult:x8}");
                return 6;
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"Fail threw wrong exception type: {ex.GetType()}");
            return 13;
        }

        try
        {
            widget.FailWithMessage();
            Console.Error.WriteLine("FailWithMessage did not throw");
            return 14;
        }
        catch (Exception ex)
        {
            if (!ex.Message.Contains("bench error detail", StringComparison.Ordinal))
            {
                Console.Error.WriteLine($"FailWithMessage lost error information: {ex.Message}");
                return 15;
            }
        }

        int observed = -1;
        using (ChangedHandler handler = ChangedHandler.Create((_, v) => observed = v))
        {
            long token = widget.AddChanged(handler);
            widget.Signal(77);
            if (observed != 77)
            {
                Console.Error.WriteLine($"Signal did not fire handler: {observed}");
                return 7;
            }

            observed = -1;
            widget.RemoveChanged(token);
            widget.Signal(88);
            if (observed != -1)
            {
                Console.Error.WriteLine($"Handler fired after removal: {observed}");
                return 8;
            }
        }

        // Revoker form: `Changed` returns an EventRevoker that unsubscribes and releases the source
        // on Dispose. Verify it fires while alive and stops after disposal.
        int revoked = -1;
        using (ChangedHandler handler = ChangedHandler.Create((_, v) => revoked = v))
        {
            WindowsCsharp.EventRevoker revoker = widget.Changed(handler);
            widget.Signal(99);
            if (revoked != 99)
            {
                Console.Error.WriteLine($"Revoker handler did not fire: {revoked}");
                return 9;
            }

            WindowsCsharp.EventRevoker alias = revoker;
            revoker.Dispose();
            alias.Dispose();
            revoked = -1;
            widget.Signal(111);
            if (revoked != -1)
            {
                Console.Error.WriteLine($"Handler fired after revoke: {revoked}");
                return 10;
            }
        }

        // The revoker owns an AddRef independently of the projected source value. Disposing the
        // source first must keep the component alive until the revoker releases its reference.
        int baseline = widget.LiveCount();
        using (ChangedHandler handler = ChangedHandler.Create((_, _) => { }))
        {
            Widget source = new();
            WindowsCsharp.EventRevoker revoker = source.Changed(handler);
            source.Dispose();
            if (widget.LiveCount() != baseline + 1)
            {
                Console.Error.WriteLine("Revoker did not keep its source alive");
                return 13;
            }
            revoker.Dispose();
            if (widget.LiveCount() != baseline)
            {
                Console.Error.WriteLine("Revoker did not release its source");
                return 14;
            }
        }

        Console.WriteLine("windows-csharp OK: Int32=123 Add=5 String=widget Echo=round-echo Cast=123 Fail=threw Event=77 Revoke=99");
        return 0;
    }
}
"#;
