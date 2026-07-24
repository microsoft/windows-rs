# RDL

> A Rust-like source format for Windows metadata.

- [windows-rdl crate docs](../../../docs/crates/windows-rdl.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

`windows-rdl` parses RDL (Rust Definition Language), a small Rust-like syntax for Windows APIs. It
emits ECMA-335 `.winmd` metadata for `windows-bindgen`. It also writes canonical RDL from `.winmd`
files.

## Getting started

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        // Define an interface
        interface ISprocket {
            fn GetStatus(&self) -> SprocketStatus;
            fn Spin(&self, speed: f32);
        }

        // Define a struct
        struct Sprocket {
            TeethCount: u32,
            Diameter: f32,
        }

        // Define an enum with explicit discriminants
        #[repr(i32)]
        enum SprocketStatus {
            Idle = 0,
            Spinning = 1,
            Locked = 2,
            Malfunctioning = 3,
        }
    }
}
```

## RDL syntax

### Basic syntax

#### Comments

```rust
// Single-line comments start with //

/* Multi-line comments
   use C-style syntax */
```

### Attributes

#### `#[winrt]`

Use `#[winrt]` on a module that declares WinRT types. It enables generic interfaces, generic
delegates, and WinRT arrays.

Syntax:

```rust
#[winrt]
mod ModuleName {
    /* ... */
}
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        /* ... */
    }
}
```

#### `#[win32]`

Use `#[win32]` on a module that declares Win32 types. It enables fixed arrays and unions.

Syntax:

```rust
#[win32]
mod ModuleName {
    /* ... */
}
```

Example:

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        /* ... */
    }
}
```

### Type definitions

#### Modules and namespaces

Modules group types and APIs. A top-level module maps to a metadata namespace.

Syntax:

```rust
mod ModuleName {
    /* ... */
}
```

Modules may be nested.

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        struct Sprocket {
            TeethCount: u32,
            Diameter: f32,
        }

        #[repr(i32)]
        enum SprocketStatus {
            Idle = 0,
            Spinning = 1,
            Malfunctioning = 2,
        }

        interface ISprocketFactory {
            fn CreateSprocket(&self, teeth: u32, diameter: f32) -> Sprocket;
            fn GetStatus(&self, s: Sprocket) -> SprocketStatus;
        }
    }
}
```

#### Enums

Enums define named constants.

Syntax:

```rust
#[repr(type)]
enum EnumName {
    Variant1 = value1,
    Variant2 = value2,
}
```

`#[repr(type)]` sets the underlying integer type. Supported types are `i8`, `u8`, `i16`, `u16`,
`i32`, `u32`, `i64`, and `u64`.

Example:

```rust
#[repr(i32)]
enum SprocketStatus {
    Idle = 0,
    Spinning = 1,
    Locked = 2,
    Malfunctioning = 3,
}
```

#### Structs

Structs define data types with named fields.

Syntax:

```rust
struct StructName {
    FieldName: Type,
    // ...
}
```

Example:

```rust
struct Sprocket {
    TeethCount: u32,
    Diameter: f32,
}
```

Fixed-size arrays in Win32:

In a `#[win32]` module, structs can include fixed-size arrays. Use them for buffers and other
fixed-layout data.

Syntax:

```rust
struct StructName {
    FieldName: [type; size],
    // ...
}
```

Example:

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        struct SprocketDataBuffer {
            HeaderSize: u32,
            Data: [u8; 256],
        }
    }
}
```

#### Unions

Unions define fields that share one memory location. All fields start at offset zero.

Syntax:

```rust
union UnionName {
    FieldName: Type,
}
```

Example:

```rust
union SprocketHandle {
    AsInt: i32,
    AsFloat: f32,
    AsBytes: [u8; 4],
}
```

#### Interfaces

Interfaces define method contracts.

Syntax:

```rust
interface InterfaceName {
    fn MethodName(&self, Parameter: Type) -> ReturnType;
}
```

Methods use `fn` and require `&self` as the first parameter. Use `->` for a return type. Omit it for
void.

Example:

```rust
interface ISprocket {
    fn GetStatus(&self) -> SprocketStatus;
    fn Spin(&self, speed: f32);
    fn Stop(&self);
}
```

Generic interfaces in WinRT:

In a `#[winrt]` module, interfaces can have type parameters. WinRT collections and event handlers
use them.

