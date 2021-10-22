# Getting Started

This tutorial will show the basics of how to use the `windows` crate.

This is *not* meant as a tutorial on Windows programming, and it assumes at least passing familiarity with the Windows API including Win32 and COM. Nor is this meant as an introduction to programming in Rust for Windows developers. You should also have a basic understanding of Rust and Cargo.

The example used here is that of a simple spellchecker utility app. The app takes a string from stdin and returns back various spellchecking suggestions. You can find the complete code for this example [here](https://github.com/microsoft/windows-samples-rs/tree/master/spellchecker).

## Bootstrapping the project

To get started, create a new Rust project named "spellchecker":

```powershell
cargo new spellchecker
```

We'll be generating bindings to the Windows API on the fly from metadata. This means that as we compile our program, the `windows` crate will read in specialized metadata which describes the entire Windows API surface and generates just the code we need to interact with the APIs we want to.

Now we have a choice to make. We _could_ stick the generated code into our `spellchecker` crate directly. However, due to how Rust compiles code, doing so would mean that our bindings are recompiled every time we want to compile our crate. In order to avoid this, we can instead put our bindings into a dedicated `bindings` crate which will be compiled once and cached on future recompilations.

To do this, we'll simply crate a `bindings` crate which our `spellchecker` crate will take a dependency on. Inside of the `spellchecker` crate's directory, run:

```powershell
cargo new bindings --lib
```

We can generate our bindings inside of the `bindings` crate. To do this, we'll need to add the `windows` crate as both a regular dependency and build dependency of the `bindings` crate. In the Cargo.toml for the `bindings` crate add the following:

```toml
# ^^ The rest of the Cargo.toml remains the same
[dependencies]
windows = "0.8" # Check https://crates.io/crates/windows for the latest version

[build-dependencies]
windows = "0.8"
```

Now that the `bindings` crate depends on the `windows` crate, we can generate our bindings inside of a build script. In case you're not familiar with build scripts, they're simply Rust files that get run automatically by cargo when building a Rust crate. To add one, we simply put a "build.rs" file in the `bindings` crate's directory. We'll add the `windows::runtime::build!` macro which is where we declare which Windows APIs we want to generate bindings for.

The question now is: which APIs do we import? Knowing exactly which API you want to use requires familiarity with the Windows API. Let's assume you've explored [the documentation](https://docs.microsoft.com/en-us/windows/apps/) and are sure that you want to use the [internationalization](https://docs.microsoft.com/en-us/windows/win32/api/_intl/) APIs and in particular the `ISpellChecker` (and the related `ISpellCheckerFactory`) located in the [spellcheck](https://docs.microsoft.com/en-us/windows/win32/api/spellcheck/) functionality for this app. To know how this API is translated into Rust, go to the [Rust for Windows](https://microsoft.github.io/windows-docs-rs/doc/bindings/Windows/) and search for what you need.

Doing the search should lead you to find the following:

![image](https://user-images.githubusercontent.com/9845234/119203528-1654cd00-ba48-11eb-8f48-555b69c3ef5e.png)

Remember the module path that `ISpellChecker` is in underneath `bindings`: `Windows::Win32::Globalization`. You'll want to use that exactly when specifying the types you need to generate.

> **Note**
> It's important to note that for COM and WinRT APIs, methods are only available when all the types use in those methods parameters and return types are also generated. When you want to use a certain type's method, make sure you're generating everything you need to use that method. For example, the `ISpellChecker::Suggest` method takes an argument of type `*mut Option<IEnumString>`. If `IEnumString` (which is located in the `Windows::Win32::System::Com` namespace), is not generated, `ISpellChecker::Suggest` will not be generated as well.

In the build.rs file add the following:

```rust
fn main() {
    windows::runtime::build! {
        // Note that we're using the `Intl` namespace which is nested inside the `Win32` namespace
        // which itself is inside the `Windows` namespace.
        Windows::Win32::Globalization::{ISpellChecker, SpellCheckerFactory, ISpellCheckerFactory, CORRECTIVE_ACTION, IEnumSpellingError, ISpellingError},
        Windows::Win32::System::SystemServices::{BOOL, PWSTR, S_FALSE},
        Windows::Win32::System::Com::IEnumString
    };
}
```

We're not done! This generates the bindings, but it just puts them into the target folder of our crate where they'll go unused. We want to actually *use* this generated code not just generate and forget about it. In order for the generated code to be exported from the `bindings` crate, we need to include it in the crate itself. Change the lib.rs file of the bindings crate to the following:

```rust
windows::runtime::include_bindings!();
```

This effectively copy/pastes the generated code into the `lib.rs` file. The `bindings` crate now exports all the bindings we've generated. Next, we need to make sure our `spellchecker` crate depends on the `bindings` crate. In the `spellchecker` crate's Cargo.toml file, add the `bindings` crate as a dependency. We'll also add the `windows` crate as a dependency since we'll be using that as well in our `spellchecker` crate.

```toml
# ^^ The rest of the Cargo.toml remains the same
[dependencies]
bindings = { path = "bindings" }
windows = "0.8" # This should match the version you used in the bindings crate
```

And we're done bootstrapping the project. Now we'll move on to the code of the `spellchecker` crate which will use both the generated bindings from the `bindings` crate as well as helper functionality from the `windows` crate.

## Initialization Code

Inside of the main.rs file for the `spellchecker` crate, we'll start by adding the set up code. We'll initialize the COM runtime on the main thread. We'll make the `main` function return a `windows::runtime::Result` which is simply a standard `Result` type with the error hardcoded as a `windows::runtime::Error`.

```rust
fn main() -> windows::runtime::Result<()> {
    // initialize the main thread as a multithreaded apartment
    windows::initialize_mta()?;
    // The rest of the code will go here!
    Ok(())
}
```

Next, we'll initialize the `ISpellCheckerFactory` which is what gives us access to spellcheckers. We'll first make sure the `intl` namespace and two types we'll need `PWSTR` and `BOOL` are in scope at the top of the main.rs file:

```rust
use bindings::Windows::Win32::Globalization;
use bindings::Windows::Win32::System::SystemServices::{BOOL, PWSTR};
```

Then we can do the initialization by calling `windows::create_instance` which calls [CoCreateInstance](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance) under the hood:

```rust

fn main() -> window::Result<()> {
    // initialize the main thread as a multithreaded apartment
    windows::initialize_mta()?;
    // Create ISpellCheckerFactory
    let factory: Intl::ISpellCheckerFactory = windows::create_instance(&Intl::SpellCheckerFactory)?;
    // The rest of the code will go here!
    Ok(())
}
```

## Using the generated APIs

Next, we'll use the `ISpellCheckerFactory` to check that we can spell check U.S. English, and then we'll get a spellchecker instance.

Let's take a look at the method signature of `ISpellCheckerFactory::IsSupported`:

```rust
pub unsafe fn IsSupported<'a>(
    &self,
    languageTag: impl IntoParam<'a, PWSTR>,
    value: *mut BOOL,
) -> HRESULT
```

This looks a little complicated, but it makes using the API straightforward. The method is generic on both a lifetime `'a` and the trait `IntoParam` defined in the `windows` crate. Essentially, `IntoParam` is a slightly specialized version of Rust's `std::convert::Into`. It is implemented on all types that can be converted to a parameter of the type its generic over. In other words, it is any type that can be converted into a parameter of type `PWSTR` that lives for at least the lifetime `'a`.

It turns out that `IntoParam<'a, PWSTR>` is implemented for `&'a str` so we can simply pass a string literal. `IntoParam<'a, PWSTR>` is also implemented on `String` and `PWSTR` itself.

```rust
fn main() -> window::Result<()> {
    /// NOTE: The initialization code from above goes here!

    // Make sure that the "en-US" locale is supported
    let mut supported: BOOL = false.into();
    let locale = "en-US";
    unsafe { factory.IsSupported(locale, &mut supported).ok()? };
    supported.expect("en-US is supported");

    // Create a ISpellChecker
    let mut checker = None;
    unsafe { factory.CreateSpellChecker(locale, &mut checker).ok()? };
    let checker = checker.unwrap();

    Ok(())
}
```

Now we can check our text for errors:

```rust
fn main() -> window::Result<()> {
    // ... Previous code is here

    // Get errors enumerator for the supplied string
    let mut errors = None;
    unsafe {
        println!("Checking the text: '{}'", input);
        checker
            .ComprehensiveCheck(input.clone(), &mut errors)
            .ok()?
    };
    let errors = errors.unwrap()

    Ok(())
}
```

Errors is of type `IEnumSpellingError` which allows us to access errors through a `Next` method. Let's loop through all the errors:

```rust
fn main() -> window::Result<()> {
    // ... Previous code is here

    loop {
        // Get the next error in the enumerator
        let mut error = None;
        let result = unsafe { errors.Next(&mut error) };
        // Getting S_FALSE means there are no more errors
        if result == windows::runtime::HRESULT::S_FALSE {
            break;
        }
        // We still need to check that `Next` didn't return an unexpected error
        result.ok()?;
        let error = error.unwrap();
    }
    Ok(())
}
```

Lastly, we'll get the "corrective action" (i.e., the thing the spellchecker recommends we do to fix the spelling error) and print it to the screen:

```rust
// Inside the `loop`

// Get the corrective action
let mut action = Intl::CORRECTIVE_ACTION::CORRECTIVE_ACTION_NONE;
unsafe { error.get_CorrectiveAction(&mut action).ok()? };
println!("Corrective Action: {:?}", action);
```

If we then try to run our code like so:

```powershell
cargo run -- "Helloo world" # Note: "Hello" is misspelled
```

We should get the following output:

```
Corrective Action: CORRECTIVE_ACTION(1)
```

Looking at the [docs for `CORRECTIVE_ACTION`](https://docs.microsoft.com/en-us/windows/win32/api/spellcheck/ne-spellcheck-corrective_action), we can see that 1-index action corresponds to `CORRECTIVE_ACTION_GET_SUGGESTIONS`, meaning the spellchecking API has suggestions.

## Next Steps

With what you've learned so far, finish the exercise of building a spellchecker that spellchecks the provided text. This will entail handling the various corrective actions and ensuring that the right thing is done for each of them.

If you get stuck, you can always see the final code [here](https://github.com/microsoft/windows-samples-rs/tree/master/spellchecker).
