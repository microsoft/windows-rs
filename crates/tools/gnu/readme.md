The libs for GNU and LLVM are generated as follows:

0. Ensure MSYS2 MinGW environment is installed (https://www.mingw-w64.org/downloads/)
1. Open `MSYS2 MINGW64`
2. Execute: `pacman -Syuu --noconfirm` (repeat until no further updates available)
3. Execute: `pacman --needed -S mingw-w64-x86_64-binutils mingw-w64-x86_64-llvm`
4. Repeat step 1 if needed
5. Navigate to crate root
6. Execute: `PATH=$USERPROFILE/.cargo/bin:$PATH cargo run -p tool_gnu -- all`
