// LLVM's intrinsic fallback templates trigger MSVC's unreachable-code warning.
#pragma warning(push)
#pragma warning(disable: 4702)
#include "CXCursor.h"
#include "CXSourceLocation.h"
#include "CXTranslationUnit.h"
#include <clang/AST/ASTContext.h>
#include <clang/AST/Attr.h>
#include <clang/AST/RecordLayout.h>
#include <clang/AST/VTableBuilder.h>
#include <clang/Frontend/ASTUnit.h>
#include <clang/Lex/Preprocessor.h>
#include <llvm/Config/llvm-config.h>
#pragma warning(pop)
#include <cstring>
#include <stdint.h>
#include "cursor_decl.inc"

static_assert(LLVM_VERSION_MAJOR == 22 && LLVM_VERSION_MINOR == 1 && LLVM_VERSION_PATCH == 8);
static_assert(LLVM_ENABLE_ABI_BREAKING_CHECKS == 0);
static_assert(_ITERATOR_DEBUG_LEVEL == 0);

struct LayoutContract {
    uint32_t packing;
    uint32_t alignment;
    uint32_t required_alignment;
    uint32_t type_visibility;
};
static_assert(sizeof(LayoutContract) == 16);

struct InterfaceContract {
    uint32_t size;
    uint32_t inherited;
    uint32_t no_vtable;
    uint32_t packing;
    char guid[36];
};
static_assert(sizeof(InterfaceContract) == 52);

extern "C" __declspec(dllexport) uint32_t windows_clang_layout_abi() {
    return 4;
}

extern "C" __declspec(dllexport) const void *windows_clang_layout_parser() {
    return reinterpret_cast<const void *>(&clang_createIndex);
}

extern "C" __declspec(dllexport) const char *windows_clang_layout_fingerprint() {
    return WINDOWS_CLANG_PARSER_SHA256;
}

struct MacroContract {
    CXSourceLocation definition;
    uint32_t function_like;
    uint32_t variadic;
};
using MacroToken = uint32_t (*)(void *, uint32_t, const char *, uint32_t);

extern "C" __declspec(dllexport) uint32_t
windows_clang_layout_macro(CXTranslationUnit unit, CXSourceLocation location,
                          const char *name, MacroContract *result,
                          void *context, MacroToken emit) {
    auto *ast = clang::cxtu::getASTUnit(unit);
    if (!ast || !name || !result || !emit ||
        location.ptr_data[0] != &ast->getSourceManager()) {
        return 2;
    }
    auto &preprocessor = ast->getPreprocessor();
    const auto definition = preprocessor.getMacroDefinitionAtLoc(
        preprocessor.getIdentifierInfo(name), clang::cxloc::translateSourceLocation(location));
    if (definition.isAmbiguous()) {
        return 2;
    }
    const auto *macro = definition.getMacroInfo();
    if (!macro) {
        return 1;
    }
    *result = {
        clang::cxloc::translateSourceLocation(ast->getSourceManager(),
                                             ast->getLangOpts(), macro->getDefinitionLoc()),
        macro->isFunctionLike() ? 1u : 0u,
        macro->isVariadic() ? 1u : 0u
    };
    if (macro->isFunctionLike()) {
        for (const auto *parameter : macro->params()) {
            const auto spelling = parameter->getName();
            if (emit(context, 0, spelling.data(), static_cast<uint32_t>(spelling.size()))) {
                return 3;
            }
        }
    }
    for (const auto &token : macro->tokens()) {
        const auto spelling = preprocessor.getSpelling(token);
        if (emit(context, 1, spelling.data(), static_cast<uint32_t>(spelling.size()))) {
            return 3;
        }
    }
    return 0;
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
        } else if (const auto *visibility = llvm::dyn_cast<clang::TypeVisibilityAttr>(attribute)) {
            contract.type_visibility = static_cast<uint32_t>(visibility->getVisibility()) + 1;
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

extern "C" __declspec(dllexport) uint32_t
windows_clang_layout_field(CXCursor cursor, uint32_t *result) {
    if (!result || cursor.kind != CXCursor_FieldDecl) {
        return 1;
    }
    const auto *field = llvm::dyn_cast_or_null<clang::FieldDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!field) {
        return 1;
    }
    *result = (field->getIdentifier() ? 1u : 0u)
            | (field->isAnonymousStructOrUnion() ? 2u : 0u);
    return 0;
}

