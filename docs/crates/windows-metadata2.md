# windows-metadata2

`windows-metadata2` is an unpublished replacement candidate for
[`windows-metadata`](windows-metadata.md). It is developed beside the existing implementation so
each layer can be compared against the current Windows metadata corpus before consumers migrate.

The first layer owns and structurally validates a PE/CLI metadata image. Successful construction
guarantees that the metadata root, stream directory, table header, row counts, table widths, and
table byte ranges are in bounds. The complete ECMA-335 table schema is declared once and drives
row-width calculation.

Current exclusions are intentional:

- semantic row and heap access;
- signature and custom-attribute decoding;
- metadata construction and serialization;
- common, Win32, and WinRT validation;
- architecture merging and namespace remapping;
- `windows-bindgen` and `windows-rdl` integration.

These capabilities will be added in independent changes with differential tests against the
existing implementation. The crate remains unpublished until both `windows-bindgen` and
`windows-rdl` can use it without changing generated output.

## Design constraints

- Each layer must establish a small invariant that later layers can test.
- Invalid external input must return a structured error rather than panic.
- The image owns its bytes; row identities must not require leaked or self-referential storage.
- ECMA table identities, columns, and coded-index relationships must have one declaration.
- Raw blobs remain available for lossless copying; semantic decoding is a separate layer.
- Windows validation, architecture merging, namespace remapping, and `Apis` projection stay
  outside the ECMA storage layer.
- Consumer migration must use differential output tests rather than assumed compatibility.

## Implementation plan

| Step | Status | Acceptance criteria |
| --- | --- | --- |
| PE/CLI image and table layout | Done | WinRT and Win32 images parse; malformed ranges fail without panics. |
| Typed row IDs and checked heaps | Done | One table declaration generates IDs and layout; string/blob/GUID access is bounded. |
| Raw row and coded-index access | Done | Every column kind decodes through checked table metadata. |
| Signatures and attribute blobs | Next | One fallible decoder handles valid encodings and reports byte offsets. |
| Multi-image database and indexes | Planned | Owned file IDs and row IDs replace leaked indexes and borrowed identities. |
| `windows-bindgen` reader adapter | Planned | Full generated Rust output matches the existing reader. |
| Deterministic metadata builder | Planned | Finalization returns a queryable image and stable row remapping. |
| `windows-rdl` builder adapter | Planned | WinRT, Win32, and WDK output remains equivalent. |
| Common and Windows validation | Planned | Existing validation corpus passes through explicit profiles. |
| Merge and namespace remap | Planned | Transformations use one lossless copier outside the core image. |
| Replace `windows-metadata` | Planned | Both consumers and generation pipelines have migrated. |

## Progress notes

The initial parser revealed two assumptions during review. Optional-header data directories must be
bounded by the declared optional-header size, and table row counts must be bounded by the declared
table stream rather than adjacent stream bytes. Both checks are now explicit. Table stream
finalization also rejects more than three bytes of zero padding or any nonzero trailing data, which
checks the declared ECMA schema against real WinRT and Win32 images.

The next review found that the first schema implementation still repeated table identities in the
enum, conversion array, and schema list. One table declaration now generates the table identifiers,
typed marker types, and column schemas.

`RowId<T>` uses the ECMA one-based row number and is explicitly local to one image. It contains no
borrowed image reference, so a later multi-image database can combine it with an owned `FileId`
rather than leak an index. `Image` checks every string, blob, and GUID column during construction.
The committed WinRT and Win32 images match the existing reader for every non-nested type identity.

Heap work exposed another container-boundary issue: checking only the containing file allowed a
malformed CLI or metadata directory to borrow bytes from adjacent section data. CLI headers,
metadata roots, stream directories, and streams are now bounded by both their declared directory
sizes and the raw PE section. The string and blob heaps must begin with their empty entry, GUID
heaps cannot end with a partial GUID, and blob lengths must use canonical compressed integers.

Raw row views retain typed table-local identities while validating the requested column kind.
Direct indexes, list starts, and coded indexes are separate column kinds. This matters because a
list start may use the target table's one-past-end sentinel while an ordinary table index may not.
Coded-index declarations record explicit tags rather than relying on target order; this preserves
the sparse tags 2 and 3 used by `CustomAttributeType`.

Complete type shapes now match the existing reader across the committed WinRT and Win32 images.
The comparison includes duplicate architecture variants, field and method lists, and base-type
coded indexes. Nonempty pointer tables and edit-and-continue tables are rejected explicitly.
Pointer tables change list interpretation and will remain unsupported until that indirection has a
tested model.
