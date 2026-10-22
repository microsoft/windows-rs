#pragma once
typedef unsigned int Count;
struct SharedRecord { Count value; };
enum SharedState { Off, On };
typedef Count (*SharedCallback)(Count value);
#if defined(_M_IX86)
typedef unsigned int PlatformCount;
#endif
