#include <intsafe.h>
#include <windows.h>
#include <xaudio2.h>

static_assert(sizeof(IXAudio2Extension) == sizeof(void*));
