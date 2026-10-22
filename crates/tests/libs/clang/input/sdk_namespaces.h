#include <windows.h>
#include <memorybuffer.h>
#include <robuffer.h>
#include <windows.storage.streams.h>
#include <windows.system.h>

struct NamespaceConsumer {
    Windows::Foundation::IMemoryBufferByteAccess* memory;
    Windows::Storage::Streams::IBufferByteAccess* access;
    ABI::Windows::Storage::Streams::IBuffer* buffer;
    ABI::Windows::System::IDispatcherQueueHandler* handler;
};
