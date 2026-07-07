// Out-of-scope "C-runtime" header (the `ucrt` analogue): it lives outside the declared
// scope, so its declarations are emitted only when in-scope code references them.
//   * `APITYPE` is referenced by the in-scope `ApiCall`, so it is a genuine cross-over
//     type and must survive the reachability sweep.
//   * `CRTNOISE` and `CrtOnly` are referenced by nothing in scope, so they are swept.
typedef int APITYPE;

typedef int CRTNOISE;

void CrtOnly(CRTNOISE x);
