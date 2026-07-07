// Regression: a "poison" macro must not swallow valid constants that follow it
// in the batch macro evaluator.
//
// `POISON_EXPANDS_UNBALANCED` has a *raw* replacement list that is balanced (a
// bare identifier), so it passes the `tokens_balanced` pre-filter and becomes an
// eval candidate — but it *expands* to an unbalanced `{`. In the synthetic
// evaluation TU this opens a brace inside `enum { __rdl_eval_NAME = (NAME); }`
// that consumes the enum declarations of every following candidate, which would
// silently drop them. The evaluator detects the swallow (the victims' probe
// enums are absent from the AST) and re-evaluates them in isolation, so the
// constants defined *after* the poison must still be recovered. The poison
// itself, and the bare `{` macro it expands, are dropped.
#define OPEN_BRACE {
#define POISON_EXPANDS_UNBALANCED OPEN_BRACE

// Valid integer constants defined after the poison. They reference another macro
// (like the real `commctrl.h` `(WM_USER + N)` message ids that regressed), so the
// token parser cannot fold them and they reach the batch evaluator — exactly the
// batch the poison can swallow.
#define POISON_BASE 100
#define AFTER_POISON_A (POISON_BASE + 1)
#define AFTER_POISON_B (POISON_BASE + 2)
#define AFTER_POISON_C (POISON_BASE + 3)
