# Changelog

# [0.7.1] - 2020-07-06

## Added

* Better error handling in `cargo winrt` [#206](https://github.com/microsoft/winrt-rs/pull/206)
* Only re-generate code if the relevant winmd files have changed [#205](https://github.com/microsoft/winrt-rs/pull/205)
* Handle local dependencies in `cargo winrt` [#217](https://github.com/microsoft/winrt-rs/pull/217)
* Don't use `r#` raw identifiers unless necessary [#216](https://github.com/microsoft/winrt-rs/pull/216)
* Improve macro error handling [#219](https://github.com/microsoft/winrt-rs/pull/219)
* Improved color support for `cargo winrt` [#224](https://github.com/microsoft/winrt-rs/pull/224)
* Internal improvement for caching of tokens during generation [#226](https://github.com/microsoft/winrt-rs/pull/226)
* Add support for array arguments [#230](https://github.com/microsoft/winrt-rs/pull/230)
* Dynamically load Win10 APIs [#228](https://github.com/microsoft/winrt-rs/pull/228)
* Add support for both local and url based NuGet dependencies [#232](https://github.com/microsoft/winrt-rs/pull/228)
* Support for agile types moving across threads [#231](https://github.com/microsoft/winrt-rs/pull/231)
* And more!

## Changed

## Fixed

* Properly move dlls over from NuGet packages when dlls are in `win10-$ARCH` folders [#234](https://github.com/microsoft/winrt-rs/pull/234)

# [0.7.0] - 2020-06-04

Initial release! 🎉🎉🎉
