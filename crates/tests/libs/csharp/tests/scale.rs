//! Opt-in scale measurements for generated WinRT generic dispatch.
//!
//! Run with:
//!
//! `cargo test -p test_csharp --test scale -- --ignored --nocapture`

use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const FOUNDATION: &str = r"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd";
const BREADTHS: &[usize] = &[1, 8, 32, 64, 80, 82, 128];

#[test]
fn broad_maps_use_static_function_specialization() {
    let scratch = scratch("generic_scale_specialization");
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch).unwrap();
    }
    std::fs::create_dir_all(&scratch).unwrap();
    let rdl = scratch.join("scale.rdl");
    std::fs::write(&rdl, synthetic_rdl(82)).unwrap();
    let winmd = scratch.join("scale.winmd");
    windows_rdl::reader()
        .input(rdl.to_str().unwrap())
        .input_default()
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap();
    let generated = scratch.join("Generated.cs");
    generate(&winmd, &generated, false);
    let source = std::fs::read_to_string(generated).unwrap();

    for expected in [
        "private static readonly delegate* managed<nint, K, V> s_lookup;",
        "private static readonly delegate* managed<nint, K, V, bool> s_insert;",
        "s_lookup = &Lookup",
        "return s_lookup(self, key);",
        "return s_lookup(_this, key);",
    ] {
        assert!(
            source.contains(expected),
            "broad map specialization omitted `{expected}`"
        );
    }
}

#[test]
#[ignore = "records generator and dotnet scale measurements"]
fn generic_dispatch_scale() {
    if !have_dotnet() {
        eprintln!("skipping: dotnet not found on PATH");
        return;
    }

    println!(
        "breadth,elements,methods,source_bytes,generic_bytes,typeof_checks,\
         generator_ms,exact_ms,build_ms,assembly_bytes"
    );

    let selected = std::env::var("WINDOWS_CSHARP_SCALE_BREADTH")
        .ok()
        .map(|value| value.parse().unwrap());
    for breadth in selected.into_iter().chain(BREADTHS.iter().copied()) {
        measure(breadth);
        if selected.is_some() {
            break;
        }
    }
}

