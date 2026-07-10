// A header whose own file name is dotted (mirrors the WinRT interop headers, e.g.
// `Windows.Devices.Display.Core.Interop.h`). Its defining-header partition must collapse
// to a single flat leaf (`dottednameinterop`), never a nested `Dotted::Name::Interop`
// module tree — the flat Win32 surface ignores header namespacing.
void DottedThing(void);
