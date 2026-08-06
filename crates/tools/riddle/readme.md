# Riddle

Riddle checks, compiles, and formats RDL API descriptions.

```text
riddle check api.rdl
riddle build api.rdl --out api.winmd
riddle fmt api.rdl
riddle fmt --check api.rdl
```

Inputs may be files, directories containing `.rdl` files, or `-` for standard input. Use
`--reference <path>` for additional `.winmd` references. The default Windows metadata is available
for type and attribute resolution unless `--no-default` is specified.
