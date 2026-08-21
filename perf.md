also # Understanding the 26x no-op update result

## Summary

The measured 26x gap is real, but it does not mean ordinary `windows-reactor-next` interaction is
26x slower than the incumbent. It compares an almost-free incumbent no-op shortcut with a
reactor-next full-root publication path that still clones and traverses a candidate tree.

The benchmark submits a new, identical 512-node root through `Pump::update_view` on every
iteration:

| Implementation | Time | Allocations |
| --- | ---: | ---: |
| Incumbent `windows-reactor` | About 7.5 us | 0 |
| `windows-reactor-next` | About 197 us | About 3,620 |

The ratio is large because the incumbent denominator is exceptionally small. The reactor-next
absolute cost is about 0.2 ms for 512 nodes, but the allocation count confirms that this is real
work rather than measurement noise.

## What the benchmark measures

The no-op row repeatedly calls the full update API with two independently constructed but equal
views. Reactor-next:

1. clones the current structural tree through copy-on-write storage;
2. walks and reconciles the submitted view;
3. validates the complete candidate;
4. determines that no native commands are required;
5. publishes the equivalent candidate.

The incumbent can memoize this case and return with almost no work. Reactor-next intentionally
retains candidate validation and a single publication boundary for full updates, even when the
desired output proves identical.

The benchmark uses `RecordingRuntime`. It measures Rust-side view construction, planning,
allocation, validation, and publication. It does not include WinUI control creation, COM calls,
layout, rendering, or presentation.

## What normal component interaction measures

Normal reactor-next applications mount a root component and then deliver typed messages to dirty
component scopes. They do not ordinarily submit a complete replacement root for every input event.

The local component path is much closer to the incumbent:

| Workload | Incumbent | Reactor next | Ratio |
| --- | ---: | ---: | ---: |
| Local component message | 593 ns | 700 ns | 1.18x |
| Local message bytes | 457 | 430 | 0.94x |

The local result remains flat from 512 through 16,384 unrelated component scopes. This is the more
representative number for a button click, field edit, task completion, or other message that
changes one well-bounded component.

Reactor-next specifically maintains a local component fast path so an isolated message does not
clone the full application tree.

## When the full-update cost maps to real applications

The no-op benchmark is deliberately adversarial, but it represents a real application pattern:
a large component or root is asked to render repeatedly while producing no structural or property
change.

The cost is unlikely to matter for:

- forms with tens or hundreds of controls;
- event-driven applications with infrequent updates;
- applications divided into components around independently changing state;
- ordinary validation, submission, navigation, and background-task completion.

It may matter for:

- one large component that owns thousands of descendants;
- timer-, pointer-, or animation-driven messages that repeatedly render a broad subtree;
- parent updates that unnecessarily recompose many unchanged descendants;
- multiple large dirty component scopes in one host turn;
- battery-sensitive applications where transient allocation volume matters;
- workloads combining Rust reconciliation with expensive WinUI layout and rendering.

At 512 nodes, approximately 197 us is small compared with a 16.7 ms frame budget. At 4,096 rows,
broad reconciliation currently measures about 4-5 ms depending on the keyed operation. A similar
cost at 60 Hz would consume a meaningful fraction of a frame before WinUI performs layout,
rendering, or composition.

## Why the relative and absolute results tell different stories

The 26x ratio answers:

> How much more Rust work does reactor-next perform than an incumbent path optimized to do almost
> nothing?

It does not answer:

> Will a reactor-next application feel 26x slower?

For perceived responsiveness, the absolute cost, update frequency, affected subtree size, native
work, and frame budget are more useful. A 0.2 ms operation can have a dramatic relative regression
while remaining invisible in an ordinary UI turn. Conversely, repeatedly paying several
milliseconds during scrolling or animation can become visible even if each individual result
passes a broad benchmark limit.

The allocations deserve attention separately. Thousands of transient allocations per redundant
update can increase allocator pressure and power consumption even when median latency remains
acceptable.

## Current performance picture

The current evidence is mixed but reasonable for the prototype:

| Measure | Result |
| --- | --- |
| Local component message | 1.18x incumbent time and fewer bytes |
| Local scaling | Flat through 16,384 unrelated scopes |
| Retained component memory | 0.70x incumbent |
| Clean library check | 0.53x incumbent |
| Source-only library check | 0.42x incumbent |
| Change all 512 leaves | 2.50x incumbent, about 410 us |
| Reverse 512 keyed leaves | 2.65x incumbent, about 278 us |
| Rotate 512 keyed leaves | 3.86x incumbent, about 271 us |
| Broad 4,096-row reconciliation | About 3.9-4.7 ms |
| Identical full 512-node update | About 26x incumbent, about 197 us |

The primary interactive path is competitive. Broad and redundant reconciliation is the clear
performance watch.

## Why this does not justify another architecture

The extra work follows from useful guarantees:

- one authoritative logical tree;
- complete candidate validation before native mutation;
- one transactional publication boundary;
- no partially published native or logical state;
- no second mutable tree or rollback system.

Removing those guarantees to win a synthetic no-op benchmark would repeat the complexity problems
the new architecture was created to avoid.

The recent performance commit already demonstrated the safer optimization direction. Two
copy-on-write guards reduced unchanged or mostly unchanged update time by about 5-8% and transient
bytes by about 6%, while keyed reordering with real structural changes remained unchanged. The
optimization avoided unnecessary cloning without changing ownership or publication.

## What should happen next

The result should remain visible rather than being dismissed or used as evidence for an immediate
redesign.

The integrated virtual task/editor sample should measure:

1. local edits in one realized row;
2. broad parent updates with mostly unchanged rows;
3. a deliberately redundant no-op message;
4. sustained scrolling and recycling;
5. controlled input, focus, effects, contexts, and background completions;
6. Rust planning time separately from WinUI layout and rendering;
7. allocation volume and frame-time percentiles, not only best-case microbenchmark time.

The important thresholds are whether real turns approach the frame budget, whether allocations
cause visible or power-related pressure, and whether performance remains local as the application
grows.

If the integrated sample exposes a problem, profile these areas first:

- repeated key and view collection;
- avoidable full-subtree reconciliation;
- copy-on-write mutation granularity;
- unchanged child-vector and property cloning;
- component boundaries around independently changing state;
- redundant messages or parent recomposition.

Any optimization should preserve the one-tree ownership model and transactional publication. A
second mutable tree, partial publication, or general rollback machinery is not justified by the
current measurements.

## Conclusion

The 26x figure reveals a genuine weakness in redundant full-root reconciliation, and it should be
reported explicitly. It is not representative of the normal local component path and does not by
itself predict poor application UX.

At the current measured scale, the absolute cost is acceptable for ordinary event-driven UI work.
It becomes a practical concern when broad unchanged subtrees are recomposed frequently, especially
at animation or scrolling rates. The integrated sample is the right place to decide whether this
remains an isolated benchmark tax or becomes an application-level bottleneck.
