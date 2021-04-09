# Getting Started

This tutorial will show the basics of how to use the `windows` crate.

This is *not* meant as a tutorial on Windows programming, and it assumes at least passing familiarity with the Windows API including Win32 and COM. Nor is this meant as an introduction to programming in Rust for Windows developers. You should also have a basic understanding of Rust and Cargo.

The example used here is that of a simple spellchecker utility app. The app takes a string from stdin and returns back various spellchecking suggestions. You can find the complete code for this example [here](../examples/spellchecker).

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
windows = { git = "https://github.com/microsoft/windows-rs" }  # TODO: change this to the crates.io version when that gets released

[build-dependencies]
windows = { git = "https://github.com/microsoft/windows-rs" }  # TODO: change this to the crates.io version when that gets released
```

Now that the `bindings` crate depends on the `windows` crate, we can generate our bindings inside of a build script. In case you're not familiar with build scripts, they're simply Rust files that get run automatically by cargo when building a Rust crate. To add one, we simply put a "build.rs" file in the `bindings` crate's directory. We'll add the `windows::build!` macro which is where we declare which Windows APIs we want to generate bindings for.

The question now is: which APIs do we import? Knowing exactly which API you want to use requires familiarity with the Windows API. Let's assume you've explored [the documentation](https://docs.microsoft.com/en-us/windows/apps/) and are sure that you want to use the [internationalization](https://docs.microsoft.com/en-us/windows/win32/api/_intl/) APIs and in particular the `ISpellChecker` (and the related `ISpellCheckerFactory`) located in the [spellcheck](https://docs.microsoft.com/en-us/windows/win32/api/spellcheck/) functionality for this app. To know how this API is translated into Rust, go to the [Rust for Windows](https://microsoft.github.io/windows-docs-rs/doc/bindings/Windows/) and search for what you need.

Doing the search should lead you to find the following:

![Search Results](/images/ISpellChecker-docs.png)

Note that the module path that ISpellChecker is in. You'll want to use that exactly when specifying the types you need to generate.

It's important to note that for COM and WinRT APIs, methods are only available when all the types use in those methods parameters and return types are also generated. When you want to use a certain type's method, make sure you're generating everything you need to use that method.

In the build.rs file add the following:

```rust
fn main() {
    windows::build!(
        // Note that we're using the `intl` namespace which is nested inside the `win32` namespace
        // which itself is inside the `windows` namespace.
        Windows::Win32::Intl::{ISpellChecker, SpellCheckerFactory, ISpellCheckerFactory, CORRECTIVE_ACTION, IEnumSpellingError, ISpellingError},
        Windows::Win32::SystemServices::{BOOL, PWSTR, S_FALSE},
        Windows::Win32::Com::IEnumString
    )
}
```

We're not done! This generates the bindings, but it puts them into the target folder of our crate. We want to actually use this generated code not just generate the code and forget about it. In order for the generated code to be exported from the `bindings` crate, we need to include it in the bindings crate. Change the lib.rs file of the bindings crate to the following:

```rust
::windows::include_bindings!();
```

This effectively copy/pastes the generated code into the `lib.rs` file. The `bindings` crate now exports all the bindings we've generated. Next, we need to make sure our `spellchecker` crate depends on the `bindings` crate. In the `spellchecker` crate's Cargo.toml file, add the `bindings` crate as a dependency. We'll also add the `windows` crate as a dependency since we'll be using that as well in our `spellchecker` crate.

```toml
# ^^ The rest of the Cargo.toml remains the same
[dependencies]
bindings = { path = "bindings" }
windows = { git = "https://github.com/microsoft/windows-rs" }  # TODO: change this to the crates.io version when that gets released
```

And we're done bootstraping the project. Now we'll move on to the code of the `spellchecker` crate which will use both the generated bindings from the `bindings` crate as well as helper functionality from the `windows` crate.

## Initialization Code

Inside of the main.rs file for the `spellchecker` crate, we'll start by adding the set up code. We'll initialize the COM runtime on the main thread. We'll make the `main` function return a `windows::Result` which is simply a standard `Result` type with the error hardcoded as a `windows::Error`.

```rust
fn main() -> windows::Result<()> {
    // initialize the main thread as a multithreaded apartment
    windows::initialize_mta()?;
    // The rest of the code will go here!
    Ok(())
}
```

Next, we'll initialize the `ISpellCheckerFactory` which is what gives us access to spellcheckers. We'll first make sure the `intl` namespace and two types we'll need `PWSTR` and `BOOL` are in scope at the top of the main.rs file:

```rust
use bindings::Windows::Win32::Intl;
use bindings::Windows::Win32::SystemServices::{BOOL, PWSTR};
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

Next, we'll use the `ISpellCheckerFactory` to check that we can spell check U.S. English, and then we'll get a spellchecker instance:

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
        if result == windows::ErrorCode::S_FALSE {
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

Looking at the [docs for `CORRECTIVE_ACTION](https://docs.microsoft.com/en-us/windows/win32/api/spellcheck/ne-spellcheck-corrective_action), we can see that 1-index action corresponds to `CORRECTIVE_ACTION_GET_SUGGESTIONS`, meaning the spellchecking API has suggestions.

## Next Steps

With what you've learned so far, finish the exercise of building a spellchecker that spellchecks the provided text. This will entail handling the various corrective actions and ensuring that the right thing is done for each of them.

If you get stuck, you can always see the final code [here](../examples/spellchecker).