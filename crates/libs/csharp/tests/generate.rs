//! Hermetic generator tests: author a small winmd with `windows-rdl` in a temp directory, run the
//! generator, and assert on the emitted C# text. These do not require `dotnet`.

const RDL: &str = r#"use Windows::Foundation::Metadata::*;

#[winrt]
mod Sample {
    #[Activatable(1)]
    #[MarshalingBehavior(Agile)]
    class Gadget {
        IGadget,
    }

    #[ExclusiveTo(Gadget)]
    interface IGadget {
        Value: i32;
        Name: String;
        fn Combine(&self, a: i32, b: i32) -> i32;
    }
}
"#;

fn generate() -> String {
    let dir = std::env::temp_dir().join(format!("windows_csharp_gen_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();

    let rdl = dir.join("sample.rdl");
    let winmd = dir.join("sample.winmd");
    let cs = dir.join("Sample.cs");
    std::fs::write(&rdl, RDL).unwrap();

    windows_rdl::reader()
        .input(rdl.to_str().unwrap())
        .input_default()
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap();

    let bytes = std::fs::read(&winmd).unwrap();
    windows_csharp::builder()
        .input_bytes(&bytes)
        .filter("Sample")
        .output(cs.to_str().unwrap())
        .write()
        .unwrap();

    let source = std::fs::read_to_string(&cs).unwrap();
    std::fs::remove_dir_all(&dir).ok();
    source
}

#[test]
fn emits_expected_projection() {
    let source = generate();

    // Namespace and owning reference-type shape.
    assert!(source.contains("namespace Sample"));
    assert!(source.contains("public sealed unsafe class Gadget : WindowsCsharp.ComObject"));
    assert!(source.contains("[module: SkipLocalsInit]"));

    // A scalar property folds get_/put_ into a property at slots 6 and 7.
    assert!(source.contains("public int Value"));
    assert!(source.contains("(*(void***)self)[6])(self, &value)"));
    assert!(source.contains("(*(void***)self)[7])(self, value)"));

    // A string property uses the HSTRING marshalling helpers at slots 8 and 9.
    assert!(source.contains("public string Name"));
    assert!(source.contains("WindowsGetStringRawBuffer"));
    assert!(source.contains("WindowsCreateStringReference"));

    // A method appends a trailing out-pointer for its return and lands at slot 10.
    assert!(source.contains("public int Combine(int a, int b)"));
    assert!(source.contains("delegate* unmanaged<nint, int, int, int*, int>)(*(void***)self)[10]"));

    // Activation plumbing and the shared runtime support.
    assert!(source.contains(
        "WindowsCsharp.WinRT.Activate(ref s_module, ref s_factory, \"Sample.Gadget\", Iid)"
    ));
    assert!(source.contains("DllGetActivationFactory"));
    assert!(source.contains("public abstract unsafe class ComObject : IDisposable"));
    assert!(source.contains("internal readonly ref struct ComLease"));
}

#[test]
fn rejects_missing_input() {
    let output = std::env::temp_dir().join("windows_csharp_missing.cs");
    let error = windows_csharp::builder()
        .input("missing.windows-csharp.winmd")
        .filter("Sample")
        .output(output.to_str().unwrap())
        .write()
        .unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn requires_output() {
    let error = windows_csharp::builder()
        .input("unused.winmd")
        .filter("Sample")
        .write()
        .unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}
