# Implement a traditional Win32-style API

Now that we know [how to create a DLL in Rust](creating-your-first-dll.md), let's consider what it takes to implement a simple Win32-style API. While WinRT is generally a better choice for new  operating system APIs, Win32-style APIs continue to be important. You might need to re-implement an existing API in Rust or just need finer control of the type system or activation model for one reason or another.

To keep things simple but realistic, let's implement a JSON validator API. The idea is to provide a way to efficiently validate a given JSON string against a known schema. Efficiency requires that the schema is pre-compiled, so we can produce a logical JSON validator object that may be created and freed separately from the process of validating the JSON string. You can imagine a hypothetical Win32-style API looking like this:

```C++
HRESULT __stdcall CreateJsonValidator(char const* schema, size_t schema_len, uintptr_t* handle);

HRESULT __stdcall ValidateJson(uintptr_t handle, char const* value, size_t value_len, char** sanitized_value, size_t* sanitized_value_len);

void __stdcall CloseJsonValidator(uintptr_t handle);
```

The `CreateJsonValidator` function should compile the schema and make it available through the returned `handle`. 

The handle can then be passed to the `ValidateJson` function to perform the validation. The function can optionally return a sanitized version of the JSON value.

The JSON validator handle can later be freed using the `CloseJsonValidator` function, causing any memory occupied by the validator "object" to be freed.

Both creation and validation can fail, so those functions return an `HRESULT`, with rich error information being available via the `GetErrorInfo` function.

Let's use the `windows` crate for basic Windows error handling and type support. The popular `serde_json` crate will be used for parsing JSON strings. Unfortunately, it doesn't provide schema validation. A quick online search reveals the `jsonschema` crate seems to be the main or only game in town. It will do for this example. The focus here is not really on the particular implementation as much as the process of building such an API in Rust generally.

Given these dependencies and what we learned about creating a DLL in Rust, here's what the project's `Cargo.toml` file should look like:

```toml
[package]
name = "json_validator"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
jsonschema = "0.17"
serde_json = "1.0"

[dependencies.windows]
version = "0.52"
features = [
    "Win32_Foundation",
    "Win32_System_Com",
]
```

We can employ a `use` declaration to make things a little easier for ourselves:

```rust
use jsonschema::JSONSchema;
use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};
```

And let's begin with the `CreateJsonValidator` API function. Here's how the C++ declaration might look in Rust:

```rust
#[no_mangle]
unsafe extern "system" fn CreateJsonValidator(
    schema: *const u8,
    schema_len: usize,
    handle: *mut usize,
) -> HRESULT {
    create_validator(schema, schema_len, handle).into()
}
```

