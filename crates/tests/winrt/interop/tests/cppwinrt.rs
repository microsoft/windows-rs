//! Projects each fixture with C++/WinRT and compiles its `client.cpp` against the projection. If
//! the fixture has a component, the client is linked with `harness/main.cpp` and run against it.

#![cfg(target_env = "msvc")]

mod support;
use support::*;

include!(concat!(env!("OUT_DIR"), "/fixtures.rs"));

fn run(name: &str, namespace: &str) {
    let fixture = Fixture::new("cppwinrt", name);

    cppwinrt::cppwinrt([
        "-in".as_ref(),
        fixture.winmd.as_os_str(),
        "-ref".as_ref(),
        WINDOWS_WINMD.as_ref(),
        "-out".as_ref(),
        fixture.out.as_os_str(),
    ]);

    let mut cl = cl();
    cl.current_dir(&fixture.out).arg("/I").arg(&fixture.out);

    match (fixture.file("client.cpp"), fixture.component()) {
        (Some(client), Some(component)) => {
            cl.arg(format!("{HARNESS}/main.cpp")).arg(client).args([
                "/Feclient.exe",
                "/link",
                "windowsapp.lib",
            ]);

            run_tool(cl);

            std::fs::copy(component, fixture.out.join(format!("{namespace}.dll"))).unwrap();
            run_tool(std::process::Command::new(fixture.out.join("client.exe")));
        }
        (client, _) => {
            let client = client.unwrap_or_else(|| {
                let client = fixture.out.join("client.cpp");
                std::fs::write(&client, format!("#include <winrt/{namespace}.h>\n")).unwrap();
                client
            });

            cl.arg("/c").arg(client);
            run_tool(cl);
        }
    }
}
