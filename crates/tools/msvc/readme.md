The libs for MSVC are generated as follows:

0. Ensure the latest Windows SDK is installed (https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/)
1. Open a Command Prompt
2. Execute: `"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars32.bat" x86`
3. Navigate to crate root
4. Execute: `cargo run -p tool_msvc`
5. Repeat steps 2-4, replacing `x86` with `amd64` and `amd64_arm64`
