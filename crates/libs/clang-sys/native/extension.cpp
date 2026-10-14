#include "extension.h"
#include "CXCursor.h"
#include "CXType.h"
#include <clang/AST/Attr.h>
#include <clang/AST/Type.h>
#include <llvm/Config/llvm-config.h>

static const char* attributeName(clang::attr::Kind kind) {
    switch (kind) {
#define ATTR(Name) case clang::attr::Name: return #Name;
#include <clang/Basic/AttrList.inc>
    }
    return nullptr;
}

unsigned windows_clang_getABIVersion(void) {
    return 1;
}

const char* windows_clang_getLLVMVersion(void) {
    return LLVM_VERSION_STRING;
}

const void* windows_clang_getRuntimeIdentity(void) {
    return reinterpret_cast<const void*>(&clang_getClangVersion);
}

const char* windows_clang_getAttributeKindName(CXCursor cursor) {
    if (!clang_isAttribute(cursor.kind)) {
        return nullptr;
    }
    return attributeName(clang::cxcursor::getCursorAttr(cursor)->getKind());
}

const char* windows_clang_getTypeAttributeKindName(CXType type) {
    auto value = GetQualType(type);
    if (value.isNull()) {
        return nullptr;
    }
    auto attribute = llvm::dyn_cast<clang::AttributedType>(value.getTypePtr());
    return attribute ? attributeName(attribute->getAttrKind()) : nullptr;
}

WindowsClangStatus windows_clang_getMaxFieldAlignment(CXCursor cursor, unsigned* alignment_bits) {
    if (!alignment_bits) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isAttribute(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto attribute = llvm::dyn_cast<clang::MaxFieldAlignmentAttr>(
        clang::cxcursor::getCursorAttr(cursor));
    if (!attribute) {
        return WindowsClangWrongKind;
    }
    *alignment_bits = attribute->getAlignment();
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getFunctionInfo(CXType type, WindowsClangFunctionInfo* info) {
    if (!info) {
        return WindowsClangInvalidArgument;
    }
    auto value = GetQualType(type);
    if (value.isNull()) {
        return WindowsClangWrongKind;
    }
    auto function = value->getAs<clang::FunctionType>();
    if (!function) {
        return WindowsClangWrongKind;
    }
    auto extension = function->getExtInfo();
    unsigned properties = 0;
    if (auto prototype = llvm::dyn_cast<clang::FunctionProtoType>(function)) {
        if (prototype->hasExtParameterInfos()) properties |= WindowsClangExtendedParameters;
        if (prototype->getExtraAttributeInfo()) properties |= WindowsClangExtraAttributes;
        if (prototype->getAArch64SMEAttributes()) properties |= WindowsClangArmSMEAttributes;
        if (!prototype->getFunctionEffects().empty()) properties |= WindowsClangFunctionEffects;
        if (!prototype->getMethodQuals().empty()) properties |= WindowsClangMethodQualifiers;
        if (prototype->getRefQualifier() != clang::RQ_None) properties |= WindowsClangRefQualifier;
        if (prototype->hasCFIUncheckedCallee()) properties |= WindowsClangCFIUncheckedCallee;
    }
    *info = {
        extension.getHasRegParm(),
        extension.getRegParm(),
        extension.getNoReturn(),
        extension.getProducesResult(),
        extension.getNoCallerSavedRegs(),
        extension.getNoCfCheck(),
        extension.getCmseNSCall(),
        properties
    };
    return WindowsClangSuccess;
}