fn measure(breadth: usize) {
    let scratch = scratch(&format!("generic_scale_{breadth}"));
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch).unwrap();
    }
    std::fs::create_dir_all(&scratch).unwrap();

    let rdl = scratch.join("scale.rdl");
    std::fs::write(&rdl, synthetic_rdl(breadth)).unwrap();
    let winmd = scratch.join("scale.winmd");
    windows_rdl::reader()
        .input(rdl.to_str().unwrap())
        .input_default()
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap();

    let project = scratch.join("project");
    std::fs::create_dir_all(&project).unwrap();
    let generated = project.join("Generated.cs");
    let generator_time = median_generation(&winmd, &generated, false);
    let exact_time = median_generation(&winmd, &scratch.join("Exact.cs"), true);
    let source = std::fs::read_to_string(&generated).unwrap();

    std::fs::write(project.join("Program.cs"), benchmark_program(breadth)).unwrap();
    std::fs::write(project.join("generic_scale.csproj"), PROJECT).unwrap();

    run(
        Command::new("dotnet.exe")
            .arg("restore")
            .arg(&project)
            .args(["--nologo", "--verbosity", "quiet"]),
        "dotnet restore",
    );

    let build_start = Instant::now();
    run(
        Command::new("dotnet.exe").arg("build").arg(&project).args([
            "--no-restore",
            "--configuration",
            "Release",
            "--nologo",
            "--verbosity",
            "quiet",
            "--disable-build-servers",
            "-p:UseSharedCompilation=false",
        ]),
        "dotnet build",
    );
    let build_time = build_start.elapsed();

    let assembly = project
        .join("bin")
        .join("x64")
        .join("Release")
        .join("net10.0")
        .join("generic_scale.dll");
    let assembly_bytes = std::fs::metadata(&assembly).unwrap().len();
    let elements = 11 + breadth * 3;

    println!(
        "{breadth},{elements},{},{},{},{},{:.3},{:.3},{:.3},{assembly_bytes}",
        elements * 5,
        source.len(),
        generic_source_bytes(&source),
        source.matches("typeof(").count(),
        milliseconds(generator_time),
        milliseconds(exact_time),
        milliseconds(build_time),
    );

    let output = Command::new("dotnet.exe")
        .arg(&assembly)
        .env("DOTNET_ReadyToRun", "0")
        .env("DOTNET_TieredCompilation", "0")
        .output()
        .expect("failed to run generic scale harness");
    assert!(
        output.status.success(),
        "generic scale harness failed\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

fn median_generation(winmd: &Path, output: &Path, exact: bool) -> Duration {
    generate(winmd, output, exact);
    let mut samples = Vec::with_capacity(3);
    for _ in 0..3 {
        let start = Instant::now();
        generate(winmd, output, exact);
        samples.push(start.elapsed());
    }
    samples.sort_unstable();
    samples[1]
}

fn generate(winmd: &Path, output: &Path, exact: bool) {
    let mut builder = windows_csharp::builder()
        .input(winmd.to_str().unwrap())
        .input(FOUNDATION)
        .output(output.to_str().unwrap());
    if exact {
        builder = builder.select("Scale.IScale");
    } else {
        builder = builder.filter("Scale");
    }
    builder.write().unwrap();
}

fn synthetic_rdl(breadth: usize) -> String {
    let mut result =
        String::from("use Windows::Foundation::Metadata::*;\n\n#[winrt]\nmod Scale {\n");

    for index in 0..breadth {
        writeln!(
            result,
            "    #[repr(i32)]\n    enum Enum{index:03} {{\n        Value = 0,\n    }}\n"
        )
        .unwrap();
        writeln!(
            result,
            "    struct Struct{index:03} {{\n        Value: i32,\n    }}\n"
        )
        .unwrap();
        writeln!(
            result,
            "    interface IObject{index:03} {{\n        Value: i32;\n    }}\n"
        )
        .unwrap();
    }

    result.push_str("    interface IScale {\n");
    for (suffix, ty) in scalar_types() {
        write_generic_methods(&mut result, suffix, ty);
    }
    write_generic_methods(&mut result, "String", "String");
    for index in 0..breadth {
        write_generic_methods(
            &mut result,
            &format!("Enum{index:03}"),
            &format!("Enum{index:03}"),
        );
        write_generic_methods(
            &mut result,
            &format!("Struct{index:03}"),
            &format!("Struct{index:03}"),
        );
        write_generic_methods(
            &mut result,
            &format!("Object{index:03}"),
            &format!("IObject{index:03}"),
        );
    }
    result.push_str("    }\n}\n");
    result
}

fn scalar_types() -> &'static [(&'static str, &'static str)] {
    &[
        ("I8", "i8"),
        ("U8", "u8"),
        ("I16", "i16"),
        ("U16", "u16"),
        ("I32", "i32"),
        ("U32", "u32"),
        ("I64", "i64"),
        ("U64", "u64"),
        ("F32", "f32"),
        ("F64", "f64"),
    ]
}

fn write_generic_methods(result: &mut String, suffix: &str, ty: &str) {
    writeln!(
        result,
        "        fn Vector{suffix}(&self) -> \
         Windows::Foundation::Collections::IVector<{ty}>;"
    )
    .unwrap();
    writeln!(
        result,
        "        fn VectorView{suffix}(&self) -> \
         Windows::Foundation::Collections::IVectorView<{ty}>;"
    )
    .unwrap();
    writeln!(
        result,
        "        fn Map{suffix}(&self) -> \
         Windows::Foundation::Collections::IMap<{ty}, {ty}>;"
    )
    .unwrap();
    writeln!(
        result,
        "        fn MapView{suffix}(&self) -> \
         Windows::Foundation::Collections::IMapView<{ty}, {ty}>;"
    )
    .unwrap();
    writeln!(
        result,
        "        fn Async{suffix}(&self) -> Windows::Foundation::IAsyncOperation<{ty}>;"
    )
    .unwrap();
}

