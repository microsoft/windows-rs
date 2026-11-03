# RDL

> A Rust-like source format for Windows metadata.

- [windows-rdl getting started](https://github.com/microsoft/windows-rs/blob/master/crates/libs/rdl/readme.md)
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
        #[guid(auto)]
        interface ISprocket {
            fn GetStatus(&self) -> SprocketStatus;
            fn Spin(&self, #[in] speed: f32);
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

Generated RDL sorts attributes alphabetically by their full names, using case-sensitive ordering.
Repeated attributes with the same name retain their relative order; their arguments are not sorted.
The shared formatter applies this rule to declarations, parameters, fields, and return types.

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

        #[guid(auto)]
        interface ISprocketFactory {
            fn CreateSprocket(&self, #[in] teeth: u32, #[in] diameter: f32) -> Sprocket;
            fn GetStatus(&self, #[in] s: Sprocket) -> SprocketStatus;
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

Enum member names may be keywords such as `Self` or `self`, or `_`. These names are preserved
in metadata; `Self` and `Self_` are distinct members. Raw identifiers such as `r#type` are also
accepted. Use the numeric value when referring to `_` in an attribute argument.

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

Inline Win32 records implicitly retain anonymous-type provenance. Shared or indirectly referenced
anonymous records use `#[anonymous(N)]` on a nested declaration, where `N` is the first owning native
field position. This is distinct from a named record whose spelling resembles a generated name.

Win32 records can own constants without adding instance storage:

```rust
#[win32]
mod Example {
    struct Palette {
        channel: u32,
        const Blue: u32 = 0xff0000ff;
    }
}
```

Constants preserve their values and static/literal flags through metadata round trips. They share
their record's architecture. Nested enums are not supported.

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

#### Bitfields

A backing field can contain logical bitfield members. Offsets accumulate from zero; anonymous
padding advances the offset without declaring a member:

```rust
struct Options {
    storage: u32 {
        enabled: bool: 1,
        _: 2,
        mode: Mode: 3,
        legacy: 2,
    },
}
```

`name: Type: width` preserves a member type independently of the backing field. Primitive, enum
and typedef names are supported. `name: width` retains the untyped form; it does not add a
`MemberType` property. Anonymous padding uses `_: width` and cannot have a member type.
Both forms encode `NativeBitfieldAttribute` with the member name, offset and width. Typed members
also encode its `MemberType` property. The full attribute spelling remains accepted. The writer
keeps full attributes for overlapping members, unrepresentable offsets or additional properties.
Names that cannot round-trip as identifiers use string literals, such as `"self": i32: 1`.
`"_": i32: 1` is a named member, distinct from anonymous padding.

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

Interfaces define method contracts. GUID generation is explicit:

| Source | Metadata |
| --- | --- |
| No GUID attribute on a native interface | No `GuidAttribute`; base-interface GUIDs are not inherited |
| `#[guid(0x...)]` | The specified 128-bit GUID |
| `#[guid(auto)]` | A deterministic GUID derived from the namespace, name and methods |

WinRT interfaces and delegates require `guid(...)` or `guid(auto)`. Native interfaces can omit
both. `#[no_guid]` is not supported. Duplicate GUID attributes, including combinations of explicit
and automatic GUIDs, are errors.

Automatic generation is for authoring identities, not recovering identities of existing native
APIs. Its value can change when the declaration changes. Use an explicit GUID when the identity
must remain fixed. WinMD stores only the resulting value; the writer emits that literal and never
infers `guid(auto)` from it. A stored zero GUID remains an explicit zero, not an absent attribute.

Syntax:

```rust
interface InterfaceName {
    fn MethodName(&self, #[in] Parameter: Type) -> ReturnType;
}
```

Methods use `fn` and require `&self` as the first parameter. Use `->` for a return type. Omit it for
void.

Example:

```rust
#[guid(auto)]
interface ISprocket {
    fn GetStatus(&self) -> SprocketStatus;
    fn Spin(&self, #[in] speed: f32);
    fn Stop(&self);
}
```

Generic interfaces in WinRT:

In a `#[winrt]` module, interfaces can have type parameters. WinRT collections and event handlers
use them.

Syntax:

```rust
#[guid(auto)]
interface InterfaceName<Type1, Type2> {
    fn MethodName(&self, #[in] Parameter: Type1) -> Type2;
}
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        #[guid(auto)]
        interface ICatalog<K, V> {
            fn Get(&self, #[in] key: K) -> V;
            fn Insert(&self, #[in] key: K, #[in] value: V);
            fn Remove(&self, #[in] key: K) -> bool;
        }
    }
}
```

#### Delegates

Delegates define callable types with method-like semantics.

Syntax:

```rust
#[guid(auto)]
delegate fn DelegateName(#[in] Parameter: Type) -> ReturnType;
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        #[guid(auto)]
        delegate fn SprocketStatusChanged(#[in] status: SprocketStatus);
    }
}
```

Generic delegates in WinRT:

In a `#[winrt]` module, delegates can also have type parameters.

Syntax:

```rust
#[guid(auto)]
delegate fn DelegateName<Type1, ...>(#[in] Parameter: Type1, ...) -> ReturnType;
```

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        #[guid(auto)]
        delegate fn SprocketEventHandler<T>(#[in] sender: IObject, #[in] event: T);
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

Classes are sealed unless marked `#[unsealed]`, and a class can only extend an unsealed class.
`#[unsealed]` sets the metadata type's flags rather than adding a custom attribute. A class without
interfaces, such as a static class, is also marked abstract because it has no instances.

Example:

```rust
#[winrt]
mod Contoso {
    mod Sprockets {
        #[unsealed]
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

        #[guid(auto)]
        interface ISprocket {}
        #[guid(auto)]
        interface ISprocketFactory {}
        #[guid(auto)]
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
fn FunctionName(#[in] Parameter: Type, ...) -> ReturnType;
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
| Neither `#[in]` nor `#[out]` | Direction is unspecified (Win32 only). | No unconditional direction evidence |

Direction is always explicit. Pointer mutability, reference types and named aliases do not imply
`#[in]` or `#[out]`. In Win32 declarations, omitting both leaves both ECMA-335 direction flags
unset. `#[opt]` is independent and does not supply a direction.

WinRT interface methods and delegates require exactly one of `#[in]` or `#[out]` on every explicit
parameter other than `&self`. Missing or combined directions are errors at the parameter's source
location. Return values do not need direction attributes. Property and event shorthand supplies
the input flags for its implicit parameters.

For Win32 declarations, write directions whenever the API contract establishes them. When `#[in]`
and `#[out]` appear on one parameter, they map to `_Inout_`. Leave direction unspecified when it
is unknown; do not add `#[in]` merely to obtain a more convenient projection.

The formatter writes `#[in]`. The reader also accepts the Rust raw-identifier spelling
`#[r#in]`.

The writer emits every established direction flag and no marker for missing direction. The
`direction(unspecified)` attribute is not supported. Inputs relying on implicit directions must
materialize their old flags before using this reader; the metadata encoding itself is unchanged.

Example:

```rust
#[win32]
mod Windows {
    mod Win32 {
        mod Api {
            #[library("example.dll")]
            extern fn ReadBuffer(
                #[in] data: *mut i32,    // _In_ - input despite mutable pointer
                #[in] count: i32,
            );

            #[library("example.dll")]
            extern fn Transform(
                #[in] #[out] value: *mut i32,  // _Inout_
            );

            #[library("example.dll")]
            extern fn LookupByName(
                #[in] #[opt] name: *const i8,  // _In_opt_
            );
        }
    }
}
```

When `windows-clang` parses Windows SDK headers, it extracts SAL annotations automatically. It emits
the matching direction attributes in generated RDL. Supported SAL macros include `_In_`, `_Out_`,
`_Inout_`, `_In_opt_`, `_Out_opt_`, `_Inout_opt_`, `_Outptr_`, `_COM_Outptr_`, `_In_reads_`,
`_Out_writes_`, and their opt, z, and bytes variants.

Raw pointer chains preserve constness at each depth. `*mut *const T` points to a mutable pointer
whose pointee is const; `*const *mut T` points to a const pointer whose pointee is mutable.
These distinctions survive RDL/WinMD round trips, including chains inside references and fixed
arrays. Mixed chains require a metadata reader that understands per-level pointer modifiers;
homogeneous chains retain their existing encoding.

On parameters, `#[buffer(count)]` describes the buffer extent supplied at the start of the call.
Counts bind to names in the owning signature. The contract does not specify exact allocation sizes,
ownership, direction, optionality, termination, initialized output length, or a success condition.

| Spelling | Contract |
| --- | --- |
| `#[buffer(count)]` | The incoming value of `count` gives the extent in elements. |
| `#[buffer(16)]` | The caller supplies an extent of 16 elements. |
| `#[buffer(bytes(*size))]` | The incoming value of `*size` gives the extent in bytes. |

Counts are an unsuffixed nonnegative integer literal, an integer parameter, or one dereference of
a non-optional integer-pointer parameter. `bytes(...)` changes the unit from elements to bytes.
Arithmetic, return counts, field counts, phase arguments and fallback expressions are not accepted.
The owner must have pointer or array storage, and each parameter can have at most one `buffer`.

The short form encodes `BufferAttribute`: exactly one of `CountConst` (`u64`) or `CountParamIndex`
(`u16`), with `Dereference` for a pointer count and `Bytes` for byte units. Indices are zero-based
signature positions. Absent Boolean properties mean false. Unknown properties, repeated contracts,
unsupported owners and unrenderable counts retain full attribute spelling rather than acquiring
different semantics. The short form does not apply to fields, return rows or containing records.

`#[must_use]` marks a callable result that callers should use rather than accidentally discard.
It applies to imports, interface methods and named callbacks and encodes the parameterless
`Windows.Win32.Metadata.MustUseAttribute`. It does not define success, change the return type,
imply purity or promise a particular language's diagnostics. Message arguments are not supported.
Output-parameter inspection and conditional result checking are separate contracts.

`#[incomplete]` marks a parameter whose header states contracts that metadata does not express. It
encodes the parameterless `Windows.Win32.Metadata.IncompleteAttribute`. Consumers must not infer
contracts, such as a buffer shape or a returned value, from facts that are absent on such a
parameter.

`#[override_slot(N)]` marks a Win32 interface method that reuses absolute inherited virtual slot
`N`. It encodes `NativeOverrideSlotAttribute` and clears the method's `NewSlot` flag.
`#[align(N)]` on a field encodes the existing `AlignmentAttribute` without changing the field type;
record alignment uses the same spelling.

| Spelling | Encodes |
| --- | --- |
| `#[deprecated]`, `#[deprecated("message")]` | `DeprecatedAttribute`; `TypedefName = "..."` names a relocated typedef owner |
| `#[base]` | `NativeBaseAttribute` on a field holding C++ base class storage |

#### Null termination

The argument-free markers describe termination of a parameter's direct buffer:

| Attribute | Representation |
| --- | --- |
| `#[null_terminated]` | A buffer terminated by one zero element. |
| `#[null_null_terminated]` | A buffer terminated by two consecutive zero elements, typically a packed string list. |

They map to the argument-free `NullTerminatedAttribute` and `NullNullTerminatedAttribute`. Use only one
marker. Neither specifies encoding, capacity, initialized length, optionality or success conditions.
They do not describe a pointer stored through the parameter.

The selected storage must be a pointer, reference or array after resolving aliases. Scalars and
callback values are not buffer storage. Fields, return rows and declarations have no short-form
termination contract; their explicit full attributes remain readable and retain full spelling.
Arguments and repeated or combined markers are errors. Direction is not inferred. The writer retains
full spelling for unsupported storage, repeated attributes and attributes with payloads.

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
        #[guid(auto)]
        interface ISprocketInventory {
            // Return an array of sprocket IDs
            fn GetAllIds(&self) -> [u32];
            
            // Input array - filter by sizes
            fn FilterBySize(&self, #[in] sizes: [f32]) -> u32;
            
            // Output array - collect names
            fn CollectNames(&self, #[out] output: &mut [String]);
            
            // Combined input/output
            fn Transform(&self, #[in] input: [u32], #[out] output: &mut [Sprocket]);
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
            fn Read(&self, #[in] buffer: [u8; 256]) -> u32;
            fn Write(&self, #[out] buffer: &mut [u8; 256]) -> u32;
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
