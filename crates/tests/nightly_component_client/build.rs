use std::collections::HashMap;
use std::fs::*;
use std::io::prelude::*;
use std::process::*;
use windows_bindgen::*;

fn main() -> std::io::Result<()> {
    create_dir_all(".windows/winmd")?;
    copy("../nightly_component/.windows/winmd/component.winmd", ".windows/winmd/component.winmd")?;

    std::fs::remove_dir_all("src/bindings").ok();
    std::fs::create_dir("src/bindings")?;

    let gen = Gen { namespace: "test_nightly_component", component: true, ..Default::default() };
    let mut bindings = File::create("src/bindings/test_nightly_component.rs")?;
    bindings.write_all(gen_namespace(&gen).as_bytes())?;
    drop(bindings);

    Command::new("rustfmt").arg("src/bindings/test_nightly_component.rs").status()?;

    let mut class_map = HashMap::new();
    let _ = class_map.insert("Microsoft.Windows.System.Power.PowerManager".to_string(), "test_nightly_component.dll".to_string());
    let gen = Gen { namespace: "Microsoft.Windows.System.Power", component: true, class_map, ..Default::default() };
    let mut bindings = File::create("src/bindings/microsoft_windows_system_power.rs")?;
    bindings.write_all(gen_namespace(&gen).as_bytes())?;
    drop(bindings);

    Command::new("rustfmt").arg("src/bindings/microsoft_windows_system_power.rs").status()?;
    Ok(())
}
