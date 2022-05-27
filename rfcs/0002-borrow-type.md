- Feature Name: `borrow_type`
- Start Date: 2022-05-27
- RFC PR: [microsoft/windows-rs#1788](https://github.com/microsoft/windows-rs/pull/1788)

# Summary
[summary]: #summary

This RFC introduces a new type `Borrow<'a, T>` which mirrors the semantics of a `&'a T` but with the difference that it has the same in memory layout as `T`.

# Motivation
[motivation]: #motivation

Windows APIs often have ownership semantics that differ slightly from Rust's model. Types can often have different ownership semantics depending on the context. In Rust we would normally model these different semantics by using two different types, in C/C++ this is usually left to documentation and convention. 

This difference is only interesting for types that are *not* "blittable" (`Copy` in Rust parlance). Non-"blittable" types have non-trivial destructors and cannot just be arbitrarily mem-copied. "Blittable" types like integers and booleans on other the hand can be, and so their ownership semantics are simple and map directly to Rust's model.

Interesting types are those with non-trivial destructors. For example, COM interfaces or reference counted types like `HSTRING`.

For the examples, we'll often use `HSTRING`. `HSTRING` is a reference counted string. Normally, we must increment the reference count if we need another handle to the string and decrement the reference count when we are done with a handle. These semantics map neatly to Rust's `Clone` and `Drop` traits which `HSTRING` implements.

However, sometimes this is not the desired behavior. The following are examples where ownership semantics of a particular type depend on context. 

## Input params

When calling a Windows API function and passing an `HSTRING` as an input param (a.k.a. "IN" param), the semantics are such that the `HSTRING` is logically "borrowed" for the duration of the function call. The caller is not responsible for incrementing the reference count. If the callee wants to store the `HSTRING` somewhere where that will outlive the function call, it is up to the callee to increment the reference count.

On the Rust side, essentially we want to keep `Clone` but temporarily turn off `Drop`. One way we could accomplish this is by passing a reference `&HSTRING`. However, the API call is expecting an `HSTRING` not a pointer to an `HSTRING`. Instead, we might be tempted to use `std::mem::ManuallyDrop`. However, `ManuallyDrop` takes ownership of the`HSTRING` and since we never see the `HSTRING` passed into the function again, we can never call `ManuallyDrop::into_inner` to turn the `Drop` implementation back on.

It might seem we're out of luck.

##  Struct Fields

Sometimes the Windows APIs will declare a struct that has a non-"blittable" type as a field. These fields are logically "borrowed". For example, if a struct had an `HSTRING` as a field, it would not be responsibility of whoever constructs the struct to first increment the reference count. Also, when the struct is no longer needed, it is not necessary to decrement the reference count. 

The struct as a whole implicitly has a lifetime associated with it whereby the fields of the struct must live for at least as long as the struct as a whole.

Like "IN" params, using a reference `&T` is not possible since the field must be exactly equivalent to `T` in-memory. And `ManuallyDrop` also has the problem of consuming the `T` and requiring when the `T` is no longer used, the field is in scope so that `ManuallyDrop::into_inner` can be called.

# Guide-level Explanation  
[guide-level-explanation]: #guide-level-explanation

***Note**: The following is what documentation for this feature might look like.*

You may see the type `Borrow<'a, T>` used in Windows APIs. There's no need to worry about this type, it's actually quite straight forward.

`Borrow<'a, T>` is exactly equivalent to `&'a T` except that in memory a `Borrow<'a, T>` looks just like a `T`.

This is important because some Windows APIs take arguments of some type `T` but treat them as they are just borrowed.

Let's take a look at an example:

`windows::Data::Json::JsonObject` has a function named `Parse` which looks like this:

```rust 
fn Parse<'a>(input: Borrow<'a, HSTRING>) -> Result<JsonObject>;
```

The reason `Parse` takes a `Borrow<'a, HSTRING>` instead of just a plain `HSTRING` is because if it did, ownership of `input` would be passed into `Parse` and `input`'s `Drop` implementation would be called possibly freeing `input`'s memory!

`Parse` really expects to logically borrow `input`. If this were a normal Rust function, you would expect `input` to be something like `&HSTRING`. However, `Parse` is not a regular Rust function - it crosses the FFI boundary into another language entirely! If were were to try to pass a `&HSTRING` into `Parse`, `Parse` would interpret the reference as if it were just a plain old `HSTRING`. This would cause all kinds of problem's all caused by the fact that `Parse` expected an `HSTRING` and we tried to give it a `&HSTRING`.

`Borrow<'a, T>` allows us to emulate a reference but at the memory level still pass around memory that looks just like the underlying `T`. We get the safety of a Rust reference when writing our Rust code, and the Windows API receives the memory it expects to. Everyone wins!

# Reference-level Explanation  
[reference-level-explanation]: #reference-level-explanation

`Borrow<'a, T>` has the following definition:

```rust
#![warn(unsafe_op_in_unsafe_fn)]

#[repr(transparent)]
struct Borrow<'a, T> {
    item: core::mem::ManuallyDrop<T>,
    lifetime: core::marker::PhantomData<&'a ()>
}

impl <'a, T> Borrow<'a, T> {
    /// # Safety
    ///
    /// It must be safe to alias `T` as long as `T`'s 
    /// destructor is not run. 
    ///
    /// For example, `IUnknown` is fine to alias as long as 
    /// the reference count is guaranteed to stay above 0 
    /// until the original `item` is dropped. On the other hand
    /// `Box<T>` is not fine to alias and thus passing a `Box` 
    /// to this function would be UB.
    unsafe fn new(item: &'a T) -> Self {
        let item = unsafe { core::mem::transmute_copy(item) };
        Self {
            item,
            lifetime: core::marker::PhantomData
        }
    }
}

impl<'a, T> std::ops::Deref for Borrow<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.item
    }
}
```

## Usages

For function calls, generic parameters should be preferred for all borrowed parameters allowing natural conversions from the main type to borrowed:


```rust    
fn Parse<'a, T0>(input: T0) -> Result<JsonObject> 
    where T0: Into<Borrow<'a, HSTRING>>;
```

However, only the following conversions should be given:

```rust
impl <'a, T> From<T> for Borrow<'a, HSTRING> where T: Into<&'a HSTRING> {
    fn from(item: T) -> Self {
        unsafe { Borrow::new(item.into()) }
    }
}
```

This makes the following calls natural:

```rust
let s = HSTRING::default();
JsonObject::Parse(&s);
```

This also allows for natural calls For types where borrows of two types are equivalent from the callee's perspective. For example, passing an `&IUri` where an `&IUnknown` is expected will just work. For example:

```rust
fn SomeFunction<'a, T0>(input: T0)
where
    T0: Into<Borrow<'a, IDWriteFontFamily>>,
{
    todo!()
}
let f: IDWriteFontFamily2 = todo!();

// Here the function expects a `IDWriteFontFamily` but we 
// succesfully pass a `IDWriteFontFamily2` which is equivalent 
// in this case.
SomeFunction(&f);
```

*Note*: This requires that all COM interface hierarchies have conversions between children and their parents.

For the example above to work, this definition is necessary:

```rust

impl <'a> From<&'a IDWriteFontFamily2> for &'a IDWriteFontFamily
    fn from(item: &'a IDWriteFontFamily2) -> Self {
        // Note: we may want to consider implementing `Deref`
        // for COM interfaces so that children can be 
        // dereferenced into their parents. But this is 
        // perhaps outside the scope of this RFC
        unsafe { std::mem::transmute(item) }
    }
}
```

*Note*: that there is no implementation of `From<T> for Borrow<'a, T>`. This would complicate things considerably as `T` must only be dropped after the borrow is over.

## String literals

Using string literals in function calls is no longer possible. 

The previous `IntoParam` mechanism allows for implicit allocation to turn Rust's UTF-8 string literals into various types (and then free the memory after the function call is done). While this is certainly convenient, it can be rather surprising especially to those who don't want implicit allocations. In fact, this was the thought behind the `alloc` feature which makes this feature opt-in. 

This will no longer be possible. Instead, it's left to a future RFC proposal for making it convenient to work with literals. For example, it has been discussed before that a `w!` macro could be introduced to create wide string literals.

## Errors

The following is the error message when trying to pass an HSTRING by value:

```rust
let s = HSTRING::default();
JsonObject::Parse(s); // here we try to ownership of `s`
```

Error:
```
error[E0277]: the trait bound `HSTRING: Into<&HSTRING>` is not satisfied
  --> src/main.rs:67:23
   |
67 |     JsonObject::Parse(s);
   |     ----------------- ^ expected an implementor of trait `Into<&HSTRING>`
   |     |
   |     required by a bound introduced by this call
   |
   = note: required because of the requirements on the impl of `Into<&HSTRING>` for `HSTRING`
note: required because of the requirements on the impl of `From<HSTRING>` for `Borrow<'_, HSTRING>`
  --> src/main.rs:14:14
   |
14 | impl <'a, T> From<T> for Borrow<'a, HSTRING> where T: Into<&'a HSTRING> {
   |              ^^^^^^^     ^^^^^^^^^^^^^^^^^^^
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Into<Borrow<'_, HSTRING>>` for `HSTRING`
note: required by a bound in `JsonObject::Parse`
  --> src/main.rs:60:22
   |
60 |     fn Parse<'a, T0: Into<Borrow<'a, HSTRING>>>(input: T0) -> JsonObject {
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `JsonObject::Parse`
help: consider borrowing here
   |
67 |     JsonObject::Parse(&s);
   |                       +
```

This error correctly offers the suggestion to borrow.

# Drawbacks
[drawbacks]: #drawbacks

* The name `Borrow` may confuse users who are used to `core::borrow::Borrow`.
    * `Param` and `Borrowed` are possibilities

# Rational and Alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

## The status quo

The status quo which is a mix between `ManuallyDrop` and `IntoParam`, both of which are replaced by this proposal.

Because `ManuallyDrop` consumes the value it wraps, but we often still want to be able to use the value after it's been wrapped, code tends to often `transmute_copy` values into `ManuallyDrop` wrappers in order to not pass ownership of the value into the `ManuallyDrop`. While this works, we should not treat any solution that requires wide use of `transmute_copy` as a valid solution.

# Unresolved Questions
[unresolved-questions]: #unresolved-questions

**Should the lifetimes of "borrowed" values all be the same or be distinct**

For example:

Possibility 1:

```rust
#[repr(C)]
pub union D3D12_RESOURCE_BARRIER_0<'a> {
    pub Transition: Borrow<'a, D3D12_RESOURCE_TRANSITION_BARRIER>,
    pub Aliasing: Borrow<'a, D3D12_RESOURCE_ALIASING_BARRIER>,
    pub UAV: Borrow<'a, D3D12_RESOURCE_UAV_BARRIER>,
}
```

Possibility 2:
```rust
#[repr(C)]
pub union D3D12_RESOURCE_BARRIER_0<'a, 'b, 'c> {
    pub Transition: Borrow<'a, D3D12_RESOURCE_TRANSITION_BARRIER>,
    pub Aliasing: Borrow<'b, D3D12_RESOURCE_ALIASING_BARRIER>,
    pub UAV: Borrow<'c, D3D12_RESOURCE_UAV_BARRIER>,
}
```

**Should there be a second type for exclusive references?**

It's possible that we may want another type `BorrowMut<'a, T>` that acts just like `Borrow<'a, T>` but replicates `&mut T` instead of `&T`.

This has been suggested as necessary for struct fields.