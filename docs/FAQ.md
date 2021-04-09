# Frequently Asked Questions

## How do I read the signatures of generated functions and methods? What's with `IntoParam`?

Let's take a look at the method signature of `ISpellCheckerFactor::IsSupported`:

```rust
pub unsafe fn IsSupported<'a, T0__: IntoParam<'a, PWSTR>>(
    &self,
    languagetag: T0__,
    value: *mut BOOL
) -> ErrorCode
```

This looks somewhat complicated, but it makes using the API pretty straight forward. The method is generic on both a lifetime `'a`, and a generic type parameter `T0__`. `T0__` is constrained by a trait `IntoParam` which is defined in the `windows` crate itself. Essentially, `IntoParam` is a slightly specialized version of Rust's `std::convert::Into`. It is implemented on all types that can be converted to a parameter of the type its generic over. 

In other words, `T0__` is any type that can be converted into a parameter of type `PWSTR` that lives for at least the lifetime `'a`. 

It turns out that `IntoParam<'a, PWSTR>` is implemented for `&'a str` so we can simply pass a string literal. `IntoParam<'a, PWSTR>` is also implemented on `String` and `PWSTR` itself. As long as we supply any type that implements `IntoParam<'a, PWSTR>` our code will compile!

If you get an error message saying that a specific type does not satisfy the trait bound, then you are passing a type which cannot be converted to the correct type. If you think this is in error (i.e., you think the type you have should be trivially convertible to the param type in question), don't hesitate to [file an issue](https://github.com/microsoft/windows-rs/issues).

## I get a "method missing" error when the documentation says the method exists.

In order to greatly reduce the amount of code generated, methods are only generated when all of their argument and return types have also been generated. Make sure to include the types of the missing method's arguments and return type in the `build!` macro.

For example, if you include `Windows::Media::SpeechSynthesis::SpeechSynthesizer` in your `build!` macro, and you want to use the `Options` method on `SpeechSynthesizer`, you'll have to also include that method's return type `Windows::Media::SpeechSynthesis::SpeechSynthesizerOptions` in the `build!` macro, otherwise the `Options` method will not be generated.

## How do I implement a WinRT or COM interface?

Implementing WinRT and COM interfaces is under active development, but will be ready soon. Follow [this issue](https://github.com/microsoft/windows-rs/issues/81) for more information.

## How do I create an event handler?

TODO: https://github.com/microsoft/windows-rs/issues/687
