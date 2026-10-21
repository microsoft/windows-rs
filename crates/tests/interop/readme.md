# test_interop

Checks that metadata produced by `windows-rdl` works across languages: each fixture API is
implemented by a Rust or C++/WinRT component and called from C++/WinRT, C#/WinRT, and windows-rs
clients that check the results at run time.

## Fixtures

Every directory under `fixtures/` is one test case. `build.rs` generates one `#[test]` per
directory in each test target, so adding a fixture needs no Rust code outside the fixture.

| File | Required | Purpose |
| --- | --- | --- |
| `api.rdl` | yes | The API under test. Compiled to `<name>.winmd`. |
| `component.rs` | no | Implements the API in Rust. |
| `component.cpp` | no | Implements the API in C++/WinRT, instead of `component.rs`. |
| `client.cpp` | no | Uses the API through the C++/WinRT projection. |
| `client.cs` | no | Uses the API through the C#/WinRT projection. |
| `client.rs` | no | Uses the API through the windows-rs projection. Requires a component. |

Fixture names are lowercase snake_case and not Rust keywords, so they can be module and test
names, such as `async_methods` rather than `async`. Each fixture declares its types in a namespace
named after the fixture in PascalCase, so `event_generic` uses `mod EventGeneric`. The build fails
with `type or namespace not found` if it does not.

Keep each fixture focused on one shape, such as a generic event or a collection parameter, and name
it after that shape.

The array fixtures cover primitive arrays (`arrays`), pass/fill/receive arrays of strings,
interfaces, and non-blittable structs (`nontrivial_arrays`), and client-implemented delegates with
array parameters and returns (`array_delegates`). The `numerics` fixture checks all seven
`Windows.Foundation.Numerics` structs, including their C# `System.Numerics` mappings.

`boxed_arrays` checks numeric, Boolean, and string arrays boxed on either side of the ABI,
including C# array-to-`Object` conversion and rejection of an incompatible boxed type.
`struct_directions` distinguishes value input, const-reference input, output, and return slots for
blittable and nested non-blittable structs, both in component methods and client delegates.

`error_relay` preserves HRESULTs and messages from client callbacks through the component and back
to the client, including concurrent calls that retain their original errors across intervening
failures. `async_methods` separates synchronous method failures from failed async results,
including custom error details originating in the component or a client-produced operation.

### Component

`component.rs` is a module of this crate's library and starts with `use super::*;`, which brings in
`windows_core`, the generated bindings, and the `activatable` and `check` helpers. It exports the
fixture's activation factories:

```rust
pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Class.Widget").then(activatable::<Widget, Class::Widget>)
}
```

The library exports `DllGetActivationFactory` and routes each class name to the fixture that owns
its namespace. `build.rs` generates the bindings for all fixtures at once with `implement_all`, so
every fixture is also checked against the Rust projection. Bindgen only generates filtered types;
add any other Windows types that fixtures reference to the filters in `build.rs`.

To test calls in the other direction, a fixture declares a non-exclusive interface that each client
implements and passes to a component method, which calls it back. A panic cannot unwind across the
ABI, so the component checks the results with `check(condition)?`, which fails the client's call
with an error naming the component's file and line.

The component and Rust client modules are included through generated `#[path]` attributes, which
`cargo fmt` does not follow. Format them with `rustfmt --edition 2024 fixtures/*/*.rs`.

The Win32 types that the component and Rust clients use are generated into a separate flat module.
Add any others to the Win32 filters in `build.rs`.

### C++ component

Use `component.cpp` for an API that a Rust component cannot implement, such as a composable class
that clients derive from. Each consumer runs `cppwinrt -component` on the fixture metadata and
builds the component into a DLL with `harness/activation.cpp`, which exports
`DllGetActivationFactory`. `component.cpp` includes the generated `<namespace>/<class>.g.h` for
each class, implements it, and defines the factory lookup:

```cpp
void* __stdcall winrt_get_activation_factory(std::wstring_view const& name) {
    if (name == L"Class.Widget") {
        return winrt::detach_abi(winrt::make<winrt::Class::factory_implementation::Widget>());
    }

    return nullptr;
}
```

A C++ component needs the MSVC toolset, like the C++ clients.

### Clients

With a component, clients define an entry point that the harness calls, and fail with a file and
line through the harness check helpers:

| Language | Entry point | Check |
| --- | --- | --- |
| C++ | `void test()` | `check(condition)` from `#include "check.h"` |
| C# | `public static class Client { public static void Test() }` | `Check(condition)` with `using static Harness;` |
| Rust | `pub fn test() -> Result<()>` after `use super::*;` | `assert!` and `?` |

Without a component, clients are compiled but not run, and need no entry point. Without
`client.cpp`, the C++ test compiles the namespace header on its own. C++/WinRT consume methods are
templates, so a client is needed to check member signatures. Without `client.cs`, the C# test
builds the CsWinRT projection alone. The Rust projection is always compiled, so a Rust client is
only needed to run code.

## Consumers

| Test target | Steps |
| --- | --- |
| `tests/cppwinrt.rs` | Runs `cppwinrt` with the fixture metadata and `-ref Windows.winmd`, then compiles the client with `cl /std:c++20 /permissive- /W4 /WX`. With a component, links it with `harness/main.cpp` and runs it. |
| `tests/cswinrt.rs` | Builds `harness/cswinrt.csproj`, which runs CsWinRT on the fixture metadata and compiles the projection and client with warnings as errors. With a component, runs it with `dotnet run` for the target architecture. |
| `tests/rust.rs` | Compiles every `client.rs` into the test with the same generated bindings. Runs the client in-process against the component DLL, which it loads through WinRT activation like the other clients. |

The C++ and C# consumers copy the component next to the client as `<namespace>.dll`, the module
WinRT activation loads for an unregistered class. The Rust consumer copies it into the fixture's
output directory and adds that directory to the DLL search path. Each test gets a clean output
directory under `OUT_DIR/<consumer>/<fixture>`, so tests can run in parallel.

### Component build

Cargo does not give tests the path of a cdylib it builds, and where it puts one depends on the
build directory layout. The first test that needs the component runs a nested `cargo build --lib`
of this crate with its own target directory under `OUT_DIR`, which puts the DLL at a known path.
The nested build sets `TEST_WINRT_INTEROP_COMPONENT` so `build.rs` skips the outputs only the tests
use. The first build compiles the dependencies; later runs are up-to-date checks.

## Running

```sh
cargo test -p test_interop
cargo test -p test_interop --test cppwinrt
cargo test -p test_interop --test cswinrt event_generic
cargo test -p test_interop --test rust
```

The C++ tests need the MSVC toolset. The C# tests need the .NET 10 SDK, a .NET 10 runtime for the
target architecture, and restore the `Microsoft.Windows.CsWinRT` package from NuGet.
