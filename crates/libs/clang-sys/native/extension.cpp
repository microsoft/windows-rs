#include "extension.h"
#include "CXCursor.h"
#include "CXString.h"
#include "CXType.h"
#include <clang/AST/Attr.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Mangle.h>
#include <clang/AST/Type.h>
#include <clang/AST/TypeLoc.h>
#include <clang/AST/VTableBuilder.h>
#include <clang/Basic/TargetInfo.h>
#include <llvm/Config/llvm-config.h>

static const char* attributeName(clang::attr::Kind kind) {
    switch (kind) {
#define ATTR(Name) case clang::attr::Name: return #Name;
#include <clang/Basic/AttrList.inc>
    }
    return nullptr;
}

unsigned windows_clang_getABIVersion(void) {
    return 17;
}

WindowsClangStatus windows_clang_getImportConvention(
    CXCursor cursor, WindowsClangImportConvention* info) {
    if (!info) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_FunctionDecl) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast_or_null<clang::FunctionDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!declaration) {
        return WindowsClangWrongKind;
    }
    *info = { WindowsClangConventionUnknown, {}, clang::cxstring::createEmpty() };
    auto source = declaration->getTypeSourceInfo();
    if (!source) {
        return WindowsClangSuccess;
    }
    auto location = source->getTypeLoc();
    while (!location.isNull()) {
        if (auto attributed = location.getAs<clang::AttributedTypeLoc>();
            !attributed.isNull() && attributed.getTypePtr()->isCallingConv()) {
            info->origin = WindowsClangConventionExplicit;
            return WindowsClangSuccess;
        }
        if (!location.getAs<clang::FunctionTypeLoc>().isNull()) {
            info->origin = WindowsClangConventionDefault;
            break;
        }
        if (auto alias = location.getAs<clang::TypedefTypeLoc>(); !alias.isNull()) {
            auto underlying = alias.getTypePtr()->getDecl()->getTypeSourceInfo();
            if (!underlying) {
                return WindowsClangSuccess;
            }
            location = underlying->getTypeLoc();
        } else {
            location = location.getNextTypeLoc();
        }
    }
    auto& context = declaration->getASTContext();
    auto prototype = declaration->getType()->getAs<clang::FunctionProtoType>();
    if (info->origin != WindowsClangConventionDefault || !prototype ||
        !context.getTargetInfo().getTriple().isWindowsMSVCEnvironment() ||
        context.getTargetInfo().getTriple().getArch() != llvm::Triple::x86 ||
        prototype->getCallConv() != clang::CC_C || prototype->isVariadic() ||
        prototype->hasExtParameterInfos() ||
        !declaration->isExternC() || llvm::isa<clang::CXXMethodDecl>(declaration) ||
        declaration->doesThisDeclarationHaveABody() || declaration->hasAttr<clang::AsmLabelAttr>()) {
        return WindowsClangSuccess;
    }
    WindowsClangFunctionInfo original{};
    auto status = windows_clang_getFunctionInfo(
        clang::cxtype::MakeCXType(declaration->getType(), clang::cxcursor::getCursorTU(cursor)),
        &original);
    if (status != WindowsClangSuccess) {
        return status;
    }
    if (original.has_regparm || original.produces_result || original.no_caller_saved_registers ||
        original.no_cf_check || original.cmse_nonsecure_call || original.additional_properties) {
        return WindowsClangSuccess;
    }

    // Change only the outer convention. All parameter/result types and prototype properties
    // remain compiler-owned; do not reparse the header under a different global default.
    auto properties = prototype->getExtProtoInfo();
    properties.ExtInfo = properties.ExtInfo.withCallingConv(clang::CC_X86StdCall);
    auto type = context.getFunctionType(
        prototype->getReturnType(), prototype->getParamTypes(), properties);
    auto scope = clang::LinkageSpecDecl::Create(
        context, context.getTranslationUnitDecl(), {}, {}, clang::LinkageSpecLanguageIDs::C, false);
    auto candidate = clang::FunctionDecl::Create(
        context, scope, declaration->getBeginLoc(),
        declaration->getLocation(), declaration->getDeclName(), type, nullptr,
        declaration->getStorageClass());
    llvm::SmallVector<clang::ParmVarDecl*, 8> parameters;
    for (auto parameter : declaration->parameters()) {
        parameters.push_back(clang::ParmVarDecl::Create(
            context, candidate, parameter->getBeginLoc(), parameter->getLocation(),
            parameter->getIdentifier(), parameter->getType(), parameter->getTypeSourceInfo(),
            parameter->getStorageClass(), nullptr));
    }
    candidate->setParams(parameters);
    auto symbol = clang::ASTNameGenerator(context).getName(candidate);
    if (symbol.empty()) {
        return WindowsClangWrongKind;
    }
    info->stdcall_type = clang::cxtype::MakeCXType(type, clang::cxcursor::getCursorTU(cursor));
    info->stdcall_symbol = clang::cxstring::createDup(symbol);
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getTypeOfOperand(CXCursor cursor, CXCursor* expression) {
    if (!expression) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_VarDecl) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast<clang::VarDecl>(clang::cxcursor::getCursorDecl(cursor));
    auto type = declaration ? declaration->getType()->getAs<clang::TypeOfExprType>() : nullptr;
    if (!type) {
        return WindowsClangWrongKind;
    }
    auto operand = type->getUnderlyingExpr();
    if (operand->isTypeDependent() || operand->isValueDependent()) {
        return WindowsClangDependent;
    }
    *expression = clang::cxcursor::MakeCXCursor(
        operand, declaration, clang::cxcursor::getCursorTU(cursor));
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getPredefinedType(CXType type, CXType* underlying) {
    if (!underlying) {
        return WindowsClangInvalidArgument;
    }
    auto value = GetQualType(type);
    if (value.isNull() || !llvm::isa<clang::PredefinedSugarType>(value.getTypePtr())) {
        return WindowsClangWrongKind;
    }
    *underlying = clang_getCanonicalType(type);
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getVirtualMethodSlot(
    CXCursor cursor, unsigned long long* slot) {
    if (!slot) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_CXXMethod) {
        return WindowsClangWrongKind;
    }
    auto method = llvm::dyn_cast<clang::CXXMethodDecl>(clang::cxcursor::getCursorDecl(cursor));
    if (!method || !method->isVirtual() || !clang::VTableContextBase::hasVtableSlot(method)) {
        return WindowsClangWrongKind;
    }
    if (method->isDependentContext() || !method->getParent()->isCompleteDefinition()) {
        return WindowsClangDependent;
    }
    auto context = llvm::dyn_cast<clang::MicrosoftVTableContext>(
        method->getASTContext().getVTableContext());
    if (!context) {
        return WindowsClangWrongKind;
    }
    auto location = context->getMethodVFTableLocation(clang::GlobalDecl(method));
    if (location.VBTableIndex || location.VBase || !location.VFPtrOffset.isZero()) {
        return WindowsClangWrongKind;
    }
    *slot = location.Index;
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getUuid(CXCursor cursor, CXString* uuid) {
    if (!uuid) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isAttribute(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto attribute = llvm::dyn_cast<clang::UuidAttr>(clang::cxcursor::getCursorAttr(cursor));
    if (!attribute) {
        return WindowsClangWrongKind;
    }
    *uuid = clang::cxstring::createDup(attribute->getGuid());
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getDeprecation(
    CXCursor cursor, CXString* message, CXString* replacement) {
    if (!message || !replacement) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isAttribute(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto attribute = llvm::dyn_cast<clang::DeprecatedAttr>(
        clang::cxcursor::getCursorAttr(cursor));
    if (!attribute) {
        return WindowsClangWrongKind;
    }
    *message = clang::cxstring::createDup(attribute->getMessage());
    *replacement = clang::cxstring::createDup(attribute->getReplacement());
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getLanguageLinkage(
    CXCursor cursor, WindowsClangLanguageLinkage* linkage) {
    if (!linkage) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isDeclaration(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto declaration = clang::cxcursor::getCursorDecl(cursor);
    clang::LanguageLinkage value;
    if (auto function = llvm::dyn_cast<clang::FunctionDecl>(declaration)) {
        value = function->getLanguageLinkage();
    } else if (auto variable = llvm::dyn_cast<clang::VarDecl>(declaration)) {
        value = variable->getLanguageLinkage();
    } else {
        return WindowsClangWrongKind;
    }
    switch (value) {
    case clang::NoLanguageLinkage: *linkage = WindowsClangNoLanguageLinkage; break;
    case clang::CLanguageLinkage: *linkage = WindowsClangCLanguageLinkage; break;
    case clang::CXXLanguageLinkage: *linkage = WindowsClangCXXLanguageLinkage; break;
    }
    return WindowsClangSuccess;
}

const char* windows_clang_getLLVMVersion(void) {
    return LLVM_VERSION_STRING;
}

WindowsClangStatus windows_clang_getIntegerConstantInitializer(
    CXCursor cursor, unsigned* is_constant) {
    if (!is_constant) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isDeclaration(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast<clang::VarDecl>(clang::cxcursor::getCursorDecl(cursor));
    if (!declaration) {
        return WindowsClangWrongKind;
    }
    auto initializer = declaration->getInit();
    *is_constant = initializer &&
        initializer->isIntegerConstantExpr(clang::cxcursor::getCursorContext(cursor));
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getConstantInitializer(
    CXCursor cursor, unsigned* is_constant) {
    if (!is_constant) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isDeclaration(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast<clang::VarDecl>(clang::cxcursor::getCursorDecl(cursor));
    if (!declaration) {
        return WindowsClangWrongKind;
    }
    auto initializer = declaration->getInit();
    auto& context = clang::cxcursor::getCursorContext(cursor);
    *is_constant = 0;
    if (initializer && initializer->isConstantInitializer(context, false)) {
        clang::Expr::EvalResult evaluated;
        *is_constant = initializer->EvaluateAsRValue(evaluated, context) &&
            !evaluated.HasSideEffects && !evaluated.HasUndefinedBehavior;
    }
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getConstantPointerInitializer(
    CXCursor cursor, long long* value) {
    if (!value) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_VarDecl) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast<clang::VarDecl>(clang::cxcursor::getCursorDecl(cursor));
    if (!declaration || !declaration->getType()->isPointerType() ||
        declaration->getType()->isFunctionPointerType() || !declaration->getInit()) {
        return WindowsClangWrongKind;
    }
    auto& context = clang::cxcursor::getCursorContext(cursor);
    auto initializer = declaration->getInit();
    if (!initializer->isConstantInitializer(context, false)) {
        return WindowsClangDependent;
    }
    clang::Expr::EvalResult evaluated;
    if (!initializer->EvaluateAsRValue(evaluated, context) ||
        evaluated.HasSideEffects || evaluated.HasUndefinedBehavior) {
        return WindowsClangDependent;
    }
    if (!evaluated.Val.isLValue() || evaluated.Val.getLValueBase()) {
        return WindowsClangWrongKind;
    }
    auto width = context.getTypeSize(declaration->getType());
    if (!width || width > 64) {
        return WindowsClangWrongKind;
    }
    llvm::APInt bits(static_cast<unsigned>(width),
        static_cast<uint64_t>(evaluated.Val.getLValueOffset().getQuantity()));
    *value = bits.getSExtValue();
    return WindowsClangSuccess;
}

const void* windows_clang_getRuntimeIdentity(void) {
    return reinterpret_cast<const void*>(&clang_getClangVersion);
}

WindowsClangStatus windows_clang_getConstantIntegerElement(
    CXCursor cursor, const unsigned* indices, unsigned count, unsigned long long* value) {
    if (!value || (count && !indices)) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isDeclaration(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast<clang::VarDecl>(clang::cxcursor::getCursorDecl(cursor));
    if (!declaration || !declaration->getInit()) {
        return WindowsClangWrongKind;
    }
    auto& context = clang::cxcursor::getCursorContext(cursor);
    auto initializer = declaration->getInit();
    if (!initializer->isCXX11ConstantExpr(context)) {
        return WindowsClangDependent;
    }
    clang::Expr::EvalResult evaluated;
    if (!initializer->EvaluateAsRValue(evaluated, context)) {
        return WindowsClangWrongKind;
    }
    const clang::APValue* element = &evaluated.Val;
    for (unsigned index = 0; index < count; ++index) {
        if (element->isStruct() && element->getStructNumBases() == 0 &&
            indices[index] < element->getStructNumFields()) {
            element = &element->getStructField(indices[index]);
        } else if (element->isArray() && indices[index] < element->getArraySize()) {
            element = indices[index] < element->getArrayInitializedElts()
                ? &element->getArrayInitializedElt(indices[index])
                : &element->getArrayFiller();
        } else {
            return WindowsClangWrongKind;
        }
    }
    if (!element->isInt() || element->getInt().isNegative() ||
        element->getInt().getActiveBits() > 64) {
        return WindowsClangWrongKind;
    }
    *value = element->getInt().getZExtValue();
    return WindowsClangSuccess;
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

static unsigned qualifierFlags(clang::Qualifiers native) {
    unsigned flags = 0;
    if (native.hasConst()) flags |= WindowsClangConst;
    if (native.hasVolatile()) flags |= WindowsClangVolatile;
    if (native.hasRestrict()) flags |= WindowsClangRestrict;
    if (native.hasUnaligned()) flags |= WindowsClangUnaligned;
    native.removeCVRQualifiers();
    native.removeUnaligned();
    if (!native.empty()) flags |= WindowsClangOtherQualifiers;
    return flags;
}

WindowsClangStatus windows_clang_getTypeQualifiers(CXType type, unsigned* qualifiers) {
    if (!qualifiers) {
        return WindowsClangInvalidArgument;
    }
    auto value = GetQualType(type);
    if (value.isNull()) {
        return WindowsClangWrongKind;
    }
    *qualifiers = qualifierFlags(value.getQualifiers());
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getMethodQualifiers(CXType type, unsigned* qualifiers) {
    if (!qualifiers) {
        return WindowsClangInvalidArgument;
    }
    auto value = GetQualType(type);
    auto prototype = value.isNull() ? nullptr : value->getAs<clang::FunctionProtoType>();
    if (!prototype) {
        return WindowsClangWrongKind;
    }
    *qualifiers = qualifierFlags(prototype->getMethodQuals());
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getParameterDefaultArgument(
    CXCursor cursor, CXCursor* expression, CXString* spelling) {
    if (!expression || !spelling) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_ParmDecl) {
        return WindowsClangWrongKind;
    }
    const auto* parameter = llvm::dyn_cast<clang::ParmVarDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!parameter) {
        return WindowsClangWrongKind;
    }
    if (parameter->hasUnparsedDefaultArg() || parameter->hasUninstantiatedDefaultArg()) {
        return WindowsClangDependent;
    }
    if (!parameter->hasDefaultArg()) {
        *expression = clang_getNullCursor();
        *spelling = clang::cxstring::createEmpty();
        return WindowsClangSuccess;
    }
    const auto* argument = parameter->getDefaultArg();
    if (argument->isValueDependent() || argument->isTypeDependent()) {
        return WindowsClangDependent;
    }
    *expression = clang::cxcursor::MakeCXCursor(
        argument, parameter, clang::cxcursor::getCursorTU(cursor));
    std::string text;
    llvm::raw_string_ostream stream(text);
    argument->printPretty(stream, nullptr,
        clang::cxcursor::getCursorContext(cursor).getPrintingPolicy());
    *spelling = clang::cxstring::createDup(text);
    return WindowsClangSuccess;
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

WindowsClangStatus windows_clang_getAlignment(CXCursor cursor, WindowsClangAlignmentInfo* info) {
    if (!info) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isAttribute(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto attribute = llvm::dyn_cast<clang::AlignedAttr>(clang::cxcursor::getCursorAttr(cursor));
    if (!attribute) {
        return WindowsClangWrongKind;
    }
    if (attribute->isAlignmentDependent() || attribute->isAlignmentErrorDependent()) {
        return WindowsClangDependent;
    }
    WindowsClangAlignmentKind kind;
    if (attribute->isGNU()) {
        kind = WindowsClangAlignmentGNU;
    } else if (attribute->isAlignas()) {
        kind = WindowsClangAlignmentStandard;
    } else if (attribute->isDeclspec()) {
        kind = WindowsClangAlignmentDeclspec;
    } else {
        return WindowsClangWrongKind;
    }
    *info = {attribute->getAlignment(clang::cxcursor::getCursorContext(cursor)), kind};
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getTypeVisibility(CXCursor cursor, CXVisibilityKind* visibility) {
    if (!visibility) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isAttribute(cursor.kind)) {
        return WindowsClangWrongKind;
    }
    auto attribute = llvm::dyn_cast<clang::TypeVisibilityAttr>(
        clang::cxcursor::getCursorAttr(cursor));
    if (!attribute) {
        return WindowsClangWrongKind;
    }
    CXVisibilityKind value;
    switch (attribute->getVisibility()) {
    case clang::TypeVisibilityAttr::Default:
        value = CXVisibility_Default;
        break;
    case clang::TypeVisibilityAttr::Hidden:
        value = CXVisibility_Hidden;
        break;
    case clang::TypeVisibilityAttr::Protected:
        value = CXVisibility_Protected;
        break;
    default:
        return WindowsClangWrongKind;
    }
    *visibility = value;
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getTypedefDefinition(CXCursor cursor, unsigned* defines_tag) {
    if (!defines_tag) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_TypedefDecl && cursor.kind != CXCursor_TypeAliasDecl) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast<clang::TypedefNameDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!declaration || !declaration->getTypeSourceInfo()) {
        return WindowsClangWrongKind;
    }
    auto tag = declaration->getTypeSourceInfo()->getTypeLoc()
        .getUnqualifiedLoc().getAs<clang::TagTypeLoc>();
    *defines_tag = !tag.isNull() && tag.isDefinition();
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getTypedefImplicit(CXCursor cursor, unsigned* is_implicit) {
    if (!is_implicit) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_TypedefDecl && cursor.kind != CXCursor_TypeAliasDecl) {
        return WindowsClangWrongKind;
    }
    auto declaration = llvm::dyn_cast<clang::TypedefNameDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!declaration) {
        return WindowsClangWrongKind;
    }
    *is_implicit = declaration->isImplicit();
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getLayoutExpression(
    CXCursor cursor, CXCursor expression, unsigned* is_layout_expression) {
    if (!is_layout_expression) {
        return WindowsClangInvalidArgument;
    }
    if (!clang_isExpression(expression.kind) ||
        (cursor.kind != CXCursor_FieldDecl && cursor.kind != CXCursor_ParmDecl &&
         cursor.kind != CXCursor_TypedefDecl && cursor.kind != CXCursor_TypeAliasDecl)) {
        return WindowsClangWrongKind;
    }
    const auto* declaration = clang::cxcursor::getCursorDecl(cursor);
    const auto* expr = clang::cxcursor::getCursorExpr(expression);
    const clang::TypeSourceInfo* source = nullptr;
    if (const auto* value = llvm::dyn_cast<clang::DeclaratorDecl>(declaration)) {
        source = value->getTypeSourceInfo();
    } else if (const auto* alias = llvm::dyn_cast<clang::TypedefNameDecl>(declaration)) {
        source = alias->getTypeSourceInfo();
    }
    if (!source || !expr) {
        return WindowsClangWrongKind;
    }
    auto matches = [expr](const clang::Expr* value) {
        return value && value->IgnoreImplicit() == expr->IgnoreImplicit();
    };
    if (const auto* field = llvm::dyn_cast<clang::FieldDecl>(declaration)) {
        if (field->isBitField() && matches(field->getBitWidth())) {
            *is_layout_expression = 1;
            return WindowsClangSuccess;
        }
    }
    for (auto location = source->getTypeLoc(); !location.isNull();
         location = location.getNextTypeLoc()) {
        auto array = location.getAs<clang::ArrayTypeLoc>();
        if (!array.isNull() && matches(array.getSizeExpr()) &&
            llvm::isa<clang::ConstantArrayType>(array.getTypePtr()) &&
            array.getTypePtr()->getSizeModifier() == clang::ArraySizeModifier::Normal &&
            array.getTypePtr()->getIndexTypeQualifiers().empty()) {
            *is_layout_expression = 1;
            return WindowsClangSuccess;
        }
    }
    *is_layout_expression = 0;
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getFieldInitializer(CXCursor cursor, unsigned* has_initializer) {
    if (!has_initializer) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_FieldDecl) {
        return WindowsClangWrongKind;
    }
    const auto* field = llvm::dyn_cast<clang::FieldDecl>(
        clang::cxcursor::getCursorDecl(cursor));
    if (!field) {
        return WindowsClangWrongKind;
    }
    *has_initializer = field->hasInClassInitializer();
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
