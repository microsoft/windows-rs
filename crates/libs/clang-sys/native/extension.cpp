#include "extension.h"
#include <clang/AST/Attr.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Mangle.h>
#include <clang/AST/Type.h>
#include <clang/AST/TypeLoc.h>
#include <clang/AST/VTableBuilder.h>
#include <clang/Basic/TargetInfo.h>
#include <clang/AST/ASTConsumer.h>
#include <clang/AST/DeclContextInternals.h>
#include <clang/Driver/CreateInvocationFromArgs.h>
#include <clang/Frontend/ASTUnit.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendAction.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Lex/PreprocessorOptions.h>
#include <clang/Parse/Parser.h>
#include <clang/Sema/Sema.h>
#include <llvm/Config/llvm-config.h>
#include <llvm/Support/MemAlloc.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/VirtualFileSystem.h>
#include <mutex>
#include <string>
#include <vector>

extern "C" {
void LLVMInitializeX86TargetInfo();
void LLVMInitializeX86TargetMC();
void LLVMInitializeX86AsmParser();
void LLVMInitializeAArch64TargetInfo();
void LLVMInitializeAArch64TargetMC();
void LLVMInitializeAArch64AsmParser();
}

// Stock libclang has no accessors for the AST nodes behind its handles. These follow the handle
// layouts of the pinned libclang: cursors carry {parent or declaration, attribute or statement,
// unit}, and types carry {QualType, unit}.
namespace {

const clang::Decl* cursorDecl(CXCursor cursor) {
    return static_cast<const clang::Decl*>(cursor.data[0]);
}

const clang::Attr* cursorAttr(CXCursor cursor) {
    return static_cast<const clang::Attr*>(cursor.data[1]);
}

const clang::Expr* cursorExpr(CXCursor cursor) {
    if (!clang_isExpression(cursor.kind)) {
        return nullptr;
    }
    return llvm::dyn_cast_or_null<clang::Expr>(static_cast<const clang::Stmt*>(cursor.data[1]));
}

clang::ASTContext& cursorContext(CXCursor cursor) {
    return cursorDecl(cursor)->getASTContext();
}

clang::QualType typeValue(CXType type) {
    return clang::QualType::getFromOpaquePtr(type.data[0]);
}

CXTranslationUnit typeUnit(CXType type) {
    return static_cast<CXTranslationUnit>(type.data[1]);
}

clang::ASTContext& unitContext(CXTranslationUnit unit) {
    return cursorContext(clang_getTranslationUnitCursor(unit));
}

// libclang builds the handle, including its kind, for the pointee of a synthesized pointer.
CXType makeType(clang::ASTContext& context, clang::QualType type, CXTranslationUnit unit) {
    return clang_getPointeeType(
        CXType{ CXType_Pointer, { context.getPointerType(type).getAsOpaquePtr(), unit } });
}

// libclang's string flags: 0 is unmanaged and 1 is released with free. Both modules link the
// static UCRT, which allocates from the process heap.
CXString emptyString() {
    return { "", 0 };
}

CXString makeString(llvm::StringRef text) {
    auto data = static_cast<char*>(llvm::safe_malloc(text.size() + 1));
    memcpy(data, text.data(), text.size());
    data[text.size()] = 0;
    return { data, 1 };
}

// libclang builds expression cursors only while visiting; find the one for a child expression.
CXCursor expressionCursor(CXCursor parent, const clang::Expr* expression) {
    struct Search {
        const clang::Expr* expression;
        CXCursor found;
    } search{ expression, clang_getNullCursor() };
    clang_visitChildren(parent, [](CXCursor child, CXCursor, CXClientData data) {
        auto search = static_cast<Search*>(data);
        if (cursorExpr(child) == search->expression) {
            search->found = child;
            return CXChildVisit_Break;
        }
        return CXChildVisit_Recurse;
    }, &search);
    return search.found;
}

}

static const char* attributeName(clang::attr::Kind kind) {
    switch (kind) {
#define ATTR(Name) case clang::attr::Name: return #Name;
#include <clang/Basic/AttrList.inc>
    }
    return nullptr;
}

