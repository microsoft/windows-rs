pub trait IDxcAssemblerImpl: Sized {
    fn AssembleToContainer();
}
pub trait IDxcBlobImpl: Sized {
    fn GetBufferPointer();
    fn GetBufferSize();
}
pub trait IDxcBlobEncodingImpl: Sized + IDxcBlobImpl {
    fn GetEncoding();
}
pub trait IDxcBlobUtf16Impl: Sized + IDxcBlobEncodingImpl + IDxcBlobImpl {
    fn GetStringPointer();
    fn GetStringLength();
}
pub trait IDxcBlobUtf8Impl: Sized + IDxcBlobEncodingImpl + IDxcBlobImpl {
    fn GetStringPointer();
    fn GetStringLength();
}
pub trait IDxcCompilerImpl: Sized {
    fn Compile();
    fn Preprocess();
    fn Disassemble();
}
pub trait IDxcCompiler2Impl: Sized + IDxcCompilerImpl {
    fn CompileWithDebug();
}
pub trait IDxcCompiler3Impl: Sized {
    fn Compile();
    fn Disassemble();
}
pub trait IDxcCompilerArgsImpl: Sized {
    fn GetArguments();
    fn GetCount();
    fn AddArguments();
    fn AddArgumentsUTF8();
    fn AddDefines();
}
pub trait IDxcContainerBuilderImpl: Sized {
    fn Load();
    fn AddPart();
    fn RemovePart();
    fn SerializeContainer();
}
pub trait IDxcContainerReflectionImpl: Sized {
    fn Load();
    fn GetPartCount();
    fn GetPartKind();
    fn GetPartContent();
    fn FindFirstPartKind();
    fn GetPartReflection();
}
pub trait IDxcExtraOutputsImpl: Sized {
    fn GetOutputCount();
    fn GetOutput();
}
pub trait IDxcIncludeHandlerImpl: Sized {
    fn LoadSource();
}
pub trait IDxcLibraryImpl: Sized {
    fn SetMalloc();
    fn CreateBlobFromBlob();
    fn CreateBlobFromFile();
    fn CreateBlobWithEncodingFromPinned();
    fn CreateBlobWithEncodingOnHeapCopy();
    fn CreateBlobWithEncodingOnMalloc();
    fn CreateIncludeHandler();
    fn CreateStreamFromBlobReadOnly();
    fn GetBlobAsUtf8();
    fn GetBlobAsUtf16();
}
pub trait IDxcLinkerImpl: Sized {
    fn RegisterLibrary();
    fn Link();
}
pub trait IDxcOperationResultImpl: Sized {
    fn GetStatus();
    fn GetResult();
    fn GetErrorBuffer();
}
pub trait IDxcOptimizerImpl: Sized {
    fn GetAvailablePassCount();
    fn GetAvailablePass();
    fn RunOptimizer();
}
pub trait IDxcOptimizerPassImpl: Sized {
    fn GetOptionName();
    fn GetDescription();
    fn GetOptionArgCount();
    fn GetOptionArgName();
    fn GetOptionArgDescription();
}
pub trait IDxcPdbUtilsImpl: Sized {
    fn Load();
    fn GetSourceCount();
    fn GetSource();
    fn GetSourceName();
    fn GetFlagCount();
    fn GetFlag();
    fn GetArgCount();
    fn GetArg();
    fn GetArgPairCount();
    fn GetArgPair();
    fn GetDefineCount();
    fn GetDefine();
    fn GetTargetProfile();
    fn GetEntryPoint();
    fn GetMainFileName();
    fn GetHash();
    fn GetName();
    fn IsFullPDB();
    fn GetFullPDB();
    fn GetVersionInfo();
    fn SetCompiler();
    fn CompileForFullPDB();
    fn OverrideArgs();
    fn OverrideRootSignature();
}
pub trait IDxcResultImpl: Sized + IDxcOperationResultImpl {
    fn HasOutput();
    fn GetOutput();
    fn GetNumOutputs();
    fn GetOutputByIndex();
    fn PrimaryOutput();
}
pub trait IDxcUtilsImpl: Sized {
    fn CreateBlobFromBlob();
    fn CreateBlobFromPinned();
    fn MoveToBlob();
    fn CreateBlob();
    fn LoadFile();
    fn CreateReadOnlyStreamFromBlob();
    fn CreateDefaultIncludeHandler();
    fn GetBlobAsUtf8();
    fn GetBlobAsUtf16();
    fn GetDxilContainerPart();
    fn CreateReflection();
    fn BuildArguments();
    fn GetPDBContents();
}
pub trait IDxcValidatorImpl: Sized {
    fn Validate();
}
pub trait IDxcValidator2Impl: Sized + IDxcValidatorImpl {
    fn ValidateWithDebug();
}
pub trait IDxcVersionInfoImpl: Sized {
    fn GetVersion();
    fn GetFlags();
}
pub trait IDxcVersionInfo2Impl: Sized + IDxcVersionInfoImpl {
    fn GetCommitInfo();
}
pub trait IDxcVersionInfo3Impl: Sized {
    fn GetCustomVersionString();
}
