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

On parameters, `#[readable(phase, count)]` and `#[writable(phase, count)]` describe accessible
buffer extents. On struct fields, use `#[readable(count)]` and `#[writable(count)]` without a call
phase: the relationship describes a valid struct instance. Counts bind to names in the owning
signature or record. These contracts do not specify exact allocation sizes, ownership, parameter
direction, optionality, termination, or a success condition.

| Spelling | Contract |
| --- | --- |
| `#[readable(entry, count)]` | `count` elements can be read when the call begins. |
| `#[writable(entry, bytes(*size))]` | The entry value of `*size` gives writable byte capacity. |
| `#[readable(success, bytes(*size))]` | The successful-exit value of `*size` gives readable bytes. |
| `#[readable(success, on_entry(*size))]` | Successful readable elements use the entry value of `*size`. |
| `#[readable(success, or(*written, capacity))]` | Use `*written` when non-null, otherwise `capacity`. |
| `#[readable(success, if_present(*written))]` | A null count pointer leaves the extent unknown, not zero. |
| `#[readable(success, unknown(), indirect)]` | The returned pointee has a readable extent whose size is not expressible. |
| `#[readable(success, return + 1)]` | Successful readable elements include one after the result. |
| `#[writable(success, bytes(*size), indirect)]` | The extent describes `*buffer`, not pointer storage. |
| `#[writable(capacity, nullable)]` | A struct's pointer field may be null; otherwise its sibling field gives capacity. |
| `#[readable(length)]` | A struct's buffer field contains the number of readable elements given by its sibling field. |

Unwrapped counts are elements; `bytes(...)` changes the unit. A count can be a nonnegative integer
literal, an integer parameter, one dereference of an integer-pointer parameter, `return`, or an
integer sibling field for a struct-field contract.
`on_entry(...)` selects entry evaluation for successful parameter-count contracts.
`or(*count + 1, fallback)` uses the expression when the count pointer is non-null, otherwise the
by-value integer fallback. The expression must contain exactly one dereferenced count reference.
Field paths are not accepted.
`if_present(*count)` permits unavailable count storage without inventing a fallback. It requires
one dereferenced integer parameter and cannot combine with `or`. `on_entry(if_present(*count))`
uses the incoming count when supplied.

`unknown()` supplies no numeric count. It does not mean zero, one element, or a count that can
be evaluated later. It cannot combine with arithmetic, `on_entry`, `if_present`, `or`, or legacy
offsets. `bytes(unknown())` retains byte units. A parameter named `unknown` remains a normal count
reference without the parentheses.

Counts also support typed integer addition, subtraction, and multiplication, such as
`(capacity - start) * 2`. References retain their owning types; arithmetic uses C integer promotions
and usual conversions. Unsigned arithmetic wraps at its resulting width; signed overflow does not
produce a valid count. `if_present(*written) - start` leaves the count unavailable when `written`
is null. `on_entry(...)` applies to the whole expression.

`count + 1`, `(count + 1)` and `count + 1i32` use the same typed arithmetic and render as `count + 1`.
`if_present(*count + 1)` renders as `if_present(*count) + 1`; a missing count leaves the whole
expression unavailable. Expressions use the versioned `CountExpression` payload.
Legacy nonzero `Offset` payloads and unknown expression versions retain full attribute spelling,
without being reinterpreted as typed arithmetic. No short syntax emits `Offset`.

`#[range(entry, 0..=14 - start)]` constrains an integer parameter at entry.
`#[range(success, >0)]` constrains its successful-exit value. Bounds can be inclusive or exclusive;
one-sided forms include `>`, `>=`, `<`, and `<=`. Record fields omit the phase, as in
`#[range(0..capacity)]`. `#[range(success, 0..=capacity, indirect)]` constrains the pointed-to
integer rather than the pointer value. These forms encode `IntegerRangeAttribute`, independently of
direction, readability, and nullability. Each bound compares against the subject using C promotions
and usual conversions. Unsuffixed literals are `i32`; `i32`, `u32`, `i64`, and `u64` suffixes
preserve explicit types. Floating-point, pointer, and enum subjects are not supported.

Return values carry their range after the arrow:
`extern fn Read(capacity: u32) -> #[range(success, 0..=capacity)] u32;`.
Return ranges require the success phase and no indirection; bounds can refer to the callable's
parameters.

`success` follows the callable's success condition. Field contracts have no call phase.
For buffer extents, `indirect` requires pointer-to-pointer storage and selects the initialized pointer
at the specified phase. `entry` describes the incoming pointer; `success` describes the returned
pointer. When argument storage is supplied, the pointer must be initialized at that phase even
when an `if_present` count is unavailable.
`nullable` describes the selected buffer, not its count pointer, and is allowed on indirect
parameter buffers or record fields. Optional argument storage still uses `opt`.
Without `nullable`, an indirect buffer is non-null at that phase. An extent of zero does not promise
one accessible element. No `out_ptr` attribute is needed to describe a returned buffer.