static WindowsClangStatus functionInfo(clang::QualType value, WindowsClangFunctionInfo* info) {
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

unsigned windows_clang_getABIVersion(void) {
    return 20;
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
        cursorDecl(cursor));
    if (!declaration) {
        return WindowsClangWrongKind;
    }
    *info = { WindowsClangConventionUnknown, {}, emptyString() };
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
    auto status = functionInfo(declaration->getType(), &original);
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
    info->stdcall_type = makeType(context, type, clang_Cursor_getTranslationUnit(cursor));
    info->stdcall_symbol = makeString(symbol);
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getTypeOfOperand(CXCursor cursor, CXCursor* expression) {
    if (!expression) {
        return WindowsClangInvalidArgument;
    }
    if (cursor.kind != CXCursor_VarDecl && cursor.kind != CXCursor_TypeAliasDecl &&
        cursor.kind != CXCursor_TypedefDecl) {
        return WindowsClangWrongKind;
    }
    auto declaration = cursorDecl(cursor);
    clang::QualType declared;
    if (auto variable = llvm::dyn_cast_or_null<clang::VarDecl>(declaration)) {
        declared = variable->getType();
    } else if (auto alias = llvm::dyn_cast_or_null<clang::TypedefNameDecl>(declaration)) {
        declared = alias->getUnderlyingType();
    }
    auto type = declared.isNull() ? nullptr : declared->getAs<clang::TypeOfExprType>();
    if (!type) {
        return WindowsClangWrongKind;
    }
    auto operand = type->getUnderlyingExpr();
    if (operand->isTypeDependent() || operand->isValueDependent()) {
        return WindowsClangDependent;
    }
    *expression = expressionCursor(cursor, operand);
    return clang_Cursor_isNull(*expression) ? WindowsClangWrongKind : WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getPredefinedType(CXType type, CXType* underlying) {
    if (!underlying) {
        return WindowsClangInvalidArgument;
    }
    auto value = typeValue(type);
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
    auto method = llvm::dyn_cast<clang::CXXMethodDecl>(cursorDecl(cursor));
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
    auto attribute = llvm::dyn_cast<clang::UuidAttr>(cursorAttr(cursor));
    if (!attribute) {
        return WindowsClangWrongKind;
    }
    *uuid = makeString(attribute->getGuid());
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
        cursorAttr(cursor));
    if (!attribute) {
        return WindowsClangWrongKind;
    }
    *message = makeString(attribute->getMessage());
    *replacement = makeString(attribute->getReplacement());
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
    auto declaration = cursorDecl(cursor);
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
    auto declaration = llvm::dyn_cast<clang::VarDecl>(cursorDecl(cursor));
    if (!declaration) {
        return WindowsClangWrongKind;
    }
    auto initializer = declaration->getInit();
    *is_constant = initializer &&
        initializer->isIntegerConstantExpr(cursorContext(cursor));
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
    auto declaration = llvm::dyn_cast<clang::VarDecl>(cursorDecl(cursor));
    if (!declaration) {
        return WindowsClangWrongKind;
    }
    auto initializer = declaration->getInit();
    auto& context = cursorContext(cursor);
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
    auto declaration = llvm::dyn_cast<clang::VarDecl>(cursorDecl(cursor));
    if (!declaration || !declaration->getType()->isPointerType() ||
        declaration->getType()->isFunctionPointerType() || !declaration->getInit()) {
        return WindowsClangWrongKind;
    }
    auto& context = cursorContext(cursor);
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
    auto declaration = llvm::dyn_cast<clang::VarDecl>(cursorDecl(cursor));
    if (!declaration || !declaration->getInit()) {
        return WindowsClangWrongKind;
    }
    auto& context = cursorContext(cursor);
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
    return attributeName(cursorAttr(cursor)->getKind());
}

const char* windows_clang_getTypeAttributeKindName(CXType type) {
    auto value = typeValue(type);
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
    auto value = typeValue(type);
    if (value.isNull()) {
        return WindowsClangWrongKind;
    }
    *qualifiers = qualifierFlags(value.getQualifiers());
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getPointerInfo(CXType type, WindowsClangPointerInfo* info) {
    if (!info) {
        return WindowsClangInvalidArgument;
    }
    auto value = typeValue(type);
    auto pointer = value.isNull() ? nullptr : value->getAs<clang::PointerType>();
    if (!pointer) {
        return WindowsClangWrongKind;
    }
    auto pointee = pointer->getPointeeType();
    auto representation = WindowsClangNativePointer;
    switch (pointee.getAddressSpace()) {
    case clang::LangAS::Default: break;
    case clang::LangAS::ptr64: representation = WindowsClangPointer64; break;
    case clang::LangAS::ptr32_sptr: representation = WindowsClangSignedPointer32; break;
    case clang::LangAS::ptr32_uptr: representation = WindowsClangUnsignedPointer32; break;
    default: representation = WindowsClangOtherPointer; break;
    }
    auto unit = typeUnit(type);
    auto& context = unitContext(unit);
    for (auto written = value;;) {
        if (auto attributed = llvm::dyn_cast<clang::AttributedType>(written.getTypePtr())) {
            auto kind = attributed->getAttrKind();
            if (kind == clang::attr::Ptr64 || kind == clang::attr::Ptr32) {
                if (representation == WindowsClangNativePointer) {
                    representation = kind == clang::attr::Ptr64
                        ? WindowsClangPointer64 : WindowsClangSignedPointer32;
                }
                break;
            }
            written = attributed->getModifiedType();
            continue;
        }
        auto next = written.getSingleStepDesugaredType(context);
        if (next == written) {
            break;
        }
        written = next;
    }
    if (clang::isPtrSizeAddressSpace(pointee.getAddressSpace())) {
        pointee = context.removeAddrSpaceQualType(pointee);
    }
    *info = { representation, makeType(context, pointee, unit) };
    return WindowsClangSuccess;
}

WindowsClangStatus windows_clang_getMethodQualifiers(CXType type, unsigned* qualifiers) {
    if (!qualifiers) {
        return WindowsClangInvalidArgument;
    }
    auto value = typeValue(type);
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
        cursorDecl(cursor));
    if (!parameter) {
        return WindowsClangWrongKind;
    }
    if (parameter->hasUnparsedDefaultArg() || parameter->hasUninstantiatedDefaultArg()) {
        return WindowsClangDependent;
    }
    if (!parameter->hasDefaultArg()) {
        *expression = clang_getNullCursor();
        *spelling = emptyString();
        return WindowsClangSuccess;
    }
    const auto* argument = parameter->getDefaultArg();
    if (argument->isValueDependent() || argument->isTypeDependent()) {
        return WindowsClangDependent;
    }
    *expression = expressionCursor(cursor, argument);
    if (clang_Cursor_isNull(*expression)) {
        return WindowsClangWrongKind;
    }
    std::string text;
    llvm::raw_string_ostream stream(text);
    argument->printPretty(stream, nullptr,
        cursorContext(cursor).getPrintingPolicy());
    *spelling = makeString(text);
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
        cursorAttr(cursor));
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
    auto attribute = llvm::dyn_cast<clang::AlignedAttr>(cursorAttr(cursor));
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
    *info = {attribute->getAlignment(cursorContext(cursor)), kind};
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
        cursorAttr(cursor));
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
        cursorDecl(cursor));
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
        cursorDecl(cursor));
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
    const auto* declaration = cursorDecl(cursor);
    const auto* expr = cursorExpr(expression);
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
        cursorDecl(cursor));
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
    return functionInfo(typeValue(type), info);
}