extern "C" __declspec(dllexport) uint32_t
windows_clang_layout_interface(CXCursor cursor, InterfaceContract *result) {
    if (!result || (cursor.kind != CXCursor_StructDecl && cursor.kind != CXCursor_ClassDecl)) {
        return 1;
    }
    const auto *record = llvm::dyn_cast_or_null<clang::CXXRecordDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!record) {
        return 1;
    }
    record = record->getDefinition();
    if (!record) {
        return 1;
    }
    const auto *uuid = record->getAttr<clang::UuidAttr>();
    if (!uuid) {
        return 1;
    }
    if (!record->isDynamicClass() || !record->field_empty() ||
        record->getNumBases() > 1 || record->getNumVBases() != 0) {
        return 2;
    }
    InterfaceContract contract{};
    for (const auto *attribute : record->attrs()) {
        if (llvm::isa<clang::UuidAttr>(attribute)) {
            continue;
        } else if (llvm::isa<clang::MSNoVTableAttr>(attribute)) {
            contract.no_vtable = 1;
        } else if (const auto *packing = llvm::dyn_cast<clang::MaxFieldAlignmentAttr>(attribute)) {
            contract.packing = packing->getAlignment() / 8;
        } else {
            return 3;
        }
    }
    for (const auto *method : record->methods()) {
        if (method->isImplicit()) {
            continue;
        }
        if (llvm::isa<clang::CXXConstructorDecl, clang::CXXDestructorDecl>(method) ||
            (method->isVirtual() && (!method->isPureVirtual() || method->size_overridden_methods()))) {
            return 4;
        }
        if (!method->isVirtual() && !method->isInlined()) {
            return 4;
        }
    }
    auto *tables = llvm::dyn_cast<clang::MicrosoftVTableContext>(
        record->getASTContext().getVTableContext());
    if (!tables) {
        return 2;
    }
    const auto extent = [tables](const clang::CXXRecordDecl *owner) {
        uint32_t count = 0;
        for (const auto &component :
             tables->getVFTableLayout(owner, clang::CharUnits::Zero()).vtable_components()) {
            if (component.isFunctionPointerKind()) {
                ++count;
            }
        }
        return count;
    };
    if (record->getNumBases()) {
        const auto &base = *record->bases_begin();
        if (base.isVirtual() || base.getAccessSpecifier() != clang::AS_public) {
            return 2;
        }
        const auto *parent = base.getType()->getAsCXXRecordDecl();
        if (!parent || !parent->isDynamicClass() ||
            !record->getASTContext().getASTRecordLayout(record).getBaseClassOffset(parent).isZero()) {
            return 2;
        }
        contract.inherited = extent(parent);
    }
    const auto guid = uuid->getGuid();
    if (guid.size() != sizeof(contract.guid)) {
        return 2;
    }
    std::memcpy(contract.guid, guid.data(), sizeof(contract.guid));
    contract.size = extent(record);
    *result = contract;
    return 0;
}

extern "C" __declspec(dllexport) uint32_t
windows_clang_layout_method(CXCursor cursor, uint32_t *result) {
    if (!result || cursor.kind != CXCursor_CXXMethod) {
        return 1;
    }
    const auto *method = llvm::dyn_cast_or_null<clang::CXXMethodDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!method || !method->isPureVirtual() || method->size_overridden_methods() ||
        method->isConst() || method->isVolatile() || method->getRefQualifier() != clang::RQ_None) {
        return 2;
    }
    auto *tables = llvm::dyn_cast<clang::MicrosoftVTableContext>(
        method->getASTContext().getVTableContext());
    if (!tables) {
        return 2;
    }
    const auto location = tables->getMethodVFTableLocation(clang::GlobalDecl(method));
    if (location.VBTableIndex || location.VBase || !location.VFPtrOffset.isZero() ||
        location.Index > UINT32_MAX) {
        return 2;
    }
    *result = static_cast<uint32_t>(location.Index);
    return 0;
}

extern "C" __declspec(dllexport) uint32_t
windows_clang_layout_helper(CXCursor cursor, uint32_t *result) {
    if (!result || (cursor.kind != CXCursor_CXXMethod &&
                    cursor.kind != CXCursor_FunctionTemplate)) {
        return 1;
    }
    const auto *declaration = clang::cxcursor::getCursorDecl(cursor);
    if (const auto *function = llvm::dyn_cast_or_null<clang::FunctionTemplateDecl>(declaration)) {
        declaration = function->getTemplatedDecl();
    }
    const auto *method = llvm::dyn_cast_or_null<clang::CXXMethodDecl>(declaration);
    if (!method || method->isVirtual() || !method->isInlined()) {
        return 2;
    }
    *result = 1;
    return 0;
}
