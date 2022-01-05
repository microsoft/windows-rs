pub trait IComprehensiveSpellCheckProviderImpl: Sized {
    fn ComprehensiveCheck();
}
pub trait IEnumCodePageImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumRfc1766Impl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumScriptImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumSpellingErrorImpl: Sized {
    fn Next();
}
pub trait IMLangCodePagesImpl: Sized {
    fn GetCharCodePages();
    fn GetStrCodePages();
    fn CodePageToCodePages();
    fn CodePagesToCodePage();
}
pub trait IMLangConvertCharsetImpl: Sized {
    fn Initialize();
    fn GetSourceCodePage();
    fn GetDestinationCodePage();
    fn GetProperty();
    fn DoConversion();
    fn DoConversionToUnicode();
    fn DoConversionFromUnicode();
}
pub trait IMLangFontLinkImpl: Sized + IMLangCodePagesImpl {
    fn GetFontCodePages();
    fn MapFont();
    fn ReleaseFont();
    fn ResetFontMapping();
}
pub trait IMLangFontLink2Impl: Sized + IMLangCodePagesImpl {
    fn GetFontCodePages();
    fn ReleaseFont();
    fn ResetFontMapping();
    fn MapFont();
    fn GetFontUnicodeRanges();
    fn GetScriptFontInfo();
    fn CodePageToScriptID();
}
pub trait IMLangLineBreakConsoleImpl: Sized {
    fn BreakLineML();
    fn BreakLineW();
    fn BreakLineA();
}
pub trait IMLangStringImpl: Sized {
    fn Sync();
    fn GetLength();
    fn SetMLStr();
    fn GetMLStr();
}
pub trait IMLangStringAStrImpl: Sized + IMLangStringImpl {
    fn SetAStr();
    fn SetStrBufA();
    fn GetAStr();
    fn GetStrBufA();
    fn LockAStr();
    fn UnlockAStr();
    fn SetLocale();
    fn GetLocale();
}
pub trait IMLangStringBufAImpl: Sized {
    fn GetStatus();
    fn LockBuf();
    fn UnlockBuf();
    fn Insert();
    fn Delete();
}
pub trait IMLangStringBufWImpl: Sized {
    fn GetStatus();
    fn LockBuf();
    fn UnlockBuf();
    fn Insert();
    fn Delete();
}
pub trait IMLangStringWStrImpl: Sized + IMLangStringImpl {
    fn SetWStr();
    fn SetStrBufW();
    fn GetWStr();
    fn GetStrBufW();
    fn LockWStr();
    fn UnlockWStr();
    fn SetLocale();
    fn GetLocale();
}
pub trait IMultiLanguageImpl: Sized {
    fn GetNumberOfCodePageInfo();
    fn GetCodePageInfo();
    fn GetFamilyCodePage();
    fn EnumCodePages();
    fn GetCharsetInfo();
    fn IsConvertible();
    fn ConvertString();
    fn ConvertStringToUnicode();
    fn ConvertStringFromUnicode();
    fn ConvertStringReset();
    fn GetRfc1766FromLcid();
    fn GetLcidFromRfc1766();
    fn EnumRfc1766();
    fn GetRfc1766Info();
    fn CreateConvertCharset();
}
pub trait IMultiLanguage2Impl: Sized {
    fn GetNumberOfCodePageInfo();
    fn GetCodePageInfo();
    fn GetFamilyCodePage();
    fn EnumCodePages();
    fn GetCharsetInfo();
    fn IsConvertible();
    fn ConvertString();
    fn ConvertStringToUnicode();
    fn ConvertStringFromUnicode();
    fn ConvertStringReset();
    fn GetRfc1766FromLcid();
    fn GetLcidFromRfc1766();
    fn EnumRfc1766();
    fn GetRfc1766Info();
    fn CreateConvertCharset();
    fn ConvertStringInIStream();
    fn ConvertStringToUnicodeEx();
    fn ConvertStringFromUnicodeEx();
    fn DetectCodepageInIStream();
    fn DetectInputCodepage();
    fn ValidateCodePage();
    fn GetCodePageDescription();
    fn IsCodePageInstallable();
    fn SetMimeDBSource();
    fn GetNumberOfScripts();
    fn EnumScripts();
    fn ValidateCodePageEx();
}
pub trait IMultiLanguage3Impl: Sized + IMultiLanguage2Impl {
    fn DetectOutboundCodePage();
    fn DetectOutboundCodePageInIStream();
}
pub trait IOptionDescriptionImpl: Sized {
    fn Id();
    fn Heading();
    fn Description();
    fn Labels();
}
pub trait ISpellCheckProviderImpl: Sized {
    fn LanguageTag();
    fn Check();
    fn Suggest();
    fn GetOptionValue();
    fn SetOptionValue();
    fn OptionIds();
    fn Id();
    fn LocalizedName();
    fn GetOptionDescription();
    fn InitializeWordlist();
}
pub trait ISpellCheckProviderFactoryImpl: Sized {
    fn SupportedLanguages();
    fn IsSupported();
    fn CreateSpellCheckProvider();
}
pub trait ISpellCheckerImpl: Sized {
    fn LanguageTag();
    fn Check();
    fn Suggest();
    fn Add();
    fn Ignore();
    fn AutoCorrect();
    fn GetOptionValue();
    fn OptionIds();
    fn Id();
    fn LocalizedName();
    fn SpellCheckerChanged();
    fn RemoveSpellCheckerChanged();
    fn GetOptionDescription();
    fn ComprehensiveCheck();
}
pub trait ISpellChecker2Impl: Sized + ISpellCheckerImpl {
    fn Remove();
}
pub trait ISpellCheckerChangedEventHandlerImpl: Sized {
    fn Invoke();
}
pub trait ISpellCheckerFactoryImpl: Sized {
    fn SupportedLanguages();
    fn IsSupported();
    fn CreateSpellChecker();
}
pub trait ISpellingErrorImpl: Sized {
    fn StartIndex();
    fn Length();
    fn CorrectiveAction();
    fn Replacement();
}
pub trait IUserDictionariesRegistrarImpl: Sized {
    fn RegisterUserDictionary();
    fn UnregisterUserDictionary();
}
