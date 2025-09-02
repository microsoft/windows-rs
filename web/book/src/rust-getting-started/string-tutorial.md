# Working with strings in the windows crate

There are several string types in the Windows APIs including:
  - `PSTR`/`PCSTR`: A pointer to a null terminated string consisting of chars (u8). Strings should be encoded using the current thread's code page. A 'C' indicates a "constant" (read-only) string.
  - `PWSTR`/`PCWSTR`: A pointer to a null terminated string consisting of 'wide chars' (u16), encoded using UTF-16. 
  - `BSTR`: A binary string commonly used in COM/OLE functions. It consists of u16 characters followed by a null terminator. The strings have their length prepended before the pointer, and some functions will use them to pass arbitrary binary data (including data containing nulls), relying on the prefix rather than the terminator. However, they *usually* can be used as normal, null-terminated wide strings. 
  - `HSTRING`: A handle to a Windows Runtime string. HSTRINGS are UTF-16 and immutable. 

Note that you can pass BSTR or HSTRING to functions expecting a PCWSTR. 

Unfortunately, none of these types match Rust types one-to-one. However, we can use the `windows-strings` crate to help us. 

## Types of API Functions (Narrow or Wide)

The Win32 API divides string functions into their "narrow" version (ending in 'A', like `MessageBoxA`), and their "wide" version (ending in 'W', like `MessageBoxW`). Narrow versions of the API expect u8 byte strings, encoded using the current thread's code page, while wide versions expect UTF-16. 

As a general recommendation, you should prefer wide versions; it's much easier to convert between Rust's UTF-8 strings and Windows' UTF-16.

## Calling APIs that Consume Strings

Let's look at an example using the simple [`MessageBox` function](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.MessageBoxW.html), which displays a pop up dialog box. We'll use the wide version (`MessageBoxW`) for this example. 

If you want to call a Windows API using a string literal, the `windows-strings` crate has macros to generate Windows strings from string literals:
  - `h!` generates an HSTRING from a string literal, adding a null terminator and converting to UTF-16.
  - `w!` does the same, but generates a PCWSTR instead of an HSTRING. 
  - `s!` generates a PCSTR with a null terminator. *Caution: this does not do any conversions, it simply adds a null terminator.*

If we wanted to call the message box we could use the windows crate with the `Win32_UI_WindowsAndMessaging` feature, and call:

```rust
// use string literals when calling a message box. 
let text = h!("Hello from rust!");
let caption = h!("From Rust");

unsafe {
    // call the MessageBox function and return MESSAGEBOX_RESULT
    UI::WindowsAndMessaging::MessageBoxW(None, 
        text, 
        caption,
        UI::WindowsAndMessaging::MESSAGEBOX_STYLE(0) // message box OK
    )
}
```

This works, but what if we wanted to call the same function with a Rust string? That gets slightly more complicated. We could manually convert to a UTF-16 series of bytes, and add the null terminator ourselves, like this:

```rust
// this works for any &str, not just literals
let text = "I am a message to display!";
let caption = "Message from Rust!";

// convert our text and caption to UTF-16 bytes,
// add null terminators using chain, and then collect
// the result into a vec
let text = text.encode_utf16()
    .chain(iter::once(0u16))
    .collect::<Vec<u16>>();
let caption = caption.encode_utf16()
    .chain(iter::once(0u16))
    .collect::<Vec<u16>>();

// call the API, wrapping our vec pointer in a PCWSTR struct.
unsafe {
    UI::WindowsAndMessaging::MessageBoxW(None, 
        PCWSTR(text.as_ptr()), 
        PCWSTR(caption.as_ptr()),
        UI::WindowsAndMessaging::MESSAGEBOX_STYLE(0) // message box OK
    )
}
```

However, this is cumbersome - we can use the convenience features in the `windows-strings` crate to make this much simpler by converting the Rust strings to HSTRING. 

```rust
let text = "I am a message to display!";
let caption = "Message from Rust!";

// convert our strings into UTF-16 
// this incurrs a performance cost because there is a copy + conversion
// from the standard rust utf-8 string. 

// we are using HSTRING, which is an immutable UTF-16 string
// in the windows-strings crate. It can be generated from a standard
// rust string, and it can be used in place of a PCWSTR anywhere in the
// windows API. 

unsafe {
    UI::WindowsAndMessaging::MessageBoxW(None, 
        &HSTRING::from(text), 
        &HSTRING::from(caption),
        UI::WindowsAndMessaging::MESSAGEBOX_STYLE(0) // message box OK
    )
    }
```

This is much more ergonomic - it handles the null termination and UTF-16 conversion for you. 

## Calling APIs that Generate Strings

