use std::collections::HashMap;
use std::fs::*;
use std::io::prelude::*;
use std::process::*;
use windows_bindgen::*;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/component.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    std::fs::create_dir_all(".windows/winmd")?;

    Command::new("midlrt.exe").arg("/winrt").arg("/nomidl").arg("/h").arg("nul").arg("/metadata_dir").arg(&metadata_dir).arg("/reference").arg(format!("{}\\Windows.Foundation.winmd", metadata_dir)).arg("/winmd").arg(".windows/winmd/component.winmd").arg("src/component.idl").status()?;

    std::fs::remove_dir_all("src/bindings").ok();
    std::fs::create_dir("src/bindings")?;

    let mut bindings = File::create("src/bindings/test_nightly_component.rs")?;
    let gen = Gen { namespace: "test_nightly_component", component: true, ..Default::default() };
    bindings.write_all(gen_namespace(&gen).as_bytes())?;
    bindings.write_all(gen_namespace_impl(&gen).as_bytes())?;
    drop(bindings);
    Command::new("rustfmt").arg("src/bindings/test_nightly_component.rs").status()?;

    let mut class_map = HashMap::new();
    let _ = class_map.insert("Microsoft.Windows.System.Power.PowerManager".to_string(), "test_nightly_component.dll".to_string());
    let mut bindings = File::create("src/bindings/microsoft_windows_system_power.rs")?;
    let gen = Gen { namespace: "Microsoft.Windows.System.Power", component: true, class_map, ..Default::default() };
    bindings.write_all(gen_namespace(&gen).as_bytes())?;
    bindings.write_all(gen_namespace_impl(&gen).as_bytes())?;
    drop(bindings);
    Command::new("rustfmt").arg("src/bindings/microsoft_windows_system_power.rs").status()?;

    Ok(())
}