`#[nullable(entry, indirect)]` separately permits a null incoming pointer without requiring
initialized/readable storage. `nullable` accepts `entry`, `success`, or both, and optional
`indirect`; without `indirect` it describes the parameter itself. It does not infer `in`, `out`,
optional argument storage, ownership, or validity. An absent nullability contract remains
unspecified. These phase-specific forms apply to pointer parameters, not return values or fields.

On a callable return value, `-> #[nullable] T` permits a null result without a phase qualifier.
It applies to the returned value independently of the callable's success condition. On a field,
`#[nullable]` permits null when the containing value is valid. Neither form promises initialized
or readable storage. These spellings encode `NullableAttribute` on their parameter, return or
field rows.

The extent forms encode `BufferExtentAttribute`. Unknown properties, unsupported owners and
unrenderable counts retain full attribute spelling. Short forms currently apply to parameters and
record fields, not return rows or containing records. Existing metadata consumers must interpret
the new attribute before relying on these contracts.

The legacy `#[len_param(N)]` and `#[size_param(N)]` record a raw zero-based parameter position.
They do not refer to a parameter by name. Reordering parameters requires updating the positions.
`#[len_const(N)]` records a constant element count and has no parameter relationship.

`#[out_ptr]` describes storage for a returned pointer. When storage is supplied, success initializes
it to a non-null pointer to one readable element. It does not specify an allocator, transfer
ownership, or turn the parameter into a COM return value.

| Spelling | Additional guarantee or permission |
| --- | --- |
| `#[out_ptr]` | The returned pointer is non-null on success. |
| `#[out_ptr(nullable)]` | The returned pointer may be null on success. |
| `#[out_ptr(null_on_failure)]` | Failure sets the returned pointer to null. |
| `#[out_ptr(nullable, null_on_failure)]` | Both the success-nullability permission and failure guarantee. |

Success and failure follow the callable's contract. Outer `#[opt]` separately permits omitting the
output storage itself; `#[out]` is still needed to establish output direction.
These forms encode both Boolean properties of `NativeReturnedPointerAttribute`. Unknown and
duplicate flags are errors. Full attributes with missing or additional properties retain their
explicit spelling rather than silently acquiring defaults.

`#[must_use]` marks a callable result that callers should use rather than accidentally discard.
It applies to imports, interface methods and named callbacks and encodes the parameterless
`Windows.Win32.Metadata.MustUseAttribute`. It does not define success, change the return type,
imply purity or promise a particular language's diagnostics. Message arguments are not supported.
Output-parameter inspection and conditional result checking are separate contracts.

`#[success(...)]` conditions ordinary postconditions, independently of `must_use`. The supported
expressions are `return == 0`, `return != 0`, `return == 1`, `return >= 0` and `return > 0`.
They encode the corresponding `SuccessCondition` enum value in
`Windows.Win32.Metadata.SuccessConditionAttribute`, not an unevaluated expression string.

On an import, method or named callback, the predicate is an explicit callable condition. On a
value typedef it supplies a return-type default; the callable's explicit condition overrides that
default. Alias links retain defaults without copying them onto every function. Competing or
uninterpreted defaults must not be treated as absent. Zero means null for pointer results; ordered
pointer comparisons are not supported by clang's lowering.

These predicates are not a complete-operation success/error split and do not request Rust `Result`
conversion. For example, partial-result postconditions may apply even when an API reports
`ERROR_MORE_DATA`. Other expressions and parameter/field owners are rejected by the short-form
reader. Unknown enum values or additional metadata properties retain full attribute spelling.

`#[override_slot(N)]` marks a Win32 interface method that reuses absolute inherited virtual slot
`N`. It encodes `NativeOverrideSlotAttribute` and clears the method's `NewSlot` flag.
`#[align(N)]` on a field encodes the existing `AlignmentAttribute` without changing the field type;
record alignment uses the same spelling.

#### Null termination

`#[null_terminated(...)]` specifies trailing zero elements in parameter storage, independently of
encoding, capacity, initialized length and optionality. Choose one or both phases:

| Flag | Contract |
| --- | --- |
| `entry` | Single-zero termination on entry. |
| `double_entry` | Double-zero termination on entry. |
| `success` | Single-zero termination on successful completion. |
| `double_success` | Double-zero termination on successful completion. |
| `indirect` | Select the pointer stored in the parameter at each specified phase. |

For example, `#[null_terminated(entry, double_success)]` preserves different termination at the
two phases. Success follows the callable's contract; this does not imply termination after failure.
`#[null_terminated(success, indirect)]` describes a returned string when non-null. Nullability,
optional output storage and readable/writable extents remain separate contracts.
The selected storage must be a pointer, reference or array after resolving aliases. Scalars and
callback values are not buffer storage. Fields, return rows and declarations have no short-form
termination contract; their explicit full attributes remain readable and retain full spelling.
An empty list, repeated phase or unknown flag is an error. Direction is not inferred from these
flags. The short form preserves the phase properties and true double-termination/indirection
properties of `NativeNullTerminatedAttribute`. The writer also retains full spelling for unsupported
storage and other payload shapes, including explicitly false double-termination properties.

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
