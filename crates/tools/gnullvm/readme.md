The Windows umbrella lib (targeting mingw-w64 LLVM tooling) is generated using the following steps:

0. Ensure MSYS2 MinGW environment is installed (https://www.mingw-w64.org/downloads/)
1. Execute: `C:\msys64\clang64.exe`
2. Execute: `pacman -Syuu --noconfirm` (repeat until no further updates available)
3. Execute `pacman --needed -S mingw-w64-clang-x86_64-toolchain`
4. Navigate to crate root
5. Execute: `PATH=$(cygpath -u $USERPROFILE)/.cargo/bin:$PATH cargo run -p tool_gnullvm --target x86_64-pc-windows-gnu -- all`
