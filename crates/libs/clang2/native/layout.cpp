#include "CXCursor.h"
#include <clang/AST/ASTContext.h>
#include <clang/AST/Attr.h>
#include <clang/AST/RecordLayout.h>
#include <llvm/Config/llvm-config.h>
#include <stdint.h>
#include "cursor_decl.inc"

static_assert(LLVM_VERSION_MAJOR == 22 && LLVM_VERSION_MINOR == 1 && LLVM_VERSION_PATCH == 8);
static_assert(LLVM_ENABLE_ABI_BREAKING_CHECKS == 0);
static_assert(_ITERATOR_DEBUG_LEVEL == 0);

struct LayoutContract {
    uint32_t packing;
    uint32_t alignment;
    uint32_t required_alignment;
};
static_assert(sizeof(LayoutContract) == 12);

extern "C" __declspec(dllexport) uint32_t windows_clang_layout_abi() {
    return 1;
}

extern "C" __declspec(dllexport) const void *windows_clang_layout_parser() {
    return reinterpret_cast<const void *>(&clang_createIndex);
}

extern "C" __declspec(dllexport) const char *windows_clang_layout_fingerprint() {
    return WINDOWS_CLANG_PARSER_SHA256;
}

extern "C" __declspec(dllexport) uint32_t
windows_clang_layout_record(CXCursor cursor, LayoutContract *result) {
    if (!result || (cursor.kind != CXCursor_StructDecl &&
                    cursor.kind != CXCursor_ClassDecl &&
                    cursor.kind != CXCursor_UnionDecl)) {
        return 1;
    }
    const auto *record = llvm::dyn_cast_or_null<clang::RecordDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!record || !record->isCompleteDefinition()) {
        return 1;
    }
    LayoutContract contract{};
    for (const auto *attribute : record->attrs()) {
        if (const auto *packing = llvm::dyn_cast<clang::MaxFieldAlignmentAttr>(attribute)) {
            contract.packing = packing->getAlignment() / 8;
        } else if (const auto *aligned = llvm::dyn_cast<clang::AlignedAttr>(attribute)) {
            if (!aligned->isDeclspec()) {
                return 2;
            }
        } else {
            return 2;
        }
    }
    contract.alignment = record->getMaxAlignment() / 8;
    contract.required_alignment = static_cast<uint32_t>(
        record->getASTContext().getASTRecordLayout(record).getRequiredAlignment().getQuantity());
    *result = contract;
    return 0;
}
