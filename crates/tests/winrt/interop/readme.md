# test_winrt_interop

Checks that metadata produced by `windows-rdl` is accepted by C++/WinRT and C#/WinRT, and that the
resulting projections can be used. Each fixture is a small API plus optional client code for each
language.

## Fixtures

Every directory under `fixtures/` is one test case. `build.rs` generates one `#[test]` per
directory in each test target, so adding a fixture needs no Rust code.

| File | Required | Purpose |
| --- | --- | --- |
| `api.rdl` | yes | The API under test, declared in `mod Test`. Compiled to `api.winmd`. |
| `client.cpp` | no | Uses the API through the C++/WinRT projection. |
| `client.cs` | no | Uses the API through the C#/WinRT projection. |

Fixture names must be lowercase snake_case so they can be test names. Clients are compiled, not
run. Without `client.cpp`, the C++ test compiles `#include <winrt/Test.h>` on its own. C++/WinRT
consume methods are templates, so a client is needed to check member signatures. Without
`client.cs`, the C# test builds the CsWinRT projection alone.

Keep each fixture focused on one shape, such as a generic event or a collection parameter, and name
it after that shape.

## Consumers

| Test target | Steps |
| --- | --- |
| `tests/cppwinrt.rs` | Runs `cppwinrt` with `api.winmd` and `-ref Windows.winmd`, then compiles the client with `cl /std:c++20 /permissive- /W4 /WX`. |
| `tests/cswinrt.rs` | Builds `cswinrt.csproj` with `dotnet build`, which runs CsWinRT on `api.winmd` and compiles the projection and client with warnings as errors. |

Both consumers reference `crates/libs/default/Windows.winmd`. `build.rs` generates the C++/WinRT
projection of the Windows namespaces once and the tests share it. Each test gets a clean output
directory under `OUT_DIR/<consumer>/<fixture>`, so tests can run in parallel. Compiler errors in a
client point at the fixture file and line.

## Running

```sh
cargo test -p test_winrt_interop
cargo test -p test_winrt_interop --test cppwinrt
cargo test -p test_winrt_interop --test cswinrt event_generic
```

The C++ tests need the MSVC toolset. The C# tests need the .NET 10 SDK and restore the
`Microsoft.Windows.CsWinRT` package from NuGet.