namespace {

struct MacroDiagnostics : clang::DiagnosticConsumer {
    std::string text;

    void HandleDiagnostic(clang::DiagnosticsEngine::Level level,
        const clang::Diagnostic& info) override {
        DiagnosticConsumer::HandleDiagnostic(level, info);
        if (level < clang::DiagnosticsEngine::Note) {
            return;
        }
        if (!text.empty()) {
            text += '\n';
        }
        if (info.hasSourceManager() && info.getLocation().isValid()) {
            auto presumed = info.getSourceManager().getPresumedLoc(info.getLocation());
            if (presumed.isValid()) {
                text += presumed.getFilename();
                text += ':' + std::to_string(presumed.getLine()) + ':'
                    + std::to_string(presumed.getColumn()) + ": ";
            }
        }
        switch (level) {
        case clang::DiagnosticsEngine::Note: text += "note: "; break;
        case clang::DiagnosticsEngine::Warning: text += "warning: "; break;
        default: text += "error: "; break;
        }
        llvm::SmallString<256> message;
        info.FormatDiagnostic(message);
        text += message.str();
    }

    void reset() {
        text.clear();
        clear();
    }
};

class MacroProbeAction : public clang::ASTFrontendAction {
public:
    MacroProbeAction(MacroDiagnostics& diagnostics, const char* const* names, unsigned count,
        WindowsClangMacroProbe* probes)
        : diagnostics(diagnostics), names(names), count(count), probes(probes) {}

