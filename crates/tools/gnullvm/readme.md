The Windows umbrella lib (targeting mingw-w64 LLVM tooling) is generated using the following steps:

0. Ensure MSYS2 MinGW environment is installed (https://www.mingw-w64.org/downloads/)
1. Ensure `~\.cargo\config` has the following block to override toolchain defaults (customize paths as needed):

    ```toml
    [target.x86_64-pc-windows-gnu]
    linker = "C:\\msys64\\mingw64\\bin\\x86_64-w64-mingw32-gcc.exe"
    ar = "C:\\msys64\\mingw64\\bin\\ar.exe"
    ```

2. Open `MSYS2 MinGW 64-bit`
3. Execute: `pacman -Syuu --noconfirm` (repeat until no further updates available)
4. Execute `pacman --needed -S mingw-w64-x86_64-llvm`
5. Navigate to crate root
6. Execute: `PATH=$USERPROFILE/.cargo/bin:$PATH cargo run -p tool_gnullvm --target x86_64-pc-windows-gnu -- all`
