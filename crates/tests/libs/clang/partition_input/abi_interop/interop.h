// In-scope interop header (the `UserConsentVerifierInterop.h` analogue). It pulls in the
// out-of-scope WinRT projection header — which drags an `ABI::Windows::*` projected type
// into the translation unit — but the interop API it declares references only plain COM
// out-pointer shapes, never a projected type.
#include "../abi_projection/proj.h"

void InteropCall(void** result);
