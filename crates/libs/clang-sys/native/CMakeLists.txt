cmake_minimum_required(VERSION 3.20)
project(WindowsClang LANGUAGES C CXX)

# The LLVM release libraries link the static CRT.
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
set(CMAKE_CXX_STANDARD 17)
if(NOT PATH_TO_LLVM)
    message(FATAL_ERROR "PATH_TO_LLVM is required")
endif()

find_package(Clang REQUIRED CONFIG PATHS "${PATH_TO_LLVM}/lib/cmake/clang" NO_DEFAULT_PATH)
add_library(windows_clang SHARED extension.cpp)
target_include_directories(windows_clang PRIVATE ${LLVM_INCLUDE_DIRS} ${CLANG_INCLUDE_DIRS})
target_compile_definitions(windows_clang PRIVATE WINDOWS_CLANG_EXPORTS)
# Macro probing runs its own incremental parse. MS inline asm in SDK headers needs the MC layers
# of every parsing target, registered in this module's copy of the LLVM target registry.
target_link_libraries(windows_clang PRIVATE
    libclang clangAST clangFrontend clangDriver clangParse clangSema clangLex clangBasic
    LLVMX86Info LLVMX86Desc LLVMX86AsmParser
    LLVMAArch64Info LLVMAArch64Desc LLVMAArch64AsmParser
)
