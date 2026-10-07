#include <ntifs.h>
#include <wdm.h>

typedef decltype(((IO_STACK_LOCATION*)nullptr)->Parameters.DeviceIoControl) DeviceIoControl;
typedef decltype(((IO_STACK_LOCATION*)nullptr)->Parameters.QuerySecurity) QuerySecurity;

extern "C" unsigned long WdkLayout(unsigned long index);
extern "C" void WdkMutate(DeviceIoControl* device, QuerySecurity* security);