Nothing too exciting here. We're just using the definition of `HRESULT` from the `windows` crate. The implementation calls a different `create_validator` function for its implementaion. We'll do this so that we can use the syntactic convenience of the standard [Result](https://doc.rust-lang.org/stable/std/result/index.html) type for error propagation. The specialization of `Result` provided by the `windows` crate further supports turning a `Result` into an `HRESULT` while discharging its rich error information to the caller. That's what the trailing `into()` is used for.

The `create_validator` function looks as follows:

```rust
unsafe fn create_validator(schema: *const u8, schema_len: usize, handle: *mut usize) -> Result<()> {
    // ...

    Ok(())
}
```

As you can see, it carries the exact same parameters and simply switches out the `HRESULT` for a `Result` returning the [unit type](https://doc.rust-lang.org/stable/std/primitive.unit.html), or nothing other than success or error information.

First up, we need to parse the provided schema using `serde_json`. Since we need to parse JSON in a couple spots, we'll just drop this in a reusable helper function:

```rust
unsafe fn json_from_raw_parts(value: *const u8, value_len: usize) -> Result<serde_json::Value> {
    if value.is_null() {
        return Err(E_POINTER.into());
    }

    let value = std::slice::from_raw_parts(value, value_len);

    let value =
        std::str::from_utf8(value).map_err(|_| Error::from(ERROR_NO_UNICODE_TRANSLATION))?;

    serde_json::from_str(value).map_err(|error| Error::new(E_INVALIDARG, format!("{error}").into()))
}
```

The `json_from_raw_parts` function starts by checking that the pointer to a UTF-8 string is not null, return `E_POINTER` in such cases. We can then turn the pointer and length into a Rust slice and from there a string slice, ensuring that it is in fact a valid UTF-8 string. Finally, we call out to `serde_json` to turn the string into a JSON value for further processing.

Now that we can parse JSON, completing the `create_validator` function is relatively straightforward:

```rust
unsafe fn create_validator(schema: *const u8, schema_len: usize, handle: *mut usize) -> Result<()> {
    let schema = json_from_raw_parts(schema, schema_len)?;

    let compiled = JSONSchema::compile(&schema)
        .map_err(|error| Error::new(E_INVALIDARG, error.to_string().into()))?;

    if handle.is_null() {
        return Err(E_POINTER.into());
    }

    *handle = Box::into_raw(Box::new(compiled)) as usize;

    Ok(())
}
```

The JSON value, in this case the JSON schema, is passed to `JSONSchema::compile` to produce the compiled representation. While the value is known to be JSON at this point, it may not in fact be a valid JSON schema. In such cases, we'll return `E_INVALIDARG` and include the error message from the JSON schema compiler to aid in debugging. Finally, provided the handle pointer is not null, we can go ahead and box the compiled representation and return it as the "handle".

Now let's move on to the `CloseJsonValidator` function since it's closely related to the boxing code above. Boxing just means to move the value on to the heap. The `CloseJsonValidator` function therefore needs to "drop" the object and free that heap allocation:

```rust
#[no_mangle]
unsafe extern "system" fn CloseJsonValidator(handle: usize) {
    if handle != 0 {
        _ = Box::from_raw(handle as *mut JSONSchema);
    }
}
```

We can add a little safeguard if a zero handle is provided. This is a pretty standard convenience feature to simplify generic programming for callers, but a caller can generally avoid the indirection cost of calling `CloseJsonValidator` if they know the handle is zero.

Finally, let's consider the `ValidateJson` function's implementation:

```rust
#[no_mangle]
unsafe extern "system" fn ValidateJson(
    handle: usize,
    value: *const u8,
    value_len: usize,
    sanitized_value: *mut *mut u8,
    sanitized_value_len: *mut usize,
) -> HRESULT {
    validate(
        handle,
        value,
        value_len,
        sanitized_value,
        sanitized_value_len,
    )
    .into()
}
```

Here again the implementation forwards to a `Result`-returning function for convenience:

```rust
unsafe fn validate(
    handle: usize,
    value: *const u8,
    value_len: usize,
    sanitized_value: *mut *mut u8,
    sanitized_value_len: *mut usize,
) -> Result<()> {
    // ...
}
```

First up, we need to ensure that we even have a valid handle, before transforming it into a `JSONSchema` object reference:

```rust
if handle == 0 {
    return Err(E_HANDLE.into());
}

let schema = &*(handle as *const JSONSchema);
```

This looks a bit tricky but we're just turning the opaque handle into a `JSONSchema` pointer and then returning a reference to avoid taking ownership of it.

Next, we need to parse the provided JSON value:

```rust
let value = json_from_raw_parts(value, value_len)?;
```

Here again we use the handy `json_from_raw_parts` helper function and allow error propagation to be handled automatically via the `?` operator.

At this point we can perform schema validation, optionally returning a sanitized copy of the JSON value:

```rust
if schema.is_valid(&value) {
    if !sanitized_value.is_null() && !sanitized_value_len.is_null() {
        let value = value.to_string();

        *sanitized_value = CoTaskMemAlloc(value.len()) as _;

        if (*sanitized_value).is_null() {
            return Err(E_OUTOFMEMORY.into());
        }

        (*sanitized_value).copy_from(value.as_ptr(), value.len());
        *sanitized_value_len = value.len();
    }

    Ok(())
} else {
    // ...
}
```

Assuming the JSON value checks out against the compiled schema, we see whether the caller provided pointers to return a sanitized copy of the JSON value. In that case, we call `to_string` to return a string representation straight from the JSON parser, use `CoTaskMemAlloc` to allocate a buffer to return to the caller and copy the resulting UTF-8 string into this buffer.

If things don't go well, we can get the compiled schema to produce a handy error message before returning `E_INVALIDARG` to the caller:

```rust
let mut message = String::new();

if let Some(error) = schema.validate(&value).unwrap_err().next() {
    message = error.to_string();
}

Err(Error::new(E_INVALIDARG, message.into()))
```

The `validate` method returns a collection of errors. We'll just return the first for simplicity.

And that's it! Your first Win32-style API in Rust. You can [find the complete example here](https://github.com/microsoft/windows-rs/tree/master/crates/samples/components/json_validator).