Syntax:

```rust
interface InterfaceName<Type1, Type2> {
    fn MethodName(&self, Parameter: Type1) -> Type2;
}
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        interface ICatalog<K, V> {
            fn Get(&self, key: K) -> V;
            fn Insert(&self, key: K, value: V);
            fn Remove(&self, key: K) -> bool;
        }
    }
}
```

#### Delegates

Delegates define callable types with method-like semantics.

Syntax:

```rust
delegate fn DelegateName(Parameter: Type) -> ReturnType;
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        delegate fn SprocketStatusChanged(status: SprocketStatus);
    }
}
```

Generic delegates in WinRT:

In a `#[winrt]` module, delegates can also have type parameters.

Syntax:

```rust
delegate fn DelegateName<Type1, ...>(Parameter: Type1, ...) -> ReturnType;
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        delegate fn SprocketEventHandler<T>(sender: IObject, event: T);
    }
}
```

#### Classes (WinRT)

```rust
class ClassName : BaseClassName {
    ImplementedInterface1,
    ImplementedInterface2
}
```

Classes can extend a base class. The first interface in the list is the default interface.

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        class Sprocket {
            ISprocket,
        }

        #[activatable(1)]
        class ActivatableSprocket: Sprocket {
            ISprocket,
            #[activatable(1)]
            ISprocketFactory,
            #[statics(1)]
            ISprocketStatics,
        }

        interface ISprocket {}
        interface ISprocketFactory {}
        interface ISprocketStatics {}
    }
}
```

`#[activatable(...)]` on a class enables default construction. `#[activatable(...)]` and
`#[statics(...)]` on interfaces mark factory and statics interfaces.

#### Attributes

Attributes define metadata annotations. Each constructor uses `fn` and a parameter list.

Syntax:

```rust
attribute AttributeName {
    fn(Parameter: Type, ...);
}
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        attribute Discontinued {
            fn(version: u32);
            fn(version: u32, note: String);
        }
    }
}
```

#### Constants

Constants define named primitive values or `GUID` values.

Syntax:

```rust
const Name: Type = value;
```

Example:

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        const MAX_TEETH: u32 = 256;
        const MIN_TEETH: u32 = 1;
        const SPROCKET_MACHINE_ID: GUID = 0xe436ebb1_524f_11ce_9f53_0020af0ba770;
    }
}
```

#### Functions

Functions declare external signatures from another library. The `#[link]` attribute sets the library
name and ABI.

Syntax:

```rust
#[link(name = "library", abi = "[system|C]")]
fn FunctionName(Parameter: Type, ...) -> ReturnType;
```

`#[link(name = "...")]` names the library that provides the API. `#[link(abi = "...")]` sets the
ABI: `"system"` or `"C"`.

Example:

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        mod Platform {
            #[link(name = "sprockets.dll", abi = "system")]
            fn InitializeSprocketFactory();
            
            #[link(name = "sprockets.dll", abi = "C")]
            fn GetSprocketCount() -> u32;
        }
    }
}
```

#### Parameter direction attributes

Parameters can carry direction and optional attributes that map to Win32 SAL annotations. These
attributes control generated binding behavior.

| Attribute | Meaning | Corresponding SAL |
|-----------|---------|-------------------|
| `#[in]` | Input parameter. Data flows into the function. | `_In_`, `_In_z_`, and related forms |
| `#[out]` | Output parameter. Data flows out of the function. | `_Out_`, `_Out_z_`, and related forms |
| `#[opt]` | Parameter is optional and can be `NULL`. | `_In_opt_`, `_Out_opt_`, `_Inout_opt_`, and related forms |

