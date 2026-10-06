## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will determine whether you need to provide a CLA and
decorate the PR appropriately (for example, with a status check or comment). Follow the bot's
instructions. You only need to do this once across all repositories using our CLA.

This project has adopted the
[Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more
information, see the
[Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact
[opencode@microsoft.com](mailto:opencode@microsoft.com).

## Before Contributing

Start by
[opening a new issue](https://github.com/microsoft/windows-rs/issues/new/choose) or commenting on
[an existing issue](https://github.com/microsoft/windows-rs/issues) to discuss a feature or bug
fix. This lets maintainers and contributors agree on an approach before you spend time on a pull
request. The [pull request template][pr-template] therefore asks you to refer to an existing issue.

[pr-template]: https://github.com/microsoft/windows-rs/blob/master/.github/pull_request_template.md

## Updating build dependencies

SDK and compiler package pins belong to their acquisition tools under `crates/tools`. Change the
owner's version constant, run that tool, then regenerate its consumers. See the
[metadata regeneration order](crates/windows-default.md#downstream-regeneration). These tools
restore the selected versions; they do not select newer releases.

Dependabot proposes Cargo, GitHub Actions, and CsWinRT NuGet updates. CsWinRT versions remain in
their individual C# projects, with major-version updates excluded to keep the sample, stable
benchmark, and preview probe on their selected generations.

CI-only pins stay with their consumers: the installed Windows SDK in
`.github/actions/fix-environment/action.yml`, LLVM-MinGW in `.github/workflows/cross.yml`, and
Cargo analysis utilities in `.github/workflows/reactor.yml`. The installed SDK is a runner
toolchain prerequisite, separate from the SDK packages used to generate metadata. The x86 .NET
runtime channels come from the C# project target frameworks.

Generated MSRV workflows take their cache action reference from
`crates/tools/yml/src/msrv.rs`. Update that constant and rerun `tool-yml` when changing the
reference; a Dependabot edit to generated YAML alone does not update its source.

Rust stable/nightly, runner-provided MSVC and .NET SDKs, distribution packages, and the WebView2
Evergreen runtime follow their upstream channels. They are not exact-version metadata inputs.
Scrapers still require installed MSVC headers; their LLVM and Windows SDK inputs are pinned.