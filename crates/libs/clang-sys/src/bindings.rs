windows_link::link!("libclang.dll" "C" fn clang_CXCursorSet_contains(cset : CXCursorSet, cursor : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXCursorSet_insert(cset : CXCursorSet, cursor : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXIndex_getGlobalOptions(param0 : CXIndex) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXIndex_setGlobalOptions(param0 : CXIndex, options : u32));
windows_link::link!("libclang.dll" "C" fn clang_CXIndex_setInvocationEmissionPathOption(param0 : CXIndex, path : *const i8));
windows_link::link!("libclang.dll" "C" fn clang_CXXConstructor_isConvertingConstructor(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXConstructor_isCopyConstructor(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXConstructor_isDefaultConstructor(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXConstructor_isMoveConstructor(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXField_isMutable(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isConst(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isCopyAssignmentOperator(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isDefaulted(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isDeleted(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isExplicit(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isMoveAssignmentOperator(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isPureVirtual(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isStatic(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXMethod_isVirtual(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_CXXRecord_isAbstract(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_Evaluate(c : CXCursor) -> CXEvalResult);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getArgument(c : CXCursor, i : u32) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getBinaryOpcode(c : CXCursor) -> CX_BinaryOperatorKind);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getBinaryOpcodeStr(op : CX_BinaryOperatorKind) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getBriefCommentText(c : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getCXXManglings(param0 : CXCursor) -> *mut CXStringSet);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getCommentRange(c : CXCursor) -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getGCCAssemblyClobber(cursor : CXCursor, index : u32) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getGCCAssemblyInput(cursor : CXCursor, index : u32, constraint : *mut CXString, expr : *mut CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getGCCAssemblyNumClobbers(cursor : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getGCCAssemblyNumInputs(param0 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getGCCAssemblyNumOutputs(param0 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getGCCAssemblyOutput(cursor : CXCursor, index : u32, constraint : *mut CXString, expr : *mut CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getGCCAssemblyTemplate(param0 : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getMangling(param0 : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getModule(c : CXCursor) -> CXModule);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getNumArguments(c : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getNumTemplateArguments(c : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getObjCDeclQualifiers(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getObjCManglings(param0 : CXCursor) -> *mut CXStringSet);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getObjCPropertyAttributes(c : CXCursor, reserved : u32) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getObjCPropertyGetterName(c : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getObjCPropertySetterName(c : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getObjCSelectorIndex(param0 : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getOffsetOfField(c : CXCursor) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getRawCommentText(c : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getReceiverType(c : CXCursor) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getSpellingNameRange(param0 : CXCursor, pieceindex : u32, options : u32) -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getStorageClass(param0 : CXCursor) -> CX_StorageClass);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getTemplateArgumentKind(c : CXCursor, i : u32) -> CXTemplateArgumentKind);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getTemplateArgumentType(c : CXCursor, i : u32) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getTemplateArgumentUnsignedValue(c : CXCursor, i : u32) -> u64);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getTemplateArgumentValue(c : CXCursor, i : u32) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getTranslationUnit(param0 : CXCursor) -> CXTranslationUnit);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_getVarDeclInitializer(cursor : CXCursor) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_hasAttrs(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_hasVarDeclExternalStorage(cursor : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_hasVarDeclGlobalStorage(cursor : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isAnonymous(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isAnonymousRecordDecl(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isBitField(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isDynamicCall(c : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isExternalSymbol(c : CXCursor, language : *mut CXString, definedin : *mut CXString, isgenerated : *mut u32) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isFunctionInlined(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isGCCAssemblyHasGoto(param0 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isGCCAssemblyVolatile(cursor : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isInlineNamespace(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isMacroBuiltin(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isMacroFunctionLike(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isNull(cursor : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isObjCOptional(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Cursor_isVariadic(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_EnumDecl_isScoped(c : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_dispose(e : CXEvalResult));
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_getAsDouble(e : CXEvalResult) -> f64);
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_getAsInt(e : CXEvalResult) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_getAsLongLong(e : CXEvalResult) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_getAsStr(e : CXEvalResult) -> *const i8);
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_getAsUnsigned(e : CXEvalResult) -> u64);
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_getKind(e : CXEvalResult) -> CXEvalResultKind);
windows_link::link!("libclang.dll" "C" fn clang_EvalResult_isUnsignedInt(e : CXEvalResult) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_File_isEqual(file1 : CXFile, file2 : CXFile) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_File_tryGetRealPathName(file : CXFile) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_IndexAction_create(cidx : CXIndex) -> CXIndexAction);
windows_link::link!("libclang.dll" "C" fn clang_IndexAction_dispose(param0 : CXIndexAction));
windows_link::link!("libclang.dll" "C" fn clang_Location_isFromMainFile(location : CXSourceLocation) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Location_isInSystemHeader(location : CXSourceLocation) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_ModuleMapDescriptor_create(options : u32) -> CXModuleMapDescriptor);
windows_link::link!("libclang.dll" "C" fn clang_ModuleMapDescriptor_dispose(param0 : CXModuleMapDescriptor));
windows_link::link!("libclang.dll" "C" fn clang_ModuleMapDescriptor_setFrameworkModuleName(param0 : CXModuleMapDescriptor, name : *const i8) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_ModuleMapDescriptor_setUmbrellaHeader(param0 : CXModuleMapDescriptor, name : *const i8) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_ModuleMapDescriptor_writeToBuffer(param0 : CXModuleMapDescriptor, options : u32, out_buffer_ptr : *mut *mut i8, out_buffer_size : *mut u32) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_Module_getASTFile(module : CXModule) -> CXFile);
windows_link::link!("libclang.dll" "C" fn clang_Module_getFullName(module : CXModule) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Module_getName(module : CXModule) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Module_getNumTopLevelHeaders(param0 : CXTranslationUnit, module : CXModule) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Module_getParent(module : CXModule) -> CXModule);
windows_link::link!("libclang.dll" "C" fn clang_Module_getTopLevelHeader(param0 : CXTranslationUnit, module : CXModule, index : u32) -> CXFile);
windows_link::link!("libclang.dll" "C" fn clang_Module_isSystem(module : CXModule) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_PrintingPolicy_dispose(policy : CXPrintingPolicy));
windows_link::link!("libclang.dll" "C" fn clang_PrintingPolicy_getProperty(policy : CXPrintingPolicy, property : CXPrintingPolicyProperty) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_PrintingPolicy_setProperty(policy : CXPrintingPolicy, property : CXPrintingPolicyProperty, value : u32));
windows_link::link!("libclang.dll" "C" fn clang_Range_isNull(range : CXSourceRange) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_TargetInfo_dispose(info : CXTargetInfo));
windows_link::link!("libclang.dll" "C" fn clang_TargetInfo_getPointerWidth(info : CXTargetInfo) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_TargetInfo_getTriple(info : CXTargetInfo) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Type_getAlignOf(t : CXType) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_Type_getCXXRefQualifier(t : CXType) -> CXRefQualifierKind);
windows_link::link!("libclang.dll" "C" fn clang_Type_getClassType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Type_getModifiedType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Type_getNamedType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Type_getNullability(t : CXType) -> CXTypeNullabilityKind);
windows_link::link!("libclang.dll" "C" fn clang_Type_getNumObjCProtocolRefs(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Type_getNumObjCTypeArgs(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Type_getNumTemplateArguments(t : CXType) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_Type_getObjCEncoding(r#type : CXType) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_Type_getObjCObjectBaseType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Type_getObjCProtocolDecl(t : CXType, i : u32) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_Type_getObjCTypeArg(t : CXType, i : u32) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Type_getOffsetOf(t : CXType, s : *const i8) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_Type_getSizeOf(t : CXType) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_Type_getTemplateArgumentAsType(t : CXType, i : u32) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Type_getValueType(ct : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_Type_isTransparentTagTypedef(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_Type_visitFields(t : CXType, visitor : CXFieldVisitor, client_data : CXClientData) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_VirtualFileOverlay_addFileMapping(param0 : CXVirtualFileOverlay, virtualpath : *const i8, realpath : *const i8) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_VirtualFileOverlay_create(options : u32) -> CXVirtualFileOverlay);
windows_link::link!("libclang.dll" "C" fn clang_VirtualFileOverlay_dispose(param0 : CXVirtualFileOverlay));
windows_link::link!("libclang.dll" "C" fn clang_VirtualFileOverlay_setCaseSensitivity(param0 : CXVirtualFileOverlay, casesensitive : i32) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_VirtualFileOverlay_writeToBuffer(param0 : CXVirtualFileOverlay, options : u32, out_buffer_ptr : *mut *mut i8, out_buffer_size : *mut u32) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_annotateTokens(tu : CXTranslationUnit, tokens : *mut CXToken, numtokens : u32, cursors : *mut CXCursor));
windows_link::link!("libclang.dll" "C" fn clang_codeCompleteAt(tu : CXTranslationUnit, complete_filename : *const i8, complete_line : u32, complete_column : u32, unsaved_files : *mut CXUnsavedFile, num_unsaved_files : u32, options : u32) -> *mut CXCodeCompleteResults);
windows_link::link!("libclang.dll" "C" fn clang_codeCompleteGetContainerKind(results : *mut CXCodeCompleteResults, isincomplete : *mut u32) -> CXCursorKind);
windows_link::link!("libclang.dll" "C" fn clang_codeCompleteGetContainerUSR(results : *mut CXCodeCompleteResults) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_codeCompleteGetContexts(results : *mut CXCodeCompleteResults) -> u64);
windows_link::link!("libclang.dll" "C" fn clang_codeCompleteGetDiagnostic(results : *mut CXCodeCompleteResults, index : u32) -> CXDiagnostic);
windows_link::link!("libclang.dll" "C" fn clang_codeCompleteGetNumDiagnostics(results : *mut CXCodeCompleteResults) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_codeCompleteGetObjCSelector(results : *mut CXCodeCompleteResults) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_constructUSR_ObjCCategory(class_name : *const i8, category_name : *const i8) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_constructUSR_ObjCClass(class_name : *const i8) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_constructUSR_ObjCIvar(name : *const i8, classusr : CXString) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_constructUSR_ObjCMethod(name : *const i8, isinstancemethod : u32, classusr : CXString) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_constructUSR_ObjCProperty(property : *const i8, classusr : CXString) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_constructUSR_ObjCProtocol(protocol_name : *const i8) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_createCXCursorSet() -> CXCursorSet);
windows_link::link!("libclang.dll" "C" fn clang_createIndex(excludedeclarationsfrompch : i32, displaydiagnostics : i32) -> CXIndex);
windows_link::link!("libclang.dll" "C" fn clang_createIndexWithOptions(options : *const CXIndexOptions) -> CXIndex);
windows_link::link!("libclang.dll" "C" fn clang_createTranslationUnit(cidx : CXIndex, ast_filename : *const i8) -> CXTranslationUnit);
windows_link::link!("libclang.dll" "C" fn clang_createTranslationUnit2(cidx : CXIndex, ast_filename : *const i8, out_tu : *mut CXTranslationUnit) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_createTranslationUnitFromSourceFile(cidx : CXIndex, source_filename : *const i8, num_clang_command_line_args : i32, clang_command_line_args : *const *const i8, num_unsaved_files : u32, unsaved_files : *mut CXUnsavedFile) -> CXTranslationUnit);
windows_link::link!("libclang.dll" "C" fn clang_defaultCodeCompleteOptions() -> u32);
windows_link::link!("libclang.dll" "C" fn clang_defaultDiagnosticDisplayOptions() -> u32);
windows_link::link!("libclang.dll" "C" fn clang_defaultEditingTranslationUnitOptions() -> u32);
windows_link::link!("libclang.dll" "C" fn clang_defaultReparseOptions(tu : CXTranslationUnit) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_defaultSaveOptions(tu : CXTranslationUnit) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_disposeCXCursorSet(cset : CXCursorSet));
windows_link::link!("libclang.dll" "C" fn clang_disposeCXPlatformAvailability(availability : *mut CXPlatformAvailability));
windows_link::link!("libclang.dll" "C" fn clang_disposeCXTUResourceUsage(usage : CXTUResourceUsage));
windows_link::link!("libclang.dll" "C" fn clang_disposeCodeCompleteResults(results : *mut CXCodeCompleteResults));
windows_link::link!("libclang.dll" "C" fn clang_disposeDiagnostic(diagnostic : CXDiagnostic));
windows_link::link!("libclang.dll" "C" fn clang_disposeDiagnosticSet(diags : CXDiagnosticSet));
windows_link::link!("libclang.dll" "C" fn clang_disposeIndex(index : CXIndex));
windows_link::link!("libclang.dll" "C" fn clang_disposeOverriddenCursors(overridden : *mut CXCursor));
windows_link::link!("libclang.dll" "C" fn clang_disposeSourceRangeList(ranges : *mut CXSourceRangeList));
windows_link::link!("libclang.dll" "C" fn clang_disposeString(string : CXString));
windows_link::link!("libclang.dll" "C" fn clang_disposeStringSet(set : *mut CXStringSet));
windows_link::link!("libclang.dll" "C" fn clang_disposeTokens(tu : CXTranslationUnit, tokens : *mut CXToken, numtokens : u32));
windows_link::link!("libclang.dll" "C" fn clang_disposeTranslationUnit(param0 : CXTranslationUnit));
windows_link::link!("libclang.dll" "C" fn clang_enableStackTraces());
windows_link::link!("libclang.dll" "C" fn clang_equalCursors(param0 : CXCursor, param1 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_equalLocations(loc1 : CXSourceLocation, loc2 : CXSourceLocation) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_equalRanges(range1 : CXSourceRange, range2 : CXSourceRange) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_equalTypes(a : CXType, b : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_executeOnThread(r#fn : Option < unsafe extern "C" fn(*mut core::ffi::c_void) >, user_data : *mut core::ffi::c_void, stack_size : u32));
windows_link::link!("libclang.dll" "C" fn clang_findIncludesInFile(tu : CXTranslationUnit, file : CXFile, visitor : CXCursorAndRangeVisitor) -> CXResult);
windows_link::link!("libclang.dll" "C" fn clang_findIncludesInFileWithBlock(param0 : CXTranslationUnit, param1 : CXFile, param2 : CXCursorAndRangeVisitorBlock) -> CXResult);
windows_link::link!("libclang.dll" "C" fn clang_findReferencesInFile(cursor : CXCursor, file : CXFile, visitor : CXCursorAndRangeVisitor) -> CXResult);
windows_link::link!("libclang.dll" "C" fn clang_findReferencesInFileWithBlock(param0 : CXCursor, param1 : CXFile, param2 : CXCursorAndRangeVisitorBlock) -> CXResult);
windows_link::link!("libclang.dll" "C" fn clang_formatDiagnostic(diagnostic : CXDiagnostic, options : u32) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_free(buffer : *mut core::ffi::c_void));
windows_link::link!("libclang.dll" "C" fn clang_getAddressSpace(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getAllSkippedRanges(tu : CXTranslationUnit) -> *mut CXSourceRangeList);
windows_link::link!("libclang.dll" "C" fn clang_getArgType(t : CXType, i : u32) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getArrayElementType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getArraySize(t : CXType) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_getBinaryOperatorKindSpelling(kind : CXBinaryOperatorKind) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getBuildSessionTimestamp() -> u64);
windows_link::link!("libclang.dll" "C" fn clang_getCString(string : CXString) -> *const i8);
windows_link::link!("libclang.dll" "C" fn clang_getCXTUResourceUsage(tu : CXTranslationUnit) -> CXTUResourceUsage);
windows_link::link!("libclang.dll" "C" fn clang_getCXXAccessSpecifier(param0 : CXCursor) -> CX_CXXAccessSpecifier);
windows_link::link!("libclang.dll" "C" fn clang_getCanonicalCursor(param0 : CXCursor) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getCanonicalType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getChildDiagnostics(d : CXDiagnostic) -> CXDiagnosticSet);
windows_link::link!("libclang.dll" "C" fn clang_getClangVersion() -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionAnnotation(completion_string : CXCompletionString, annotation_number : u32) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionAvailability(completion_string : CXCompletionString) -> CXAvailabilityKind);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionBriefComment(completion_string : CXCompletionString) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionChunkCompletionString(completion_string : CXCompletionString, chunk_number : u32) -> CXCompletionString);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionChunkKind(completion_string : CXCompletionString, chunk_number : u32) -> CXCompletionChunkKind);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionChunkText(completion_string : CXCompletionString, chunk_number : u32) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionFixIt(results : *mut CXCodeCompleteResults, completion_index : u32, fixit_index : u32, replacement_range : *mut CXSourceRange) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionNumAnnotations(completion_string : CXCompletionString) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionNumFixIts(results : *mut CXCodeCompleteResults, completion_index : u32) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionParent(completion_string : CXCompletionString, kind : *mut CXCursorKind) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCompletionPriority(completion_string : CXCompletionString) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getCursor(param0 : CXTranslationUnit, param1 : CXSourceLocation) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getCursorAvailability(cursor : CXCursor) -> CXAvailabilityKind);
windows_link::link!("libclang.dll" "C" fn clang_getCursorBinaryOperatorKind(cursor : CXCursor) -> CXBinaryOperatorKind);
windows_link::link!("libclang.dll" "C" fn clang_getCursorCompletionString(cursor : CXCursor) -> CXCompletionString);
windows_link::link!("libclang.dll" "C" fn clang_getCursorDefinition(param0 : CXCursor) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getCursorDisplayName(param0 : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCursorExceptionSpecificationType(c : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_getCursorExtent(param0 : CXCursor) -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_getCursorKind(param0 : CXCursor) -> CXCursorKind);
windows_link::link!("libclang.dll" "C" fn clang_getCursorKindSpelling(kind : CXCursorKind) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCursorLanguage(cursor : CXCursor) -> CXLanguageKind);
windows_link::link!("libclang.dll" "C" fn clang_getCursorLexicalParent(cursor : CXCursor) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getCursorLinkage(cursor : CXCursor) -> CXLinkageKind);
windows_link::link!("libclang.dll" "C" fn clang_getCursorLocation(param0 : CXCursor) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getCursorPlatformAvailability(cursor : CXCursor, always_deprecated : *mut i32, deprecated_message : *mut CXString, always_unavailable : *mut i32, unavailable_message : *mut CXString, availability : *mut CXPlatformAvailability, availability_size : i32) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_getCursorPrettyPrinted(cursor : CXCursor, policy : CXPrintingPolicy) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCursorPrintingPolicy(param0 : CXCursor) -> CXPrintingPolicy);
windows_link::link!("libclang.dll" "C" fn clang_getCursorReferenceNameRange(c : CXCursor, nameflags : u32, pieceindex : u32) -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_getCursorReferenced(param0 : CXCursor) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getCursorResultType(c : CXCursor) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getCursorSemanticParent(cursor : CXCursor) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getCursorSpelling(param0 : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCursorTLSKind(cursor : CXCursor) -> CXTLSKind);
windows_link::link!("libclang.dll" "C" fn clang_getCursorType(c : CXCursor) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getCursorUSR(param0 : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getCursorUnaryOperatorKind(cursor : CXCursor) -> CXUnaryOperatorKind);
windows_link::link!("libclang.dll" "C" fn clang_getCursorVisibility(cursor : CXCursor) -> CXVisibilityKind);
windows_link::link!("libclang.dll" "C" fn clang_getDeclObjCTypeEncoding(c : CXCursor) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getDefinitionSpellingAndExtent(param0 : CXCursor, startbuf : *mut *const i8, endbuf : *mut *const i8, startline : *mut u32, startcolumn : *mut u32, endline : *mut u32, endcolumn : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_getDiagnostic(unit : CXTranslationUnit, index : u32) -> CXDiagnostic);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticCategory(param0 : CXDiagnostic) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticCategoryName(category : u32) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticCategoryText(param0 : CXDiagnostic) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticFixIt(diagnostic : CXDiagnostic, fixit : u32, replacementrange : *mut CXSourceRange) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticInSet(diags : CXDiagnosticSet, index : u32) -> CXDiagnostic);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticLocation(param0 : CXDiagnostic) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticNumFixIts(diagnostic : CXDiagnostic) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticNumRanges(param0 : CXDiagnostic) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticOption(diag : CXDiagnostic, disable : *mut CXString) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticRange(diagnostic : CXDiagnostic, range : u32) -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticSetFromTU(unit : CXTranslationUnit) -> CXDiagnosticSet);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticSeverity(param0 : CXDiagnostic) -> CXDiagnosticSeverity);
windows_link::link!("libclang.dll" "C" fn clang_getDiagnosticSpelling(param0 : CXDiagnostic) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getElementType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getEnumConstantDeclUnsignedValue(c : CXCursor) -> u64);
windows_link::link!("libclang.dll" "C" fn clang_getEnumConstantDeclValue(c : CXCursor) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_getEnumDeclIntegerType(c : CXCursor) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getExceptionSpecificationType(t : CXType) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_getExpansionLocation(location : CXSourceLocation, file : *mut CXFile, line : *mut u32, column : *mut u32, offset : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_getFieldDeclBitWidth(c : CXCursor) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_getFile(tu : CXTranslationUnit, file_name : *const i8) -> CXFile);
windows_link::link!("libclang.dll" "C" fn clang_getFileContents(tu : CXTranslationUnit, file : CXFile, size : *mut size_t) -> *const i8);
windows_link::link!("libclang.dll" "C" fn clang_getFileLocation(location : CXSourceLocation, file : *mut CXFile, line : *mut u32, column : *mut u32, offset : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_getFileName(sfile : CXFile) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getFileTime(sfile : CXFile) -> time_t);
windows_link::link!("libclang.dll" "C" fn clang_getFileUniqueID(file : CXFile, outid : *mut CXFileUniqueID) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_getFullyQualifiedName(ct : CXType, policy : CXPrintingPolicy, withglobalnsprefix : u32) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getFunctionTypeCallingConv(t : CXType) -> CXCallingConv);
windows_link::link!("libclang.dll" "C" fn clang_getIBOutletCollectionType(param0 : CXCursor) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getIncludedFile(cursor : CXCursor) -> CXFile);
windows_link::link!("libclang.dll" "C" fn clang_getInclusions(tu : CXTranslationUnit, visitor : CXInclusionVisitor, client_data : CXClientData));
windows_link::link!("libclang.dll" "C" fn clang_getInstantiationLocation(location : CXSourceLocation, file : *mut CXFile, line : *mut u32, column : *mut u32, offset : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_getLocation(tu : CXTranslationUnit, file : CXFile, line : u32, column : u32) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getLocationForOffset(tu : CXTranslationUnit, file : CXFile, offset : u32) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getModuleForFile(param0 : CXTranslationUnit, param1 : CXFile) -> CXModule);
windows_link::link!("libclang.dll" "C" fn clang_getNonReferenceType(ct : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getNullCursor() -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getNullLocation() -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getNullRange() -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_getNumArgTypes(t : CXType) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_getNumCompletionChunks(completion_string : CXCompletionString) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getNumDiagnostics(unit : CXTranslationUnit) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getNumDiagnosticsInSet(diags : CXDiagnosticSet) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getNumElements(t : CXType) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_getNumOverloadedDecls(cursor : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_getOffsetOfBase(parent : CXCursor, base : CXCursor) -> i64);
windows_link::link!("libclang.dll" "C" fn clang_getOverloadedDecl(cursor : CXCursor, index : u32) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getOverriddenCursors(cursor : CXCursor, overridden : *mut *mut CXCursor, num_overridden : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_getPointeeType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getPresumedLocation(location : CXSourceLocation, filename : *mut CXString, line : *mut u32, column : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_getRange(begin : CXSourceLocation, end : CXSourceLocation) -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_getRangeEnd(range : CXSourceRange) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getRangeStart(range : CXSourceRange) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getRemappings(param0 : *const i8) -> CXRemapping);
windows_link::link!("libclang.dll" "C" fn clang_getRemappingsFromFileList(param0 : *mut *const i8, param1 : u32) -> CXRemapping);
windows_link::link!("libclang.dll" "C" fn clang_getResultType(t : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getSkippedRanges(tu : CXTranslationUnit, file : CXFile) -> *mut CXSourceRangeList);
windows_link::link!("libclang.dll" "C" fn clang_getSpecializedCursorTemplate(c : CXCursor) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getSpellingLocation(location : CXSourceLocation, file : *mut CXFile, line : *mut u32, column : *mut u32, offset : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_getTUResourceUsageName(kind : CXTUResourceUsageKind) -> *const i8);
windows_link::link!("libclang.dll" "C" fn clang_getTemplateCursorKind(c : CXCursor) -> CXCursorKind);
windows_link::link!("libclang.dll" "C" fn clang_getToken(tu : CXTranslationUnit, location : CXSourceLocation) -> *mut CXToken);
windows_link::link!("libclang.dll" "C" fn clang_getTokenExtent(param0 : CXTranslationUnit, param1 : CXToken) -> CXSourceRange);
windows_link::link!("libclang.dll" "C" fn clang_getTokenKind(param0 : CXToken) -> CXTokenKind);
windows_link::link!("libclang.dll" "C" fn clang_getTokenLocation(param0 : CXTranslationUnit, param1 : CXToken) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_getTokenSpelling(param0 : CXTranslationUnit, param1 : CXToken) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getTranslationUnitCursor(param0 : CXTranslationUnit) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getTranslationUnitSpelling(ctunit : CXTranslationUnit) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getTranslationUnitTargetInfo(ctunit : CXTranslationUnit) -> CXTargetInfo);
windows_link::link!("libclang.dll" "C" fn clang_getTypeDeclaration(t : CXType) -> CXCursor);
windows_link::link!("libclang.dll" "C" fn clang_getTypeKindSpelling(k : CXTypeKind) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getTypePrettyPrinted(ct : CXType, cxpolicy : CXPrintingPolicy) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getTypeSpelling(ct : CXType) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getTypedefDeclUnderlyingType(c : CXCursor) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_getTypedefName(ct : CXType) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getUnaryOperatorKindSpelling(kind : CXUnaryOperatorKind) -> CXString);
windows_link::link!("libclang.dll" "C" fn clang_getUnqualifiedType(ct : CXType) -> CXType);
windows_link::link!("libclang.dll" "C" fn clang_hashCursor(param0 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_indexLoc_getCXSourceLocation(loc : CXIdxLoc) -> CXSourceLocation);
windows_link::link!("libclang.dll" "C" fn clang_indexLoc_getFileLocation(loc : CXIdxLoc, indexfile : *mut CXIdxClientFile, file : *mut CXFile, line : *mut u32, column : *mut u32, offset : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_indexSourceFile(param0 : CXIndexAction, client_data : CXClientData, index_callbacks : *mut IndexerCallbacks, index_callbacks_size : u32, index_options : u32, source_filename : *const i8, command_line_args : *const *const i8, num_command_line_args : i32, unsaved_files : *mut CXUnsavedFile, num_unsaved_files : u32, out_tu : *mut CXTranslationUnit, tu_options : u32) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_indexSourceFileFullArgv(param0 : CXIndexAction, client_data : CXClientData, index_callbacks : *mut IndexerCallbacks, index_callbacks_size : u32, index_options : u32, source_filename : *const i8, command_line_args : *const *const i8, num_command_line_args : i32, unsaved_files : *mut CXUnsavedFile, num_unsaved_files : u32, out_tu : *mut CXTranslationUnit, tu_options : u32) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_indexTranslationUnit(param0 : CXIndexAction, client_data : CXClientData, index_callbacks : *mut IndexerCallbacks, index_callbacks_size : u32, index_options : u32, param5 : CXTranslationUnit) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_index_getCXXClassDeclInfo(param0 : *const CXIdxDeclInfo) -> *const CXIdxCXXClassDeclInfo);
windows_link::link!("libclang.dll" "C" fn clang_index_getClientContainer(param0 : *const CXIdxContainerInfo) -> CXIdxClientContainer);
windows_link::link!("libclang.dll" "C" fn clang_index_getClientEntity(param0 : *const CXIdxEntityInfo) -> CXIdxClientEntity);
windows_link::link!("libclang.dll" "C" fn clang_index_getIBOutletCollectionAttrInfo(param0 : *const CXIdxAttrInfo) -> *const CXIdxIBOutletCollectionAttrInfo);
windows_link::link!("libclang.dll" "C" fn clang_index_getObjCCategoryDeclInfo(param0 : *const CXIdxDeclInfo) -> *const CXIdxObjCCategoryDeclInfo);
windows_link::link!("libclang.dll" "C" fn clang_index_getObjCContainerDeclInfo(param0 : *const CXIdxDeclInfo) -> *const CXIdxObjCContainerDeclInfo);
windows_link::link!("libclang.dll" "C" fn clang_index_getObjCInterfaceDeclInfo(param0 : *const CXIdxDeclInfo) -> *const CXIdxObjCInterfaceDeclInfo);
windows_link::link!("libclang.dll" "C" fn clang_index_getObjCPropertyDeclInfo(param0 : *const CXIdxDeclInfo) -> *const CXIdxObjCPropertyDeclInfo);
windows_link::link!("libclang.dll" "C" fn clang_index_getObjCProtocolRefListInfo(param0 : *const CXIdxDeclInfo) -> *const CXIdxObjCProtocolRefListInfo);
windows_link::link!("libclang.dll" "C" fn clang_index_isEntityObjCContainerKind(param0 : CXIdxEntityKind) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_index_setClientContainer(param0 : *const CXIdxContainerInfo, param1 : CXIdxClientContainer));
windows_link::link!("libclang.dll" "C" fn clang_index_setClientEntity(param0 : *const CXIdxEntityInfo, param1 : CXIdxClientEntity));
windows_link::link!("libclang.dll" "C" fn clang_isAttribute(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isBeforeInTranslationUnit(loc1 : CXSourceLocation, loc2 : CXSourceLocation) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isConstQualifiedType(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isCursorDefinition(param0 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isDeclaration(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isExpression(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isFileMultipleIncludeGuarded(tu : CXTranslationUnit, file : CXFile) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isFunctionTypeVariadic(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isInvalid(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isInvalidDeclaration(param0 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isPODType(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isPreprocessing(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isReference(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isRestrictQualifiedType(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isStatement(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isTranslationUnit(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isUnexposed(param0 : CXCursorKind) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isVirtualBase(param0 : CXCursor) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_isVolatileQualifiedType(t : CXType) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_loadDiagnostics(file : *const i8, error : *mut CXLoadDiag_Error, errorstring : *mut CXString) -> CXDiagnosticSet);
windows_link::link!("libclang.dll" "C" fn clang_parseTranslationUnit(cidx : CXIndex, source_filename : *const i8, command_line_args : *const *const i8, num_command_line_args : i32, unsaved_files : *mut CXUnsavedFile, num_unsaved_files : u32, options : u32) -> CXTranslationUnit);
windows_link::link!("libclang.dll" "C" fn clang_parseTranslationUnit2(cidx : CXIndex, source_filename : *const i8, command_line_args : *const *const i8, num_command_line_args : i32, unsaved_files : *mut CXUnsavedFile, num_unsaved_files : u32, options : u32, out_tu : *mut CXTranslationUnit) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_parseTranslationUnit2FullArgv(cidx : CXIndex, source_filename : *const i8, command_line_args : *const *const i8, num_command_line_args : i32, unsaved_files : *mut CXUnsavedFile, num_unsaved_files : u32, options : u32, out_tu : *mut CXTranslationUnit) -> CXErrorCode);
windows_link::link!("libclang.dll" "C" fn clang_remap_dispose(param0 : CXRemapping));
windows_link::link!("libclang.dll" "C" fn clang_remap_getFilenames(param0 : CXRemapping, param1 : u32, param2 : *mut CXString, param3 : *mut CXString));
windows_link::link!("libclang.dll" "C" fn clang_remap_getNumFiles(param0 : CXRemapping) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_reparseTranslationUnit(tu : CXTranslationUnit, num_unsaved_files : u32, unsaved_files : *mut CXUnsavedFile, options : u32) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_saveTranslationUnit(tu : CXTranslationUnit, filename : *const i8, options : u32) -> i32);
windows_link::link!("libclang.dll" "C" fn clang_sortCodeCompletionResults(results : *mut CXCompletionResult, numresults : u32));
windows_link::link!("libclang.dll" "C" fn clang_suspendTranslationUnit(param0 : CXTranslationUnit) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_toggleCrashRecovery(isenabled : u32));
windows_link::link!("libclang.dll" "C" fn clang_tokenize(tu : CXTranslationUnit, range : CXSourceRange, tokens : *mut *mut CXToken, numtokens : *mut u32));
windows_link::link!("libclang.dll" "C" fn clang_visitCXXBaseClasses(t : CXType, visitor : CXFieldVisitor, client_data : CXClientData) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_visitCXXMethods(t : CXType, visitor : CXFieldVisitor, client_data : CXClientData) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_visitChildren(parent : CXCursor, visitor : CXCursorVisitor, client_data : CXClientData) -> u32);
windows_link::link!("libclang.dll" "C" fn clang_visitChildrenWithBlock(parent : CXCursor, block : CXCursorVisitorBlock) -> u32);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getABIVersion() -> u32);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getAlignment(cursor : CXCursor, info : *mut WindowsClangAlignmentInfo) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getAttributeKindName(cursor : CXCursor) -> *const i8);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getConstantInitializer(cursor : CXCursor, is_constant : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getConstantIntegerElement(cursor : CXCursor, indices : *const u32, count : u32, value : *mut u64) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getConstantPointerInitializer(cursor : CXCursor, value : *mut i64) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getDeprecation(cursor : CXCursor, message : *mut CXString, replacement : *mut CXString) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getFieldInitializer(cursor : CXCursor, has_initializer : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getFunctionInfo(r#type : CXType, info : *mut WindowsClangFunctionInfo) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getImportConvention(cursor : CXCursor, info : *mut WindowsClangImportConvention) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getIntegerConstantInitializer(cursor : CXCursor, is_constant : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getLLVMVersion() -> *const i8);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getLanguageLinkage(cursor : CXCursor, linkage : *mut WindowsClangLanguageLinkage) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getLatestFunctionDeclaration(cursor : CXCursor, declaration : *mut CXCursor) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getLayoutExpression(declaration : CXCursor, expression : CXCursor, is_layout_expression : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getMacroDefinition(unit : CXTranslationUnit, name : *const i8, location : CXSourceLocation, definition : *mut CXCursor) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getMaxFieldAlignment(cursor : CXCursor, alignment_bits : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getMethodQualifiers(r#type : CXType, qualifiers : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getParameterDefaultArgument(parameter : CXCursor, expression : *mut CXCursor, spelling : *mut CXString) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getPointerInfo(r#type : CXType, info : *mut WindowsClangPointerInfo) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getPredefinedType(r#type : CXType, underlying : *mut CXType) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getRecordInfo(cursor : CXCursor, info : *mut WindowsClangRecordInfo) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getRuntimeIdentity() -> *const core::ffi::c_void);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getStringLiteral(cursor : CXCursor, value : *mut CXString) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getTypeAttributeKindName(r#type : CXType) -> *const i8);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getTypeOfOperand(cursor : CXCursor, expression : *mut CXCursor) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getTypeQualifiers(r#type : CXType, qualifiers : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getTypeVisibility(cursor : CXCursor, visibility : *mut CXVisibilityKind) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getTypedefDefinition(cursor : CXCursor, defines_tag : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getTypedefImplicit(cursor : CXCursor, is_implicit : *mut u32) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getUuid(cursor : CXCursor, uuid : *mut CXString) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getVirtualMethodSlot(cursor : CXCursor, slot : *mut u64) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_getWrittenName(cursor : CXCursor, name : *mut CXString) -> WindowsClangStatus);
windows_link::link!("windows_clang.dll" "C" fn windows_clang_probeMacros(source_filename : *const i8, command_line_args : *const *const i8, num_command_line_args : i32, unsaved_files : *mut CXUnsavedFile, num_unsaved_files : u32, names : *const *const i8, num_names : u32, probes : *mut WindowsClangMacroProbe, error : *mut CXString) -> WindowsClangStatus);
pub const CINDEX_VERSION: i32 = 64;
pub const CINDEX_VERSION_MAJOR: i32 = 0;
pub const CINDEX_VERSION_MINOR: i32 = 64;
pub type CXAvailabilityKind = i32;
pub const CXAvailability_Available: CXAvailabilityKind = 0;
pub const CXAvailability_Deprecated: CXAvailabilityKind = 1;
pub const CXAvailability_NotAccessible: CXAvailabilityKind = 3;
pub const CXAvailability_NotAvailable: CXAvailabilityKind = 2;
pub type CXBinaryOperatorKind = i32;
pub const CXBinaryOperator_Add: CXBinaryOperatorKind = 6;
pub const CXBinaryOperator_AddAssign: CXBinaryOperatorKind = 26;
pub const CXBinaryOperator_And: CXBinaryOperatorKind = 17;
pub const CXBinaryOperator_AndAssign: CXBinaryOperatorKind = 30;
pub const CXBinaryOperator_Assign: CXBinaryOperatorKind = 22;
pub const CXBinaryOperator_Cmp: CXBinaryOperatorKind = 10;
pub const CXBinaryOperator_Comma: CXBinaryOperatorKind = 33;
pub const CXBinaryOperator_Div: CXBinaryOperatorKind = 4;
pub const CXBinaryOperator_DivAssign: CXBinaryOperatorKind = 24;
pub const CXBinaryOperator_EQ: CXBinaryOperatorKind = 15;
pub const CXBinaryOperator_GE: CXBinaryOperatorKind = 14;
pub const CXBinaryOperator_GT: CXBinaryOperatorKind = 12;
pub const CXBinaryOperator_Invalid: CXBinaryOperatorKind = 0;
pub const CXBinaryOperator_LAnd: CXBinaryOperatorKind = 20;
pub const CXBinaryOperator_LE: CXBinaryOperatorKind = 13;
pub const CXBinaryOperator_LOr: CXBinaryOperatorKind = 21;
pub const CXBinaryOperator_LT: CXBinaryOperatorKind = 11;
pub const CXBinaryOperator_Last: CXBinaryOperatorKind = 33;
pub const CXBinaryOperator_Mul: CXBinaryOperatorKind = 3;
pub const CXBinaryOperator_MulAssign: CXBinaryOperatorKind = 23;
pub const CXBinaryOperator_NE: CXBinaryOperatorKind = 16;
pub const CXBinaryOperator_Or: CXBinaryOperatorKind = 19;
pub const CXBinaryOperator_OrAssign: CXBinaryOperatorKind = 32;
pub const CXBinaryOperator_PtrMemD: CXBinaryOperatorKind = 1;
pub const CXBinaryOperator_PtrMemI: CXBinaryOperatorKind = 2;
pub const CXBinaryOperator_Rem: CXBinaryOperatorKind = 5;
pub const CXBinaryOperator_RemAssign: CXBinaryOperatorKind = 25;
pub const CXBinaryOperator_Shl: CXBinaryOperatorKind = 8;
pub const CXBinaryOperator_ShlAssign: CXBinaryOperatorKind = 28;
pub const CXBinaryOperator_Shr: CXBinaryOperatorKind = 9;
pub const CXBinaryOperator_ShrAssign: CXBinaryOperatorKind = 29;
pub const CXBinaryOperator_Sub: CXBinaryOperatorKind = 7;
pub const CXBinaryOperator_SubAssign: CXBinaryOperatorKind = 27;
pub const CXBinaryOperator_Xor: CXBinaryOperatorKind = 18;
pub const CXBinaryOperator_XorAssign: CXBinaryOperatorKind = 31;
pub type CXCallingConv = i32;
pub const CXCallingConv_AAPCS: CXCallingConv = 6;
pub const CXCallingConv_AAPCS_VFP: CXCallingConv = 7;
pub const CXCallingConv_AArch64SVEPCS: CXCallingConv = 18;
pub const CXCallingConv_AArch64VectorCall: CXCallingConv = 16;
pub const CXCallingConv_C: CXCallingConv = 1;
pub const CXCallingConv_Default: CXCallingConv = 0;
pub const CXCallingConv_IntelOclBicc: CXCallingConv = 9;
pub const CXCallingConv_Invalid: CXCallingConv = 100;
pub const CXCallingConv_M68kRTD: CXCallingConv = 19;
pub const CXCallingConv_PreserveAll: CXCallingConv = 15;
pub const CXCallingConv_PreserveMost: CXCallingConv = 14;
pub const CXCallingConv_PreserveNone: CXCallingConv = 20;
pub const CXCallingConv_RISCVVLSCall_1024: CXCallingConv = 27;
pub const CXCallingConv_RISCVVLSCall_128: CXCallingConv = 24;
pub const CXCallingConv_RISCVVLSCall_16384: CXCallingConv = 31;
pub const CXCallingConv_RISCVVLSCall_2048: CXCallingConv = 28;
pub const CXCallingConv_RISCVVLSCall_256: CXCallingConv = 25;
pub const CXCallingConv_RISCVVLSCall_32: CXCallingConv = 22;
pub const CXCallingConv_RISCVVLSCall_32768: CXCallingConv = 32;
pub const CXCallingConv_RISCVVLSCall_4096: CXCallingConv = 29;
pub const CXCallingConv_RISCVVLSCall_512: CXCallingConv = 26;
pub const CXCallingConv_RISCVVLSCall_64: CXCallingConv = 23;
pub const CXCallingConv_RISCVVLSCall_65536: CXCallingConv = 33;
pub const CXCallingConv_RISCVVLSCall_8192: CXCallingConv = 30;
pub const CXCallingConv_RISCVVectorCall: CXCallingConv = 21;
pub const CXCallingConv_Swift: CXCallingConv = 13;
pub const CXCallingConv_SwiftAsync: CXCallingConv = 17;
pub const CXCallingConv_Unexposed: CXCallingConv = 200;
pub const CXCallingConv_Win64: CXCallingConv = 10;
pub const CXCallingConv_X86FastCall: CXCallingConv = 3;
pub const CXCallingConv_X86Pascal: CXCallingConv = 5;
pub const CXCallingConv_X86RegCall: CXCallingConv = 8;
pub const CXCallingConv_X86StdCall: CXCallingConv = 2;
pub const CXCallingConv_X86ThisCall: CXCallingConv = 4;
pub const CXCallingConv_X86VectorCall: CXCallingConv = 12;
pub const CXCallingConv_X86_64SysV: CXCallingConv = 11;
pub const CXCallingConv_X86_64Win64: CXCallingConv = 10;
pub type CXChildVisitResult = i32;
pub const CXChildVisit_Break: CXChildVisitResult = 0;
pub const CXChildVisit_Continue: CXChildVisitResult = 1;
pub const CXChildVisit_Recurse: CXChildVisitResult = 2;
pub type CXChoice = i32;
pub const CXChoice_Default: CXChoice = 0;
pub const CXChoice_Disabled: CXChoice = 2;
pub const CXChoice_Enabled: CXChoice = 1;
pub type CXClientData = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXCodeCompleteResults {
    pub Results: *mut CXCompletionResult,
    pub NumResults: u32,
}
pub type CXCodeComplete_Flags = i32;
pub const CXCodeComplete_IncludeBriefComments: CXCodeComplete_Flags = 4;
pub const CXCodeComplete_IncludeCodePatterns: CXCodeComplete_Flags = 2;
pub const CXCodeComplete_IncludeCompletionsWithFixIts: CXCodeComplete_Flags = 16;
pub const CXCodeComplete_IncludeMacros: CXCodeComplete_Flags = 1;
pub const CXCodeComplete_SkipPreamble: CXCodeComplete_Flags = 8;
pub type CXCompletionChunkKind = i32;
pub const CXCompletionChunk_Colon: CXCompletionChunkKind = 16;
pub const CXCompletionChunk_Comma: CXCompletionChunkKind = 14;
pub const CXCompletionChunk_CurrentParameter: CXCompletionChunkKind = 5;
pub const CXCompletionChunk_Equal: CXCompletionChunkKind = 18;
pub const CXCompletionChunk_HorizontalSpace: CXCompletionChunkKind = 19;
pub const CXCompletionChunk_Informative: CXCompletionChunkKind = 4;
pub const CXCompletionChunk_LeftAngle: CXCompletionChunkKind = 12;
pub const CXCompletionChunk_LeftBrace: CXCompletionChunkKind = 10;
pub const CXCompletionChunk_LeftBracket: CXCompletionChunkKind = 8;
pub const CXCompletionChunk_LeftParen: CXCompletionChunkKind = 6;
pub const CXCompletionChunk_Optional: CXCompletionChunkKind = 0;
pub const CXCompletionChunk_Placeholder: CXCompletionChunkKind = 3;
pub const CXCompletionChunk_ResultType: CXCompletionChunkKind = 15;
pub const CXCompletionChunk_RightAngle: CXCompletionChunkKind = 13;
pub const CXCompletionChunk_RightBrace: CXCompletionChunkKind = 11;
pub const CXCompletionChunk_RightBracket: CXCompletionChunkKind = 9;
pub const CXCompletionChunk_RightParen: CXCompletionChunkKind = 7;
pub const CXCompletionChunk_SemiColon: CXCompletionChunkKind = 17;
pub const CXCompletionChunk_Text: CXCompletionChunkKind = 2;
pub const CXCompletionChunk_TypedText: CXCompletionChunkKind = 1;
pub const CXCompletionChunk_VerticalSpace: CXCompletionChunkKind = 20;
pub type CXCompletionContext = i32;
pub const CXCompletionContext_AnyType: CXCompletionContext = 1;
pub const CXCompletionContext_AnyValue: CXCompletionContext = 2;
pub const CXCompletionContext_ArrowMemberAccess: CXCompletionContext = 64;
pub const CXCompletionContext_CXXClassTypeValue: CXCompletionContext = 16;
pub const CXCompletionContext_ClassTag: CXCompletionContext = 2048;
pub const CXCompletionContext_DotMemberAccess: CXCompletionContext = 32;
pub const CXCompletionContext_EnumTag: CXCompletionContext = 256;
pub const CXCompletionContext_IncludedFile: CXCompletionContext = 4194304;
pub const CXCompletionContext_MacroName: CXCompletionContext = 1048576;
pub const CXCompletionContext_Namespace: CXCompletionContext = 4096;
pub const CXCompletionContext_NaturalLanguage: CXCompletionContext = 2097152;
pub const CXCompletionContext_NestedNameSpecifier: CXCompletionContext = 8192;
pub const CXCompletionContext_ObjCCategory: CXCompletionContext = 65536;
pub const CXCompletionContext_ObjCClassMessage: CXCompletionContext = 262144;
pub const CXCompletionContext_ObjCInstanceMessage: CXCompletionContext = 131072;
pub const CXCompletionContext_ObjCInterface: CXCompletionContext = 16384;
pub const CXCompletionContext_ObjCObjectValue: CXCompletionContext = 4;
pub const CXCompletionContext_ObjCPropertyAccess: CXCompletionContext = 128;
pub const CXCompletionContext_ObjCProtocol: CXCompletionContext = 32768;
pub const CXCompletionContext_ObjCSelectorName: CXCompletionContext = 524288;
pub const CXCompletionContext_ObjCSelectorValue: CXCompletionContext = 8;
pub const CXCompletionContext_StructTag: CXCompletionContext = 1024;
pub const CXCompletionContext_Unexposed: CXCompletionContext = 0;
pub const CXCompletionContext_UnionTag: CXCompletionContext = 512;
pub const CXCompletionContext_Unknown: CXCompletionContext = 8388607;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXCompletionResult {
    pub CursorKind: CXCursorKind,
    pub CompletionString: CXCompletionString,
}
pub type CXCompletionString = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXCursor {
    pub kind: CXCursorKind,
    pub xdata: i32,
    pub data: [*const core::ffi::c_void; 3],
}
impl Default for CXCursor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXCursorAndRangeVisitor {
    pub context: *mut core::ffi::c_void,
    pub visit: Option<
        unsafe extern "C" fn(*mut core::ffi::c_void, CXCursor, CXSourceRange) -> CXVisitorResult,
    >,
}
pub type CXCursorAndRangeVisitorBlock = *mut _CXCursorAndRangeVisitorBlock;
pub type CXCursorKind = i32;
pub type CXCursorSet = *mut CXCursorSetImpl;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXCursorSetImpl(pub u8);
pub type CXCursorVisitor = Option<
    unsafe extern "C" fn(
        cursor: CXCursor,
        parent: CXCursor,
        client_data: CXClientData,
    ) -> CXChildVisitResult,
>;
pub type CXCursorVisitorBlock = *mut _CXChildVisitResult;
pub const CXCursor_AddrLabelExpr: CXCursorKind = 120;
pub const CXCursor_AlignedAttr: CXCursorKind = 441;
pub const CXCursor_AnnotateAttr: CXCursorKind = 406;
pub const CXCursor_ArraySectionExpr: CXCursorKind = 147;
pub const CXCursor_ArraySubscriptExpr: CXCursorKind = 113;
pub const CXCursor_AsmLabelAttr: CXCursorKind = 407;
pub const CXCursor_AsmStmt: CXCursorKind = 215;
pub const CXCursor_BinaryOperator: CXCursorKind = 114;
pub const CXCursor_BlockExpr: CXCursorKind = 105;
pub const CXCursor_BreakStmt: CXCursorKind = 213;
pub const CXCursor_BuiltinBitCastExpr: CXCursorKind = 280;
pub const CXCursor_CStyleCastExpr: CXCursorKind = 117;
pub const CXCursor_CUDAConstantAttr: CXCursorKind = 412;
pub const CXCursor_CUDADeviceAttr: CXCursorKind = 413;
pub const CXCursor_CUDAGlobalAttr: CXCursorKind = 414;
pub const CXCursor_CUDAHostAttr: CXCursorKind = 415;
pub const CXCursor_CUDASharedAttr: CXCursorKind = 416;
pub const CXCursor_CXXAccessSpecifier: CXCursorKind = 39;
pub const CXCursor_CXXAddrspaceCastExpr: CXCursorKind = 152;
pub const CXCursor_CXXBaseSpecifier: CXCursorKind = 44;
pub const CXCursor_CXXBoolLiteralExpr: CXCursorKind = 130;
pub const CXCursor_CXXCatchStmt: CXCursorKind = 223;
pub const CXCursor_CXXConstCastExpr: CXCursorKind = 127;
pub const CXCursor_CXXDeleteExpr: CXCursorKind = 135;
pub const CXCursor_CXXDynamicCastExpr: CXCursorKind = 125;
pub const CXCursor_CXXFinalAttr: CXCursorKind = 404;
pub const CXCursor_CXXForRangeStmt: CXCursorKind = 225;
pub const CXCursor_CXXFunctionalCastExpr: CXCursorKind = 128;
pub const CXCursor_CXXMethod: CXCursorKind = 21;
pub const CXCursor_CXXNewExpr: CXCursorKind = 134;
pub const CXCursor_CXXNullPtrLiteralExpr: CXCursorKind = 131;
pub const CXCursor_CXXOverrideAttr: CXCursorKind = 405;
pub const CXCursor_CXXParenListInitExpr: CXCursorKind = 155;
pub const CXCursor_CXXReinterpretCastExpr: CXCursorKind = 126;
pub const CXCursor_CXXStaticCastExpr: CXCursorKind = 124;
pub const CXCursor_CXXThisExpr: CXCursorKind = 132;
pub const CXCursor_CXXThrowExpr: CXCursorKind = 133;
pub const CXCursor_CXXTryStmt: CXCursorKind = 224;
pub const CXCursor_CXXTypeidExpr: CXCursorKind = 129;
pub const CXCursor_CallExpr: CXCursorKind = 103;
pub const CXCursor_CaseStmt: CXCursorKind = 203;
pub const CXCursor_CharacterLiteral: CXCursorKind = 110;
pub const CXCursor_ClassDecl: CXCursorKind = 4;
pub const CXCursor_ClassTemplate: CXCursorKind = 31;
pub const CXCursor_ClassTemplatePartialSpecialization: CXCursorKind = 32;
pub const CXCursor_CompoundAssignOperator: CXCursorKind = 115;
pub const CXCursor_CompoundLiteralExpr: CXCursorKind = 118;
pub const CXCursor_CompoundStmt: CXCursorKind = 202;
pub const CXCursor_ConceptDecl: CXCursorKind = 604;
pub const CXCursor_ConceptSpecializationExpr: CXCursorKind = 153;
pub const CXCursor_ConditionalOperator: CXCursorKind = 116;
pub const CXCursor_ConstAttr: CXCursorKind = 410;
pub const CXCursor_Constructor: CXCursorKind = 24;
pub const CXCursor_ContinueStmt: CXCursorKind = 212;
pub const CXCursor_ConvergentAttr: CXCursorKind = 438;
pub const CXCursor_ConversionFunction: CXCursorKind = 26;
pub const CXCursor_DLLExport: CXCursorKind = 418;
pub const CXCursor_DLLImport: CXCursorKind = 419;
pub const CXCursor_DeclRefExpr: CXCursorKind = 101;
pub const CXCursor_DeclStmt: CXCursorKind = 231;
pub const CXCursor_DefaultStmt: CXCursorKind = 204;
pub const CXCursor_Destructor: CXCursorKind = 25;
pub const CXCursor_DoStmt: CXCursorKind = 208;
pub const CXCursor_EnumConstantDecl: CXCursorKind = 7;
pub const CXCursor_EnumDecl: CXCursorKind = 5;
pub type CXCursor_ExceptionSpecificationKind = i32;
pub const CXCursor_ExceptionSpecificationKind_BasicNoexcept: CXCursor_ExceptionSpecificationKind =
    4;
pub const CXCursor_ExceptionSpecificationKind_ComputedNoexcept:
    CXCursor_ExceptionSpecificationKind = 5;
pub const CXCursor_ExceptionSpecificationKind_Dynamic: CXCursor_ExceptionSpecificationKind = 2;
pub const CXCursor_ExceptionSpecificationKind_DynamicNone: CXCursor_ExceptionSpecificationKind = 1;
pub const CXCursor_ExceptionSpecificationKind_MSAny: CXCursor_ExceptionSpecificationKind = 3;
pub const CXCursor_ExceptionSpecificationKind_NoThrow: CXCursor_ExceptionSpecificationKind = 9;
pub const CXCursor_ExceptionSpecificationKind_None: CXCursor_ExceptionSpecificationKind = 0;
pub const CXCursor_ExceptionSpecificationKind_Unevaluated: CXCursor_ExceptionSpecificationKind = 6;
pub const CXCursor_ExceptionSpecificationKind_Uninstantiated: CXCursor_ExceptionSpecificationKind =
    7;
pub const CXCursor_ExceptionSpecificationKind_Unparsed: CXCursor_ExceptionSpecificationKind = 8;
pub const CXCursor_FieldDecl: CXCursorKind = 6;
pub const CXCursor_FirstAttr: CXCursorKind = 400;
pub const CXCursor_FirstDecl: CXCursorKind = 1;
pub const CXCursor_FirstExpr: CXCursorKind = 100;
pub const CXCursor_FirstExtraDecl: CXCursorKind = 600;
pub const CXCursor_FirstInvalid: CXCursorKind = 70;
pub const CXCursor_FirstPreprocessing: CXCursorKind = 500;
pub const CXCursor_FirstRef: CXCursorKind = 40;
pub const CXCursor_FirstStmt: CXCursorKind = 200;
pub const CXCursor_FixedPointLiteral: CXCursorKind = 149;
pub const CXCursor_FlagEnum: CXCursorKind = 437;
pub const CXCursor_FloatingLiteral: CXCursorKind = 107;
pub const CXCursor_ForStmt: CXCursorKind = 209;
pub const CXCursor_FriendDecl: CXCursorKind = 603;
pub const CXCursor_FunctionDecl: CXCursorKind = 8;
pub const CXCursor_FunctionTemplate: CXCursorKind = 30;
pub const CXCursor_GCCAsmStmt: CXCursorKind = 215;
pub const CXCursor_GNUNullExpr: CXCursorKind = 123;
pub const CXCursor_GenericSelectionExpr: CXCursorKind = 122;
pub const CXCursor_GotoStmt: CXCursorKind = 210;
pub const CXCursor_IBActionAttr: CXCursorKind = 401;
pub const CXCursor_IBOutletAttr: CXCursorKind = 402;
pub const CXCursor_IBOutletCollectionAttr: CXCursorKind = 403;
pub const CXCursor_IfStmt: CXCursorKind = 205;
pub const CXCursor_ImaginaryLiteral: CXCursorKind = 108;
pub const CXCursor_InclusionDirective: CXCursorKind = 503;
pub const CXCursor_IndirectGotoStmt: CXCursorKind = 211;
pub const CXCursor_InitListExpr: CXCursorKind = 119;
pub const CXCursor_IntegerLiteral: CXCursorKind = 106;
pub const CXCursor_InvalidCode: CXCursorKind = 73;
pub const CXCursor_InvalidFile: CXCursorKind = 70;
pub const CXCursor_LabelRef: CXCursorKind = 48;
pub const CXCursor_LabelStmt: CXCursorKind = 201;
pub const CXCursor_LambdaExpr: CXCursorKind = 144;
pub const CXCursor_LastAttr: CXCursorKind = 441;
pub const CXCursor_LastDecl: CXCursorKind = 39;
pub const CXCursor_LastExpr: CXCursorKind = 156;
pub const CXCursor_LastExtraDecl: CXCursorKind = 604;
pub const CXCursor_LastInvalid: CXCursorKind = 73;
pub const CXCursor_LastPreprocessing: CXCursorKind = 503;
pub const CXCursor_LastRef: CXCursorKind = 50;
pub const CXCursor_LastStmt: CXCursorKind = 333;
pub const CXCursor_LinkageSpec: CXCursorKind = 23;
pub const CXCursor_MSAsmStmt: CXCursorKind = 229;
pub const CXCursor_MacroDefinition: CXCursorKind = 501;
pub const CXCursor_MacroExpansion: CXCursorKind = 502;
pub const CXCursor_MacroInstantiation: CXCursorKind = 502;
pub const CXCursor_MemberRef: CXCursorKind = 47;
pub const CXCursor_MemberRefExpr: CXCursorKind = 102;
pub const CXCursor_ModuleImportDecl: CXCursorKind = 600;
pub const CXCursor_NSConsumed: CXCursorKind = 424;
pub const CXCursor_NSConsumesSelf: CXCursorKind = 423;
pub const CXCursor_NSReturnsAutoreleased: CXCursorKind = 422;
pub const CXCursor_NSReturnsNotRetained: CXCursorKind = 421;
pub const CXCursor_NSReturnsRetained: CXCursorKind = 420;
pub const CXCursor_Namespace: CXCursorKind = 22;
pub const CXCursor_NamespaceAlias: CXCursorKind = 33;
pub const CXCursor_NamespaceRef: CXCursorKind = 46;
pub const CXCursor_NoDeclFound: CXCursorKind = 71;
pub const CXCursor_NoDuplicateAttr: CXCursorKind = 411;
pub const CXCursor_NonTypeTemplateParameter: CXCursorKind = 28;
pub const CXCursor_NotImplemented: CXCursorKind = 72;
pub const CXCursor_NullStmt: CXCursorKind = 230;
pub const CXCursor_OMPArrayShapingExpr: CXCursorKind = 150;
pub const CXCursor_OMPAssumeDirective: CXCursorKind = 309;
pub const CXCursor_OMPAtomicDirective: CXCursorKind = 249;
pub const CXCursor_OMPBarrierDirective: CXCursorKind = 244;
pub const CXCursor_OMPCancelDirective: CXCursorKind = 256;
pub const CXCursor_OMPCancellationPointDirective: CXCursorKind = 255;
pub const CXCursor_OMPCanonicalLoop: CXCursorKind = 289;
pub const CXCursor_OMPCriticalDirective: CXCursorKind = 242;
pub const CXCursor_OMPDepobjDirective: CXCursorKind = 286;
pub const CXCursor_OMPDispatchDirective: CXCursorKind = 291;
pub const CXCursor_OMPDistributeDirective: CXCursorKind = 260;
pub const CXCursor_OMPDistributeParallelForDirective: CXCursorKind = 266;
pub const CXCursor_OMPDistributeParallelForSimdDirective: CXCursorKind = 267;
pub const CXCursor_OMPDistributeSimdDirective: CXCursorKind = 268;
pub const CXCursor_OMPErrorDirective: CXCursorKind = 305;
pub const CXCursor_OMPFlushDirective: CXCursorKind = 246;
pub const CXCursor_OMPForDirective: CXCursorKind = 234;
pub const CXCursor_OMPForSimdDirective: CXCursorKind = 250;
pub const CXCursor_OMPFuseDirective: CXCursorKind = 311;
pub const CXCursor_OMPGenericLoopDirective: CXCursorKind = 295;
pub const CXCursor_OMPInterchangeDirective: CXCursorKind = 308;
pub const CXCursor_OMPInteropDirective: CXCursorKind = 290;
pub const CXCursor_OMPIteratorExpr: CXCursorKind = 151;
pub const CXCursor_OMPMaskedDirective: CXCursorKind = 292;
pub const CXCursor_OMPMaskedTaskLoopDirective: CXCursorKind = 301;
pub const CXCursor_OMPMaskedTaskLoopSimdDirective: CXCursorKind = 302;
pub const CXCursor_OMPMasterDirective: CXCursorKind = 241;
pub const CXCursor_OMPMasterTaskLoopDirective: CXCursorKind = 281;
pub const CXCursor_OMPMasterTaskLoopSimdDirective: CXCursorKind = 283;
pub const CXCursor_OMPMetaDirective: CXCursorKind = 294;
pub const CXCursor_OMPOrderedDirective: CXCursorKind = 248;
pub const CXCursor_OMPParallelDirective: CXCursorKind = 232;
pub const CXCursor_OMPParallelForDirective: CXCursorKind = 238;
pub const CXCursor_OMPParallelForSimdDirective: CXCursorKind = 251;
pub const CXCursor_OMPParallelGenericLoopDirective: CXCursorKind = 298;
pub const CXCursor_OMPParallelMaskedDirective: CXCursorKind = 300;
pub const CXCursor_OMPParallelMaskedTaskLoopDirective: CXCursorKind = 303;
pub const CXCursor_OMPParallelMaskedTaskLoopSimdDirective: CXCursorKind = 304;
pub const CXCursor_OMPParallelMasterDirective: CXCursorKind = 285;
pub const CXCursor_OMPParallelMasterTaskLoopDirective: CXCursorKind = 282;
pub const CXCursor_OMPParallelMasterTaskLoopSimdDirective: CXCursorKind = 284;
pub const CXCursor_OMPParallelSectionsDirective: CXCursorKind = 239;
pub const CXCursor_OMPReverseDirective: CXCursorKind = 307;
pub const CXCursor_OMPScanDirective: CXCursorKind = 287;
pub const CXCursor_OMPScopeDirective: CXCursorKind = 306;
pub const CXCursor_OMPSectionDirective: CXCursorKind = 236;
pub const CXCursor_OMPSectionsDirective: CXCursorKind = 235;
pub const CXCursor_OMPSimdDirective: CXCursorKind = 233;
pub const CXCursor_OMPSingleDirective: CXCursorKind = 237;
pub const CXCursor_OMPStripeDirective: CXCursorKind = 310;
pub const CXCursor_OMPTargetDataDirective: CXCursorKind = 257;
pub const CXCursor_OMPTargetDirective: CXCursorKind = 252;
pub const CXCursor_OMPTargetEnterDataDirective: CXCursorKind = 261;
pub const CXCursor_OMPTargetExitDataDirective: CXCursorKind = 262;
pub const CXCursor_OMPTargetParallelDirective: CXCursorKind = 263;
pub const CXCursor_OMPTargetParallelForDirective: CXCursorKind = 264;
pub const CXCursor_OMPTargetParallelForSimdDirective: CXCursorKind = 269;
pub const CXCursor_OMPTargetParallelGenericLoopDirective: CXCursorKind = 299;
pub const CXCursor_OMPTargetSimdDirective: CXCursorKind = 270;
pub const CXCursor_OMPTargetTeamsDirective: CXCursorKind = 275;
pub const CXCursor_OMPTargetTeamsDistributeDirective: CXCursorKind = 276;
pub const CXCursor_OMPTargetTeamsDistributeParallelForDirective: CXCursorKind = 277;
pub const CXCursor_OMPTargetTeamsDistributeParallelForSimdDirective: CXCursorKind = 278;
pub const CXCursor_OMPTargetTeamsDistributeSimdDirective: CXCursorKind = 279;
pub const CXCursor_OMPTargetTeamsGenericLoopDirective: CXCursorKind = 297;
pub const CXCursor_OMPTargetUpdateDirective: CXCursorKind = 265;
pub const CXCursor_OMPTaskDirective: CXCursorKind = 240;
pub const CXCursor_OMPTaskLoopDirective: CXCursorKind = 258;
pub const CXCursor_OMPTaskLoopSimdDirective: CXCursorKind = 259;
pub const CXCursor_OMPTaskgroupDirective: CXCursorKind = 254;
pub const CXCursor_OMPTaskwaitDirective: CXCursorKind = 245;
pub const CXCursor_OMPTaskyieldDirective: CXCursorKind = 243;
pub const CXCursor_OMPTeamsDirective: CXCursorKind = 253;
pub const CXCursor_OMPTeamsDistributeDirective: CXCursorKind = 271;
pub const CXCursor_OMPTeamsDistributeParallelForDirective: CXCursorKind = 274;
pub const CXCursor_OMPTeamsDistributeParallelForSimdDirective: CXCursorKind = 273;
pub const CXCursor_OMPTeamsDistributeSimdDirective: CXCursorKind = 272;
pub const CXCursor_OMPTeamsGenericLoopDirective: CXCursorKind = 296;
pub const CXCursor_OMPTileDirective: CXCursorKind = 288;
pub const CXCursor_OMPUnrollDirective: CXCursorKind = 293;
pub const CXCursor_ObjCAtCatchStmt: CXCursorKind = 217;
pub const CXCursor_ObjCAtFinallyStmt: CXCursorKind = 218;
pub const CXCursor_ObjCAtSynchronizedStmt: CXCursorKind = 220;
pub const CXCursor_ObjCAtThrowStmt: CXCursorKind = 219;
pub const CXCursor_ObjCAtTryStmt: CXCursorKind = 216;
pub const CXCursor_ObjCAutoreleasePoolStmt: CXCursorKind = 221;
pub const CXCursor_ObjCAvailabilityCheckExpr: CXCursorKind = 148;
pub const CXCursor_ObjCBoolLiteralExpr: CXCursorKind = 145;
pub const CXCursor_ObjCBoxable: CXCursorKind = 436;
pub const CXCursor_ObjCBridgedCastExpr: CXCursorKind = 141;
pub const CXCursor_ObjCCategoryDecl: CXCursorKind = 12;
pub const CXCursor_ObjCCategoryImplDecl: CXCursorKind = 19;
pub const CXCursor_ObjCClassMethodDecl: CXCursorKind = 17;
pub const CXCursor_ObjCClassRef: CXCursorKind = 42;
pub const CXCursor_ObjCDesignatedInitializer: CXCursorKind = 434;
pub const CXCursor_ObjCDynamicDecl: CXCursorKind = 38;
pub const CXCursor_ObjCEncodeExpr: CXCursorKind = 138;
pub const CXCursor_ObjCException: CXCursorKind = 425;
pub const CXCursor_ObjCExplicitProtocolImpl: CXCursorKind = 433;
pub const CXCursor_ObjCForCollectionStmt: CXCursorKind = 222;
pub const CXCursor_ObjCImplementationDecl: CXCursorKind = 18;
pub const CXCursor_ObjCIndependentClass: CXCursorKind = 427;
pub const CXCursor_ObjCInstanceMethodDecl: CXCursorKind = 16;
pub const CXCursor_ObjCInterfaceDecl: CXCursorKind = 11;
pub const CXCursor_ObjCIvarDecl: CXCursorKind = 15;
pub const CXCursor_ObjCMessageExpr: CXCursorKind = 104;
pub const CXCursor_ObjCNSObject: CXCursorKind = 426;
pub const CXCursor_ObjCPreciseLifetime: CXCursorKind = 428;
pub const CXCursor_ObjCPropertyDecl: CXCursorKind = 14;
pub const CXCursor_ObjCProtocolDecl: CXCursorKind = 13;
pub const CXCursor_ObjCProtocolExpr: CXCursorKind = 140;
pub const CXCursor_ObjCProtocolRef: CXCursorKind = 41;
pub const CXCursor_ObjCRequiresSuper: CXCursorKind = 430;
pub const CXCursor_ObjCReturnsInnerPointer: CXCursorKind = 429;
pub const CXCursor_ObjCRootClass: CXCursorKind = 431;
pub const CXCursor_ObjCRuntimeVisible: CXCursorKind = 435;
pub const CXCursor_ObjCSelectorExpr: CXCursorKind = 139;
pub const CXCursor_ObjCSelfExpr: CXCursorKind = 146;
pub const CXCursor_ObjCStringLiteral: CXCursorKind = 137;
pub const CXCursor_ObjCSubclassingRestricted: CXCursorKind = 432;
pub const CXCursor_ObjCSuperClassRef: CXCursorKind = 40;
pub const CXCursor_ObjCSynthesizeDecl: CXCursorKind = 37;
pub const CXCursor_OpenACCAtomicConstruct: CXCursorKind = 332;
pub const CXCursor_OpenACCCacheConstruct: CXCursorKind = 333;
pub const CXCursor_OpenACCCombinedConstruct: CXCursorKind = 322;
pub const CXCursor_OpenACCComputeConstruct: CXCursorKind = 320;
pub const CXCursor_OpenACCDataConstruct: CXCursorKind = 323;
pub const CXCursor_OpenACCEnterDataConstruct: CXCursorKind = 324;
pub const CXCursor_OpenACCExitDataConstruct: CXCursorKind = 325;
pub const CXCursor_OpenACCHostDataConstruct: CXCursorKind = 326;
pub const CXCursor_OpenACCInitConstruct: CXCursorKind = 328;
pub const CXCursor_OpenACCLoopConstruct: CXCursorKind = 321;
pub const CXCursor_OpenACCSetConstruct: CXCursorKind = 330;
pub const CXCursor_OpenACCShutdownConstruct: CXCursorKind = 329;
pub const CXCursor_OpenACCUpdateConstruct: CXCursorKind = 331;
pub const CXCursor_OpenACCWaitConstruct: CXCursorKind = 327;
pub const CXCursor_OverloadCandidate: CXCursorKind = 700;
pub const CXCursor_OverloadedDeclRef: CXCursorKind = 49;
pub const CXCursor_PackExpansionExpr: CXCursorKind = 142;
pub const CXCursor_PackIndexingExpr: CXCursorKind = 156;
pub const CXCursor_PackedAttr: CXCursorKind = 408;
pub const CXCursor_ParenExpr: CXCursorKind = 111;
pub const CXCursor_ParmDecl: CXCursorKind = 10;
pub const CXCursor_PreprocessingDirective: CXCursorKind = 500;
pub const CXCursor_PureAttr: CXCursorKind = 409;
pub const CXCursor_RequiresExpr: CXCursorKind = 154;
pub const CXCursor_ReturnStmt: CXCursorKind = 214;
pub const CXCursor_SEHExceptStmt: CXCursorKind = 227;
pub const CXCursor_SEHFinallyStmt: CXCursorKind = 228;
pub const CXCursor_SEHLeaveStmt: CXCursorKind = 247;
pub const CXCursor_SEHTryStmt: CXCursorKind = 226;
pub const CXCursor_SizeOfPackExpr: CXCursorKind = 143;
pub const CXCursor_StaticAssert: CXCursorKind = 602;
pub const CXCursor_StmtExpr: CXCursorKind = 121;
pub const CXCursor_StringLiteral: CXCursorKind = 109;
pub const CXCursor_StructDecl: CXCursorKind = 2;
pub const CXCursor_SwitchStmt: CXCursorKind = 206;
pub const CXCursor_TemplateRef: CXCursorKind = 45;
pub const CXCursor_TemplateTemplateParameter: CXCursorKind = 29;
pub const CXCursor_TemplateTypeParameter: CXCursorKind = 27;
pub const CXCursor_TranslationUnit: CXCursorKind = 350;
pub const CXCursor_TypeAliasDecl: CXCursorKind = 36;
pub const CXCursor_TypeAliasTemplateDecl: CXCursorKind = 601;
pub const CXCursor_TypeRef: CXCursorKind = 43;
pub const CXCursor_TypedefDecl: CXCursorKind = 20;
pub const CXCursor_UnaryExpr: CXCursorKind = 136;
pub const CXCursor_UnaryOperator: CXCursorKind = 112;
pub const CXCursor_UnexposedAttr: CXCursorKind = 400;
pub const CXCursor_UnexposedDecl: CXCursorKind = 1;
pub const CXCursor_UnexposedExpr: CXCursorKind = 100;
pub const CXCursor_UnexposedStmt: CXCursorKind = 200;
pub const CXCursor_UnionDecl: CXCursorKind = 3;
pub const CXCursor_UsingDeclaration: CXCursorKind = 35;
pub const CXCursor_UsingDirective: CXCursorKind = 34;
pub const CXCursor_VarDecl: CXCursorKind = 9;
pub const CXCursor_VariableRef: CXCursorKind = 50;
pub const CXCursor_VisibilityAttr: CXCursorKind = 417;
pub const CXCursor_WarnUnusedAttr: CXCursorKind = 439;
pub const CXCursor_WarnUnusedResultAttr: CXCursorKind = 440;
pub const CXCursor_WhileStmt: CXCursorKind = 207;
pub type CXDiagnostic = *mut core::ffi::c_void;
pub type CXDiagnosticDisplayOptions = i32;
pub type CXDiagnosticSet = *mut core::ffi::c_void;
pub type CXDiagnosticSeverity = i32;
pub const CXDiagnostic_DisplayCategoryId: CXDiagnosticDisplayOptions = 16;
pub const CXDiagnostic_DisplayCategoryName: CXDiagnosticDisplayOptions = 32;
pub const CXDiagnostic_DisplayColumn: CXDiagnosticDisplayOptions = 2;
pub const CXDiagnostic_DisplayOption: CXDiagnosticDisplayOptions = 8;
pub const CXDiagnostic_DisplaySourceLocation: CXDiagnosticDisplayOptions = 1;
pub const CXDiagnostic_DisplaySourceRanges: CXDiagnosticDisplayOptions = 4;
pub const CXDiagnostic_Error: CXDiagnosticSeverity = 3;
pub const CXDiagnostic_Fatal: CXDiagnosticSeverity = 4;
pub const CXDiagnostic_Ignored: CXDiagnosticSeverity = 0;
pub const CXDiagnostic_Note: CXDiagnosticSeverity = 1;
pub const CXDiagnostic_Warning: CXDiagnosticSeverity = 2;
pub type CXErrorCode = i32;
pub const CXError_ASTReadError: CXErrorCode = 4;
pub const CXError_Crashed: CXErrorCode = 2;
pub const CXError_Failure: CXErrorCode = 1;
pub const CXError_InvalidArguments: CXErrorCode = 3;
pub const CXError_Success: CXErrorCode = 0;
pub type CXEvalResult = *mut core::ffi::c_void;
pub type CXEvalResultKind = i32;
pub const CXEval_CFStr: CXEvalResultKind = 5;
pub const CXEval_Float: CXEvalResultKind = 2;
pub const CXEval_Int: CXEvalResultKind = 1;
pub const CXEval_ObjCStrLiteral: CXEvalResultKind = 3;
pub const CXEval_Other: CXEvalResultKind = 6;
pub const CXEval_StrLiteral: CXEvalResultKind = 4;
pub const CXEval_UnExposed: CXEvalResultKind = 0;
pub type CXFieldVisitor =
    Option<unsafe extern "C" fn(c: CXCursor, client_data: CXClientData) -> CXVisitorResult>;
pub type CXFile = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXFileUniqueID {
    pub data: [u64; 3],
}
impl Default for CXFileUniqueID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CXGlobalOptFlags = i32;
pub const CXGlobalOpt_None: CXGlobalOptFlags = 0;
pub const CXGlobalOpt_ThreadBackgroundPriorityForAll: CXGlobalOptFlags = 3;
pub const CXGlobalOpt_ThreadBackgroundPriorityForEditing: CXGlobalOptFlags = 2;
pub const CXGlobalOpt_ThreadBackgroundPriorityForIndexing: CXGlobalOptFlags = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxAttrInfo {
    pub kind: CXIdxAttrKind,
    pub cursor: CXCursor,
    pub loc: CXIdxLoc,
}
pub type CXIdxAttrKind = i32;
pub const CXIdxAttr_IBAction: CXIdxAttrKind = 1;
pub const CXIdxAttr_IBOutlet: CXIdxAttrKind = 2;
pub const CXIdxAttr_IBOutletCollection: CXIdxAttrKind = 3;
pub const CXIdxAttr_Unexposed: CXIdxAttrKind = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxBaseClassInfo {
    pub base: *const CXIdxEntityInfo,
    pub cursor: CXCursor,
    pub loc: CXIdxLoc,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxCXXClassDeclInfo {
    pub declInfo: *const CXIdxDeclInfo,
    pub bases: *const *const CXIdxBaseClassInfo,
    pub numBases: u32,
}
pub type CXIdxClientASTFile = *mut core::ffi::c_void;
pub type CXIdxClientContainer = *mut core::ffi::c_void;
pub type CXIdxClientEntity = *mut core::ffi::c_void;
pub type CXIdxClientFile = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxContainerInfo {
    pub cursor: CXCursor,
}
pub const CXIdxDeclFlag_Skipped: CXIdxDeclInfoFlags = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxDeclInfo {
    pub entityInfo: *const CXIdxEntityInfo,
    pub cursor: CXCursor,
    pub loc: CXIdxLoc,
    pub semanticContainer: *const CXIdxContainerInfo,
    pub lexicalContainer: *const CXIdxContainerInfo,
    pub isRedeclaration: i32,
    pub isDefinition: i32,
    pub isContainer: i32,
    pub declAsContainer: *const CXIdxContainerInfo,
    pub isImplicit: i32,
    pub attributes: *const *const CXIdxAttrInfo,
    pub numAttributes: u32,
    pub flags: u32,
}
pub type CXIdxDeclInfoFlags = i32;
pub type CXIdxEntityCXXTemplateKind = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxEntityInfo {
    pub kind: CXIdxEntityKind,
    pub templateKind: CXIdxEntityCXXTemplateKind,
    pub lang: CXIdxEntityLanguage,
    pub name: *const i8,
    pub USR: *const i8,
    pub cursor: CXCursor,
    pub attributes: *const *const CXIdxAttrInfo,
    pub numAttributes: u32,
}
pub type CXIdxEntityKind = i32;
pub const CXIdxEntityLang_C: CXIdxEntityLanguage = 1;
pub const CXIdxEntityLang_CXX: CXIdxEntityLanguage = 3;
pub const CXIdxEntityLang_None: CXIdxEntityLanguage = 0;
pub const CXIdxEntityLang_ObjC: CXIdxEntityLanguage = 2;
pub const CXIdxEntityLang_Swift: CXIdxEntityLanguage = 4;
pub type CXIdxEntityLanguage = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxEntityRefInfo {
    pub kind: CXIdxEntityRefKind,
    pub cursor: CXCursor,
    pub loc: CXIdxLoc,
    pub referencedEntity: *const CXIdxEntityInfo,
    pub parentEntity: *const CXIdxEntityInfo,
    pub container: *const CXIdxContainerInfo,
    pub role: CXSymbolRole,
}
pub type CXIdxEntityRefKind = i32;
pub const CXIdxEntityRef_Direct: CXIdxEntityRefKind = 1;
pub const CXIdxEntityRef_Implicit: CXIdxEntityRefKind = 2;
pub const CXIdxEntity_CXXClass: CXIdxEntityKind = 16;
pub const CXIdxEntity_CXXConcept: CXIdxEntityKind = 27;
pub const CXIdxEntity_CXXConstructor: CXIdxEntityKind = 22;
pub const CXIdxEntity_CXXConversionFunction: CXIdxEntityKind = 24;
pub const CXIdxEntity_CXXDestructor: CXIdxEntityKind = 23;
pub const CXIdxEntity_CXXInstanceMethod: CXIdxEntityKind = 21;
pub const CXIdxEntity_CXXInterface: CXIdxEntityKind = 26;
pub const CXIdxEntity_CXXNamespace: CXIdxEntityKind = 17;
pub const CXIdxEntity_CXXNamespaceAlias: CXIdxEntityKind = 18;
pub const CXIdxEntity_CXXStaticMethod: CXIdxEntityKind = 20;
pub const CXIdxEntity_CXXStaticVariable: CXIdxEntityKind = 19;
pub const CXIdxEntity_CXXTypeAlias: CXIdxEntityKind = 25;
pub const CXIdxEntity_Enum: CXIdxEntityKind = 13;
pub const CXIdxEntity_EnumConstant: CXIdxEntityKind = 5;
pub const CXIdxEntity_Field: CXIdxEntityKind = 4;
pub const CXIdxEntity_Function: CXIdxEntityKind = 2;
pub const CXIdxEntity_NonTemplate: CXIdxEntityCXXTemplateKind = 0;
pub const CXIdxEntity_ObjCCategory: CXIdxEntityKind = 8;
pub const CXIdxEntity_ObjCClass: CXIdxEntityKind = 6;
pub const CXIdxEntity_ObjCClassMethod: CXIdxEntityKind = 10;
pub const CXIdxEntity_ObjCInstanceMethod: CXIdxEntityKind = 9;
pub const CXIdxEntity_ObjCIvar: CXIdxEntityKind = 12;
pub const CXIdxEntity_ObjCProperty: CXIdxEntityKind = 11;
pub const CXIdxEntity_ObjCProtocol: CXIdxEntityKind = 7;
pub const CXIdxEntity_Struct: CXIdxEntityKind = 14;
pub const CXIdxEntity_Template: CXIdxEntityCXXTemplateKind = 1;
pub const CXIdxEntity_TemplatePartialSpecialization: CXIdxEntityCXXTemplateKind = 2;
pub const CXIdxEntity_TemplateSpecialization: CXIdxEntityCXXTemplateKind = 3;
pub const CXIdxEntity_Typedef: CXIdxEntityKind = 1;
pub const CXIdxEntity_Unexposed: CXIdxEntityKind = 0;
pub const CXIdxEntity_Union: CXIdxEntityKind = 15;
pub const CXIdxEntity_Variable: CXIdxEntityKind = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxIBOutletCollectionAttrInfo {
    pub attrInfo: *const CXIdxAttrInfo,
    pub objcClass: *const CXIdxEntityInfo,
    pub classCursor: CXCursor,
    pub classLoc: CXIdxLoc,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxImportedASTFileInfo {
    pub file: CXFile,
    pub module: CXModule,
    pub loc: CXIdxLoc,
    pub isImplicit: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxIncludedFileInfo {
    pub hashLoc: CXIdxLoc,
    pub filename: *const i8,
    pub file: CXFile,
    pub isImport: i32,
    pub isAngled: i32,
    pub isModuleImport: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXIdxLoc {
    pub ptr_data: [*mut core::ffi::c_void; 2],
    pub int_data: u32,
}
impl Default for CXIdxLoc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxObjCCategoryDeclInfo {
    pub containerInfo: *const CXIdxObjCContainerDeclInfo,
    pub objcClass: *const CXIdxEntityInfo,
    pub classCursor: CXCursor,
    pub classLoc: CXIdxLoc,
    pub protocols: *const CXIdxObjCProtocolRefListInfo,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxObjCContainerDeclInfo {
    pub declInfo: *const CXIdxDeclInfo,
    pub kind: CXIdxObjCContainerKind,
}
pub type CXIdxObjCContainerKind = i32;
pub const CXIdxObjCContainer_ForwardRef: CXIdxObjCContainerKind = 0;
pub const CXIdxObjCContainer_Implementation: CXIdxObjCContainerKind = 2;
pub const CXIdxObjCContainer_Interface: CXIdxObjCContainerKind = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxObjCInterfaceDeclInfo {
    pub containerInfo: *const CXIdxObjCContainerDeclInfo,
    pub superInfo: *const CXIdxBaseClassInfo,
    pub protocols: *const CXIdxObjCProtocolRefListInfo,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxObjCPropertyDeclInfo {
    pub declInfo: *const CXIdxDeclInfo,
    pub getter: *const CXIdxEntityInfo,
    pub setter: *const CXIdxEntityInfo,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxObjCProtocolRefInfo {
    pub protocol: *const CXIdxEntityInfo,
    pub cursor: CXCursor,
    pub loc: CXIdxLoc,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIdxObjCProtocolRefListInfo {
    pub protocols: *const *const CXIdxObjCProtocolRefInfo,
    pub numProtocols: u32,
}
pub type CXInclusionVisitor = Option<
    unsafe extern "C" fn(
        included_file: CXFile,
        inclusion_stack: *mut CXSourceLocation,
        include_len: u32,
        client_data: CXClientData,
    ),
>;
pub type CXIndex = *mut core::ffi::c_void;
pub type CXIndexAction = *mut core::ffi::c_void;
pub type CXIndexOptFlags = i32;
pub const CXIndexOpt_IndexFunctionLocalSymbols: CXIndexOptFlags = 2;
pub const CXIndexOpt_IndexImplicitTemplateInstantiations: CXIndexOptFlags = 4;
pub const CXIndexOpt_None: CXIndexOptFlags = 0;
pub const CXIndexOpt_SkipParsedBodiesInSession: CXIndexOptFlags = 16;
pub const CXIndexOpt_SuppressRedundantRefs: CXIndexOptFlags = 1;
pub const CXIndexOpt_SuppressWarnings: CXIndexOptFlags = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXIndexOptions {
    pub Size: u32,
    pub ThreadBackgroundPriorityForIndexing: u8,
    pub ThreadBackgroundPriorityForEditing: u8,
    pub _bitfield: u32,
    pub PreambleStoragePath: *const i8,
    pub InvocationEmissionPath: *const i8,
}
pub type CXLanguageKind = i32;
pub const CXLanguage_C: CXLanguageKind = 1;
pub const CXLanguage_CPlusPlus: CXLanguageKind = 3;
pub const CXLanguage_Invalid: CXLanguageKind = 0;
pub const CXLanguage_ObjC: CXLanguageKind = 2;
pub type CXLinkageKind = i32;
pub const CXLinkage_External: CXLinkageKind = 4;
pub const CXLinkage_Internal: CXLinkageKind = 2;
pub const CXLinkage_Invalid: CXLinkageKind = 0;
pub const CXLinkage_NoLinkage: CXLinkageKind = 1;
pub const CXLinkage_UniqueExternal: CXLinkageKind = 3;
pub const CXLoadDiag_CannotLoad: CXLoadDiag_Error = 2;
pub type CXLoadDiag_Error = i32;
pub const CXLoadDiag_InvalidFile: CXLoadDiag_Error = 3;
pub const CXLoadDiag_None: CXLoadDiag_Error = 0;
pub const CXLoadDiag_Unknown: CXLoadDiag_Error = 1;
pub type CXModule = *mut core::ffi::c_void;
pub type CXModuleMapDescriptor = *mut CXModuleMapDescriptorImpl;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXModuleMapDescriptorImpl(pub u8);
pub const CXNameRange_WantQualifier: CXNameRefFlags = 1;
pub const CXNameRange_WantSinglePiece: CXNameRefFlags = 4;
pub const CXNameRange_WantTemplateArgs: CXNameRefFlags = 2;
pub type CXNameRefFlags = i32;
pub type CXObjCDeclQualifierKind = i32;
pub const CXObjCDeclQualifier_Bycopy: CXObjCDeclQualifierKind = 8;
pub const CXObjCDeclQualifier_Byref: CXObjCDeclQualifierKind = 16;
pub const CXObjCDeclQualifier_In: CXObjCDeclQualifierKind = 1;
pub const CXObjCDeclQualifier_Inout: CXObjCDeclQualifierKind = 2;
pub const CXObjCDeclQualifier_None: CXObjCDeclQualifierKind = 0;
pub const CXObjCDeclQualifier_Oneway: CXObjCDeclQualifierKind = 32;
pub const CXObjCDeclQualifier_Out: CXObjCDeclQualifierKind = 4;
pub type CXObjCPropertyAttrKind = i32;
pub const CXObjCPropertyAttr_assign: CXObjCPropertyAttrKind = 4;
pub const CXObjCPropertyAttr_atomic: CXObjCPropertyAttrKind = 256;
pub const CXObjCPropertyAttr_class: CXObjCPropertyAttrKind = 4096;
pub const CXObjCPropertyAttr_copy: CXObjCPropertyAttrKind = 32;
pub const CXObjCPropertyAttr_getter: CXObjCPropertyAttrKind = 2;
pub const CXObjCPropertyAttr_noattr: CXObjCPropertyAttrKind = 0;
pub const CXObjCPropertyAttr_nonatomic: CXObjCPropertyAttrKind = 64;
pub const CXObjCPropertyAttr_readonly: CXObjCPropertyAttrKind = 1;
pub const CXObjCPropertyAttr_readwrite: CXObjCPropertyAttrKind = 8;
pub const CXObjCPropertyAttr_retain: CXObjCPropertyAttrKind = 16;
pub const CXObjCPropertyAttr_setter: CXObjCPropertyAttrKind = 128;
pub const CXObjCPropertyAttr_strong: CXObjCPropertyAttrKind = 1024;
pub const CXObjCPropertyAttr_unsafe_unretained: CXObjCPropertyAttrKind = 2048;
pub const CXObjCPropertyAttr_weak: CXObjCPropertyAttrKind = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXPlatformAvailability {
    pub Platform: CXString,
    pub Introduced: CXVersion,
    pub Deprecated: CXVersion,
    pub Obsoleted: CXVersion,
    pub Unavailable: i32,
    pub Message: CXString,
}
pub type CXPrintingPolicy = *mut core::ffi::c_void;
pub type CXPrintingPolicyProperty = i32;
pub const CXPrintingPolicy_Alignof: CXPrintingPolicyProperty = 14;
pub const CXPrintingPolicy_AnonymousTagLocations: CXPrintingPolicyProperty = 8;
pub const CXPrintingPolicy_Bool: CXPrintingPolicyProperty = 12;
pub const CXPrintingPolicy_ConstantArraySizeAsWritten: CXPrintingPolicyProperty = 7;
pub const CXPrintingPolicy_ConstantsAsWritten: CXPrintingPolicyProperty = 23;
pub const CXPrintingPolicy_FullyQualifiedName: CXPrintingPolicyProperty = 25;
pub const CXPrintingPolicy_Half: CXPrintingPolicyProperty = 19;
pub const CXPrintingPolicy_IncludeNewlines: CXPrintingPolicyProperty = 21;
pub const CXPrintingPolicy_IncludeTagDefinition: CXPrintingPolicyProperty = 3;
pub const CXPrintingPolicy_Indentation: CXPrintingPolicyProperty = 0;
pub const CXPrintingPolicy_LastProperty: CXPrintingPolicyProperty = 25;
pub const CXPrintingPolicy_MSVCFormatting: CXPrintingPolicyProperty = 22;
pub const CXPrintingPolicy_MSWChar: CXPrintingPolicyProperty = 20;
pub const CXPrintingPolicy_PolishForDeclaration: CXPrintingPolicyProperty = 18;
pub const CXPrintingPolicy_Restrict: CXPrintingPolicyProperty = 13;
pub const CXPrintingPolicy_SuppressImplicitBase: CXPrintingPolicyProperty = 24;
pub const CXPrintingPolicy_SuppressInitializers: CXPrintingPolicyProperty = 6;
pub const CXPrintingPolicy_SuppressLifetimeQualifiers: CXPrintingPolicyProperty = 10;
pub const CXPrintingPolicy_SuppressScope: CXPrintingPolicyProperty = 4;
pub const CXPrintingPolicy_SuppressSpecifiers: CXPrintingPolicyProperty = 1;
pub const CXPrintingPolicy_SuppressStrongLifetime: CXPrintingPolicyProperty = 9;
pub const CXPrintingPolicy_SuppressTagKeyword: CXPrintingPolicyProperty = 2;
pub const CXPrintingPolicy_SuppressTemplateArgsInCXXConstructors: CXPrintingPolicyProperty = 11;
pub const CXPrintingPolicy_SuppressUnwrittenScope: CXPrintingPolicyProperty = 5;
pub const CXPrintingPolicy_TerseOutput: CXPrintingPolicyProperty = 17;
pub const CXPrintingPolicy_UnderscoreAlignof: CXPrintingPolicyProperty = 15;
pub const CXPrintingPolicy_UseVoidForZeroParams: CXPrintingPolicyProperty = 16;
pub type CXRefQualifierKind = i32;
pub const CXRefQualifier_LValue: CXRefQualifierKind = 1;
pub const CXRefQualifier_None: CXRefQualifierKind = 0;
pub const CXRefQualifier_RValue: CXRefQualifierKind = 2;
pub type CXRemapping = *mut core::ffi::c_void;
pub type CXReparse_Flags = i32;
pub const CXReparse_None: CXReparse_Flags = 0;
pub type CXResult = i32;
pub const CXResult_Invalid: CXResult = 1;
pub const CXResult_Success: CXResult = 0;
pub const CXResult_VisitBreak: CXResult = 2;
pub type CXSaveError = i32;
pub const CXSaveError_InvalidTU: CXSaveError = 3;
pub const CXSaveError_None: CXSaveError = 0;
pub const CXSaveError_TranslationErrors: CXSaveError = 2;
pub const CXSaveError_Unknown: CXSaveError = 1;
pub type CXSaveTranslationUnit_Flags = i32;
pub const CXSaveTranslationUnit_None: CXSaveTranslationUnit_Flags = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXSourceLocation {
    pub ptr_data: [*const core::ffi::c_void; 2],
    pub int_data: u32,
}
impl Default for CXSourceLocation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXSourceRange {
    pub ptr_data: [*const core::ffi::c_void; 2],
    pub begin_int_data: u32,
    pub end_int_data: u32,
}
impl Default for CXSourceRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXSourceRangeList {
    pub count: u32,
    pub ranges: *mut CXSourceRange,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXString {
    pub data: *const core::ffi::c_void,
    pub private_flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXStringSet {
    pub Strings: *mut CXString,
    pub Count: u32,
}
pub type CXSymbolRole = i32;
pub const CXSymbolRole_AddressOf: CXSymbolRole = 128;
pub const CXSymbolRole_Call: CXSymbolRole = 32;
pub const CXSymbolRole_Declaration: CXSymbolRole = 1;
pub const CXSymbolRole_Definition: CXSymbolRole = 2;
pub const CXSymbolRole_Dynamic: CXSymbolRole = 64;
pub const CXSymbolRole_Implicit: CXSymbolRole = 256;
pub const CXSymbolRole_None: CXSymbolRole = 0;
pub const CXSymbolRole_Read: CXSymbolRole = 8;
pub const CXSymbolRole_Reference: CXSymbolRole = 4;
pub const CXSymbolRole_Write: CXSymbolRole = 16;
pub type CXTLSKind = i32;
pub const CXTLS_Dynamic: CXTLSKind = 1;
pub const CXTLS_None: CXTLSKind = 0;
pub const CXTLS_Static: CXTLSKind = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXTUResourceUsage {
    pub data: *mut core::ffi::c_void,
    pub numEntries: u32,
    pub entries: *mut CXTUResourceUsageEntry,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXTUResourceUsageEntry {
    pub kind: CXTUResourceUsageKind,
    pub amount: u32,
}
pub type CXTUResourceUsageKind = i32;
pub const CXTUResourceUsage_AST: CXTUResourceUsageKind = 1;
pub const CXTUResourceUsage_AST_SideTables: CXTUResourceUsageKind = 6;
pub const CXTUResourceUsage_ExternalASTSource_Membuffer_MMap: CXTUResourceUsageKind = 10;
pub const CXTUResourceUsage_ExternalASTSource_Membuffer_Malloc: CXTUResourceUsageKind = 9;
pub const CXTUResourceUsage_First: CXTUResourceUsageKind = 1;
pub const CXTUResourceUsage_GlobalCompletionResults: CXTUResourceUsageKind = 4;
pub const CXTUResourceUsage_Identifiers: CXTUResourceUsageKind = 2;
pub const CXTUResourceUsage_Last: CXTUResourceUsageKind = 14;
pub const CXTUResourceUsage_MEMORY_IN_BYTES_BEGIN: CXTUResourceUsageKind = 1;
pub const CXTUResourceUsage_MEMORY_IN_BYTES_END: CXTUResourceUsageKind = 14;
pub const CXTUResourceUsage_PreprocessingRecord: CXTUResourceUsageKind = 12;
pub const CXTUResourceUsage_Preprocessor: CXTUResourceUsageKind = 11;
pub const CXTUResourceUsage_Preprocessor_HeaderSearch: CXTUResourceUsageKind = 14;
pub const CXTUResourceUsage_Selectors: CXTUResourceUsageKind = 3;
pub const CXTUResourceUsage_SourceManagerContentCache: CXTUResourceUsageKind = 5;
pub const CXTUResourceUsage_SourceManager_DataStructures: CXTUResourceUsageKind = 13;
pub const CXTUResourceUsage_SourceManager_Membuffer_MMap: CXTUResourceUsageKind = 8;
pub const CXTUResourceUsage_SourceManager_Membuffer_Malloc: CXTUResourceUsageKind = 7;
pub type CXTargetInfo = *mut CXTargetInfoImpl;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXTargetInfoImpl(pub u8);
pub type CXTemplateArgumentKind = i32;
pub const CXTemplateArgumentKind_Declaration: CXTemplateArgumentKind = 2;
pub const CXTemplateArgumentKind_Expression: CXTemplateArgumentKind = 7;
pub const CXTemplateArgumentKind_Integral: CXTemplateArgumentKind = 4;
pub const CXTemplateArgumentKind_Invalid: CXTemplateArgumentKind = 9;
pub const CXTemplateArgumentKind_Null: CXTemplateArgumentKind = 0;
pub const CXTemplateArgumentKind_NullPtr: CXTemplateArgumentKind = 3;
pub const CXTemplateArgumentKind_Pack: CXTemplateArgumentKind = 8;
pub const CXTemplateArgumentKind_Template: CXTemplateArgumentKind = 5;
pub const CXTemplateArgumentKind_TemplateExpansion: CXTemplateArgumentKind = 6;
pub const CXTemplateArgumentKind_Type: CXTemplateArgumentKind = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXToken {
    pub int_data: [u32; 4],
    pub ptr_data: *mut core::ffi::c_void,
}
impl Default for CXToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CXTokenKind = i32;
pub const CXToken_Comment: CXTokenKind = 4;
pub const CXToken_Identifier: CXTokenKind = 2;
pub const CXToken_Keyword: CXTokenKind = 1;
pub const CXToken_Literal: CXTokenKind = 3;
pub const CXToken_Punctuation: CXTokenKind = 0;
pub type CXTranslationUnit = *mut CXTranslationUnitImpl;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXTranslationUnitImpl(pub u8);
pub const CXTranslationUnit_CXXChainedPCH: CXTranslationUnit_Flags = 32;
pub const CXTranslationUnit_CacheCompletionResults: CXTranslationUnit_Flags = 8;
pub const CXTranslationUnit_CreatePreambleOnFirstParse: CXTranslationUnit_Flags = 256;
pub const CXTranslationUnit_DetailedPreprocessingRecord: CXTranslationUnit_Flags = 1;
pub type CXTranslationUnit_Flags = i32;
pub const CXTranslationUnit_ForSerialization: CXTranslationUnit_Flags = 16;
pub const CXTranslationUnit_IgnoreNonErrorsFromIncludedFiles: CXTranslationUnit_Flags = 16384;
pub const CXTranslationUnit_IncludeAttributedTypes: CXTranslationUnit_Flags = 4096;
pub const CXTranslationUnit_IncludeBriefCommentsInCodeCompletion: CXTranslationUnit_Flags = 128;
pub const CXTranslationUnit_Incomplete: CXTranslationUnit_Flags = 2;
pub const CXTranslationUnit_KeepGoing: CXTranslationUnit_Flags = 512;
pub const CXTranslationUnit_LimitSkipFunctionBodiesToPreamble: CXTranslationUnit_Flags = 2048;
pub const CXTranslationUnit_None: CXTranslationUnit_Flags = 0;
pub const CXTranslationUnit_PrecompiledPreamble: CXTranslationUnit_Flags = 4;
pub const CXTranslationUnit_RetainExcludedConditionalBlocks: CXTranslationUnit_Flags = 32768;
pub const CXTranslationUnit_SingleFileParse: CXTranslationUnit_Flags = 1024;
pub const CXTranslationUnit_SkipFunctionBodies: CXTranslationUnit_Flags = 64;
pub const CXTranslationUnit_VisitImplicitAttributes: CXTranslationUnit_Flags = 8192;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXType {
    pub kind: CXTypeKind,
    pub data: [*mut core::ffi::c_void; 2],
}
impl Default for CXType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CXTypeKind = i32;
pub type CXTypeLayoutError = i32;
pub const CXTypeLayoutError_Dependent: CXTypeLayoutError = -3;
pub const CXTypeLayoutError_Incomplete: CXTypeLayoutError = -2;
pub const CXTypeLayoutError_Invalid: CXTypeLayoutError = -1;
pub const CXTypeLayoutError_InvalidFieldName: CXTypeLayoutError = -5;
pub const CXTypeLayoutError_NotConstantSize: CXTypeLayoutError = -4;
pub const CXTypeLayoutError_Undeduced: CXTypeLayoutError = -6;
pub type CXTypeNullabilityKind = i32;
pub const CXTypeNullability_Invalid: CXTypeNullabilityKind = 3;
pub const CXTypeNullability_NonNull: CXTypeNullabilityKind = 0;
pub const CXTypeNullability_Nullable: CXTypeNullabilityKind = 1;
pub const CXTypeNullability_NullableResult: CXTypeNullabilityKind = 4;
pub const CXTypeNullability_Unspecified: CXTypeNullabilityKind = 2;
pub const CXType_Accum: CXTypeKind = 34;
pub const CXType_Atomic: CXTypeKind = 177;
pub const CXType_Attributed: CXTypeKind = 163;
pub const CXType_Auto: CXTypeKind = 118;
pub const CXType_BFloat16: CXTypeKind = 39;
pub const CXType_BTFTagAttributed: CXTypeKind = 178;
pub const CXType_BlockPointer: CXTypeKind = 102;
pub const CXType_Bool: CXTypeKind = 3;
pub const CXType_Char16: CXTypeKind = 6;
pub const CXType_Char32: CXTypeKind = 7;
pub const CXType_Char_S: CXTypeKind = 13;
pub const CXType_Char_U: CXTypeKind = 4;
pub const CXType_Complex: CXTypeKind = 100;
pub const CXType_ConstantArray: CXTypeKind = 112;
pub const CXType_Dependent: CXTypeKind = 26;
pub const CXType_DependentSizedArray: CXTypeKind = 116;
pub const CXType_Double: CXTypeKind = 22;
pub const CXType_Elaborated: CXTypeKind = 119;
pub const CXType_Enum: CXTypeKind = 106;
pub const CXType_ExtVector: CXTypeKind = 176;
pub const CXType_FirstBuiltin: CXTypeKind = 2;
pub const CXType_Float: CXTypeKind = 21;
pub const CXType_Float128: CXTypeKind = 30;
pub const CXType_Float16: CXTypeKind = 32;
pub const CXType_FunctionNoProto: CXTypeKind = 110;
pub const CXType_FunctionProto: CXTypeKind = 111;
pub const CXType_HLSLAttributedResource: CXTypeKind = 180;
pub const CXType_HLSLInlineSpirv: CXTypeKind = 181;
pub const CXType_HLSLResource: CXTypeKind = 179;
pub const CXType_Half: CXTypeKind = 31;
pub const CXType_Ibm128: CXTypeKind = 40;
pub const CXType_IncompleteArray: CXTypeKind = 114;
pub const CXType_Int: CXTypeKind = 17;
pub const CXType_Int128: CXTypeKind = 20;
pub const CXType_Invalid: CXTypeKind = 0;
pub const CXType_LValueReference: CXTypeKind = 103;
pub const CXType_LastBuiltin: CXTypeKind = 40;
pub const CXType_Long: CXTypeKind = 18;
pub const CXType_LongAccum: CXTypeKind = 35;
pub const CXType_LongDouble: CXTypeKind = 23;
pub const CXType_LongLong: CXTypeKind = 19;
pub const CXType_MemberPointer: CXTypeKind = 117;
pub const CXType_NullPtr: CXTypeKind = 24;
pub const CXType_OCLEvent: CXTypeKind = 158;
pub const CXType_OCLImage1dArrayRO: CXTypeKind = 122;
pub const CXType_OCLImage1dArrayRW: CXTypeKind = 146;
pub const CXType_OCLImage1dArrayWO: CXTypeKind = 134;
pub const CXType_OCLImage1dBufferRO: CXTypeKind = 123;
pub const CXType_OCLImage1dBufferRW: CXTypeKind = 147;
pub const CXType_OCLImage1dBufferWO: CXTypeKind = 135;
pub const CXType_OCLImage1dRO: CXTypeKind = 121;
pub const CXType_OCLImage1dRW: CXTypeKind = 145;
pub const CXType_OCLImage1dWO: CXTypeKind = 133;
pub const CXType_OCLImage2dArrayDepthRO: CXTypeKind = 127;
pub const CXType_OCLImage2dArrayDepthRW: CXTypeKind = 151;
pub const CXType_OCLImage2dArrayDepthWO: CXTypeKind = 139;
pub const CXType_OCLImage2dArrayMSAADepthRO: CXTypeKind = 131;
pub const CXType_OCLImage2dArrayMSAADepthRW: CXTypeKind = 155;
pub const CXType_OCLImage2dArrayMSAADepthWO: CXTypeKind = 143;
pub const CXType_OCLImage2dArrayMSAARO: CXTypeKind = 129;
pub const CXType_OCLImage2dArrayMSAARW: CXTypeKind = 153;
pub const CXType_OCLImage2dArrayMSAAWO: CXTypeKind = 141;
pub const CXType_OCLImage2dArrayRO: CXTypeKind = 125;
pub const CXType_OCLImage2dArrayRW: CXTypeKind = 149;
pub const CXType_OCLImage2dArrayWO: CXTypeKind = 137;
pub const CXType_OCLImage2dDepthRO: CXTypeKind = 126;
pub const CXType_OCLImage2dDepthRW: CXTypeKind = 150;
pub const CXType_OCLImage2dDepthWO: CXTypeKind = 138;
pub const CXType_OCLImage2dMSAADepthRO: CXTypeKind = 130;
pub const CXType_OCLImage2dMSAADepthRW: CXTypeKind = 154;
pub const CXType_OCLImage2dMSAADepthWO: CXTypeKind = 142;
pub const CXType_OCLImage2dMSAARO: CXTypeKind = 128;
pub const CXType_OCLImage2dMSAARW: CXTypeKind = 152;
pub const CXType_OCLImage2dMSAAWO: CXTypeKind = 140;
pub const CXType_OCLImage2dRO: CXTypeKind = 124;
pub const CXType_OCLImage2dRW: CXTypeKind = 148;
pub const CXType_OCLImage2dWO: CXTypeKind = 136;
pub const CXType_OCLImage3dRO: CXTypeKind = 132;
pub const CXType_OCLImage3dRW: CXTypeKind = 156;
pub const CXType_OCLImage3dWO: CXTypeKind = 144;
pub const CXType_OCLIntelSubgroupAVCImeDualRefStreamin: CXTypeKind = 175;
pub const CXType_OCLIntelSubgroupAVCImeDualReferenceStreamin: CXTypeKind = 175;
pub const CXType_OCLIntelSubgroupAVCImePayload: CXTypeKind = 165;
pub const CXType_OCLIntelSubgroupAVCImeResult: CXTypeKind = 169;
pub const CXType_OCLIntelSubgroupAVCImeResultDualRefStreamout: CXTypeKind = 173;
pub const CXType_OCLIntelSubgroupAVCImeResultDualReferenceStreamout: CXTypeKind = 173;
pub const CXType_OCLIntelSubgroupAVCImeResultSingleRefStreamout: CXTypeKind = 172;
pub const CXType_OCLIntelSubgroupAVCImeResultSingleReferenceStreamout: CXTypeKind = 172;
pub const CXType_OCLIntelSubgroupAVCImeSingleRefStreamin: CXTypeKind = 174;
pub const CXType_OCLIntelSubgroupAVCImeSingleReferenceStreamin: CXTypeKind = 174;
pub const CXType_OCLIntelSubgroupAVCMcePayload: CXTypeKind = 164;
pub const CXType_OCLIntelSubgroupAVCMceResult: CXTypeKind = 168;
pub const CXType_OCLIntelSubgroupAVCRefPayload: CXTypeKind = 166;
pub const CXType_OCLIntelSubgroupAVCRefResult: CXTypeKind = 170;
pub const CXType_OCLIntelSubgroupAVCSicPayload: CXTypeKind = 167;
pub const CXType_OCLIntelSubgroupAVCSicResult: CXTypeKind = 171;
pub const CXType_OCLQueue: CXTypeKind = 159;
pub const CXType_OCLReserveID: CXTypeKind = 160;
pub const CXType_OCLSampler: CXTypeKind = 157;
pub const CXType_ObjCClass: CXTypeKind = 28;
pub const CXType_ObjCId: CXTypeKind = 27;
pub const CXType_ObjCInterface: CXTypeKind = 108;
pub const CXType_ObjCObject: CXTypeKind = 161;
pub const CXType_ObjCObjectPointer: CXTypeKind = 109;
pub const CXType_ObjCSel: CXTypeKind = 29;
pub const CXType_ObjCTypeParam: CXTypeKind = 162;
pub const CXType_Overload: CXTypeKind = 25;
pub const CXType_Pipe: CXTypeKind = 120;
pub const CXType_Pointer: CXTypeKind = 101;
pub const CXType_RValueReference: CXTypeKind = 104;
pub const CXType_Record: CXTypeKind = 105;
pub const CXType_SChar: CXTypeKind = 14;
pub const CXType_Short: CXTypeKind = 16;
pub const CXType_ShortAccum: CXTypeKind = 33;
pub const CXType_Typedef: CXTypeKind = 107;
pub const CXType_UAccum: CXTypeKind = 37;
pub const CXType_UChar: CXTypeKind = 5;
pub const CXType_UInt: CXTypeKind = 9;
pub const CXType_UInt128: CXTypeKind = 12;
pub const CXType_ULong: CXTypeKind = 10;
pub const CXType_ULongAccum: CXTypeKind = 38;
pub const CXType_ULongLong: CXTypeKind = 11;
pub const CXType_UShort: CXTypeKind = 8;
pub const CXType_UShortAccum: CXTypeKind = 36;
pub const CXType_Unexposed: CXTypeKind = 1;
pub const CXType_VariableArray: CXTypeKind = 115;
pub const CXType_Vector: CXTypeKind = 113;
pub const CXType_Void: CXTypeKind = 2;
pub const CXType_WChar: CXTypeKind = 15;
pub type CXUnaryOperatorKind = i32;
pub const CXUnaryOperator_AddrOf: CXUnaryOperatorKind = 5;
pub const CXUnaryOperator_Coawait: CXUnaryOperatorKind = 14;
pub const CXUnaryOperator_Deref: CXUnaryOperatorKind = 6;
pub const CXUnaryOperator_Extension: CXUnaryOperatorKind = 13;
pub const CXUnaryOperator_Imag: CXUnaryOperatorKind = 12;
pub const CXUnaryOperator_Invalid: CXUnaryOperatorKind = 0;
pub const CXUnaryOperator_LNot: CXUnaryOperatorKind = 10;
pub const CXUnaryOperator_Minus: CXUnaryOperatorKind = 8;
pub const CXUnaryOperator_Not: CXUnaryOperatorKind = 9;
pub const CXUnaryOperator_Plus: CXUnaryOperatorKind = 7;
pub const CXUnaryOperator_PostDec: CXUnaryOperatorKind = 2;
pub const CXUnaryOperator_PostInc: CXUnaryOperatorKind = 1;
pub const CXUnaryOperator_PreDec: CXUnaryOperatorKind = 4;
pub const CXUnaryOperator_PreInc: CXUnaryOperatorKind = 3;
pub const CXUnaryOperator_Real: CXUnaryOperatorKind = 11;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXUnsavedFile {
    pub Filename: *const i8,
    pub Contents: *const i8,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXVersion {
    pub Major: i32,
    pub Minor: i32,
    pub Subminor: i32,
}
pub type CXVirtualFileOverlay = *mut CXVirtualFileOverlayImpl;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXVirtualFileOverlayImpl(pub u8);
pub type CXVisibilityKind = i32;
pub const CXVisibility_Default: CXVisibilityKind = 3;
pub const CXVisibility_Hidden: CXVisibilityKind = 1;
pub const CXVisibility_Invalid: CXVisibilityKind = 0;
pub const CXVisibility_Protected: CXVisibilityKind = 2;
pub const CXVisit_Break: CXVisitorResult = 0;
pub const CXVisit_Continue: CXVisitorResult = 1;
pub type CXVisitorResult = i32;
pub const CX_BO_Add: CX_BinaryOperatorKind = 6;
pub const CX_BO_AddAssign: CX_BinaryOperatorKind = 26;
pub const CX_BO_And: CX_BinaryOperatorKind = 17;
pub const CX_BO_AndAssign: CX_BinaryOperatorKind = 30;
pub const CX_BO_Assign: CX_BinaryOperatorKind = 22;
pub const CX_BO_Cmp: CX_BinaryOperatorKind = 10;
pub const CX_BO_Comma: CX_BinaryOperatorKind = 33;
pub const CX_BO_Div: CX_BinaryOperatorKind = 4;
pub const CX_BO_DivAssign: CX_BinaryOperatorKind = 24;
pub const CX_BO_EQ: CX_BinaryOperatorKind = 15;
pub const CX_BO_GE: CX_BinaryOperatorKind = 14;
pub const CX_BO_GT: CX_BinaryOperatorKind = 12;
pub const CX_BO_Invalid: CX_BinaryOperatorKind = 0;
pub const CX_BO_LAST: CX_BinaryOperatorKind = 33;
pub const CX_BO_LAnd: CX_BinaryOperatorKind = 20;
pub const CX_BO_LE: CX_BinaryOperatorKind = 13;
pub const CX_BO_LOr: CX_BinaryOperatorKind = 21;
pub const CX_BO_LT: CX_BinaryOperatorKind = 11;
pub const CX_BO_Mul: CX_BinaryOperatorKind = 3;
pub const CX_BO_MulAssign: CX_BinaryOperatorKind = 23;
pub const CX_BO_NE: CX_BinaryOperatorKind = 16;
pub const CX_BO_Or: CX_BinaryOperatorKind = 19;
pub const CX_BO_OrAssign: CX_BinaryOperatorKind = 32;
pub const CX_BO_PtrMemD: CX_BinaryOperatorKind = 1;
pub const CX_BO_PtrMemI: CX_BinaryOperatorKind = 2;
pub const CX_BO_Rem: CX_BinaryOperatorKind = 5;
pub const CX_BO_RemAssign: CX_BinaryOperatorKind = 25;
pub const CX_BO_Shl: CX_BinaryOperatorKind = 8;
pub const CX_BO_ShlAssign: CX_BinaryOperatorKind = 28;
pub const CX_BO_Shr: CX_BinaryOperatorKind = 9;
pub const CX_BO_ShrAssign: CX_BinaryOperatorKind = 29;
pub const CX_BO_Sub: CX_BinaryOperatorKind = 7;
pub const CX_BO_SubAssign: CX_BinaryOperatorKind = 27;
pub const CX_BO_Xor: CX_BinaryOperatorKind = 18;
pub const CX_BO_XorAssign: CX_BinaryOperatorKind = 31;
pub type CX_BinaryOperatorKind = i32;
pub type CX_CXXAccessSpecifier = i32;
pub const CX_CXXInvalidAccessSpecifier: CX_CXXAccessSpecifier = 0;
pub const CX_CXXPrivate: CX_CXXAccessSpecifier = 3;
pub const CX_CXXProtected: CX_CXXAccessSpecifier = 2;
pub const CX_CXXPublic: CX_CXXAccessSpecifier = 1;
pub const CX_SC_Auto: CX_StorageClass = 6;
pub const CX_SC_Extern: CX_StorageClass = 2;
pub const CX_SC_Invalid: CX_StorageClass = 0;
pub const CX_SC_None: CX_StorageClass = 1;
pub const CX_SC_OpenCLWorkGroupLocal: CX_StorageClass = 5;
pub const CX_SC_PrivateExtern: CX_StorageClass = 4;
pub const CX_SC_Register: CX_StorageClass = 7;
pub const CX_SC_Static: CX_StorageClass = 3;
pub type CX_StorageClass = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IndexerCallbacks {
    pub abortQuery: Option<unsafe extern "C" fn(CXClientData, *mut core::ffi::c_void) -> i32>,
    pub diagnostic:
        Option<unsafe extern "C" fn(CXClientData, CXDiagnosticSet, *mut core::ffi::c_void)>,
    pub enteredMainFile: Option<
        unsafe extern "C" fn(CXClientData, CXFile, *mut core::ffi::c_void) -> CXIdxClientFile,
    >,
    pub ppIncludedFile:
        Option<unsafe extern "C" fn(CXClientData, *const CXIdxIncludedFileInfo) -> CXIdxClientFile>,
    pub importedASTFile: Option<
        unsafe extern "C" fn(CXClientData, *const CXIdxImportedASTFileInfo) -> CXIdxClientASTFile,
    >,
    pub startedTranslationUnit:
        Option<unsafe extern "C" fn(CXClientData, *mut core::ffi::c_void) -> CXIdxClientContainer>,
    pub indexDeclaration: Option<unsafe extern "C" fn(CXClientData, *const CXIdxDeclInfo)>,
    pub indexEntityReference: Option<unsafe extern "C" fn(CXClientData, *const CXIdxEntityRefInfo)>,
}
pub const WindowsClangAlignmentDeclspec: WindowsClangAlignmentKind = 2;
pub const WindowsClangAlignmentGNU: WindowsClangAlignmentKind = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WindowsClangAlignmentInfo {
    pub bits: u32,
    pub kind: WindowsClangAlignmentKind,
}
pub type WindowsClangAlignmentKind = i32;
pub const WindowsClangAlignmentStandard: WindowsClangAlignmentKind = 1;
pub const WindowsClangArmSMEAttributes: WindowsClangFunctionProperties = 4;
pub const WindowsClangCFIUncheckedCallee: WindowsClangFunctionProperties = 64;
pub const WindowsClangCLanguageLinkage: WindowsClangLanguageLinkage = 1;
pub const WindowsClangCXXLanguageLinkage: WindowsClangLanguageLinkage = 2;
pub const WindowsClangConst: WindowsClangTypeQualifiers = 1;
pub const WindowsClangConventionDefault: WindowsClangConventionOrigin = 1;
pub const WindowsClangConventionExplicit: WindowsClangConventionOrigin = 2;
pub type WindowsClangConventionOrigin = i32;
pub const WindowsClangConventionUnknown: WindowsClangConventionOrigin = 0;
pub const WindowsClangDependent: WindowsClangStatus = 3;
pub const WindowsClangExtendedParameters: WindowsClangFunctionProperties = 1;
pub const WindowsClangExtraAttributes: WindowsClangFunctionProperties = 2;
pub const WindowsClangFunctionEffects: WindowsClangFunctionProperties = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WindowsClangFunctionInfo {
    pub has_regparm: u32,
    pub regparm: u32,
    pub no_return: u32,
    pub produces_result: u32,
    pub no_caller_saved_registers: u32,
    pub no_cf_check: u32,
    pub cmse_nonsecure_call: u32,
    pub additional_properties: u32,
}
pub type WindowsClangFunctionProperties = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WindowsClangImportConvention {
    pub origin: WindowsClangConventionOrigin,
    pub stdcall_type: CXType,
    pub stdcall_symbol: CXString,
}
pub const WindowsClangInvalidArgument: WindowsClangStatus = 1;
pub type WindowsClangLanguageLinkage = i32;
pub const WindowsClangMacroEmpty: WindowsClangMacroKind = 2;
pub const WindowsClangMacroFunction: WindowsClangMacroKind = 5;
pub const WindowsClangMacroFunctionLike: WindowsClangMacroKind = 1;
pub type WindowsClangMacroKind = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WindowsClangMacroProbe {
    pub kind: WindowsClangMacroKind,
    pub file: CXString,
    pub line: u32,
    pub column: u32,
    pub diagnostics: CXString,
    pub target: CXString,
}
pub const WindowsClangMacroRejected: WindowsClangMacroKind = 3;
pub const WindowsClangMacroUndefined: WindowsClangMacroKind = 0;
pub const WindowsClangMacroValue: WindowsClangMacroKind = 4;
pub const WindowsClangMethodQualifiers: WindowsClangFunctionProperties = 16;
pub const WindowsClangNativePointer: WindowsClangPointerRepresentation = 0;
pub const WindowsClangNoLanguageLinkage: WindowsClangLanguageLinkage = 0;
pub const WindowsClangOtherPointer: WindowsClangPointerRepresentation = 4;
pub const WindowsClangOtherQualifiers: WindowsClangTypeQualifiers = 16;
pub const WindowsClangParseFailed: WindowsClangStatus = 4;
pub const WindowsClangPointer64: WindowsClangPointerRepresentation = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WindowsClangPointerInfo {
    pub representation: WindowsClangPointerRepresentation,
    pub pointee: CXType,
}
pub type WindowsClangPointerRepresentation = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WindowsClangRecordInfo {
    pub can_pass_in_registers: u32,
    pub empty: u32,
    pub non_trivial_copy_assignment: u32,
    pub deleted_copy_assignment: u32,
    pub non_trivial_destructor: u32,
    pub user_provided_constructor: u32,
    pub constructor_template: u32,
}
pub const WindowsClangRefQualifier: WindowsClangFunctionProperties = 32;
pub const WindowsClangRestrict: WindowsClangTypeQualifiers = 4;
pub const WindowsClangSignedPointer32: WindowsClangPointerRepresentation = 2;
pub type WindowsClangStatus = i32;
pub const WindowsClangSuccess: WindowsClangStatus = 0;
pub type WindowsClangTypeQualifiers = i32;
pub const WindowsClangUnaligned: WindowsClangTypeQualifiers = 8;
pub const WindowsClangUnsignedPointer32: WindowsClangPointerRepresentation = 3;
pub const WindowsClangVolatile: WindowsClangTypeQualifiers = 2;
pub const WindowsClangWrongKind: WindowsClangStatus = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _CXChildVisitResult(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _CXCursorAndRangeVisitorBlock(pub u8);
pub type __time64_t = i64;
pub type size_t = usize;
pub type time_t = i64;