When neither `#[in]` nor `#[out]` is set, the reader infers direction from the type: mutable
pointers and references (`*mut T`, `&mut T`) default to `#[out]`. Everything else defaults to
`#[in]`. Use explicit attributes only when the SAL annotation differs from the inferred direction.

When `#[in]` and `#[out]` appear on one parameter, they map to `_Inout_`. The parameter is both
input and output.

Example:

```rust
#[win32]
mod Windows {
    mod Win32 {
        mod Api {
            #[library("example.dll")]
            extern fn ReadBuffer(
                #[in] data: *mut i32,    // _In_ - input despite mutable pointer
                count: i32,              // plain In (inferred)
            );

            #[library("example.dll")]
            extern fn Transform(
                #[in] #[out] value: *mut i32,  // _Inout_
            );

            #[library("example.dll")]
            extern fn LookupByName(
                #[opt] name: *const i8,  // _In_opt_
            );
        }
    }
}
```

When `windows-clang` parses Windows SDK headers, it extracts SAL annotations automatically. It emits
the matching direction attributes in generated RDL. Supported SAL macros include `_In_`, `_Out_`,
`_Inout_`, `_In_opt_`, `_Out_opt_`, `_Inout_opt_`, `_Outptr_`, `_COM_Outptr_`, `_In_reads_`,
`_Out_writes_`, and their opt, z, and bytes variants.

### Array types

#### WinRT arrays

WinRT arrays are dynamic arrays managed by the runtime. RDL supports three forms:

1. Input arrays - passed as read-only input.
2. Output arrays - allocated by the callee and freed by the caller with `CoTaskMemFree`.
3. Return arrays - returned from methods and managed with `CoTaskMemFree`.

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        interface ISprocketInventory {
            // Return an array of sprocket IDs
            fn GetAllIds(&self) -> [u32];
            
            // Input array - filter by sizes
            fn FilterBySize(&self, sizes: [f32]) -> u32;
            
            // Output array - collect names
            fn CollectNames(&self, output: &mut [String]);
            
            // Combined input/output
            fn Transform(&self, input: [u32], output: &mut [Sprocket]);
        }
    }
}
```

WinRT arrays only work in `#[winrt]` modules. Use fixed-size arrays for `#[win32]` modules.

#### Fixed-size arrays

Fixed-size arrays have a compile-time size. They can appear in struct fields and method parameters
in `#[win32]` modules.

Example:

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        struct SprocketHeader {
            Signature: [u8; 4],
            Version: u32,
            Reserved: [u8; 24],
        }
        
        interface ISprocketBuffer {
            fn Read(&self, buffer: [u8; 256]) -> u32;
            fn Write(&self, buffer: &mut [u8; 256]) -> u32;
        }
    }
}
```

WinRT does not support fixed arrays.

### Built-in types

| RDL Type   | Description                        |
|------------|------------------------------------|
| `i8`       | 8-bit signed integer               |
| `u8`       | 8-bit unsigned integer             |
| `i16`      | 16-bit signed integer              |
| `u16`      | 16-bit unsigned integer            |
| `i32`      | 32-bit signed integer              |
| `u32`      | 32-bit unsigned integer            |
| `i64`      | 64-bit signed integer              |
| `u64`      | 64-bit unsigned integer            |
| `f32`      | 32-bit float                       |
| `f64`      | 64-bit float                       |
| `isize`    | Pointer-sized signed integer       |
| `usize`    | Pointer-sized unsigned integer     |
| `bool`     | Boolean                            |
| `String`   | String (HSTRING)                   |
| `GUID`     | Globally unique identifier         |
| `HRESULT`  | Windows error code                 |

### Pointer and reference types

| RDL Type     | Description                   |
|--------------|-------------------------------|
| `*mut T`     | Mutable raw pointer to T      |
| `*const T`   | Const raw pointer to T        |
| `&mut T`     | Mutable reference to T        |
| `&T`         | Const reference to T          |
