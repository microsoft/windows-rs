# Creating your first DLL in Rust

As a systems programming language with similar linkage support to that of C and C++, it is quite straightforward to build a DLL in Rust. Rust does however have it's own notion of libraries that are quite different to that of C and C++, so it's just a matter of finding the right configuration to produce the desired output.

As with most Rust projects, you can start with Cargo and get started with a basic template but it's so simple we'll just build it by hand here to see what's involved. Let's create a directory structure as follows:

```
> hello_world
  Cargo.toml
  > src
    lib.rs
```

Just two directories and two files. There's the `hello_world` directory that contains the project as a whole. In that directory we have a `Cargo.toml` file that contains metadata for the project or package, information needed to compile the package:

```toml
[package]
name = "hello_world"
edition = "2021"

[lib]
crate-type = ["cdylib"]
```

At a minimum, the `[package]` section includes the name and Rust edition your package is compiled with.

Rust-only libraries don't generally include a `[lib]`  section. This is necessary when you need to specifically control how the project will be used and linked. In this case, we're using 'cdylib' that represents a dynamic system library and maps to a DLL on Windows.

The `src` sub directory contains the `lib.rs` Rust source file where we can add any functions that we'd like to export from the DLL. Here's a simple example:

```
#[no_mangle]
extern "system" fn HelloWorld() -> i32 {
    123
}
```

The `[no_mangle]` attribute just tells the compiler to disable any name mangling and use the function name verbatim as the exported identifier. The `extern "system"` function qualifier indicates the ABI or calling convention expected for the function. The "system" string represents the system-specific calling convention which generally maps to "stdcall" on Windows. 

And that's it! You can now build the package and it will produce a DLL:

```
> cargo build -p hello_world
```

Cargo will drop the resulting binaries in the target directory where you can then use them from any other programming language:

```
> dir /b target\debug\hello_world.*
hello_world.d
hello_world.dll
hello_world.dll.exp
hello_world.dll.lib
hello_world.pdb
```

Here's a simple example in C++:

```C++
#include <stdint.h>
#include <stdio.h>

extern "C" {
    int32_t __stdcall HelloWorld();
}

int main() {
    printf("%d\n", HelloWorld());
}
```

You can build it with MSVC as follows:

```
cl hello_world.cpp hello_world.dll.lib
```

The dumpbin tool can be used to further inspect imports and exports.

```
> dumpbin /nologo /exports hello_world.dll

Dump of file hello_world.dll

File Type: DLL

  Section contains the following exports for hello_world.dll

    00000000 characteristics
    FFFFFFFF time date stamp
        0.00 version
           1 ordinal base
           1 number of functions
           1 number of names

    ordinal hint RVA      name

          1    0 00001000 HelloWorld = HelloWorld
```

```
> dumpbin /nologo /imports hello_world.exe

Dump of file hello_world.exe

File Type: EXECUTABLE IMAGE

  Section contains the following imports:

    hello_world.dll
             140017258 Import Address Table
             140021200 Import Name Table
                     0 time date stamp
                     0 Index of first forwarder reference

                           0 HelloWorld
```
