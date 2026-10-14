cmake_minimum_required(VERSION 3.20)
project(WindowsClang LANGUAGES C CXX)

set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
if(NOT CLANGSHARP_SOURCE OR NOT PATH_TO_LLVM)
    message(FATAL_ERROR "CLANGSHARP_SOURCE and PATH_TO_LLVM are required")
endif()

add_subdirectory("${CLANGSHARP_SOURCE}" clangsharp)
target_sources(ClangSharp PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/extension.cpp")
target_include_directories(ClangSharp PRIVATE
    "${CLANGSHARP_SOURCE}/sources/libClangSharp"
    "${CMAKE_CURRENT_SOURCE_DIR}"
)
target_compile_definitions(ClangSharp PRIVATE WINDOWS_CLANG_EXPORTS)
