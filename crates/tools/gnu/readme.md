The Windows umbrella lib (targeting GNU tooling) is generated using the following steps:

0. Ensure MSYS2 MinGW environment is installed (https://www.mingw-w64.org/downloads/)
1. Open `MSYS2 MinGW 64-bit`
2. Execute: `pacman -Syuu --noconfirm` (repeat until no further updates available)
3. Repeat step 1 if needed
4. Navigate to crate root
5. Execute: `PATH=$USERPROFILE/.cargo/bin:$PATH cargo run -p tool_gnu -- all`
