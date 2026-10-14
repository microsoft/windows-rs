#pragma once
#include <clang-c/Index.h>

#ifdef WINDOWS_CLANG_EXPORTS
#define WINDOWS_CLANG_API __declspec(dllexport)
#else
#define WINDOWS_CLANG_API __declspec(dllimport)
#endif

#ifdef __cplusplus
extern "C" {
#endif

enum WindowsClangStatus {
    WindowsClangSuccess = 0,
    WindowsClangInvalidArgument = 1,
    WindowsClangWrongKind = 2,
    WindowsClangDependent = 3
};

enum WindowsClangFunctionProperties {
    WindowsClangExtendedParameters = 1,
    WindowsClangExtraAttributes = 2,
    WindowsClangArmSMEAttributes = 4,
    WindowsClangFunctionEffects = 8,
    WindowsClangMethodQualifiers = 16,
    WindowsClangRefQualifier = 32,
    WindowsClangCFIUncheckedCallee = 64
};

enum WindowsClangTypeQualifiers {
    WindowsClangConst = 1,
    WindowsClangVolatile = 2,
    WindowsClangRestrict = 4,
    WindowsClangUnaligned = 8,
    WindowsClangOtherQualifiers = 16
};

struct WindowsClangFunctionInfo {
    unsigned has_regparm;
    unsigned regparm;
    unsigned no_return;
    unsigned produces_result;
    unsigned no_caller_saved_registers;
    unsigned no_cf_check;
    unsigned cmse_nonsecure_call;
    unsigned additional_properties;
};

enum WindowsClangAlignmentKind {
    WindowsClangAlignmentGNU = 0,
    WindowsClangAlignmentStandard = 1,
    WindowsClangAlignmentDeclspec = 2
};

struct WindowsClangAlignmentInfo {
    unsigned bits;
    enum WindowsClangAlignmentKind kind;
};

WINDOWS_CLANG_API unsigned windows_clang_getABIVersion(void);
WINDOWS_CLANG_API const char* windows_clang_getLLVMVersion(void);
WINDOWS_CLANG_API const void* windows_clang_getRuntimeIdentity(void);
// Names are static strings owned by the library; non-attribute inputs return null.
WINDOWS_CLANG_API const char* windows_clang_getAttributeKindName(CXCursor cursor);
WINDOWS_CLANG_API const char* windows_clang_getTypeAttributeKindName(CXType type);
// Includes qualifiers inherited through typedefs; OtherQualifiers requires further decoding.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getTypeQualifiers(
    CXType type, unsigned* qualifiers);
// Writes the requested maximum field alignment in bits, not the observed record alignment.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getMaxFieldAlignment(
    CXCursor cursor, unsigned* alignment_bits);
// Evaluated request, including zero, rather than the resulting declaration layout.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getAlignment(
    CXCursor cursor, struct WindowsClangAlignmentInfo* info);
// additional_properties flags prototype information that this structure does not decode.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getFunctionInfo(
    CXType type, struct WindowsClangFunctionInfo* info);

#ifdef __cplusplus
}
#endif
