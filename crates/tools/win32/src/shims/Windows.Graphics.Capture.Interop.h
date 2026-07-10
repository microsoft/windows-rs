// Scrape shim for `Windows.Graphics.Capture.Interop.h`.
//
// The real SDK header `#include`s the C++/WinRT projection headers
// `windows.ui.composition.h` and `windows.graphics.capture.h`, which do not
// parse under this scrape's C++ definition mode (a self-conflicting typedef in
// `Windows.Devices.Sensors.h`). The interop interfaces themselves only depend on
// `HWND`/`HMONITOR`/`REFIID`/`IUnknown` (from `windows.h`, already in the
// prelude), so this shim declares them without the heavy projection includes.
// It shadows the SDK header via a `-I` search path ahead of the SDK `-isystem`
// directories.

#pragma once

#undef INTERFACE
#define INTERFACE IGraphicsCaptureItemInterop
DECLARE_INTERFACE_IID_(IGraphicsCaptureItemInterop, IUnknown, "3628E81B-3CAC-4C60-B7F4-23CE0E0C3356")
{
    IFACEMETHOD(CreateForWindow)(
        HWND window,
        REFIID riid,
        _COM_Outptr_ void ** result
        ) PURE;

    IFACEMETHOD(CreateForMonitor)(
        HMONITOR monitor,
        REFIID riid,
        _COM_Outptr_ void ** result
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IWindowGraphicsCaptureItemInterop
DECLARE_INTERFACE_IID_(IWindowGraphicsCaptureItemInterop, IUnknown, "38E4C48B-94E6-4C44-9CFA-968193316C0C")
{
    IFACEMETHOD(GetWindow)(
        HWND* window
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IMonitorGraphicsCaptureItemInterop
DECLARE_INTERFACE_IID_(IMonitorGraphicsCaptureItemInterop, IUnknown, "33274D14-A076-4048-8416-747E9B04DB7B")
{
    IFACEMETHOD(GetMonitor)(
        HMONITOR* monitor
        ) PURE;
};
