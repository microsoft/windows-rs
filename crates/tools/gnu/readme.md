The Windows umbrella lib (targeting GNU tooling) is generated using the following steps:

0. Ensure MSYS2 MinGW environment is installed (https://www.mingw-w64.org/downloads/)
1. Ensure `~\.cargo\config` has the following blocks to override toolchain defaults (customize paths as needed):

    ```toml
    [target.i686-pc-windows-gnu]
    linker = "C:\\msys64\\mingw32\\bin\\i686-w64-mingw32-gcc.exe"
    ar = "C:\\msys64\\mingw32\\bin\\ar.exe"

    [target.x86_64-pc-windows-gnu]
    linker = "C:\\msys64\\mingw64\\bin\\x86_64-w64-mingw32-gcc.exe"
    ar = "C:\\msys64\\mingw64\\bin\\ar.exe"
    ```

1. Open `MSYS2 MinGW 32-bit`
2. Execute: `pacman -Syuu --noconfirm` (repeat until no further updates available)
3. Repeat step 1 if needed
4. Navigate to crate root
5. Execute: `PATH=$USERPROFILE/.cargo/bin:$PATH cargo run -p tool_gnu --target i686-pc-windows-gnu`
6. Repeat steps 1-5, using the `MSYS MinGW 64-bit` environment and `x86_64-pc-windows-gnu` target
