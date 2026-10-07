use super::*;
use windows_clang2::{Input, ProjectionOptions, ReferenceKind, TypeReference};

pub fn audio() {
    let time = std::time::Instant::now();
    let include_dirs = sdk_include_dirs();
    let inputs = clang_inputs(&["mmdeviceapi.h", "endpointvolume.h"], &include_dirs, false)
        .into_iter()
        .map(|input| Input::new(input.name, input.source));
    let include_args: Vec<_> = include_dirs
        .into_iter()
        .flat_map(|dir| ["-isystem".into(), dir])
        .collect();
    let arguments = clang_arguments(&Arch::known("x64").unwrap(), &include_args, None);
    let roots = [
        "GUID",
        "IMMDeviceEnumerator",
        "IMMDeviceCollection",
        "IMMDevice",
        "IMMNotificationClient",
        "IAudioEndpointVolume",
        "IAudioEndpointVolumeEx",
        "IAudioEndpointVolumeCallback",
        "AUDIO_VOLUME_NOTIFICATION_DATA",
    ];
    let snapshot = windows_clang2::capture(
        inputs,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &roots,
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    // Keep candidate definitions distinct from the bundled metadata used for external references.
    let mut options = ProjectionOptions::new("Win32Audio");
    for (native, namespace, name, kind) in [
        (
            "HRESULT",
            "Windows.Foundation",
            "HResult",
            ReferenceKind::Value,
        ),
        ("BOOL", "Windows.Win32", "BOOL", ReferenceKind::Value),
        (
            "IUnknown",
            "Windows.Win32",
            "IUnknown",
            ReferenceKind::Interface,
        ),
        (
            "IPropertyStore",
            "Windows.Win32",
            "IPropertyStore",
            ReferenceKind::Interface,
        ),
        (
            "tagPROPVARIANT",
            "Windows.Win32",
            "PROPVARIANT",
            ReferenceKind::Value,
        ),
        ("LPWSTR", "Windows.Win32", "PWSTR", ReferenceKind::Value),
        ("LPCWSTR", "Windows.Win32", "PCWSTR", ReferenceKind::Value),
    ] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: namespace.into(),
                name: name.into(),
                kind,
            },
        );
    }
    let plan = resolved.project(&options).unwrap();
    assert!(
        plan.omitted().is_empty(),
        "omitted audio roots: {:?}",
        plan.omitted()
    );
    let output = std::path::Path::new("target/win32-clang2/audio");
    std::fs::create_dir_all(output).unwrap();
    let rdl = output.join("audio.rdl");
    let winmd = output.join("audio.winmd");
    std::fs::write(&rdl, plan.rdl()).unwrap();
    windows_rdl::reader()
        .input(&rdl)
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen([
        "--in",
        "default",
        winmd.to_str().unwrap(),
        "--out",
        output.join("src/bindings.rs").to_str().unwrap(),
        "--flat",
        "--minimal",
        "--filter",
        "Win32Audio",
        "Windows.Win32.PROPVARIANT",
        "Windows.Win32.IPropertyStore",
        "Windows.Win32.CoCreateInstance",
        "Windows.Win32.CoTaskMemFree",
    ]);
    std::fs::write(
        output.join("Cargo.toml"),
        "[workspace]\n[package]\nname = \"clang2-audio-smoke\"\nversion = \"0.0.0\"\n\
         edition = \"2024\"\n[dependencies]\n\
         windows-core = { path = \"../../../crates/libs/core\" }\n",
    )
    .unwrap();
    std::fs::write(output.join("src/main.rs"), include_str!("audio_smoke.rs")).unwrap();
    println!(
        "clang2 audio: {} groups, {} observations, {} declaration pairs; generated in {:.2}s at {}",
        resolved.group_count(),
        resolved.report().observations,
        resolved.report().declaration_pairs,
        time.elapsed().as_secs_f32(),
        output.display()
    );
}