    bool failed = false;

protected:
    std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
        clang::CompilerInstance&, llvm::StringRef) override {
        return std::make_unique<clang::ASTConsumer>();
    }

    clang::TranslationUnitKind getTranslationUnitKind() override {
        return clang::TU_Incremental;
    }

    void ExecuteAction() override {
        auto& compiler = getCompilerInstance();
        if (!compiler.hasSema()) {
            compiler.createSema(getTranslationUnitKind(), nullptr);
        }
        auto& sema = compiler.getSema();
        auto& preprocessor = compiler.getPreprocessor();
        auto& sources = compiler.getSourceManager();
        clang::Parser parser(preprocessor, sema, false);
        preprocessor.EnterMainSourceFile();
        parser.Initialize();
        sema.getASTConsumer().Initialize(sema.getASTContext());
        clang::Parser::DeclGroupPtrTy group;
        clang::Sema::ModuleImportState state;
        for (bool end = parser.ParseFirstTopLevelDecl(group, state); !end;
             end = parser.ParseTopLevelDecl(group, state)) {
        }
        if (diagnostics.getNumErrors()) {
            failed = true;
            return;
        }
        diagnostics.reset();
        unsigned serial = 0;
        auto& context = compiler.getASTContext();
        // Each probe is a separate incremental input, so error recovery cannot reach the next.
        // This follows clang-repl's IncrementalParser: each input gets its own partial TU, and a
        // failed input's declarations are removed from lookup before the next input.
        auto parse = [&](const std::string& text, std::vector<clang::Decl*>& declarations) {
            auto location = sources.getLocForStartOfFile(sources.getMainFileID());
            auto file = sources.createFileID(
                llvm::MemoryBuffer::getMemBufferCopy(text, "__windows_clang_probe"),
                clang::SrcMgr::C_User, 0, 0, location);
            if (preprocessor.EnterSourceFile(file, nullptr, location)) {
                return false;
            }
            context.addTranslationUnitDecl();
            if (parser.getCurToken().is(clang::tok::annot_repl_input_end)) {
                parser.ConsumeAnyToken();
                parser.ExitScope();
                sema.CurContext = nullptr;
                parser.EnterScope(clang::Scope::DeclScope);
                sema.ActOnTranslationUnitScope(parser.getCurScope());
            }
            clang::Parser::DeclGroupPtrTy group;
            auto state = clang::Sema::ModuleImportState::NotACXX20Module;
            bool end;
            do {
                end = parser.ParseTopLevelDecl(group, state);
                if (group) {
                    for (auto declaration : group.get()) {
                        declarations.push_back(declaration);
                    }
                }
            } while (!end);
            bool valid = !diagnostics.getNumErrors();
            for (auto declaration : declarations) {
                valid &= !declaration->isInvalidDecl();
            }
            if (!valid) {
                discard(context.getTranslationUnitDecl());
            }
            // Delayed template parsing can leave tokens that would confuse the next input.
            if (preprocessor.getLangOpts().DelayedTemplateParsing) {
                for (clang::Token token; !token.is(clang::tok::annot_repl_input_end);) {
                    preprocessor.Lex(token);
                }
            }
            return valid;
        };
        for (unsigned index = 0; index != count; ++index) {
            auto& probe = probes[index];
            auto identifier = preprocessor.getIdentifierInfo(names[index]);
            auto macro = preprocessor.getMacroInfo(identifier);
            if (!macro) {
                continue;
            }
            auto location = sources.getExpansionLoc(macro->getDefinitionLoc());
            clang_disposeString(probe.file);
            probe.file = makeString(sources.getFilename(location));
            probe.line = sources.getExpansionLineNumber(location);
            probe.column = sources.getExpansionColumnNumber(location);
            if (macro->isFunctionLike()) {
                probe.kind = WindowsClangMacroFunctionLike;
                continue;
            }
            if (!macro->getNumTokens()) {
                probe.kind = WindowsClangMacroEmpty;
                continue;
            }
            // Incremental inputs must be self-contained: an unclosed bracket would carry parser
            // state into later probes. Unbalanced replacement lists are never expressions.
            if (!balanced(*macro)) {
                probe.kind = WindowsClangMacroRejected;
                clang_disposeString(probe.diagnostics);
                probe.diagnostics = makeString(std::string(probe_location(location))
                    + "error: macro replacement list has unbalanced brackets");
                continue;
            }
            // These forms match the declarations windows-clang generates for accepted macros.
            std::string name = names[index];
            std::vector<clang::Decl*> declarations;
            bool function = false;
            bool valid = parse("\nusing __windows_clang_probe_type_" + std::to_string(serial++)
                    + " = __typeof__((" + name + "));\n",
                declarations);
            if (valid) {
                for (auto declaration : declarations) {
                    auto alias = llvm::dyn_cast<clang::TypeAliasDecl>(declaration);
                    auto type = alias ? llvm::dyn_cast<clang::TypeOfExprType>(
                        alias->getUnderlyingType().getTypePtr()) : nullptr;
                    if (type) {
                        function = type->getUnderlyingExpr()->getType()->isFunctionType();
                    }
                }
                declarations.clear();
                auto id = std::to_string(serial++);
                // Parentheses would turn initializer fragments into comma expressions.
                valid = parse(function
                        ? "\nstatic __typeof__((" + name + ")) &__windows_clang_probe_" + id
                            + " = " + name + ";\n"
                        : "\nstatic const __typeof__((" + name + ")) __windows_clang_probe_" + id
                            + " = " + name + ";\n",
                    declarations);
            }
            if (valid) {
                probe.kind = function ? WindowsClangMacroFunction : WindowsClangMacroValue;
            } else {
                probe.kind = WindowsClangMacroRejected;
                clang_disposeString(probe.diagnostics);
                probe.diagnostics = makeString(diagnostics.text);
                compiler.getDiagnostics().Reset(true);
            }
            diagnostics.reset();
        }
    }

private:
    static bool balanced(const clang::MacroInfo& macro) {
        std::vector<clang::tok::TokenKind> open;
        for (auto& token : macro.tokens()) {
            switch (token.getKind()) {
            case clang::tok::l_paren: open.push_back(clang::tok::r_paren); break;
            case clang::tok::l_square: open.push_back(clang::tok::r_square); break;
            case clang::tok::l_brace: open.push_back(clang::tok::r_brace); break;
            case clang::tok::r_paren:
            case clang::tok::r_square:
            case clang::tok::r_brace:
                if (open.empty() || open.back() != token.getKind()) {
                    return false;
                }
                open.pop_back();
                break;
            default: break;
            }
        }
        return open.empty();
    }

