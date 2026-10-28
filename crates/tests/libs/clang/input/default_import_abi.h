typedef int (*ImportCallback)(int);

struct ImportRequest {
    unsigned int count;
    void* data;
};

#ifndef IMPORT_CONVENTION
#define IMPORT_CONVENTION
#endif

extern "C" int IMPORT_CONVENTION FetchImport(ImportRequest* request, ImportCallback callback);
extern "C" int __cdecl FixedImport(int value);
