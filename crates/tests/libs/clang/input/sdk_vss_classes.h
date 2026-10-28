#include <intsafe.h>
#include <winsock2.h>
#include <windows.h>
#include <vss.h>
#include <vswriter.h>
#include <vsbackup.h>

static_assert(sizeof(IVssExpressWriter) == sizeof(void*));
static_assert(sizeof(IVssCreateExpressWriterMetadata) == sizeof(void*));
static_assert(sizeof(IVssExamineWriterMetadata) == sizeof(void*));
