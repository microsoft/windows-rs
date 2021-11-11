#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AddStroke();
    fn AddWordsToWordList();
    fn AdviseInkChange();
    fn CreateContext();
    fn CreateRecognizer();
    fn DestroyContext();
    fn DestroyRecognizer();
    fn DestroyWordList();
    fn EndInkInput();
    fn GetAllRecognizers();
    fn GetBestResultString();
    fn GetLatticePtr();
    fn GetLeftSeparator();
    fn GetRecoAttributes();
    fn GetResultPropertyList();
    fn GetRightSeparator();
    fn GetUnicodeRanges();
    fn IsStringSupported();
    fn LoadCachedAttributes();
    fn MakeWordList();
    fn Process();
    fn SetEnabledUnicodeRanges();
    fn SetFactoid();
    fn SetFlags();
    fn SetGuide();
    fn SetTextContext();
    fn SetWordList();
}