Windows APIs that generate strings usually require a two-step call. The first time you call the API, you pass in a NULL pointer for the string buffer, and retrieve the length of the string to be generated. 

This allows you to allocate the buffer accordingly, and then call the function again with an appopriately sized buffer. 

For this example, we'll use the [`GetComputerNameW` function](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/WindowsProgramming/fn.GetComputerNameW.html). This requires the `Win32_System_WindowsProgramming` feature from the windows crate.

```rust
let mut buff_len = 0u32;

unsafe {
    // this function will return an error code because it
    // did not actually write the string. This is normal.
    let e = GetComputerNameW(None, &mut buff_len).unwrap_err();
    debug_assert_eq!(e.code(), HRESULT::from(ERROR_BUFFER_OVERFLOW));
}

// buff len now has the length of the string (in UTF-16 characters)
// the function would like to write. This *does include* the
// null terminator. Let's create a vector buffer and feed that to the function.
let mut buffer = Vec::<u16>::with_capacity(buff_len as usize);

unsafe {
    WindowsProgramming::GetComputerNameW(
        Some(PWSTR(buffer.as_mut_ptr())), 
        &mut buff_len).unwrap();

    // set the vector length
    // buff_len now includes the size, which *does not include* the null terminator.
    // let's set the length to just before the terminator so we don't have to worry
    // about it in later conversions.
    buffer.set_len(buff_len);
}

// we can now convert this to a valid Rust string
// omitting the null terminator
String::from_utf16_lossy(&buffer)
```

It's worth calling out how the length parameter works. For GetComputerNameW:
  - On input, it represents the size of the buffer *including the null terminator* in wchar. 
  - If the function returns a buffer overflow, the length parameter returned is how
    large a buffer it needs *including the null terminator* in wchars.
  - If the function successfully wrote to the buffer, the length is the number
    of wchars written *not including the null terminator.*

[This behavior is documented in the function's documentation](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew) - when using the Windows API, be careful and check what the function expectes with respect to null terminators.

Regardless, this does work, but we can do better. Computer names can only be up to `MAX_COMPUTERNAME_LENGTH` which is a meager 16 characters. We can avoid a heap allocation here and just use arrays, since we know our buffer length at compile time.

```rust
// avoid the heap allocation since we already know how big this 
// buffer needs to be at compile time. 

let mut name = [0u16; MAX_COMPUTERNAME_LENGTH as usize + 1];
let mut len = name.len() as u32;

// we can also skip the two-step call, since we know our buffer
// is already larger than any possible computer name

unsafe {
    GetComputerNameW(
        Some(PWSTR(name.as_mut_ptr())), 
        &mut len)
        .unwrap();
}

// the function writes to len with the number of 
// UTF-16 characters in the string. We can use this
// to slice the buffer. 
String::from_utf16_lossy(&name[..len as usize])
```

However, if we don't mind the heap allocation (and a few extra system calls), there is a more ergonomic option. The `windows-strings` crate includes `HStringBuilder` that we can use in place of the array. This gives us much easier conversions. 

```rust
// pre-allocate a HSTRING buffer on the heap
// (you do not need to add one to len for the null terminator,
// the hstring builder will handle that automatically)

let mut buffer = HStringBuilder::new(
    MAX_COMPUTERNAME_LENGTH as usize);

let mut len = buffer.len() as u32 + 1;

unsafe {
    GetComputerNameW(
        Some(PWSTR(buffer.as_mut_ptr())), 
        &mut len).unwrap();
}

// we can now generate a valid HSTRING from the HStringBuilder
let buffer = HSTRING::from(buffer);

// and we can now return a rust string from the HSTRING:
buffer.to_string_lossy()
```

If you need to work with UTF-16 strings directly, consider using the `widestring` crate, which is UTF-16 aware. This will enable you to push/pop/append elements without having to convert the string to a native rust UTF-8 string. For completeness, here's an example of returning a widestring, and appending some exclaimation marks. 

```rust
// for this example, we'll just use an array again

let mut name = [0u16; MAX_COMPUTERNAME_LENGTH as usize + 1];
let mut len = name.len() as u32;

unsafe {
    GetComputerNameW(
        Some(PWSTR(name.as_mut_ptr())), 
        &mut len)
        .unwrap();
}

// we can make a UTF16Str slice directly from the buffer,
// without needing to do any copy. This will error if the buffer
// isn't valid UTF-16. 
let wstr = Utf16Str::from_slice(&name[..len as usize])
    .unwrap();

// this can be displayed as is.
println!("Computer name is {}", wstr);

// we can also transfer it into owned string, which can
// be appended or modified. 
let mut wstring = Utf16String::from(wstr);

// let's append another string. We'll use a macro to avoid
// any UTF conversion at runtime. 
wstring = wstring + utf16str!("!!!");
```