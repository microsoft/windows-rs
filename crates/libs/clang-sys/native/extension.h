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

enum WindowsClangLanguageLinkage {
    WindowsClangNoLanguageLinkage = 0,
    WindowsClangCLanguageLinkage = 1,
    WindowsClangCXXLanguageLinkage = 2
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
// Function prototype method qualifiers, distinct from qualifiers on the function type itself.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getMethodQualifiers(
    CXType type, unsigned* qualifiers);
// Absolute Microsoft vftable slot; rejects virtual-base or secondary-vptr adjustment.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getVirtualMethodSlot(
    CXCursor cursor, unsigned long long* slot);
// Null cursor/empty string means no default. The caller disposes the compiler-printed expression.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getParameterDefaultArgument(
    CXCursor parameter, CXCursor* expression, CXString* spelling);
// Writes the requested maximum field alignment in bits, not the observed record alignment.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getMaxFieldAlignment(
    CXCursor cursor, unsigned* alignment_bits);
// Evaluated request, including zero, rather than the resulting declaration layout.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getAlignment(
    CXCursor cursor, struct WindowsClangAlignmentInfo* info);
// Reads TypeVisibilityAttr, not the declaration's effective symbol visibility.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getTypeVisibility(
    CXCursor cursor, enum CXVisibilityKind* visibility);
// Whether a typedef's direct type specifier defines its underlying record or enum.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getTypedefDefinition(
    CXCursor cursor, unsigned* defines_tag);
// Whether a child expression is a fixed array bound or bitfield width, not an initializer.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getLayoutExpression(
    CXCursor declaration, CXCursor expression, unsigned* is_layout_expression);
// Includes initializers that the stock cursor visitor omits, such as bitfield initializers.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getFieldInitializer(
    CXCursor cursor, unsigned* has_initializer);
// additional_properties flags prototype information that this structure does not decode.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getFunctionInfo(
    CXType type, struct WindowsClangFunctionInfo* info);
// The caller owns the returned string and releases it with clang_disposeString.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getUuid(
    CXCursor cursor, CXString* uuid);
// Reads one DeprecatedAttr, not effective platform availability. Both strings are caller-owned.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getDeprecation(
    CXCursor cursor, CXString* message, CXString* replacement);
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getLanguageLinkage(
    CXCursor cursor, enum WindowsClangLanguageLinkage* linkage);
// Unlike clang_Cursor_Evaluate, this rejects folding that ignores runtime side effects.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getIntegerConstantInitializer(
    CXCursor cursor, unsigned* is_constant);
// C++ constant expressions, including floating-point values, without runtime side effects.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getConstantInitializer(
    CXCursor cursor, unsigned* is_constant);
// Evaluates an unsigned integer leaf in a constant aggregate initializer. Indices select
// struct fields or array elements; no source tokens or object-memory layout are interpreted.
WINDOWS_CLANG_API enum WindowsClangStatus windows_clang_getConstantIntegerElement(
    CXCursor cursor, const unsigned* indices, unsigned count, unsigned long long* value);

#ifdef __cplusplus
}
#endif
