//! Builds `harness/cswinrt.csproj` for each fixture: CsWinRT projects the fixture's metadata and
//! the C# compiler builds the projection with the fixture's optional `client.cs`. If the fixture
//! has a component, the client is run against it with `dotnet run`.

#![cfg(windows)]

mod support;
use std::process::Command;
use support::*;

include!(concat!(env!("OUT_DIR"), "/fixtures.rs"));

fn run(name: &str, namespace: &str) {
    let fixture = Fixture::new("cswinrt", name);
    let project = format!("{HARNESS}/cswinrt.csproj");
    let mut dotnet = Command::new("dotnet");

    match (fixture.file("client.cs"), fixture.component()) {
        (Some(client), Some(component)) => {
            // The client process must match the component's architecture.
            let (runtime, platform) = match env!("TARGET").split('-').next().unwrap() {
                "x86_64" => ("win-x64", "x64"),
                "i686" => ("win-x86", "x86"),
                "aarch64" => ("win-arm64", "arm64"),
                arch => panic!("unsupported architecture `{arch}`"),
            };

            dotnet
                .args(["run", "--project", &project, "-r", runtime])
                .arg(format!("--property:PlatformTarget={platform}"))
                .arg(format!("--property:FixtureClient={}", client.display()))
                .arg(format!(
                    "--property:FixtureComponent={}",
                    component.display()
                ));
        }
        (client, _) => {
            dotnet.args(["build", &project]);

            if let Some(client) = client {
                dotnet.arg(format!("--property:FixtureClient={}", client.display()));
            }
        }
    }

    dotnet
        .args(["--disable-build-servers", "--artifacts-path"])
        .arg(&fixture.out)
        .arg(format!("--property:FixtureNamespace={namespace}"))
        .arg(format!(
            "--property:FixtureWinmd={}",
            fixture.winmd.display()
        ));

    run_tool(dotnet);
}