    std::string probe_location(clang::SourceLocation location) {
        auto presumed = getCompilerInstance().getSourceManager().getPresumedLoc(location);
        if (presumed.isInvalid()) {
            return {};
        }
        return std::string(presumed.getFilename()) + ':' + std::to_string(presumed.getLine())
            + ':' + std::to_string(presumed.getColumn()) + ": ";
    }

    // Removes a failed partial TU's declarations from lookup, as clang-repl's CleanUpPTU does.
    void discard(clang::TranslationUnitDecl* unit) {
        if (auto map = unit->getPrimaryContext()->getLookupPtr()) {
            for (auto&& [key, list] : *map) {
                std::vector<clang::NamedDecl*> removed;
                bool all = true;
                for (auto declaration : list.getLookupResult()) {
                    if (declaration->getTranslationUnitDecl() == unit) {
                        removed.push_back(declaration);
                    } else {
                        all = false;
                    }
                }
                if (all) {
                    map->erase(key);
                } else {
                    for (auto declaration : removed) {
                        list.remove(declaration);
                    }
                }
            }
        }
        auto& sema = getCompilerInstance().getSema();
        for (auto declaration : unit->decls()) {
            auto named = llvm::dyn_cast<clang::NamedDecl>(declaration);
            if (named && !named->getDeclName().isEmpty()
                && named->getDeclName().getFETokenInfo() && !declaration->getLangOpts().ObjC
                && !declaration->getLangOpts().CPlusPlus) {
                sema.IdResolver.RemoveDecl(named);
            }
        }
    }

