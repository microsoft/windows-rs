//---------------------------------------------------------------------------
// Stub replacing the SDK's `winrt\windows.ui.composition.h` C++/WinRT ABI
// projection header (~39k lines) during the `tool_win32` scrape.
//
// The real projection header drags in the full WinRT projection closure, which
// does not parse in the definition-mode scrape (a transitive typedef/enum
// conflict, e.g. `winrt\Windows.Devices.Sensors.h`'s `MagnetometerAccuracy`,
// aborts the whole translation unit). The only reason the interop headers
// `#include` it is to name a handful of `ABI::Windows::UI::Composition::*`
// interfaces as opaque pointer parameters; forward-declaring those here lets the
// interop interfaces scrape while the resolution winmd (`RESOLUTION_WINMDS`)
// maps each name to its real `Windows.winmd` projection (a cross-winmd
// reference), exactly as it does for `roregistrationapi.h`.
//---------------------------------------------------------------------------
#pragma once

#ifndef BUILD_WINDOWS
namespace ABI {
#endif
namespace Windows {
namespace UI {
namespace Composition {

typedef interface ICompositionSurface ICompositionSurface;
typedef interface ICompositionGraphicsDevice ICompositionGraphicsDevice;
typedef interface ICompositionCapabilities ICompositionCapabilities;

} // namespace Composition
} // namespace UI
} // namespace Windows
#ifndef BUILD_WINDOWS
} // namespace ABI
#endif
