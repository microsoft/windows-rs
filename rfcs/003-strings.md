# Strings in windows-rs

## Summary

Rust and Windows each have a fair amount of string types. This RFC proposes a system for making it easy and obvious how to make the string types from both environments work well together.

## Motivation

Making Rust and Windows string types work together is integral to ensuring correct, performant, and boiler-plate free code.

### What we want to accomplish

In general, we are trying to accomplish two goals:

* Correctly model the Windows string types in a way that leverages Rust's type and ownership systems to prevent incorrect use.
* Allow for easy and low cost ways to convert between Windows and Rust types.

On top of this, we want these APIs to feel natural and obvious to Rust developers.

## Explanation

### The Different Types of Strings

#### Windows

Windows has the following string types:

* `PSTR`: a mutable, nul-terminated string (i.e., composed of "characters" a.k.a. `u8`s) that is often but not always ANSI encoded.
* `PCSTR`: an immutable version of `PSTR`.
  * **QUESTION**: what are the practical differences between this and `PSTR`?
* `PWSTR`: a mutable, nul-terminated "wide" string (i.e., composed of "wide characters" a.k.a. `u16`s) that is often but not always UTF-16 encoded.
* `PCWSTR`: an immutable version of `PWSTR`.
* `BSTR`: an immutable, nul-terminated "wide" string used in COM.
* `HSTRING`: an immutable reference counted, nul-terminated "wide" string used in WinRT.
  * Note: `HSTRING`s can sometimes be "fast pass" where the buffer and header is stack allocated. In this case the string must only be used while the stack frame is valid.

Generally, callees are expected only keep reference to string types during their stake frame, and copy (or bump the reference count in the case of `HSTRING`) if they want to keep the string around for longer.

#### Rust

Rust's `alloc` core library and by extension it's `std` library have the following string types:

* `String`: an owned pointer to UTF-8 encoded data together with a known length and capacity.
* `&'a str`: a view into UTF-8 encoded data that lasts for the lifetime `'a`.
* `CString`: an owned, C-compatible, nul-terminated string of bytes.
* `&'a CStr`: a view into a `CString` data that lasts for lifetime `'a`.
* `OsString` (on Windows): an owned sequence of 8-bit values, encoded in a less-strict variant of UTF-8.
* `OsStr` (on Windows): a view into `OsString` data that lasts for lifetime `'a`.

#### Equivalences

Windows' `PCSTR` has a direct in-memory equivalent to Rust's `CString` and `&CStr` where the Rust types have clear ownership semantics while the Windows version does not.

### Modeling Windows String types

#### Ownership for Win32 Strings

Often when dealing with Win32 strings (i.e., `PSTR`, `PCSTR`, `PWSTR`, and `PCWSTR`) who owns the data or how long the data is valid for is well known:

* As "IN" params to a function, the string should only be considered valid for the lifetime of the function call.
* As string literals, the string data is valid for the entirety of the program.

However, sometimes we don't know the ownership or lifetime of a value outside of documentation:

* As "OUT" params to a function, the caller is sometimes owns the data, but it may also be owned by someone else and thus only valid for some lifetime.
  * [Example of a "borrowed" OUT param](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Globalization/struct.IOptionDescription.html#method.Id). The `id` string is owned by the `IOwnDescription` object.
  * [Example of an "owned" OUT param](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Com/struct.IEnumString.html#method.Next). The caller is responsible for freeing the `rgelt` param.
* As fields in a struct, the ownership of the data is unclear. The string can sometimes be freed when the struct is no longer needed but sometimes something else owns that string.

**In short**: there is no way to programmatically know the lifetime of a win32 string except when it is used as an "IN" param.

#### Ownership for `BSTR`

`BSTR` has the same story as a win32 string type.

#### Ownership for `HSTRING`

Ownership for `HSTRING` is fairly clear:

* As "IN" params, the HSTRING is only being borrowed.
* Typically in any other case, the `HSTRING` handle has had its reference counted incremented and when that handle is no longer needed, the reference count should be decremented.

**In short**: `HSTRING`s are always valid when in scope, and when they are no longer used, their reference counts should be decremented **except** in the case of "IN" params where they are logically "borrowed".

### Proposed API

It's clear that "IN" params are the scenario where ownership differs from the status quo. Therefore, the `windows` crate will treat the types as the following.

* Introduce a new type `CWString` which is the wide string equivalent to `CString`.
* win32 strings and `BSTR`s should always be treated like raw pointers. Accessing them is unsafe since the compiler can not verify that they are still live with the exception of their use as "IN" params which we discuss below.
* `HSTRING`s are always live and should have their reference count decremented in their `Drop` impl.

### Converting between String types

#### From Windows to Rust types

The main distinction between `HSTRING` and the other Windows string types is that `HSTRING` is always valid while the other string types are essentially wrappers around raw pointers and thus cannot be assumed to point to valid memory. Therefore, while `HSTRING` can host a variety of safe conversions into Rust strings, the other strings cannot.

The main conversions afforded to these types are to:

* `&[u16]` or `&[u8]` (depending if the given Windows type is a wide string or not)
* `String`

Conversion of most types to `CString` and `OString` doesn't make much sense as all Windows types are already FFI safe and those Rust types are almost never needed outside of FFI. However, the user should be given access to the underlying buffers (including the trailing nul byte), and there should be conveniences to converting to `String` since this unlocks the entire rich `std` library functionality for string manipulation.

All string types that aren't `HSTRING` can only expose unsafe functions as it is not well known that they point to valid memory. Additionally, each type should allow conversion to a raw pointer to avoid having to use `std::mem::transmute` in cases where an API expects a raw pointer.

#### From Rust to Windows types

It is common for Rust functionality to want to interact with some Windows APIs which require the Windows string types.

Converting from Rust types usually requires copy the UTF-8 encoded bytes to UTF-16. This can be done with any type that can be referenced as a `str`, so it makes sense to provide `From<&T> where T: AsRef<str>` for `HSTRING` as `HSTRING` will ensure the newly allocated buffer is freed when appropriate.

All the other Windows string types do not have clear ownership semantics and thus providing convenient conversions might risk leading the user to leak memory. This is the same reason logically borrowed types like `CStr` don't allow converting from owned types like `String`. As such, only direct conversions will be provided: conversion from `*const u8` and `*const u16`.

#### API Proposal

With the above in mind, here is what the API should look like:

```rust
impl HSTRING {
    // String data without the trailing 0
    fn as_wide(&self) -> &[u16] {}
    // String data with the trailing 0
    fn as_wide_with_nul(&self) -> &[u16] {}
    fn to_string(&self) -> Result<String, FromUtf16Error> {}
    fn to_string_lossy(&self) -> String {}
    fn as_ptr(&self) -> *const c_void;
}

// Display shows the string with non-valid utf16 replaced with �
impl Display for HSTRING {}
// Same as Display but surrounded by ""
impl Debug for HSTRING {}

// Uses the `to_string_lossy` function
impl From<HSTRING> for String {}
// Uses the `to_string` function
impl TryFrom<HSTRING> for STRING {
    type Error = FromUtf16Error;
}
impl<T: ?Sized + AsRef<str>> From<&'_ T> for HSTRING {}
impl<T: ?Sized + AsRef<str>> TryFrom<&'_ T> for HSTRING {
    type Error = FromUtf16Error;
}

// --------------------------------

/// The wide equivalent to std::ffi::CString;
#[repr(transparent)]
struct CWString(..);

impl CWString {
    // This mirrors CString
    fn new<T: Into<Vec<u16>>>(t: T) -> Result<Self, NulError> {}
    fn from_str<T: AsRef<str>>(t: T) -> Result<Self, NulError> {}
    fn as_wide(&self) -> &[u16] {}
    // String data with the trailing 0
    fn as_wide_with_nul(&self) -> &[u16] {}
    fn as_pcwstr(&self) -> PCWSTR {}
    fn to_string(&self) -> Result<String, FromUtf16Error> {}
    fn to_string_lossy(&self) -> String {}
}

// This allows `CWString` to be turned into a `Borrowed<'a, PCWSTR>`
impl From<&CWString> for &PCWSTR {}
impl From<CWString> for HSTRING {}
impl From<HSTRING> for CWString {}

impl Drop for CWString {
    // The `CWString` owns its memory and will free when its dropped
}
// Display shows the string with non-valid utf16 replaced with �
impl Display for CWSTRING {}
// Same as Display but surrounded by ""
impl Debug for CWSTRING {}

// HSTRING should also include implementations of `PartialEq` for 
// `String`, `&str`, `OsString`, and `&OsStr`

// --------------------------------

impl BSTR {
    fn from_ptr(ptr: *const c_void) -> Self {}
    fn as_ptr(&self) -> *const c_void;
    // String data without the trailing 0
    unsafe fn as_wide(&self) -> &[u16] {}
    // String data with the trailing 0
    unsafe fn as_wide_with_nul(&self) -> &[u16] {}
    unsafe fn to_string(&self) -> Result<String, FromUtf16Error> {}
    unsafe fn to_string_lossy(&self) -> String {}
}


// --------------------------------

impl PCWSTR {
    fn from_ptr(ptr: *const u16) -> Self {}
    fn as_ptr(&self) -> *const u16;
    fn is_null(&self) -> bool {}
    // String data without the trailing 0
    unsafe fn as_wide(&self) -> &[u16] {}
    // String data with the trailing 0
    unsafe fn as_wide_with_nul(&self) -> &[u16] {}
    unsafe fn to_string(&self) -> Result<String, FromUtf16Error> {}
    unsafe fn to_string_lossy(&self) -> String {}
    unsafe fn display(&self) -> impl Display {}
}
// This allows `CString` and `CSTR` to be turned into a `Borrowed<'a, PCSTR>`
impl From<&CString> for &PCSTR {}
impl From<&CStr> for &PCSTR {}

// --------------------------------

impl PCSTR {
    fn from_ptr(ptr: *const u8) -> Self {}
    fn as_ptr(&self) -> *const u8;
    fn is_null(&self) -> bool {}
    // String data without the trailing 0
    unsafe fn as_bytes(&self) -> &[u8] {}
    // String data with the trailing 0
    unsafe fn as_bytes_with_nul(&self) -> &[u8] {}
    // Converts to `&str` checking for valid UTF-8
    unsafe fn as_str(&self) -> Result<&str, FromUtf8Error> {}
    // Converts to `&str` not checking for valid UTF-8
    unsafe fn as_str_unchecked(&self) -> &str {}
}

// Note that `Display`, `Debug`, and the conversion traits are not 
// included for non-`HSTRING` types because those traits are not 
// marked as unsafe
```

#### String literals

Many times the user simply wants to use a string literal to build a string of a certain type.

The windows crate provides the following macros for convenience:

```rust
let x: CWString = w!("hello");
let y: CString = c!("hello");
let z: HSTRING = h!("hello");
```

These function build null terminated string data of the appropriate width (`u8` in the case of `PCSTR` and `u16` in the other cases).

**question**: for all these types built from static strings, could we special case them so that they don't use reference counting at all?

### Interaction with `InParam`

In params that at the FFI layer take a PCWSTR will be project as `P: Into<InParam<'a, CWString>`. In params that at the FFI layer take a PCSTR will be project as `P: Into<InParam<'a, CString>`.

**QUESTION**: This will just work when the user owns the strings their passing over the FFI layer. But we'll need someway to turn `PCWSTR` and `PCSTR` into `InParam<'a, CWString>` and `InParam<'a, CString>` respectively. This cannot be a safe operation since the `PCWSTR` and `PCSTR` might be invalid. Perhaps a `unsafe fn InParam::from_abi_unchecked()`?

## Examples

Here is the usage of some of these APIs

```rust
let locale = w!("en-US");
let supported = unsafe { factory.IsSupported(&locale)? };
supported.expect("en-US is supported");

// Create a ISpellChecker
let checker = unsafe { factory.CreateSpellChecker(&locale)? };

// Get errors enumerator for the supplied string
println!("Checking the text: '{}'", input);
let i = CWString::from_str(input).unwrap();
let errors = unsafe { checker.ComprehensiveCheck(&i)? };

// Loop through all the errors
while let Ok(error) = unsafe { errors.Next() } {
    // Get the start index and length of the error
    let start_index = unsafe { error.StartIndex()? };
    let length = unsafe { error.Length()? };

    // Get the substring from the utf8 encoded string
    let substring = &input[start_index as usize..(start_index + length) as usize];

    // Get the corrective action
    let action = unsafe { error.CorrectiveAction()? };
    println!("{:?}", action);

    match action {
        CORRECTIVE_ACTION_DELETE => {
            println!("Delete '{}'", substring);
        }
        CORRECTIVE_ACTION_REPLACE => {
            // Get the replacement as a widestring and convert to a Rust String
            let replacement = unsafe { error.Replacement()? };

            println!("Replace: {} with {}", substring, unsafe { replacement.display() });

            unsafe { CoTaskMemFree(replacement.as_ptr() as *mut _) };
        }
        CORRECTIVE_ACTION_GET_SUGGESTIONS => {
            // Get an enumerator for all the suggestions for a substring
            let suggestions = unsafe { checker.Suggest(CWString::from_str(substring).unwrap())? };

            // Loop through the suggestions
            loop {
                // Get the next suggestion breaking if the call to `Next` failed
                let mut suggestion = [PWSTR::default()];
                unsafe {
                    let _ = suggestions.Next(&mut suggestion, std::ptr::null_mut());
                }
                if suggestion[0].is_null() {
                    break;
                }

                println!("Maybe replace: {} with {}", substring, unsafe { suggestion[0].display() });

                unsafe { CoTaskMemFree(suggestion[0].as_ptr() as *mut _) };
            }
        }
        _ => {}
    }
}
```

```rust
Uri::CreateUri(&h!("http://test/"))
```
