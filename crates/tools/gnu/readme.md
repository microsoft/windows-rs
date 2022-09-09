The Windows umbrella lib (targeting GNU and LLVM tooling) is generated using the following steps:

0. Ensure MSYS2 MinGW environment is installed (https://www.mingw-w64.org/downloads/)
1. Open `MSYS2 MinGW 64-bit`
2. Execute: `pacman -Syuu --noconfirm` (repeat until no further updates available)
3. Execute: `pacman --needed -S mingw-w64-x86_64-llvm`
4. Repeat step 1 if needed
5. Navigate to crate root
6. Execute: `PATH=$USERPROFILE/.cargo/bin:$PATH cargo run -p tool_gnu -- all`
