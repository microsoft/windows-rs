## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Getting started

This guide is meant to help developers that are perhaps unfamiliar with Rust development on Windows. It will help you get your developer machine configured to work on the Rust/WinRT project. If you're already a seasoned Rust developer you will likely not find anything of value here, but feel free to contribute improvements or suggestions. Other documentation will be provided for actually using and understanding how Rust/WinRT works. 

Install the following prerequisites:

* [Visual Studio 2019](https://visualstudio.microsoft.com/downloads/) - be sure to install the C++ tools as this is required by the Rust compiler (only the linker is required).
* [Visual Studio Code for Windows](https://code.visualstudio.com/Download) - this is the default IDE used for Rust.
* [Python](https://www.python.org/downloads/) - be sure to install the x64 version as this is required for debugging support.
* [Git](https://git-scm.com/downloads)
* [Rust](https://rustup.rs/) - this installs `rustup` which is a tool for installing Rust toolchains and common Rust related tooling.

Now clone the repo as follows:

```
git clone https://github.com/microsoft/windows-rs
```

Then change to the `windows-rs` directory created by git and type `code .` to open Visual Studio Code.

Type `Ctrl+Shift+X` to open the extensions panel and install the following extensions:

* rust-analyzer - there are others, but this is the only Rust extension that I've tried that actually works reliably.
* CodeLLDB - you can also use the Microsoft C++ extension for debugging, but this one does a better job of integrating with the IDE.
* C/C++ - the Microsoft C++ extension doesn't integrate as well with the IDE, but provides superior debugging information, so you may want to have that on hand for an emergency.

You should be prompted to download and install the Rust language server. Go ahead and let that install. You may need to restart VS Code and give it a few moments to load, after which it should all be ready and working pretty well.

For example, you can open the `tests/enum.rs` source file and you should see inlay links for running tests and debugging. 

You can run the tests and the terminal panel should display the results.

You can debug the tests by placing a breakpoint (F9) and then clicking the Debug link.

You can also run all of the tests from the command line as follows:

```
cargo test --all
```

Make sure there are no errors or warnings before submitting any code. Before submitting a PR you must also format the code. Failure to do this will result in the PR being rejected by the GitHub build action. You can format the code as follows:

```
cargo fmt --all
```

You can do this automatically every time a file is saved by turning on the "Format on Save" option in VS Code. 
