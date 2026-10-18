//! Projects each fixture with C++/WinRT and compiles its optional `client.cpp` against the
//! projection. Without a client, the test compiles the namespace header on its own.

#![cfg(target_env = "msvc")]

use test_winrt_interop::{Fixture, WINDOWS_WINMD, run_tool};

include!(concat!(env!("OUT_DIR"), "/fixtures.rs"));

fn run(name: &str) {
    let fixture = Fixture::new("cppwinrt", name);

    cppwinrt::cppwinrt([
        "-in".as_ref(),
        fixture.winmd.as_os_str(),
        "-ref".as_ref(),
        WINDOWS_WINMD.as_ref(),
        "-out".as_ref(),
        fixture.out.as_os_str(),
    ]);

    let client = fixture.file("client.cpp").unwrap_or_else(|| {
        let client = fixture.out.join("client.cpp");
        std::fs::write(&client, "#include <winrt/Test.h>\n").unwrap();
        client
    });

    let mut cl = cc::windows_registry::find(env!("TARGET"), "cl.exe").expect("cl.exe not found");
    cl.args([
        "/nologo",
        "/c",
        "/std:c++20",
        "/EHsc",
        "/permissive-",
        "/W4",
        "/WX",
    ])
    .arg("/I")
    .arg(&fixture.out)
    .arg("/I")
    .arg(concat!(env!("OUT_DIR"), "/cppwinrt"))
    .arg(format!("/Fo{}", fixture.out.join("client.obj").display()))
    .arg(client);

    run_tool(cl);
}
