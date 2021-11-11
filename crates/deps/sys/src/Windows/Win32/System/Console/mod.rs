#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConsoleAliasA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConsoleAliasW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocConsole();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AttachConsole();
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn ClosePseudoConsole();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateConsoleScreenBuffer();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePseudoConsole();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpungeConsoleCommandHistoryA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpungeConsoleCommandHistoryW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputAttribute();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputCharacterA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputCharacterW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushConsoleInputBuffer();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeConsole();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateConsoleCtrlEvent();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasExesA();
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleAliasExesLengthA();
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleAliasExesLengthW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasExesW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesLengthA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesLengthW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesW();
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleCP();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryLengthA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryLengthW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCursorInfo();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleDisplayMode();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleFontSize();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleHistoryInfo();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleMode();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleOriginalTitleA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleOriginalTitleW();
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleOutputCP();
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleProcessList();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleScreenBufferInfo();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleScreenBufferInfoEx();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleSelectionInfo();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleTitleA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleTitleW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleWindow();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentConsoleFont();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentConsoleFontEx();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLargestConsoleWindowSize();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfConsoleInputEvents();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfConsoleMouseButtons();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStdHandle();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekConsoleInputA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekConsoleInputW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleInputA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleInputW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputAttribute();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputCharacterA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputCharacterW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleW();
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn ResizePseudoConsole();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollConsoleScreenBufferA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollConsoleScreenBufferW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleActiveScreenBuffer();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCP();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCtrlHandler();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCursorInfo();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCursorPosition();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleDisplayMode();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleHistoryInfo();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleMode();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleNumberOfCommandsA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleNumberOfCommandsW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleOutputCP();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleScreenBufferInfoEx();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleScreenBufferSize();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTextAttribute();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTitleA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTitleW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleWindowInfo();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentConsoleFontEx();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStdHandle();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStdHandleEx();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleInputA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleInputW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputAttribute();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputCharacterA();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputCharacterW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputW();
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleW();
}
