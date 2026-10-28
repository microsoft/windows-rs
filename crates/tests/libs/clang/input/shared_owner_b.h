#pragma once
#ifdef CONFLICT
typedef signed int Count;
#else
typedef unsigned int Count;
#endif
struct SharedRecord { Count value; };
enum SharedState { Off, On };
typedef Count (*SharedCallback)(Count value);
#if !defined(_M_IX86)
typedef unsigned int PlatformCount;
#endif
