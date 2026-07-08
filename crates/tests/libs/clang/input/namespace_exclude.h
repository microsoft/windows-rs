//! library test.dll
//! reference namespace_exclude_ref

// `OwnedHandle` is in the reference under the *target* namespace (Test), so the
// scraper must re-emit it from source rather than dedup it away. `OtherHandle`
// is in the reference under a *different* namespace (Other), so it must resolve
// to a qualified cross-namespace reference and not be re-emitted locally.
typedef void *OwnedHandle;
typedef void *OtherHandle;

void UseHandles(OwnedHandle a, OtherHandle b);