fn benchmark_program(breadth: usize) -> String {
    let middle = breadth / 2;
    let last = breadth - 1;
    let mut result = String::from(
        r#"using System;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Windows.Foundation;
using Windows.Foundation.Collections;

internal static unsafe class Program
{
    private const int Iterations = 200_000;
    private static object? s_sink;

    [StructLayout(LayoutKind.Sequential)]
    private struct Instance
    {
        internal nint Vtable;
    }

    private static void Main()
    {
        nint* valueCollectionVtable = Vtable(11);
        valueCollectionVtable[6] =
            (nint)(delegate* unmanaged<nint, uint, int*, int>)&ReturnValue;
        valueCollectionVtable[8] =
            (nint)(delegate* unmanaged<nint, uint, byte*, int>)&HasValue;
        valueCollectionVtable[10] =
            (nint)(delegate* unmanaged<nint, uint, int, byte*, int>)&InsertValue;
        valueCollectionVtable[11] =
            (nint)(delegate* unmanaged<nint, uint, int>)&RemoveValue;
        nint* vectorReferenceVtable = Vtable(6);
        vectorReferenceVtable[6] =
            (nint)(delegate* unmanaged<nint, uint, nint*, int>)&ReturnVectorReference;
        nint* mapReferenceVtable = Vtable(11);
        mapReferenceVtable[6] =
            (nint)(delegate* unmanaged<nint, nint, nint*, int>)&ReturnMapReference;
        mapReferenceVtable[8] =
            (nint)(delegate* unmanaged<nint, nint, byte*, int>)&HasReference;
        mapReferenceVtable[10] =
            (nint)(delegate* unmanaged<nint, nint, nint, byte*, int>)&InsertReference;
        mapReferenceVtable[11] =
            (nint)(delegate* unmanaged<nint, nint, int>)&RemoveReference;
        nint* asyncValueVtable = Vtable(8);
        asyncValueVtable[8] =
            (nint)(delegate* unmanaged<nint, int*, int>)&ReturnAsyncValue;
        nint* asyncReferenceVtable = Vtable(8);
        asyncReferenceVtable[8] =
            (nint)(delegate* unmanaged<nint, nint*, int>)&ReturnAsyncReference;

        Instance* valueCollection = Allocate(valueCollectionVtable);
        Instance* vectorReference = Allocate(vectorReferenceVtable);
        Instance* mapReference = Allocate(mapReferenceVtable);
        Instance* asyncValue = Allocate(asyncValueVtable);
        Instance* asyncReference = Allocate(asyncReferenceVtable);

        try
        {
            Console.WriteLine($"cold-value-us,{ColdValue():F3}");
            Console.WriteLine($"cold-object-us,{ColdObject():F3}");
            using (var strings = new IMap<string, string>((nint)mapReference, true))
            {
                if (!strings.HasKey(null!) || !strings.Insert(null!, null!))
                    throw new InvalidOperationException();
                if (strings.Lookup(null!) != "")
                    throw new InvalidOperationException();
                strings.Remove(null!);
            }
"#,
    );

    for (label, index) in [("first", 0), ("middle", middle), ("last", last)] {
        writeln!(
            result,
            "            using (var value = new IVector<Scale.Enum{index:03}>\
             ((nint)valueCollection, true))\n            {{\n                Console.WriteLine(\
             $\"vector-value-{label}-ns,{{MeasureVector(value):F3}}\");\n            }}"
        )
        .unwrap();
        writeln!(
            result,
            "            using (var value = new IVector<Scale.IObject{index:03}>\
             ((nint)vectorReference, true))\n            {{\n                Console.WriteLine(\
             $\"vector-object-{label}-ns,{{MeasureVector(value):F3}}\");\n            }}"
        )
        .unwrap();
        writeln!(
            result,
            "            using (var value = new IMap<Scale.Enum{index:03}, \
             Scale.Enum{index:03}>((nint)valueCollection, true))\n            {{\n                \
             if (!value.HasKey(default) || !value.Insert(default, default)) throw new \
             InvalidOperationException();\n                value.Remove(default);\n                \
             Console.WriteLine($\"map-value-{label}-ns,{{MeasureMap(value, default):F3}}\");\n\
                         }}"
        )
        .unwrap();
        writeln!(
            result,
            "            using (var value = new IMap<Scale.IObject{index:03}, \
             Scale.IObject{index:03}>((nint)mapReference, true))\n            {{\n                \
             if (!value.HasKey(null!) || !value.Insert(null!, null!)) throw new \
             InvalidOperationException();\n                value.Remove(null!);\n                \
             Console.WriteLine($\"map-object-{label}-ns,{{MeasureMap(value, default!):F3}}\");\n\
                         }}"
        )
        .unwrap();
        writeln!(
            result,
            "            using (var value = new IAsyncOperation<Scale.Enum{index:03}>\
             ((nint)asyncValue, true))\n            {{\n                Console.WriteLine(\
             $\"async-value-{label}-ns,{{MeasureAsync(value):F3}}\");\n            }}"
        )
        .unwrap();
        writeln!(
            result,
            "            using (var value = new IAsyncOperation<Scale.IObject{index:03}>\
             ((nint)asyncReference, true))\n            {{\n                Console.WriteLine(\
             $\"async-object-{label}-ns,{{MeasureAsync(value):F3}}\");\n            }}"
        )
        .unwrap();
    }

    write!(
        result,
        r#"        }}
        finally
        {{
            NativeMemory.Free(valueCollection);
            NativeMemory.Free(vectorReference);
            NativeMemory.Free(mapReference);
            NativeMemory.Free(asyncValue);
            NativeMemory.Free(asyncReference);
            NativeMemory.Free(valueCollectionVtable);
            NativeMemory.Free(vectorReferenceVtable);
            NativeMemory.Free(mapReferenceVtable);
            NativeMemory.Free(asyncValueVtable);
            NativeMemory.Free(asyncReferenceVtable);
        }}
    }}

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static double ColdValue()
    {{
        long start = Stopwatch.GetTimestamp();
        int hash = IVector<Scale.Enum{last:03}>.Iid.GetHashCode();
        hash ^= IVectorView<Scale.Enum{last:03}>.Iid.GetHashCode();
        hash ^= IMap<Scale.Enum{last:03}, Scale.Enum{last:03}>.Iid.GetHashCode();
        hash ^= IMapView<Scale.Enum{last:03}, Scale.Enum{last:03}>.Iid.GetHashCode();
        hash ^= IAsyncOperation<Scale.Enum{last:03}>.Iid.GetHashCode();
        long elapsed = Stopwatch.GetTimestamp() - start;
        s_sink = hash;
        return elapsed * 1_000_000.0 / Stopwatch.Frequency;
    }}

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static double ColdObject()
    {{
        long start = Stopwatch.GetTimestamp();
        int hash = IVector<Scale.IObject{last:03}>.Iid.GetHashCode();
        hash ^= IVectorView<Scale.IObject{last:03}>.Iid.GetHashCode();
        hash ^= IMap<Scale.IObject{last:03}, Scale.IObject{last:03}>.Iid.GetHashCode();
        hash ^= IMapView<Scale.IObject{last:03}, Scale.IObject{last:03}>.Iid.GetHashCode();
        hash ^= IAsyncOperation<Scale.IObject{last:03}>.Iid.GetHashCode();
        long elapsed = Stopwatch.GetTimestamp() - start;
        s_sink = hash;
        return elapsed * 1_000_000.0 / Stopwatch.Frequency;
    }}

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static double MeasureVector<T>(IVector<T> value)
    {{
        T result = default!;
        for (int index = 0; index < 20_000; index++)
        {{
            result = value.GetAt(0);
        }}
        long allocatedBefore = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int index = 0; index < Iterations; index++)
        {{
            result = value.GetAt(0);
        }}
        long elapsed = Stopwatch.GetTimestamp() - start;
        long allocated = GC.GetAllocatedBytesForCurrentThread() - allocatedBefore;
        if (allocated != 0)
            throw new InvalidOperationException($"vector allocated {{allocated}} bytes");
        s_sink = result;
        return elapsed * 1_000_000_000.0 / Stopwatch.Frequency / Iterations;
    }}

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static double MeasureMap<K, V>(IMap<K, V> value, K key)
    {{
        V result = default!;
        for (int index = 0; index < 20_000; index++)
        {{
            result = value.Lookup(key);
        }}
        long allocatedBefore = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int index = 0; index < Iterations; index++)
        {{
            result = value.Lookup(key);
        }}
        long elapsed = Stopwatch.GetTimestamp() - start;
        long allocated = GC.GetAllocatedBytesForCurrentThread() - allocatedBefore;
        if (allocated != 0)
            throw new InvalidOperationException($"map allocated {{allocated}} bytes");
        s_sink = result;
        return elapsed * 1_000_000_000.0 / Stopwatch.Frequency / Iterations;
    }}

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static double MeasureAsync<T>(IAsyncOperation<T> value)
    {{
        T result = default!;
        for (int index = 0; index < 20_000; index++)
        {{
            result = value.GetResults();
        }}
        long allocatedBefore = GC.GetAllocatedBytesForCurrentThread();
        long start = Stopwatch.GetTimestamp();
        for (int index = 0; index < Iterations; index++)
        {{
            result = value.GetResults();
        }}
        long elapsed = Stopwatch.GetTimestamp() - start;
        long allocated = GC.GetAllocatedBytesForCurrentThread() - allocatedBefore;
        if (allocated != 0)
            throw new InvalidOperationException($"async allocated {{allocated}} bytes");
        s_sink = result;
        return elapsed * 1_000_000_000.0 / Stopwatch.Frequency / Iterations;
    }}

    private static nint* Vtable(int lastSlot)
    {{
        nint* value = (nint*)NativeMemory.AllocZeroed(
            (nuint)(lastSlot + 1), (nuint)sizeof(nint));
        value[2] = (nint)(delegate* unmanaged<nint, uint>)&Release;
        return value;
    }}

    private static Instance* Allocate(nint* vtable)
    {{
        Instance* value = (Instance*)NativeMemory.AllocZeroed(1, (nuint)sizeof(Instance));
        value->Vtable = (nint)vtable;
        return value;
    }}

    [UnmanagedCallersOnly]
    private static uint Release(nint self) => 1;

    [UnmanagedCallersOnly]
    private static int ReturnValue(nint self, uint key, int* result)
    {{
        *result = 0;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int ReturnVectorReference(nint self, uint key, nint* result)
    {{
        *result = 0;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int ReturnMapReference(nint self, nint key, nint* result)
    {{
        *result = 0;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int HasValue(nint self, uint key, byte* result)
    {{
        *result = 1;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int InsertValue(nint self, uint key, int value, byte* result)
    {{
        *result = 1;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int RemoveValue(nint self, uint key) => 0;

    [UnmanagedCallersOnly]
    private static int HasReference(nint self, nint key, byte* result)
    {{
        *result = 1;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int InsertReference(nint self, nint key, nint value, byte* result)
    {{
        *result = 1;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int RemoveReference(nint self, nint key) => 0;

    [UnmanagedCallersOnly]
    private static int ReturnAsyncValue(nint self, int* result)
    {{
        *result = 0;
        return 0;
    }}

    [UnmanagedCallersOnly]
    private static int ReturnAsyncReference(nint self, nint* result)
    {{
        *result = 0;
        return 0;
    }}
}}
"#,
    )
    .unwrap();
    result
}

fn generic_source_bytes(source: &str) -> usize {
    let Some(start) = source.find("namespace Windows.Foundation") else {
        return 0;
    };
    let end = source[start..]
        .find("namespace WindowsCsharp")
        .map_or(source.len(), |offset| start + offset);
    end - start
}

fn scratch(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(name)
}

fn milliseconds(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn have_dotnet() -> bool {
    Command::new("dotnet.exe")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

fn run(command: &mut Command, description: &str) {
    let output = command.output().unwrap_or_else(|error| {
        panic!("{description} failed to start: {error}");
    });
    assert!(
        output.status.success(),
        "{description} failed\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

const PROJECT: &str = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
    <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
    <Optimize>true</Optimize>
    <Nullable>enable</Nullable>
    <ImplicitUsings>disable</ImplicitUsings>
    <AssemblyName>generic_scale</AssemblyName>
    <InvariantGlobalization>true</InvariantGlobalization>
  </PropertyGroup>
</Project>
"#;
