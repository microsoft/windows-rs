#define LAST_ERROR __attribute__((annotate("Win32Metadata.SetLastError")))
#define PRESERVE __attribute__((annotate("Win32Metadata.PreserveResult")))
#define ASSOCIATED __attribute__((annotate("Win32Metadata.AssociatedEnum", "Win32Metadata.EnumName:WAIT_EVENT")))
#define IMPORT __attribute__((annotate("Win32Metadata.ImportLibrary", "fixture.dll")))

typedef enum WAIT_EVENT { WAIT_READY = 0, WAIT_TIMEOUT = 258 } WAIT_EVENT;

extern "C" {
#ifndef REVERSE
LAST_ERROR ASSOCIATED int Wait(int first);
LAST_ERROR PRESERVE IMPORT int Wait(int second);
#else
LAST_ERROR PRESERVE IMPORT int Wait(int second);
LAST_ERROR ASSOCIATED int Wait(int first);
#endif
int Wait(int third);
}
