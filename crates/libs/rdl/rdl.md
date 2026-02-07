# Rust Definition Language (RDL) Documentation

RDL provides a Rust-like syntax for defining Windows types and APIs. It serves as an interface definition language for generating bindings and implementations for use with any language across the Windows ecosystem.

## Quick Start

```rust
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
```

## Language Reference

### Basic Syntax

#### Comments

```rust
// Single-line comments start with //

/* Multi-line comments
   use C-style syntax */
```

### Type Definitions

#### Modules (Namespaces)

Modules provide a way to group types and APIs, simulating namespaces.

**Syntax:**
```rust
mod ModuleName {
    /* ... */
}
```

Modules may be nested:
```rust
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

**Attributes:**
- `#[repr(type)]` - Specifies the underlying integer type
  - Supported types: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`

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

---

#### Unions

Unions define a type where all fields share the same memory location.

**Syntax:**
```rust
union UnionName {
    FieldName: Type,
}
```

**Example:**
```rust
union SprocketId {
    as_int: i32,
    as_float: f32,
    as_bytes: [u8; 4],
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

---

#### Functions

Functions declare external function signatures provided by another library. The `#[link]` attribute specifies the library name and ABI.

**Syntax:**
```rust
mod ModuleName {
    #[link(name = "library.dll", abi = "system")]
    fn FunctionName(Param: Type) -> ReturnType;
}
```

**Attributes:**
- `#[link(name = "...")]` - Specifies the library that provides the API
- `#[link(abi = "...")]` - Specifies the ABI (`"system"` or `"C"`)

**Example:**
```rust
mod Factory {
    #[link(name = "sensors.dll", abi = "system")]
    fn ReadSprocketSpeed(sprocket_id: u32) -> f32;

    #[link(name = "control.dll", abi = "C")]
    fn GetFactoryTicks() -> u64;
}
```

---

### Built-in Types

| RDL Type | Description             |
|----------|-------------------------|
| `i8`     | 8-bit signed integer    |
| `u8`     | 8-bit unsigned integer  |
| `i16`    | 16-bit signed integer   |
| `u16`    | 16-bit unsigned integer |
| `i32`    | 32-bit signed integer   |
| `u32`    | 32-bit unsigned integer |
| `i64`    | 64-bit signed integer   |
| `u64`    | 64-bit unsigned integer |
| `f32`    | 32-bit float            |
| `f64`    | 64-bit float            |
| `bool`   | Boolean                 |
| `String` | String type             |

### Pointer and Reference Types

| RDL Type     | Description                   |
|--------------|-------------------------------|
| `*mut T`     | Mutable raw pointer to T      |
| `*const T`   | Const raw pointer to T        |
| `&mut T`     | Mutable reference to T        |
| `&T`         | Const reference to T          |

---

## Attributes Reference

### `#[repr(type)]`

Specifies the underlying integer representation of an enum.

**Usage:**
```rust
#[repr(i32)]
enum SprocketStatus {
    Idle = 0,
    Spinning = 1,
}
```

---
