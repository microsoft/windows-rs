# Rust Definition Language (RDL) Documentation

RDL provides a Rust-like syntax for defining Windows types and APIs. It serves as an interface definition language for generating bindings and implementations for use with any language across the Windows ecosystem.

## Quick Start

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

## Language Reference

### Basic Syntax

#### Comments

```rust
// Single-line comments start with //

/* Multi-line comments
   use C-style syntax */
```

---

### Attributes

#### `#[winrt]`

The `#[winrt]` attribute marks a module as containing WinRT types. When applied to a module, it enables WinRT-specific features like generic interfaces, generic delegates, and WinRT-style arrays.

**Syntax:**

```rust
#[winrt]
mod ModuleName {
    /* ... */
}
```

**Example:**

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        /* ... */
    }
}
```

---

#### `#[win32]`

The `#[win32]` attribute marks a module as containing Win32 types. When applied to a module, it enables Win32-specific features like fixed-size arrays and union types.

**Syntax:**

```rust
#[win32]
mod ModuleName {
    /* ... */
}
```

**Example:**

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        /* ... */
    }
}
```

---

### Type Definitions

#### Modules (Namespaces)

Modules provide a way to group types and APIs, simulating namespaces.

**Syntax:**

```rust
mod ModuleName {
    /* ... */
}
```

Modules may be nested.

**Example:**

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

---

#### Enums

Enums define a set of named constants.

**Syntax:**

```rust
#[repr(type)]
enum EnumName {
    Variant1 = value1,
    Variant2 = value2,
}
```

The `#[repr(type)]` attribute specifies the underlying integer type. Supported types include `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, and `u64`.

**Example:**

```rust
#[repr(i32)]
enum SprocketStatus {
    Idle = 0,
    Spinning = 1,
    Locked = 2,
    Malfunctioning = 3,
}
```

---

#### Structs

Structs define composite data types with named fields.

**Syntax:**

```rust
struct StructName {
    FieldName: Type,
    // ...
}
```

**Example:**

```rust
struct Sprocket {
    TeethCount: u32,
    Diameter: f32,
}
```

**Fixed-Size Arrays (Win32):**

In a `#[win32]`-annotated module, structs can include fixed-size arrays. This is useful for low-level data structures like buffers.

**Syntax:**

```rust
struct StructName {
    FieldName: [type; size],
    // ...
}
```

**Example:**

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

---

#### Unions

Unions define a type where all fields share the same memory location. All fields start at offset zero, which makes unions useful for representing data that can be interpreted in multiple ways.

**Syntax:**

```rust
union UnionName {
    FieldName: Type,
}
```

**Example:**

```rust
union SprocketHandle {
    AsInt: i32,
    AsFloat: f32,
    AsBytes: [u8; 4],
}
```

---

#### Interfaces

Interfaces define contracts for method implementations.

**Syntax:**

```rust
interface InterfaceName {
    fn MethodName(&self, Parameter: Type) -> ReturnType;
}
```

Methods use the `fn` keyword and require `&self` as the first parameter. Return type is specified with `->` (omit for void/no return).

**Example:**

```rust
interface ISprocket {
    fn GetStatus(&self) -> SprocketStatus;
    fn Spin(&self, speed: f32);
    fn Stop(&self);
}
```

**Generic Interfaces (WinRT):**

In a `#[winrt]`-annotated module, interfaces can have type parameters. This is essential for WinRT collections and event handlers.

**Syntax:**

```rust
interface InterfaceName<Type1, Type2> {
    fn MethodName(&self, Parameter: Type1) -> Type2;
}
```

**Example:**

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

---

#### Delegates

Delegates define callable types. These are essentially function pointers with method-like semantics.

**Syntax:**

```rust
delegate fn DelegateName(Parameter: Type) -> ReturnType;
```

**Example:**

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        delegate fn SprocketStatusChanged(status: SprocketStatus);
    }
}
```

**Generic Delegates (WinRT):**

In a `#[winrt]`-annotated module, delegates can also have type parameters.

**Syntax:**

```rust
delegate fn DelegateName<Type1, ...>(Parameter: Type1, ...) -> ReturnType;
```

**Example:**

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        delegate fn SprocketEventHandler<T>(sender: Object, event: T);
    }
}
```

---

#### Classes

Classes define concrete WinRT types that implement one or more interfaces.

**Syntax:**

```rust
class ClassName {
    InterfaceName,
}
```

Classes may optionally extend a base class and mark a default interface.

**Example:**

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        class Sprocket {
            #[default]
            ISprocket,
        }

        #[activatable(1)]
        class ActivatableSprocket: Sprocket {
            #[default]
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

The `#[activatable(1)]` attribute on the class enables default construction. The `#[default]` attribute on an interface marks it as the default interface. The `#[activatable(1)]` and `#[statics(1)]` attributes on interfaces mark them as factory and statics interfaces, respectively.

---

#### Attributes

Attributes define custom metadata annotations. Each constructor is defined with the `fn` keyword and a parameter list.

**Syntax:**

```rust
attribute AttributeName {
    fn(Parameter: Type, ...);
}
```

**Example:**

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        attribute SprocketAttribute {
            fn(version: u32);
            fn(version: u32, name: String);
        }
    }
}
```

---

#### Constants

Constants define named values of a primitive type or `GUID`.

**Syntax:**

```rust
const NAME: Type = value;
```

**Example:**

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        const MAX_TEETH: u32 = 256;
        const MIN_TEETH: i32 = -1;
        const SPROCKET_ID: GUID = 0xe436ebb1_524f_11ce_9f53_0020af0ba770;
    }
}
```

---

#### Functions

Functions declare external function signatures provided by another library. The `#[link]` attribute specifies the library name and ABI.

**Syntax:**

```rust
#[link(name = "library", abi = "[system|C]")]
fn FunctionName(Parameter: Type, ...) -> ReturnType;
```

The `#[link(name = "...")]` attribute specifies the library that provides the API, while `#[link(abi = "...")]` specifies the ABI (either `"system"` or `"C"`).

**Example:**

```rust
#[win32]
mod Contoso {
    mod Sprockets {
        #[link(name = "sprockets.dll", abi = "system")]
        fn InitializeSprocketFactory();

        #[link(name = "sprockets.dll", abi = "C")]
        fn GetSprocketCount() -> u32;
    }
}
```

---

### Array Types

#### WinRT Arrays (Dynamic Arrays)

WinRT-style arrays are dynamic arrays managed by the runtime. There are three ways to use them:

1. **Input arrays** - Passed as input, like slices in Rust (read-only)
2. **Output arrays** - Allocated by the callee, freed by the caller using `CoTaskMemFree`
3. **Return arrays** - Returned from methods, also managed with `CoTaskMemFree`

**Example:**

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

Note that WinRT arrays only work in `#[winrt]` annotated modules. For fixed-size arrays, see the next section.

---

#### Fixed-Size Arrays (Win32)

Fixed-size arrays have a compile-time known size. They can appear in struct fields and method parameters, but only in `#[win32]` modules.

**Example:**

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

---

### Built-in Types

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
| `Object`   | WinRT object reference             |
| `Type`     | System type reference              |
| `GUID`     | Globally unique identifier         |
| `HRESULT`  | Windows error code                 |

---

### Pointer and Reference Types

| RDL Type     | Description                   |
|--------------|-------------------------------|
| `*mut T`     | Mutable raw pointer to T      |
| `*const T`   | Const raw pointer to T        |
| `&mut T`     | Mutable reference to T        |
| `&T`         | Const reference to T          |

---
