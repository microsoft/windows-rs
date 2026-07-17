//---------------------------------------------------------------------------
// Stub replacing the SDK's `winrt\windows.graphics.capture.h` C++/WinRT ABI
// projection header during the `tool_win32` scrape (see the companion
// `windows.ui.composition.h` stub for the rationale).
//
// `Windows.Graphics.Capture.Interop.h`'s interop interfaces name no projected
// type — every method takes `HWND`/`HMONITOR`/`REFIID`/`void**` — so this stub
// is intentionally empty; it exists only to satisfy the `#include` without
// pulling in the unparseable projection closure.
//---------------------------------------------------------------------------
#pragma once
