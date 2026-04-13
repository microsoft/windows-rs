/* Minimal COM interface that inherits from IUnknown.
 *
 * IUnknown is provided by the system-include shim and is also present in the
 * Windows.Win32.winmd reference metadata.  The converter should suppress the
 * IUnknown definition from the RDL output and fully-qualify the base class as
 * Windows::Win32::System::Com::IUnknown.
 */
#include <unknwn.h>

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) IWidget : public IUnknown {
    virtual long GetId(int* id) = 0;
    virtual long SetName(const char* name) = 0;
};
