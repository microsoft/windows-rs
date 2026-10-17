use windows_clang2::*;
use windows_metadata::{Type, Value, reader, reader::HasAttributes};

const FUNCTIONS: &[(&str, &str, usize)] = &[
    ("u_errorName", "icuuc.dll", 1),
    ("u_strlen", "icuuc.dll", 1),
    ("u_strToUTF8", "icuuc.dll", 6),
    ("GetCurrentProcessId", "kernel32.dll", 0),
    ("GetLastError", "kernel32.dll", 0),
    ("SetLastError", "kernel32.dll", 1),
    ("CloseHandle", "kernel32.dll", 1),
    ("CreateFileW", "kernel32.dll", 7),
    ("ReadFile", "kernel32.dll", 5),
    ("WriteFile", "kernel32.dll", 5),
    ("GetFileSizeEx", "kernel32.dll", 2),
    ("CreateEventW", "kernel32.dll", 4),
    ("WaitForSingleObject", "kernel32.dll", 2),
    ("CreateThread", "kernel32.dll", 6),
    ("EnumWindows", "user32.dll", 2),
    ("MessageBoxW", "user32.dll", 4),
    ("CoInitializeEx", "ole32.dll", 2),
    ("CoUninitialize", "ole32.dll", 0),
    ("CoCreateInstance", "ole32.dll", 5),
    ("RegOpenKeyExW", "advapi32.dll", 5),
    ("RegQueryValueExW", "advapi32.dll", 6),
    ("RegSetValueExW", "advapi32.dll", 6),
    ("RegCloseKey", "advapi32.dll", 1),
    ("RegEnumKeyExW", "advapi32.dll", 8),
    ("RegCreateKeyExW", "advapi32.dll", 9),
    ("OpenProcessToken", "advapi32.dll", 3),
    ("GetTokenInformation", "advapi32.dll", 5),
    ("WSAStartup", "ws2_32.dll", 2),
    ("WSACleanup", "ws2_32.dll", 0),
    ("socket", "ws2_32.dll", 3),
    ("bind", "ws2_32.dll", 3),
    ("listen", "ws2_32.dll", 2),
    ("accept", "ws2_32.dll", 3),
    ("connect", "ws2_32.dll", 3),
    ("send", "ws2_32.dll", 4),
    ("recv", "ws2_32.dll", 4),
    ("closesocket", "ws2_32.dll", 1),
    ("getaddrinfo", "ws2_32.dll", 4),
    ("freeaddrinfo", "ws2_32.dll", 1),
    ("BCryptOpenAlgorithmProvider", "bcrypt.dll", 4),
    ("BCryptCloseAlgorithmProvider", "bcrypt.dll", 2),
    ("BCryptGenRandom", "bcrypt.dll", 4),
    ("BCryptGetProperty", "bcrypt.dll", 6),
    ("CreateCompatibleDC", "gdi32.dll", 1),
    ("DeleteDC", "gdi32.dll", 1),
    ("CreateCompatibleBitmap", "gdi32.dll", 3),
    ("SelectObject", "gdi32.dll", 2),
    ("DeleteObject", "gdi32.dll", 1),
    ("BitBlt", "gdi32.dll", 9),
    ("GetStockObject", "gdi32.dll", 1),
    ("GetDC", "user32.dll", 1),
    ("ReleaseDC", "user32.dll", 2),
    ("GetClientRect", "user32.dll", 2),
    ("InvalidateRect", "user32.dll", 3),
    ("QueryPerformanceCounter", "kernel32.dll", 1),
    ("QueryPerformanceFrequency", "kernel32.dll", 1),
    ("GetTickCount64", "kernel32.dll", 0),
    ("Sleep", "kernel32.dll", 1),
    ("VirtualAlloc", "kernel32.dll", 4),
    ("VirtualFree", "kernel32.dll", 3),
    ("VirtualProtect", "kernel32.dll", 4),
    ("CreateDXGIFactory2", "dxgi.dll", 3),
    ("WinHttpOpen", "winhttp.dll", 5),
    ("WinHttpConnect", "winhttp.dll", 4),
    ("WinHttpOpenRequest", "winhttp.dll", 7),
    ("WinHttpSendRequest", "winhttp.dll", 7),
    ("WinHttpReceiveResponse", "winhttp.dll", 2),
    ("WinHttpReadData", "winhttp.dll", 4),
    ("WinHttpWriteData", "winhttp.dll", 4),
    ("WinHttpQueryHeaders", "winhttp.dll", 6),
    ("WinHttpQueryDataAvailable", "winhttp.dll", 2),
    ("WinHttpCloseHandle", "winhttp.dll", 1),
    ("WinHttpSetOption", "winhttp.dll", 4),
    ("WinHttpQueryOption", "winhttp.dll", 4),
    ("EvtQuery", "wevtapi.dll", 4),
    ("EvtNext", "wevtapi.dll", 6),
    ("EvtRender", "wevtapi.dll", 7),
    ("EvtClose", "wevtapi.dll", 1),
    ("EvtOpenLog", "wevtapi.dll", 3),
    ("EvtGetLogInfo", "wevtapi.dll", 5),
    ("EvtSubscribe", "wevtapi.dll", 8),
    ("OpenSCManagerW", "advapi32.dll", 3),
    ("OpenServiceW", "advapi32.dll", 3),
    ("CloseServiceHandle", "advapi32.dll", 1),
    ("StartServiceW", "advapi32.dll", 3),
    ("ControlService", "advapi32.dll", 3),
    ("QueryServiceStatusEx", "advapi32.dll", 5),
    ("EnumServicesStatusExW", "advapi32.dll", 10),
    ("CreateServiceW", "advapi32.dll", 13),
    ("DeleteService", "advapi32.dll", 1),
    ("OpenProcess", "kernel32.dll", 3),
    ("ReadProcessMemory", "kernel32.dll", 5),
    ("WriteProcessMemory", "kernel32.dll", 5),
    ("VirtualQuery", "kernel32.dll", 3),
    ("GetExitCodeProcess", "kernel32.dll", 2),
    ("QueryFullProcessImageNameW", "kernel32.dll", 4),
    ("CreateToolhelp32Snapshot", "kernel32.dll", 2),
    ("Process32FirstW", "kernel32.dll", 2),
    ("Process32NextW", "kernel32.dll", 2),
    ("Module32FirstW", "kernel32.dll", 2),
    ("Module32NextW", "kernel32.dll", 2),
    ("GetEnvironmentVariableW", "kernel32.dll", 3),
    ("SetEnvironmentVariableW", "kernel32.dll", 2),
    ("D3D11CreateDevice", "d3d11.dll", 10),
    ("D3D11CreateDeviceAndSwapChain", "d3d11.dll", 12),
    ("D3D12CreateDevice", "d3d12.dll", 4),
    ("D3D12GetDebugInterface", "d3d12.dll", 2),
    ("D3D12SerializeRootSignature", "d3d12.dll", 4),
    ("CreateDXGIFactory", "dxgi.dll", 2),
    ("CreateDXGIFactory1", "dxgi.dll", 2),
    ("DWriteCreateFactory", "dwrite.dll", 3),
    ("D2D1MakeRotateMatrix", "d2d1.dll", 3),
    ("D2D1MakeSkewMatrix", "d2d1.dll", 4),
    ("D2D1IsMatrixInvertible", "d2d1.dll", 1),
    ("D2D1InvertMatrix", "d2d1.dll", 1),
    ("D2D1CreateDevice", "d2d1.dll", 3),
    ("D2D1CreateDeviceContext", "d2d1.dll", 3),
];

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_functions() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_functions.h".into();
    input.source = test_clang2::fixture("sdk_functions").source;
    input.roots = FUNCTIONS
        .iter()
        .map(|(name, _, _)| (*name).into())
        .collect();
    input.roots.push("IClassFactory".into());
    input.macros.extend(
        [
            "MAX_PATH",
            "D3D11_DEFAULT_DEPTH_BIAS_CLAMP",
            "D3D11_FLOAT32_MAX",
            "D3D11_SRGB_TO_FLOAT_THRESHOLD",
        ]
        .map(String::from),
    );
    let imports: projection::Imports = FUNCTIONS
        .iter()
        .map(|(name, library, _)| {
            (
                (*name).into(),
                projection::Import {
                    library: (*library).into(),
                    name: None,
                },
            )
        })
        .collect();
    let targets = [Target::X86, Target::X64, Target::Arm64];
    let snapshots = capture(std::slice::from_ref(&input), &targets).unwrap();
    for snapshot in &snapshots {
        for name in ["D2D1CreateDevice", "D2D1CreateDeviceContext"] {
            assert!(
                snapshot
                    .inline_functions
                    .iter()
                    .any(|helper| helper.name == name),
                "{name}"
            );
        }
    }
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, FUNCTIONS.len());
    assert_eq!(output.constants, 4);
    let partial = projection::project_partial_with_imports(
        capture_partial(&[input], &targets).unwrap(),
        "Fixture",
        &imports,
    )
    .unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-sdk-functions.winmd");
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|part| &part.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&again)
        .write()
        .unwrap();
    for path in [&winmd, &again] {
        for bit in [1, 2, 4] {
            let index =
                reader::Index::new_for_architecture(vec![reader::File::read(path).unwrap()], bit);
            let apis = index.expect("Fixture", "Apis");
            let methods: Vec<_> = apis
                .methods()
                .filter(|method| method.arches() == 0 || method.arches() & bit != 0)
                .collect();
            assert_eq!(methods.len(), FUNCTIONS.len());
            for &(name, library, count) in FUNCTIONS {
                let method = methods.iter().find(|method| method.name() == name).unwrap();
                assert_eq!(method.params().count(), count, "{name}");
                assert_eq!(method.signature(&[]).types.len(), count, "{name}");
                let import = method.impl_map().unwrap();
                assert_eq!(import.import_name(), name);
                assert_eq!(import.import_scope().name(), library);
                assert_eq!(
                    method.calling_convention(),
                    if library != "icuuc.dll" {
                        "system"
                    } else {
                        "C"
                    }
                );
            }
            let enumeration = methods
                .iter()
                .find(|method| method.name() == "EnumWindows")
                .unwrap();
            assert_eq!(
                enumeration.signature(&[]).types[0],
                Type::class_named("Fixture", "WNDENUMPROC")
            );
            let thread = methods
                .iter()
                .find(|method| method.name() == "CreateThread")
                .unwrap();
            assert_eq!(
                thread.signature(&[]).types[2],
                Type::class_named("Fixture", "LPTHREAD_START_ROUTINE")
            );
            let subscription = methods
                .iter()
                .find(|method| method.name() == "EvtSubscribe")
                .unwrap();
            assert_eq!(
                subscription.signature(&[]).types[6],
                Type::class_named("Fixture", "EVT_SUBSCRIBE_CALLBACK")
            );
            let start = methods
                .iter()
                .find(|method| method.name() == "StartServiceW")
                .unwrap();
            assert_eq!(
                start.signature(&[]).types[2],
                Type::PtrMut(Box::new(Type::value_named("Fixture", "LPCWSTR")), 1)
            );
            let create = methods
                .iter()
                .find(|method| method.name() == "D3D11CreateDevice")
                .unwrap();
            assert_eq!(
                create.signature(&[]).types[7],
                Type::PtrMut(Box::new(Type::class_named("Fixture", "ID3D11Device")), 1)
            );
            assert_eq!(
                index.expect("Fixture", "ID3DBlob").underlying_type(),
                Some(Type::class_named("Fixture", "ID3D10Blob"))
            );
            let socket = methods
                .iter()
                .find(|method| method.name() == "socket")
                .unwrap();
            assert_eq!(
                socket.signature(&[]).return_type,
                Type::value_named("Fixture", "SOCKET")
            );
            let mut storage = Type::value_named("Fixture", "SOCKET");
            while let Type::ValueName(name) = storage {
                storage = index
                    .expect(&name.namespace, &name.name)
                    .underlying_type()
                    .unwrap();
            }
            assert_eq!(storage, if bit == 1 { Type::U32 } else { Type::U64 });
            let token = methods
                .iter()
                .find(|method| method.name() == "GetTokenInformation")
                .unwrap();
            assert_eq!(
                token.signature(&[]).types[1],
                Type::value_named("Fixture", "TOKEN_INFORMATION_CLASS")
            );
            for (name, expected) in [
                ("D3D11_DEFAULT_DEPTH_BIAS_CLAMP", 0.0f32),
                ("D3D11_FLOAT32_MAX", f32::MAX),
                ("D3D11_SRGB_TO_FLOAT_THRESHOLD", 0.04045),
            ] {
                let field = apis.fields().find(|field| field.name() == name).unwrap();
                let Value::F32(actual) = field.constant().unwrap().value() else {
                    panic!("{name}: expected f32")
                };
                assert_eq!(actual.to_bits(), expected.to_bits(), "{name}");
            }
        }
    }
    let bindings = winmd.with_extension("rs");
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture.ID3DBlob")
        .flat()
        .output(&bindings)
        .write();
    let rust = std::fs::read_to_string(&bindings).unwrap();
    assert!(rust.contains("pub type ID3DBlob = ID3D10Blob;"), "{rust}");
    for path in [winmd, rdl, again, bindings] {
        std::fs::remove_file(path).unwrap();
    }
}