    MacroDiagnostics& diagnostics;
    const char* const* names;
    unsigned count;
    WindowsClangMacroProbe* probes;
};

}

WindowsClangStatus windows_clang_probeMacros(const char* source_filename,
    const char* const* command_line_args, int num_command_line_args,
    CXUnsavedFile* unsaved_files, unsigned num_unsaved_files, const char* const* names,
    unsigned num_names, WindowsClangMacroProbe* probes, CXString* error) {
    if (!source_filename || !error || num_command_line_args < 0
        || (num_command_line_args && !command_line_args) || (num_unsaved_files && !unsaved_files)
        || (num_names && (!names || !probes))) {
        return WindowsClangInvalidArgument;
    }
    *error = emptyString();
    for (unsigned index = 0; index != num_names; ++index) {
        if (!names[index]) {
            return WindowsClangInvalidArgument;
        }
        probes[index] = { WindowsClangMacroUndefined, emptyString(), 0, 0,
            emptyString() };
    }
    static std::once_flag targets;
    std::call_once(targets, [] {
        LLVMInitializeX86TargetInfo();
        LLVMInitializeX86TargetMC();
        LLVMInitializeX86AsmParser();
        LLVMInitializeAArch64TargetInfo();
        LLVMInitializeAArch64TargetMC();
        LLVMInitializeAArch64AsmParser();
    });
    // Match the arguments libclang adds when it parses a translation unit.
    std::vector<const char*> arguments{ "clang" };
    bool spelling = false;
    for (int index = 0; index != num_command_line_args; ++index) {
        llvm::StringRef argument = command_line_args[index];
        spelling |= argument == "-fno-spell-checking" || argument == "-fspell-checking";
        arguments.push_back(command_line_args[index]);
    }
    if (!spelling) {
        arguments.insert(arguments.begin() + 1, "-fno-spell-checking");
    }
    arguments.insert(arguments.end(),
        { "-fsyntax-only", "-ferror-limit=0", "-fallow-editor-placeholders", source_filename });
    MacroDiagnostics diagnostics;
    clang::DiagnosticOptions options;
    auto system = llvm::vfs::getRealFileSystem();
    clang::CreateInvocationOptions creation;
    creation.Diags = clang::CompilerInstance::createDiagnostics(
        *system, options, &diagnostics, false);
    creation.VFS = system;
    auto invocation = clang::createInvocation(arguments, creation);
    if (!invocation) {
        clang_disposeString(*error);
        *error = makeString(diagnostics.text);
        return WindowsClangParseFailed;
    }
    invocation->getLangOpts().IncrementalExtensions = 1;
    auto& preprocessor = invocation->getPreprocessorOpts();
    for (unsigned index = 0; index != num_unsaved_files; ++index) {
        auto& file = unsaved_files[index];
        preprocessor.addRemappedFile(file.Filename,
            llvm::MemoryBuffer::getMemBufferCopy(
                llvm::StringRef(file.Contents, file.Length), file.Filename).release());
    }
    preprocessor.RetainRemappedFileBuffers = false;
    clang::CompilerInstance compiler(std::move(invocation));
    compiler.createVirtualFileSystem(system);
    compiler.createDiagnostics(&diagnostics, false);
    MacroProbeAction action(diagnostics, names, num_names, probes);
    if (!compiler.ExecuteAction(action) || action.failed) {
        clang_disposeString(*error);
        *error = makeString(diagnostics.text);
        return WindowsClangParseFailed;
    }
    return WindowsClangSuccess;
}