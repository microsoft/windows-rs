#include "reference_types.h"
typedef HRESULT LocalResult;
typedef CountAlias LocalCount;
typedef Count* LocalPointer;
typedef Completion LocalCompletion;
const HRESULT LOCAL_FAILURE = (HRESULT)0x80004005;
struct Consumer {
    HRESULT result;
    CountAlias count;
    Text text;
    NativeSize size;
    State state;
    Payload payload;
    VARIANT variant;
    Completion completion;
    LocalPointer pointer;
    LocalCount local_count;
    Envelope::Inner nested;
};
extern "C" HRESULT Consume(Payload* payload, Completion completion);
