//! Builds `cswinrt.csproj` for each fixture: CsWinRT projects the fixture's metadata and the C#
//! compiler builds the projection together with the fixture's optional `client.cs`.

#![cfg(windows)]

use std::process::Command;
use test_winrt_interop::{Fixture, run_tool};

include!(concat!(env!("OUT_DIR"), "/fixtures.rs"));

fn run(name: &str) {
    let fixture = Fixture::new("cswinrt", name);

    let mut dotnet = Command::new("dotnet");
    dotnet
        .args([
            "build",
            concat!(env!("CARGO_MANIFEST_DIR"), "/cswinrt.csproj"),
        ])
        .args([
            "-nologo",
            "-v:q",
            "--disable-build-servers",
            "--artifacts-path",
        ])
        .arg(&fixture.out)
        .arg(format!("-p:FixtureWinmd={}", fixture.winmd.display()));

    if let Some(client) = fixture.file("client.cs") {
        dotnet.arg(format!("-p:FixtureClient={}", client.display()));
    }

    run_tool(dotnet);
}
