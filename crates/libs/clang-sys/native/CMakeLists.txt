cmake_minimum_required(VERSION 3.20)
project(WindowsClang LANGUAGES C CXX)

set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
if(NOT CLANGSHARP_SOURCE OR NOT PATH_TO_LLVM)
    message(FATAL_ERROR "CLANGSHARP_SOURCE and PATH_TO_LLVM are required")
endif()

# Import Clang here so the macro probing link libraries below share ClangSharp's targets.
find_package(Clang REQUIRED CONFIG PATHS "${PATH_TO_LLVM}/lib/cmake/clang" NO_DEFAULT_PATH)
add_subdirectory("${CLANGSHARP_SOURCE}" clangsharp)
target_sources(ClangSharp PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/extension.cpp")
target_include_directories(ClangSharp PRIVATE
    "${CLANGSHARP_SOURCE}/sources/libClangSharp"
    "${CMAKE_CURRENT_SOURCE_DIR}"
)
target_compile_definitions(ClangSharp PRIVATE WINDOWS_CLANG_EXPORTS)
# Macro probing runs its own incremental parse. MS inline asm in SDK headers needs the MC layers
# of every parsing target, registered in this module's copy of the LLVM target registry.
target_link_libraries(ClangSharp PRIVATE
    clangDriver clangParse clangSema clangLex clangBasic
    LLVMX86Info LLVMX86Desc LLVMX86AsmParser
    LLVMAArch64Info LLVMAArch64Desc LLVMAArch64AsmParser
)
